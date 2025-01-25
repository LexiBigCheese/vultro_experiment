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
    fn cmd(&self) -> Vec<u32>;
}

pub trait GpuCmdDisable {
    fn cmd_disable(&self) -> Vec<u32>;
}

impl<T:GpuCmd> std::ops::Add<T> for CommandEncoder {
    type Output = CommandEncoder;
    fn add(mut self, rhs: T) -> Self::Output {
        let other = rhs.cmd();
        self.buf.buf.extend_from_slice(&other);
        self
    }
}

impl<T:GpuCmdDisable> std::ops::Sub<T> for CommandEncoder {
    type Output = CommandEncoder;
    fn sub(mut self, rhs: T) -> Self::Output {
        let other = rhs.cmd_disable();
        self.buf.buf.extend_from_slice(&other);
        self
    }
}
