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

use super::{mask, GpuCmd, GpuCmdDisable, GpuCmdByMut, Root};

///https://www.3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_SH_OUTMAP_Oi
#[derive(Clone, Copy)]
pub(crate) struct OutMap(pub(crate) u32, pub(crate) Component, pub(crate) Component, pub(crate) Component, pub(crate) Component);
///https://www.3dbrew.org/wiki/GPU/Internal_Registers#GPUREG_SH_OUTMAP_TOTAL
#[derive(Clone, Copy)]
pub(crate) struct OutMapTotal(pub(crate) u32);

pub(crate) fn unused(reg: u32) -> OutMap {
    OutMap(reg, Unused, Unused, Unused, Unused)
}
pub(crate) fn reset() -> impl GpuCmdByMut + Clone + Copy {
    Root + unused(0)
        + unused(1)
        + unused(2)
        + unused(3)
        + unused(4)
        + unused(5)
        + unused(6)
        + OutMapTotal(0)
}

//TODO: Utilise Consecutive Writing Mode

impl GpuCmd for OutMapTotal {
    type Out = [u32; 2];

    fn cmd(self) -> Self::Out {
        [self.0, GPUREG_SH_OUTMAP_TOTAL | mask(0xF)]
    }
}

impl GpuCmd for OutMap {
    type Out = [u32; 2];
    fn cmd(self) -> Self::Out {
        [
            u32::from_le_bytes([self.1 as u8, self.2 as u8, self.3 as u8, self.4 as u8]),
            (GPUREG_SH_OUTMAP_O0 + self.0) | mask(0xF),
        ]
    }
}

#[derive(Clone,Copy)]
pub(crate) struct UseTextureCoordinates;

impl GpuCmd for UseTextureCoordinates {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [1, GPUREG_SH_OUTATTR_MODE | mask(0xF)]
    }
}

impl GpuCmdDisable for UseTextureCoordinates {
    type Out = [u32;2];

    fn cmd_disable(self) -> Self::Out {
        [0, GPUREG_SH_OUTATTR_MODE | mask(0xF)]
    }
}

#[derive(Clone,Copy)]
pub(crate) struct Clock {
    pub(crate) position_z: bool,
    pub(crate) color: bool,
    pub(crate) texcoord0: bool,
    pub(crate) texcoord1: bool,
    pub(crate) texcoord2: bool,
    pub(crate) texcoord0w: bool,
    pub(crate) normquat_or_view: bool,
}

impl GpuCmd for Clock {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        [
            if self.position_z {1} else {0}
            | if self.color {1 << 1} else {0}
            | if self.texcoord0 {1 << 8} else {0}
            | if self.texcoord1 {1 << 9} else {0}
            | if self.texcoord2 {1 << 10} else {0}
            | if self.texcoord0w {1 << 16} else {0}
            | if self.normquat_or_view {1 << 24} else {0},
            GPUREG_SH_OUTATTR_CLOCK | mask(0xF)
        ]
    }
}
