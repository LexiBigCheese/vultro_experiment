use ctru_sys::*;

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

use super::{GpuCmd, GpuCmdByMut, Root};

///https://www.3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_SH_OUTMAP_Oi
pub(crate) struct OutMap(u32, Component, Component, Component, Component);
///https://www.3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_SH_OUTMAP_TOTAL
pub(crate) struct OutMapTotal(u32);

impl OutMap {
    pub(crate) fn unused(reg: u32) -> Self {
        OutMap(reg, Unused, Unused, Unused, Unused)
    }
    pub(crate) fn reset() -> impl GpuCmdByMut {
        Root + OutMap::unused(0)
            + OutMap::unused(1)
            + OutMap::unused(2)
            + OutMap::unused(3)
            + OutMap::unused(4)
            + OutMap::unused(5)
            + OutMap::unused(6)
            + OutMapTotal(0)
    }
}

impl GpuCmd for OutMapTotal {
    type Out = [u32; 2];

    fn cmd(self) -> Self::Out {
        [GPUREG_SH_OUTMAP_TOTAL, self.0]
    }
}

impl GpuCmd for OutMap {
    type Out = [u32; 2];
    fn cmd(self) -> Self::Out {
        [
            GPUREG_SH_OUTMAP_O0 + self.0,
            u32::from_le_bytes([self.1 as u8, self.2 as u8, self.3 as u8, self.4 as u8]),
        ]
    }
}
