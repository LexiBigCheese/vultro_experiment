use super::{extra_params, GpuCmdByMut};

///Note you are expected to add the `mask` to the `reg` by yourself, as well as `CONSECUTIVE_WRITE`.
///Meanwhile, Transfer will handle `extra_params`
#[derive(Clone)]
pub struct Transfer{pub reg: u32,pub data: Vec<u32>}

impl GpuCmdByMut for Transfer {
    fn cmd_by_mut<A:std::alloc::Allocator>(self, buf: &mut Vec<u32,A>) {
        // Step 1: Chunks of 256 (0xFF + 1)
        for chunk in self.data.chunks(256) {
            buf.push(chunk[0]);
            let chunk = &chunk[1..];
            buf.push(self.reg | extra_params(chunk.len() as u32));
            buf.extend_from_slice(chunk);
            if (chunk.len() & 1) == 1 {
                buf.push(0);
            }
        }
    }
}
