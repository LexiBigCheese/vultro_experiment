use ctru_sys::*;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Component {
    PositionX,
    PositionY,
    PositionZ,
    PositionW,
    NormQuatX,
    NormQuatY,
    NormQuatZ,
    NormQuatW,
    ColorR,
    ColorG,
    ColorB,
    ColorA,
    TexCoord0U,
    TexCoord0V,
    TexCoord1U,
    TexCoord1V,
    TexCoord0W,
    ViewX,
    ViewY,
    ViewZ,
    TexCoord2U,
    TexCoord2V,
    Unused,
}

pub use Component::*;

use super::GpuCmdByMut;

//The chain is OUTMAP_TOTAL -> OUTMAP_O{0..=6}

///This is the full command buffer required to set the outmap
#[derive(Clone, Copy)]
pub struct OutMap([u32;10]);

impl GpuCmdByMut for OutMap {
    fn cmd_by_mut<A:std::alloc::Allocator>(self, buf: &mut Vec<u32,A>) {
        buf.extend_from_slice(&self.0);
    }
}
