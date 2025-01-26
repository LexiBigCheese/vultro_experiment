use ctru_sys::*;
use std::alloc::Allocator;

use super::{
    GpuCmdByMut,
    chain::{Chain, Chainable, ChainableNext},
    mask,
};

pub trait TexEnv: Sized + Default {
    const BASE: u32;
    fn base(self) -> u32 {
        Self::BASE
    }
}

macro_rules! texenv_n {
    ($name:ident,$base:expr) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
        pub struct $name;

        impl TexEnv for $name {
            const BASE: u32 = $base;
        }
    };
}

texenv_n!(E0, ctru_sys::GPUREG_TEXENV0_SOURCE);
texenv_n!(E1, ctru_sys::GPUREG_TEXENV1_SOURCE);
texenv_n!(E2, ctru_sys::GPUREG_TEXENV2_SOURCE);
texenv_n!(E3, ctru_sys::GPUREG_TEXENV3_SOURCE);
texenv_n!(E4, ctru_sys::GPUREG_TEXENV4_SOURCE);
texenv_n!(E5, ctru_sys::GPUREG_TEXENV5_SOURCE);

// E0 * Source = impl Chainable + ChainableNext<Next = TEC<E0,Operand>>

///https://www.3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_TEXENVi_SOURCE
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Source {
    PrimaryColor,
    FragmentPrimaryColor,
    FragmentSecondaryColor,
    Texture0,
    Texture1,
    Texture2,
    Texture3,
    ///Always returns Zero
    PreviousBuffer = 13,
    ///From `Color`
    Constant = 14,
    ///Using Previous (15) as a source in the first TEV stage returns the value of source 3. If source 3 has Previous it returns zero.
    Previous = 15,
}

///Successor: Operand
pub fn source_both<TE: TexEnv>(a: Source, b: Source, c: Source) -> SourceSplit<TE> {
    SourceSplit {
        rgb: (a, b, c),
        alpha: (a, b, c),
        te: Default::default(),
    }
}

//Successor: Operand
#[derive(Clone, Copy)]
pub struct SourceSplit<TE: TexEnv> {
    pub rgb: (Source, Source, Source),
    pub alpha: (Source, Source, Source),
    pub te: TE,
}

impl<TE: TexEnv> Chainable for SourceSplit<TE> {
    fn reg(&self) -> u32 {
        TE::BASE + 0
    }
    fn param(self) -> u32 {
        (self.rgb.0 as u32)
            | ((self.rgb.1 as u32) << 4)
            | ((self.rgb.2 as u32) << 8)
            | ((self.alpha.0 as u32) << 16)
            | ((self.alpha.1 as u32) << 20)
            | ((self.alpha.2 as u32) << 24)
    }
}

impl<TE: TexEnv> ChainableNext for SourceSplit<TE> {
    type Next = Operand<TE>;
}

///https://www.3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_TEXENVi_OPERAND
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorOp {
    SourceColor = 0,
    OneMinusSourceColor = 1,
    SourceAlpha = 2,
    OneMinusSourceAlpha = 3,
    SourceRed = 4,
    OneMinusSourceRed = 5,
    SourceGreen = 8,
    OneMinusSourceGreen = 9,
    SourceBlue = 12,
    OneMinusSourceBlue = 13,
}

///https://www.3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_TEXENVi_OPERAND
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AlphaOp {
    SourceAlpha = 0,
    OneMinusSourceAlpha = 1,
    SourceRed = 2,
    OneMinusSourceRed = 3,
    SourceGreen = 4,
    OneMinusSourceGreen = 5,
    SourceBlue = 6,
    OneMinusSourceBlue = 7,
}

///Successor = `CombinerSplit` or `combiner_both`
#[derive(Clone, Copy)]
pub struct Operand<TE: TexEnv> {
    pub rgb: (ColorOp, ColorOp, ColorOp),
    pub alpha: (AlphaOp, AlphaOp, AlphaOp),
    pub te: TE,
}

