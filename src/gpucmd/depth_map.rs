use super::{GpuCmd, GpuCmdDisable};
use ctru_sys::*;

pub struct Enabled;

impl GpuCmd for Enabled {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [GPUREG_DEPTHMAP_ENABLE,1]
    }
}

impl GpuCmdDisable for Enabled {
    type Out = [u32;2];
    fn cmd_disable(self) -> Self::Out {
        [GPUREG_DEPTHMAP_ENABLE,0]
    }
}

pub struct Scale(pub f32);

impl GpuCmd for Scale {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [GPUREG_DEPTHMAP_SCALE, unsafe{f32tof24(self.0)}]
    }
}

pub struct Offset(pub f32);

impl GpuCmd for Offset {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [GPUREG_DEPTHMAP_OFFSET, unsafe{f32tof24(self.0)}]
    }
}
