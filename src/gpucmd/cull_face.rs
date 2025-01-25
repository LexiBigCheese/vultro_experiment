use ctru_sys::*;

use super::GpuCmd;

pub struct No;

impl GpuCmd for No {
    fn cmd(&self) -> Vec<u32> {
        vec![GPUREG_FACECULLING_CONFIG,0]
    }
}

pub struct FrontCCW;

impl GpuCmd for FrontCCW {
    fn cmd(&self) -> Vec<u32> {
        vec![GPUREG_FACECULLING_CONFIG,1]
    }
}

pub struct BackCCW;

impl GpuCmd for BackCCW {
    fn cmd(&self) -> Vec<u32> {
        vec![GPUREG_FACECULLING_CONFIG,2]
    }
}
