pub mod depth_map;
pub mod cull_face;
pub mod shader_outmap;
pub mod alpha;
pub mod logic_op;
pub mod color_operation;
pub mod texenv;
pub mod depth_color_mask;

use std::alloc::Allocator;

pub struct CommandBuffer<A> where A:Allocator {
    pub buf: Vec<u32,A>
}

pub struct CommandEncoder<A> where A:Allocator {
    buf: CommandBuffer<A>
}

pub trait GpuCmd {
    type Out: AsRef<[u32]>;
    fn cmd(self) -> Self::Out;
}

pub trait GpuCmdDisable {
    type Out: AsRef<[u32]>;
    fn cmd_disable(self) -> Self::Out;
}

pub trait GpuCmdByMut {
    fn cmd_by_mut<A:Allocator>(self, buf: &mut Vec<u32,A>);
}

impl<C:GpuCmd> GpuCmdByMut for C {
    fn cmd_by_mut<A:Allocator>(self, buf: &mut Vec<u32,A>) {
        buf.extend_from_slice(self.cmd().as_ref());
    }
}

pub trait GpuCmdDisableByMut {
    fn cmd_disable_by_mut<A:Allocator>(self, buf: &mut Vec<u32,A>);
}

impl<C:GpuCmdDisable> GpuCmdDisableByMut for C {
    fn cmd_disable_by_mut<A:Allocator>(self, buf: &mut Vec<u32,A>) {
        buf.extend_from_slice(self.cmd_disable().as_ref());
    }
}

impl<T:GpuCmdByMut,A:Allocator> std::ops::Add<T> for CommandEncoder<A> {
    type Output = CommandEncoder<A>;
    fn add(mut self, rhs: T) -> Self::Output {
        rhs.cmd_by_mut(&mut self.buf.buf);
        self
    }
}

impl<T:GpuCmdByMut,A:Allocator> std::ops::AddAssign<T> for CommandEncoder<A> {
    fn add_assign(&mut self, rhs: T) {
        rhs.cmd_by_mut(&mut self.buf.buf);
    }
}

impl<T:GpuCmdDisableByMut,A:Allocator> std::ops::Sub<T> for CommandEncoder<A> {
    type Output = CommandEncoder<A>;
    fn sub(mut self, rhs: T) -> Self::Output {
        rhs.cmd_disable_by_mut(&mut self.buf.buf);
        self
    }
}


#[derive(Clone, Copy)]
pub struct Cons<A,B>(A,B);
#[derive(Clone, Copy)]
pub struct ConsNeg<A,B>(A,B);
#[derive(Clone, Copy)]
pub struct Root;

impl<A:GpuCmdByMut,B:GpuCmdByMut> GpuCmdByMut for Cons<A,B> {
    fn cmd_by_mut<Alloc:Allocator>(self, buf: &mut Vec<u32,Alloc>) {
        self.0.cmd_by_mut(buf);
        self.1.cmd_by_mut(buf);
    }
}

impl<A:GpuCmdByMut,B:GpuCmdDisableByMut> GpuCmdByMut for ConsNeg<A,B> {
    fn cmd_by_mut<Alloc:Allocator>(self, buf: &mut Vec<u32,Alloc>) {
        self.0.cmd_by_mut(buf);
        self.1.cmd_disable_by_mut(buf);
    }
}

impl<A:GpuCmdByMut,B:GpuCmdByMut,C:GpuCmdByMut> std::ops::Add<C> for Cons<A,B> {
    type Output = Cons<Cons<A,B>,C>;
    fn add(self, rhs: C) -> Self::Output {
        Cons(self,rhs)
    }
}

impl<A:GpuCmdByMut,B:GpuCmdByMut,C:GpuCmdDisableByMut> std::ops::Sub<C> for Cons<A,B> {
    type Output = ConsNeg<Cons<A,B>,C>;
    fn sub(self, rhs: C) -> Self::Output {
        ConsNeg(self,rhs)
    }
}

impl GpuCmd for Root {
    type Out = [u32;0];
    fn cmd(self) -> Self::Out {
        []
    }
}

impl<A:GpuCmdByMut> std::ops::Add<A> for Root {
    type Output = Cons<Root,A>;
    fn add(self, rhs: A) -> Self::Output {
        Cons(Root,rhs)
    }
}

impl<A:GpuCmdDisableByMut> std::ops::Sub<A> for Root {
    type Output = ConsNeg<Root,A>;
    fn sub(self, rhs: A) -> Self::Output {
        ConsNeg(Root,rhs)
    }
}
