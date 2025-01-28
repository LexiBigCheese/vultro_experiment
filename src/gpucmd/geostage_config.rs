use ctru_sys::*;

use super::{mask, GpuCmd};

#[derive(Clone,Copy)]
pub(crate) struct Config {
    pub(crate) geometry_shader_in_use: bool,
    pub(crate) drawing_triangle_elements: bool,
    pub(crate) use_reserved_geometry_shader_subdivision: bool
}

impl GpuCmd for Config {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            if self.geometry_shader_in_use {2} else {0}
            | if self.drawing_triangle_elements {1 << 8} else {0}
            | if self.use_reserved_geometry_shader_subdivision {1 << 31} else {0},
            GPUREG_GEOSTAGE_CONFIG | mask(0xF)
        ]
    }
}
