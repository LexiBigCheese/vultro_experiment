use super::{GpuCmd, impl_gpucmd, mask};
use ctru_sys::*;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Function {
    Never,
    Always,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

///Note that setting the "Depth test enabled" bit to 0 will not also disable depth writes.
///It will instead behave as if the depth function were set to "Always".
///To completely disable depth-related operations,
///both the depth test and depth write bits must be disabled.
#[derive(Clone, Copy)]
pub struct DepthColorMask {
    pub enabled: bool,
    pub function: Function,
    pub red_write: bool,
    pub green_write: bool,
    pub blue_write: bool,
    pub alpha_write: bool,
    pub depth_write: bool,
}

impl_gpucmd!(
    DepthColorMask,
    |this: DepthColorMask| if this.enabled { 1 } else { 0 }
        | ((this.function as u32) << 4)
        | if this.red_write { 1 << 8 } else { 0 }
        | if this.green_write { 1 << 9 } else { 0 }
        | if this.blue_write { 1 << 10 } else { 0 }
        | if this.alpha_write { 1 << 11 } else { 0 }
        | if this.depth_write { 1 << 12 } else { 0 },
    GPUREG_DEPTH_COLOR_MASK
);
