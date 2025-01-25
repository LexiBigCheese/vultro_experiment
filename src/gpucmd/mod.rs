pub mod depth_map;
pub mod cull_face;

use ctru::linear::LinearAllocator;

pub struct CommandBuffer {
    pub buf: Vec<u32,LinearAllocator>
}

pub struct CommandEncoder {
    buf: CommandBuffer
}

pub trait GpuCmd {
    type Out: AsRef<[u32]>;
    fn cmd(&self) -> Self::Out;
}

pub trait GpuCmdDisable {
    type Out: AsRef<[u32]>;
    fn cmd_disable(&self) -> Self::Out;
}

impl<T:GpuCmd> std::ops::Add<T> for CommandEncoder {
    type Output = CommandEncoder;
    fn add(mut self, rhs: T) -> Self::Output {
        let other = rhs.cmd();
        self.buf.buf.extend_from_slice(other.as_ref());
        self
    }
}

impl<T:GpuCmdDisable> std::ops::Sub<T> for CommandEncoder {
    type Output = CommandEncoder;
    fn sub(mut self, rhs: T) -> Self::Output {
        let other = rhs.cmd_disable();
        self.buf.buf.extend_from_slice(other.as_ref());
        self
    }
}
