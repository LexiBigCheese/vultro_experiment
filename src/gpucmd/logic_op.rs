use super::{GpuCmd, impl_gpucmd, mask};
use ctru_sys::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    InvertedOr,
}

pub use LogicOp::*;

impl_gpucmd!(LogicOp, |this: LogicOp| this as u32, GPUREG_LOGIC_OP);
