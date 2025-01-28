use ctru_sys::*;

use super::{mask, GpuCmd};

#[derive(Clone,Copy)]
pub(crate) struct NumAttr(pub(crate) u32);

impl GpuCmd for NumAttr {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            self.0,
            GPUREG_VSH_NUM_ATTR | mask(0xF)
        ]
    }
}

#[derive(Clone,Copy)]
pub(crate) struct NumVertices(pub(crate) u32);

impl GpuCmd for NumVertices {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            self.0,
            GPUREG_NUMVERTICES | mask(0xF)
        ]
    }
}

#[derive(Clone,Copy)]
pub(crate) struct DrawingMode;

impl GpuCmd for DrawingMode {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            0,
            GPUREG_START_DRAW_FUNC0 | mask(0xF)
        ]
    }
}

#[derive(Clone,Copy)]
pub(crate) struct ConfigurationMode;

impl GpuCmd for ConfigurationMode {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            1,
            GPUREG_START_DRAW_FUNC0 | mask(0xF)
        ]
    }
}

#[derive(Clone,Copy)]
pub(crate) struct ClearPostVertexCache;

impl GpuCmd for ClearPostVertexCache {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            1,
            GPUREG_VTX_FUNC | mask(0xF)
        ]
    }
}

#[derive(Clone,Copy)]
pub(crate) struct FlushFramebuffer;

impl GpuCmd for FlushFramebuffer {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            1,
            GPUREG_FRAMEBUFFER_FLUSH | mask(0xF)
        ]
    }
}
