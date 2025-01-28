pub mod depth_map;
pub mod cull_face;
pub mod shader_outmap;
pub mod alpha;
pub mod logic_op;
pub mod color_operation;
pub mod texenv;
pub mod depth_color_mask;
pub mod transfer;
pub mod chain;
pub mod geostage_config;
pub mod primitive;
pub mod fixed_attrib;
pub mod misc;

use std::alloc::Allocator;

use ctru::linear::LinearAllocator;

///Note: The Buffer **MUST** be `0x10` bytes aligned!
///Note: The Buffer's SIZE **MUST ALSO** be `0x10` bytes aligned!
#[derive(Clone)]
pub struct CommandBuffer<A> where A:Allocator {
    pub buf: Vec<u32,A>
}

#[derive(Clone)]
pub struct CommandEncoder<A> where A:Allocator {
    buf: CommandBuffer<A>
}

impl CommandEncoder<CmdBufAllocator> {
    //TODO: Make `try` versions
    pub fn new() -> CommandEncoder<CmdBufAllocator> {
        CommandEncoder {
            buf: CommandBuffer {
                buf: Vec::new_in(CmdBufAllocator)
            }
        }
    }
    pub fn new_with_capacity(capacity: usize) -> CommandEncoder<CmdBufAllocator> {
        CommandEncoder {
            buf: CommandBuffer {
                buf: Vec::with_capacity_in(capacity,CmdBufAllocator)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct CmdBufAllocator;

unsafe impl Allocator for CmdBufAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        let layout = layout.align_to(0x10).expect("Could not 0x10 Byte align Command Buffer").pad_to_align();
        LinearAllocator.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
        let layout = layout.align_to(0x10).expect("Could not 0x10 Byte align Command Buffer").pad_to_align();
        unsafe {LinearAllocator.deallocate(ptr,layout);}
    }
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
pub struct Finish;

impl<A:Allocator> std::ops::Add<Finish> for CommandEncoder<A> {
    type Output = CommandBuffer<A>;
    fn add(self, rhs: Finish) -> Self::Output {
        use ctru_sys::*;
        let mut buf = self.buf;
        buf.buf.extend_from_slice(&[0x12345678,GPUREG_FINALIZE | mask(0xF)]);
        buf
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
    type Output = Cons<Self,C>;
    fn add(self, rhs: C) -> Self::Output {
        Cons(self,rhs)
    }
}

impl<A:GpuCmdByMut,B:GpuCmdByMut,C:GpuCmdDisableByMut> std::ops::Sub<C> for Cons<A,B> {
    type Output = ConsNeg<Self,C>;
    fn sub(self, rhs: C) -> Self::Output {
        ConsNeg(self,rhs)
    }
}

impl<A:GpuCmdByMut,B:GpuCmdByMut,C:GpuCmdByMut> std::ops::Add<C> for ConsNeg<A,B> {
    type Output = Cons<Self,C>;
    fn add(self, rhs: C) -> Self::Output {
        Cons(self,rhs)
    }
}

impl<A:GpuCmdByMut,B:GpuCmdByMut,C:GpuCmdDisableByMut> std::ops::Sub<C> for ConsNeg<A,B> {
    type Output = ConsNeg<Self,C>;
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

pub const fn mask(m: u32) -> u32 {
    (m & 0xF) << 16
}

pub const fn extra_params(n: u32) -> u32 {
    (n & 0xFF) << 20
}

pub const CONSECUTIVE_WRITING: u32 = 1 << 31;
