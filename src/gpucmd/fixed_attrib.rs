use ctru_sys::*;

use super::{extra_params, mask, GpuCmd, CONSECUTIVE_WRITING};

///For immediate mode, use `Index(0xF)`
#[derive(Clone,Copy)]
pub(crate) struct Index(pub(crate) u32);

impl GpuCmd for Index {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            self.0,
            GPUREG_FIXEDATTRIB_INDEX | mask(0xF)
        ]
    }
}

///Obtain this data from `floater::f32x4tof24x4`
#[derive(Clone,Copy)]
pub(crate) struct Data(pub(crate) [u32;3]);

impl GpuCmd for Data {
    type Out = [u32;4];
    fn cmd(self) -> Self::Out {
        [
            self.0[0],
            GPUREG_FIXEDATTRIB_DATA0 | mask(0xF) | extra_params(2) | CONSECUTIVE_WRITING,
            self.0[1],
            self.0[2]
        ]
    }
}