impl<TE: TexEnv> Chainable for Operand<TE> {
    fn reg(&self) -> u32 {
        TE::BASE + 1
    }
    fn param(self) -> u32 {
        (self.rgb.0 as u32)
            | ((self.rgb.1 as u32) << 4)
            | ((self.rgb.2 as u32) << 8)
            | ((self.alpha.0 as u32) << 12)
            | ((self.alpha.1 as u32) << 16)
            | ((self.alpha.2 as u32) << 20)
    }
}

impl<TE: TexEnv> ChainableNext for Operand<TE> {
    type Next = CombinerSplit<TE>;
}

///https://www.3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_TEXENVi_COMBINER
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CombineMode {
    Replace,
    Modulate,
    Add,
    AddSigned,
    Interpolate,
    Subtract,
    Dot3RGB,
    Dot3RGBA,
    MultiplyThenAdd,
    AddThenMultiply,
}

///Successor = `Color`
#[derive(Clone, Copy)]
pub struct CombinerSplit<TE: TexEnv> {
    pub rgb: CombineMode,
    pub alpha: CombineMode,
    pub te: TE,
}

impl<TE: TexEnv> Chainable for CombinerSplit<TE> {
    fn reg(&self) -> u32 {
        TE::BASE + 2
    }
    fn param(self) -> u32 {
        (self.rgb as u32) | ((self.alpha as u32) << 16)
    }
}

impl<TE: TexEnv> ChainableNext for CombinerSplit<TE> {
    type Next = Color<TE>;
}

///Successor = `Color`
pub fn combiner_both<TE: TexEnv>(m: CombineMode) -> CombinerSplit<TE> {
    CombinerSplit {
        rgb: m,
        alpha: m,
        te: Default::default(),
    }
}

///Note: `u32::from_le_bytes([r,g,b,a])`
///Successor = `scale_both` or `ScaleSplit`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color<TE: TexEnv>(pub u32, pub TE);

impl<TE: TexEnv> Chainable for Color<TE> {
    fn reg(&self) -> u32 {
        TE::BASE + 3
    }
    fn param(self) -> u32 {
        self.0
    }
}

impl<TE: TexEnv> ChainableNext for Color<TE> {
    type Next = ScaleSplit<TE>;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Scale {
    X1,
    X2,
    X4,
}

pub fn scale_both<TE: TexEnv>(s: Scale) -> ScaleSplit<TE> {
    ScaleSplit {
        rgb: s,
        alpha: s,
        te: Default::default(),
    }
}

#[derive(Clone, Copy)]
pub struct ScaleSplit<TE: TexEnv> {
    pub rgb: Scale,
    pub alpha: Scale,
    pub te: TE,
}

impl<TE: TexEnv> Chainable for ScaleSplit<TE> {
    fn reg(&self) -> u32 {
        TE::BASE + 4
    }
    fn param(self) -> u32 {
        (self.rgb as u32) | ((self.alpha as u32) << 16)
    }
}

pub fn default_for<TE: TexEnv>() -> impl GpuCmdByMut {
    Chain
        * source_both::<TE>(Source::Previous, Source::Previous, Source::Previous)
        * Operand::<TE> {
            rgb: (
                ColorOp::SourceColor,
                ColorOp::SourceColor,
                ColorOp::SourceColor,
            ),
            alpha: (
                AlphaOp::SourceAlpha,
                AlphaOp::SourceAlpha,
                AlphaOp::SourceAlpha,
            ),
            te: Default::default(),
        }
        * combiner_both(CombineMode::Replace)
        * Color::<TE>(0x00000000, Default::default())
        * scale_both(Scale::X1)
}

pub fn all_defaults() -> impl GpuCmdByMut {
    super::Root + default_for::<E0>()
        + default_for::<E1>()
        + default_for::<E2>()
        + default_for::<E3>()
        + default_for::<E4>()
        + default_for::<E5>()
}
