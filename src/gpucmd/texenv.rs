use ctru_sys::*;
use std::alloc::Allocator;

use super::{mask, GpuCmdByMut};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TexEnv {
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
}

impl TexEnv {
    pub const ALL: [TexEnv; 6] = [
        TexEnv::E0,
        TexEnv::E1,
        TexEnv::E2,
        TexEnv::E3,
        TexEnv::E4,
        TexEnv::E5,
    ];
    fn base(self) -> u32 {
        match self {
            TexEnv::E0 => GPUREG_TEXENV0_SOURCE,
            TexEnv::E1 => GPUREG_TEXENV1_SOURCE,
            TexEnv::E2 => GPUREG_TEXENV2_SOURCE,
            TexEnv::E3 => GPUREG_TEXENV3_SOURCE,
            TexEnv::E4 => GPUREG_TEXENV4_SOURCE,
            TexEnv::E5 => GPUREG_TEXENV5_SOURCE,
        }
    }
}

pub trait TexEnvCmdByMut {
    fn te_cmd_by_mut<Alloc: Allocator>(self, texenv: TexEnv, buf: &mut Vec<u32, Alloc>);
}

pub trait TexEnvCmd {
    fn te_cmd(self) -> (u32, u32);
}

impl<A: TexEnvCmd> TexEnvCmdByMut for A {
    fn te_cmd_by_mut<Alloc: Allocator>(self, texenv: TexEnv, buf: &mut Vec<u32, Alloc>) {
        let (cmd_offset, cmd_param) = self.te_cmd();
        buf.extend_from_slice(&[cmd_param, (texenv.base() + cmd_offset) | mask(0xF)]);
    }
}

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

#[derive(Clone, Copy)]
pub struct SourceBoth(pub Source, pub Source, pub Source);

impl TexEnvCmd for SourceBoth {
    fn te_cmd(self) -> (u32, u32) {
        let n = (self.0 as u32) | ((self.1 as u32) << 4) | ((self.2 as u32) << 8);
        (0, n | (n << 16))
    }
}

#[derive(Clone, Copy)]
pub struct SourceSplit {
    pub rgb: (Source, Source, Source),
    pub alpha: (Source, Source, Source),
}

impl TexEnvCmd for SourceSplit {
    fn te_cmd(self) -> (u32, u32) {
        (
            0,
            (self.rgb.0 as u32)
                | ((self.rgb.1 as u32) << 4)
                | ((self.rgb.2 as u32) << 8)
                | ((self.alpha.0 as u32) << 16)
                | ((self.alpha.1 as u32) << 20)
                | ((self.alpha.2 as u32) << 24),
        )
    }
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

#[derive(Clone, Copy)]
pub struct Operand {
    pub rgb: (ColorOp, ColorOp, ColorOp),
    pub alpha: (AlphaOp, AlphaOp, AlphaOp),
}

impl TexEnvCmd for Operand {
    fn te_cmd(self) -> (u32, u32) {
        (
            1,
            (self.rgb.0 as u32)
                | ((self.rgb.1 as u32) << 4)
                | ((self.rgb.2 as u32) << 8)
                | ((self.alpha.0 as u32) << 12)
                | ((self.alpha.1 as u32) << 16)
                | ((self.alpha.2 as u32) << 20),
        )
    }
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

#[derive(Clone, Copy)]
pub struct CombinerSplit {
    pub rgb: CombineMode,
    pub alpha: CombineMode,
}

impl TexEnvCmd for CombinerSplit {
    fn te_cmd(self) -> (u32, u32) {
        (2, (self.rgb as u32) | ((self.alpha as u32) << 16))
    }
}

#[derive(Clone, Copy)]
pub struct CombinerBoth(pub CombineMode);

impl TexEnvCmd for CombinerBoth {
    fn te_cmd(self) -> (u32, u32) {
        (2, (self.0 as u32) | ((self.0 as u32) << 16))
    }
}

///Note: `u32::from_le_bytes([r,g,b,a])`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color(pub u32);

impl TexEnvCmd for Color {
    fn te_cmd(self) -> (u32, u32) {
        (3, self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Scale {
    X1,
    X2,
    X4,
}

#[derive(Clone, Copy)]
pub struct ScaleBoth(pub Scale);

impl TexEnvCmd for ScaleBoth {
    fn te_cmd(self) -> (u32, u32) {
        (4, (self.0 as u32) | ((self.0 as u32) << 16))
    }
}

#[derive(Clone, Copy)]
pub struct ScaleSplit {
    pub rgb: Scale,
    pub alpha: Scale,
}

impl TexEnvCmd for ScaleSplit {
    fn te_cmd(self) -> (u32, u32) {
        (4, (self.rgb as u32) | ((self.alpha as u32) << 16))
    }
}

#[derive(Clone, Copy)]
pub struct TexEnvCons<A, B>(A, B);

impl<A: TexEnvCmdByMut, B: TexEnvCmdByMut> TexEnvCmdByMut for TexEnvCons<A, B> {
    fn te_cmd_by_mut<Alloc: Allocator>(self, texenv: TexEnv, buf: &mut Vec<u32, Alloc>) {
        self.0.te_cmd_by_mut(texenv, buf);
        self.1.te_cmd_by_mut(texenv, buf);
    }
}

impl<A: TexEnvCmdByMut, B: TexEnvCmdByMut, C: TexEnvCmdByMut> std::ops::Mul<C>
    for TexEnvCons<A, B>
{
    type Output = TexEnvCons<TexEnvCons<A, B>, C>;

    fn mul(self, rhs: C) -> Self::Output {
        TexEnvCons(self, rhs)
    }
}

#[derive(Clone, Copy)]
pub struct TexEnvRoot<A>(TexEnv, A);

impl<A: TexEnvCmdByMut> GpuCmdByMut for TexEnvRoot<A> {
    fn cmd_by_mut<Alloc: Allocator>(self, buf: &mut Vec<u32, Alloc>) {
        self.1.te_cmd_by_mut(self.0, buf);
    }
}

impl<A: TexEnvCmdByMut> std::ops::Mul<A> for TexEnv {
    type Output = TexEnvRoot<A>;

    fn mul(self, rhs: A) -> Self::Output {
        TexEnvRoot(self, rhs)
    }
}

impl<A: TexEnvCmdByMut, B: TexEnvCmdByMut> std::ops::Mul<B> for TexEnvRoot<A> {
    type Output = TexEnvRoot<TexEnvCons<A, B>>;
    fn mul(self, rhs: B) -> Self::Output {
        TexEnvRoot(self.0, TexEnvCons(self.1, rhs))
    }
}

#[derive(Clone, Copy)]
pub struct Defaults;

impl GpuCmdByMut for Defaults {
    fn cmd_by_mut<A: Allocator>(self, buf: &mut Vec<u32, A>) {
        for te in TexEnv::ALL {
            (te * SourceBoth(Source::Constant, Source::Constant, Source::Constant)
                * Operand {
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
                }
                * CombinerBoth(CombineMode::Replace)
                * Color(0)
                * ScaleBoth(Scale::X1))
            .cmd_by_mut(buf);
        }
    }
}
