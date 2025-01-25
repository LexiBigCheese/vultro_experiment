use ctru_sys::*;
use super::GpuCmd;

#[repr(u8)]
pub enum FragOp {
    Default,
    Gas,
    Shadow
}

#[repr(u8)]
pub enum BlendMode {
    LogicOp,
    Blend
}

pub struct ColorOperation(pub FragOp, pub BlendMode);

impl GpuCmd for ColorOperation {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [GPUREG_COLOR_OPERATION,u32::from_le_bytes([self.0 as u8,self.1 as u8,0xE4,0x00])]
    }
}
