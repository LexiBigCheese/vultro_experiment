//! Chaining allows the use of Incremental Writes, thus shortening command buffer sizes,
//! While also letting you write fluent Command Buffer Code.
//!
//! Please note that when implementing chains, you can only have `0xFF` extra params after the first param
//!
//! ```rust
//! Root + Chain * shader.uniform("twovecs") * UniformF24(twovecs)
//! Root + shader.uniform("matrix") + UniformF24(matrix)
//! ```

use std::alloc::Allocator;

use super::{extra_params, mask, GpuCmd, GpuCmdByMut, CONSECUTIVE_WRITING};

///Incremental Writes
pub trait ChainableByMut {
    fn param_size(&self) -> std::num::NonZeroU8;
    /// The size passed in here considers your own param_size
    fn start_chain_by_mut<Alloc:Allocator>(self, size: u8, buf: &mut Vec<u32,Alloc>);
}

pub trait ChainContinueByMut {
    /// Append your parameters
    fn continue_chain_by_mut<Alloc:Allocator>(self, buf: &mut Vec<u32,Alloc>);
}

pub trait Chainable {
    fn reg(&self) -> u32;
    fn param(self) -> u32;
}

impl<T:Chainable> GpuCmd for T {
    type Out = [u32;2];

    fn cmd(self) -> Self::Out {
        let reg = self.reg();
        [self.param(),reg | mask(0xF)]
    }
}

impl<T:Chainable> ChainableByMut for T {
    fn param_size(&self) -> std::num::NonZeroU8 {
        std::num::NonZeroU8::try_from(1).unwrap()
    }

    fn start_chain_by_mut<Alloc:Allocator>(self, size: u8, buf: &mut Vec<u32,Alloc>) {
        let reg = self.reg();
        buf.extend_from_slice(&[self.param(),reg | mask(0xF) | if size > 0 {extra_params(size as u32) | CONSECUTIVE_WRITING} else {0}]);
    }
}

impl<T:Chainable> ChainContinueByMut for T {
    fn continue_chain_by_mut<Alloc:Allocator>(self, buf: &mut Vec<u32,Alloc>) {
        buf.push(self.param());
    }
}

pub trait ChainableNext {
    type Next;
}

pub struct Chain;
pub struct ChainOne<T>(T);
pub struct ChainCons<A,B>(A,B);
pub struct ChainMore<R,M>{
    size: u8,
    root: R,
    more: M
}

impl<T: ChainableByMut> std::ops::Mul<T> for Chain {
    type Output = ChainOne<T>;
    fn mul(self, rhs: T) -> Self::Output {
        ChainOne(rhs)
    }
}

impl<T: ChainableByMut> GpuCmdByMut for ChainOne<T> {
    fn cmd_by_mut<A:Allocator>(self, buf: &mut Vec<u32,A>) {
        self.0.start_chain_by_mut(0, buf);
    }
}

impl<M: ChainableByMut,R:ChainableNext<Next=M>+ChainableByMut> std::ops::Mul<M> for ChainOne<R> {
    type Output = ChainMore<R,M>;

    fn mul(self, rhs: M) -> Self::Output {
        let size = self.0.param_size().get() + rhs.param_size().get() - 1;
        ChainMore {
            size,
            root: self.0,
            more: rhs
        }
    }
}

impl<MA,MB:ChainableNext> ChainableNext for ChainCons<MA,MB> {
    type Next = MB::Next;
}

impl<MB:ChainableByMut,MA:ChainableNext<Next=MB>,R:ChainableNext<Next=MA>+ChainableByMut> std::ops::Mul<MB> for ChainMore<R,MA> {
    type Output = ChainMore<R,ChainCons<MA,MB>>;
    fn mul(self, rhs: MB) -> Self::Output {
        let size = self.size + rhs.param_size().get();
        ChainMore {
            size,
            root: self.root,
            more: ChainCons(self.more,rhs)
        }
    }
}

impl<M:ChainContinueByMut,R:ChainableByMut> GpuCmdByMut for ChainMore<R,M> {
    fn cmd_by_mut<A:Allocator>(self, buf: &mut Vec<u32,A>) {
        self.root.start_chain_by_mut(self.size, buf);
        self.more.continue_chain_by_mut(buf);
    }
}
