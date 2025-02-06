use super::{GpuCmd, GpuCmdByMut, GpuCmdDisable, Root, impl_gpucmd, impl_gpucmd_disable, mask};
use ctru_sys::*;

///Subtract to Disable
#[doc(alias = "Disable")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Enabled;

impl_gpucmd!(Enabled, |_this: Enabled| 1, GPUREG_DEPTHMAP_ENABLE);

impl_gpucmd_disable!(Enabled, |_this: Enabled| 0, GPUREG_DEPTHMAP_ENABLE);

#[derive(Clone, Copy)]
pub struct Scale(pub f32);

impl_gpucmd!(
    Scale,
    |this: Scale| unsafe { f32tof24(this.0) },
    GPUREG_DEPTHMAP_SCALE
);

#[derive(Clone, Copy)]
pub struct Offset(pub f32);

impl_gpucmd!(
    Offset,
    |this: Offset| unsafe { f32tof24(this.0) },
    GPUREG_DEPTHMAP_OFFSET
);

pub fn EnabledScaleOffset(scale: f32, offset: f32) -> impl GpuCmdByMut + Clone + Copy {
    Root + Enabled + Scale(scale) + Offset(offset)
}
