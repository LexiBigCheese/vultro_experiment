use ctru_sys::*;

use super::{GpuCmd,mask};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Mode {
    Triangles,
    TriangleStrip,
    TriangleFan,
    GeometryPrimitive
}

#[derive(Clone,Copy)]
pub(crate) struct Config {
    pub(crate) outmap_total_minus_1: u32,
    pub(crate) primitive_mode: Mode
}

impl GpuCmd for Config {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            self.outmap_total_minus_1 | ((self.primitive_mode as u32) << 8),
            GPUREG_PRIMITIVE_CONFIG | mask(0xF)
        ]
    }
}

#[derive(Clone, Copy)]
pub struct Restart;

impl GpuCmd for Restart {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [
            1,
            GPUREG_RESTART_PRIMITIVE | mask(0xF)
        ]
    }
}
