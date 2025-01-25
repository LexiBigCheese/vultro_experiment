use ctru_sys::*;
use super::{GpuCmd,mask};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FragOp {
    Default,
    Gas,
    Shadow
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum BlendMode {
    LogicOp,
    Blend
}

#[derive(Clone, Copy)]
pub struct ColorOperation(pub FragOp, pub BlendMode);

impl GpuCmd for ColorOperation {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [u32::from_le_bytes([self.0 as u8,self.1 as u8,0xE4,0x00]),GPUREG_COLOR_OPERATION | mask(0xF)]
    }
}
