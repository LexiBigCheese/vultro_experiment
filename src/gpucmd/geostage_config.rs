use ctru_sys::*;

use super::{GpuCmd, impl_gpucmd, mask};

#[derive(Clone, Copy)]
pub(crate) struct Config {
    pub(crate) geometry_shader_in_use: bool,
    pub(crate) drawing_triangle_elements: bool,
    pub(crate) use_reserved_geometry_shader_subdivision: bool,
}

impl_gpucmd!(
    Config,
    |this: Config| if this.geometry_shader_in_use { 2 } else { 0 }
        | if this.drawing_triangle_elements {
            1 << 8
        } else {
            0
        }
        | if this.use_reserved_geometry_shader_subdivision {
            1 << 31
        } else {
            0
        },
    GPUREG_GEOSTAGE_CONFIG
);
