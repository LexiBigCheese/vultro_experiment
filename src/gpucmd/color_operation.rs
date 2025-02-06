use super::{GpuCmd, impl_gpucmd, mask};
use ctru_sys::*;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FragOp {
    Default,
    Gas,
    Shadow,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum BlendMode {
    LogicOp,
    Blend,
}

#[derive(Clone, Copy)]
pub struct ColorOperation(pub FragOp, pub BlendMode);

impl_gpucmd!(
    ColorOperation,
    |this: ColorOperation| u32::from_le_bytes([this.0 as u8, this.1 as u8, 0xE4, 0x00]),
    GPUREG_COLOR_OPERATION
);
