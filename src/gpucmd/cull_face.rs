use ctru_sys::*;

use super::{mask, GpuCmd};

#[derive(Clone, Copy)]
pub struct No;

impl GpuCmd for No {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [0,GPUREG_FACECULLING_CONFIG | mask(0xF)]
    }
}

#[derive(Clone, Copy)]
pub struct FrontCCW;

impl GpuCmd for FrontCCW {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [1,GPUREG_FACECULLING_CONFIG | mask(0xF)]
    }
}

#[derive(Clone, Copy)]
pub struct BackCCW;

impl GpuCmd for BackCCW {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [2,GPUREG_FACECULLING_CONFIG | mask(0xF)]
    }
}
