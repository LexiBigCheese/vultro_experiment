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

pub(crate) fn unused(reg: u32) -> OutMap {
    OutMap(reg, Unused, Unused, Unused, Unused)
}
pub(crate) fn reset() -> impl GpuCmdByMut {
    Root + unused(0)
        + unused(1)
        + unused(2)
        + unused(3)
        + unused(4)
        + unused(5)
        + unused(6)
        + OutMapTotal(0)
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
