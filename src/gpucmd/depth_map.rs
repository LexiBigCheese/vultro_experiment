use super::{GpuCmd, GpuCmdDisable};
use ctru_sys::*;

pub struct Enabled;

impl GpuCmd for Enabled {
    fn cmd(&self) -> Vec<u32> {
        vec![GPUREG_DEPTHMAP_ENABLE,1]
    }
}

impl GpuCmdDisable for Enabled {
    fn cmd_disable(&self) -> Vec<u32> {
        vec![GPUREG_DEPTHMAP_ENABLE,0]
    }
}

pub struct Scale(pub f32);

impl GpuCmd for Scale {
    fn cmd(&self) -> Vec<u32> {
        vec![GPUREG_DEPTHMAP_SCALE, unsafe{f32tof24(self.0)}]
    }
}

pub struct Offset(pub f32);

impl GpuCmd for Offset {
    fn cmd(&self) -> Vec<u32> {
        vec![GPUREG_DEPTHMAP_OFFSET, unsafe{f32tof24(self.0)}]
    }
}
