use super::{mask, GpuCmd, GpuCmdByMut, GpuCmdDisable, Root};
use ctru_sys::*;

///Subtract to Disable
#[doc(alias = "Disable")]
#[derive(Clone, Copy,PartialEq, Eq)]
pub struct Enabled;

impl GpuCmd for Enabled {
    type Out = [u32; 2];
    fn cmd(self) -> Self::Out {
        [1, GPUREG_DEPTHMAP_ENABLE | mask(0xF)]
    }
}

impl GpuCmdDisable for Enabled {
    type Out = [u32; 2];
    fn cmd_disable(self) -> Self::Out {
        [0, GPUREG_DEPTHMAP_ENABLE | mask(0xF)]
    }
}

#[derive(Clone, Copy)]
pub struct Scale(pub f32);

impl GpuCmd for Scale {
    type Out = [u32; 2];
    fn cmd(self) -> Self::Out {
        [unsafe { f32tof24(self.0) }, GPUREG_DEPTHMAP_SCALE | mask(0xF)]
    }
}

#[derive(Clone, Copy)]
pub struct Offset(pub f32);

impl GpuCmd for Offset {
    type Out = [u32; 2];
    fn cmd(self) -> Self::Out {
        [unsafe { f32tof24(self.0) }, GPUREG_DEPTHMAP_OFFSET | mask(0xF)]
    }
}

pub fn EnabledScaleOffset(scale: f32, offset: f32) -> impl GpuCmdByMut + Clone + Copy {
    Root + Enabled + Scale(scale) + Offset(offset)
}
