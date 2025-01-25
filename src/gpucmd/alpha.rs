use ctru_sys::{GPUREG_BLEND_FUNC, GPUREG_FRAGOP_ALPHA_TEST};

use super::{mask, GpuCmd};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Equation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

pub use Equation::*;

///This is a nibble (half-byte)
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Factor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SaturatedAlpha,
}

pub use Factor::*;

#[derive(Clone, Copy)]
pub struct Blend {
    pub color_eq: Equation,
    pub alpha_eq: Equation,
    pub color_src: Factor,
    pub color_dst: Factor,
    pub alpha_src: Factor,
    pub alpha_dst: Factor,
}

impl Blend {
    pub fn new(
        color_eq: Equation,
        alpha_eq: Equation,
        color_src: Factor,
        color_dst: Factor,
        alpha_src: Factor,
        alpha_dst: Factor,
    ) -> Self {
        Blend {
            color_eq,
            alpha_eq,
            color_src,
            color_dst,
            alpha_src,
            alpha_dst,
        }
    }
}

impl GpuCmd for Blend {
    type Out = [u32; 2];

    fn cmd(self) -> Self::Out {
        [
            u32::from_le_bytes([
                self.color_eq as u8,
                self.alpha_eq as u8,
                self.color_src as u8 | ((self.color_dst as u8) << 4),
                self.alpha_src as u8 | ((self.alpha_dst as u8) << 4),
            ]),
            GPUREG_BLEND_FUNC | mask(0xF),
        ]
    }
}

#[derive(Clone, Copy)]
pub struct Color(pub u32);

const GPUREG_BLEND_COLOR: u32 = 0x0103;

impl GpuCmd for Color {
    type Out = [u32; 2];
    fn cmd(self) -> Self::Out {
        [self.0, GPUREG_BLEND_COLOR | mask(0xF)]
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Function {
    Never,
    Always,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

///In modern engines, you might know this as Alpha Clip or Alpha Threshold
#[doc(alias = "Clip")]
#[derive(Clone, Copy)]
pub struct Test {
    pub enabled: bool,
    pub function: Function,
    pub reference_value: u16,
}

impl Test {
    pub fn disabled() -> Test {
        Test {
            enabled: false,
            function: Function::Always,
            reference_value: 0x0000,
        }
    }
}

impl GpuCmd for Test {
    type Out = [u32; 2];
    fn cmd(self) -> Self::Out {
        [
            if self.enabled { 0 } else { 1 }
                | ((self.function as u32) << 4)
                | ((self.reference_value as u32) << 8),
            GPUREG_FRAGOP_ALPHA_TEST | mask(0xF),
        ]
    }
}
