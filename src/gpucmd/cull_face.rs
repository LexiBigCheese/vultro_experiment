use ctru_sys::*;

use super::GpuCmd;

#[derive(Clone, Copy)]
pub struct No;

impl GpuCmd for No {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [GPUREG_FACECULLING_CONFIG,0]
    }
}

#[derive(Clone, Copy)]
pub struct FrontCCW;

impl GpuCmd for FrontCCW {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [GPUREG_FACECULLING_CONFIG,1]
    }
}

#[derive(Clone, Copy)]
pub struct BackCCW;

impl GpuCmd for BackCCW {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [GPUREG_FACECULLING_CONFIG,2]
    }
}
