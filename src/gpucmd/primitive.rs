use ctru_sys::*;

use super::{GpuCmd, impl_gpucmd, mask};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Mode {
    Triangles,
    TriangleStrip,
    TriangleFan,
    GeometryPrimitive,
}

#[derive(Clone, Copy)]
pub(crate) struct Config {
    pub(crate) outmap_total_minus_1: u32,
    pub(crate) primitive_mode: Mode,
}

impl_gpucmd!(
    Config,
    |this: Config| this.outmap_total_minus_1 | ((this.primitive_mode as u32) << 8),
    GPUREG_PRIMITIVE_CONFIG
);

#[derive(Clone, Copy)]
pub struct Restart;

impl_gpucmd!(Restart, |_this: Restart| 1, GPUREG_RESTART_PRIMITIVE);
