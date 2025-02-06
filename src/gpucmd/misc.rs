use ctru_sys::*;

use super::{GpuCmd, impl_gpucmd, mask};

#[derive(Clone, Copy)]
pub(crate) struct NumAttr(pub(crate) u32);

#[derive(Clone, Copy)]
pub(crate) struct NumVertices(pub(crate) u32);

#[derive(Clone, Copy)]
pub(crate) struct DrawingMode;

#[derive(Clone, Copy)]
pub(crate) struct ConfigurationMode;

#[derive(Clone, Copy)]
pub(crate) struct ClearPostVertexCache;

#[derive(Clone, Copy)]
pub(crate) struct FlushFramebuffer;

impl GpuCmd for FlushFramebuffer {
    type Out = [u32; 4];

    fn cmd(self) -> Self::Out {
        [
            1,
            GPUREG_FRAMEBUFFER_FLUSH | mask(0xF),
            1,
            GPUREG_FRAMEBUFFER_INVALIDATE | mask(0xF),
        ]
    }
}

#[derive(Clone, Copy)]
pub(crate) struct VshEntrypoint(pub(crate) u32);

impl_gpucmd!(NumAttr, |this: NumAttr| this.0, GPUREG_VSH_NUM_ATTR);
impl_gpucmd!(NumVertices, |this: NumVertices| this.0, GPUREG_NUMVERTICES);
impl_gpucmd!(DrawingMode, |_this: DrawingMode| 0, GPUREG_START_DRAW_FUNC0);
impl_gpucmd!(
    ConfigurationMode,
    |_this: ConfigurationMode| 1,
    GPUREG_START_DRAW_FUNC0
);
impl_gpucmd!(
    ClearPostVertexCache,
    |_this: ClearPostVertexCache| 1,
    GPUREG_VTX_FUNC
);
impl_gpucmd!(
    VshEntrypoint,
    |this: VshEntrypoint| this.0,
    GPUREG_VSH_ENTRYPOINT
);
