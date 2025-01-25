use ctru_sys::*;
use super::GpuCmd;

#[derive(Clone, Copy,PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LogicOp {
    /// `0`
    Clear,
    /// `s & d`
    And,
    /// `s & ~d`
    ReverseAnd,
    /// `s`
    Copy,
    /// `1`
    Set,
    /// `~s`
    InvertedCopy,
    /// `d`
    Noop,
    /// `~d`
    Invert,
    /// `~(s & d)`
    Nand,
    /// `s | d`
    Or,
    /// `~(s | d)`
    Nor,
    /// `s ^ d`
    Xor,
    /// `~(s ^ d)`
    Equivalent,
    /// `~s & d`
    InvertedAnd,
    /// `s | ~d`
    ReverseOr,
    /// `~s | d`
    InvertedOr
}

pub use LogicOp::*;

impl GpuCmd for LogicOp {
    type Out = [u32;2];
    fn cmd(self) -> Self::Out {
        [GPUREG_LOGIC_OP,self as u32]
    }
}
