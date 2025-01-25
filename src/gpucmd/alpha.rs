use ctru_sys::GPUREG_BLEND_FUNC;

use super::GpuCmd;

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
            GPUREG_BLEND_FUNC,
            u32::from_le_bytes([
                self.color_eq as u8,
                self.alpha_eq as u8,
                self.color_src as u8 | ((self.color_dst as u8) << 4),
                self.alpha_src as u8 | ((self.alpha_dst as u8) << 4)
            ]),
        ]
    }
}

pub struct Color(pub u32);

const GPUREG_BLEND_COLOR: u32 = 0x0103;

impl GpuCmd for Color {
    type Out = [u32; 2];
    fn cmd(self) -> Self::Out {
        [
            GPUREG_BLEND_COLOR,
            self.0
        ]
    }
}
