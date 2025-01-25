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

pub struct Blend {
    color_eq: Equation,
    alpha_eq: Equation,
    color_src: Factor,
    color_dst: Factor,
    alpha_src: Factor,
    alpha_dst: Factor,
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

pub struct Color(u32);

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
