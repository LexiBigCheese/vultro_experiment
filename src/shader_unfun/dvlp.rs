use super::Error::UnexpectedEof;
use crate::gpucmd::{mask, GpuCmdByMut};

///Stuff that needs to be transferred to the GPU.
///however, you need to know if you're transferring a GSH or a VSH to transfer it correctly
pub struct DVLP {
    pub code: Vec<u32>,
    pub opcdesc: Vec<u32>
}

#[derive(Clone, Copy)]
pub struct GSH;
#[derive(Clone, Copy)]
pub struct VSH;

impl DVLP {
    pub fn parse_dvlp(data: &[u32]) -> Result<DVLP, super::Error> {
        let code_size = *data.get(3).ok_or(UnexpectedEof)? as usize;
        let code_start_in_bytes = *data.get(2).ok_or(UnexpectedEof)? as usize;
        let code_start = code_start_in_bytes / 4;
        let code_data = data
            .split_at_checked(code_start)
            .ok_or(UnexpectedEof)?
            .1
            .split_at_checked(code_size)
            .ok_or(UnexpectedEof)?
            .0;
        let opdesc_size = *data.get(5).ok_or(UnexpectedEof)? as usize;
        let opdesc_start_in_bytes = *data.get(4).ok_or(UnexpectedEof)? as usize;
        let opdesc_start = opdesc_start_in_bytes / 4;
        let mut opcdesc_data: Vec<u32> = Vec::with_capacity(opdesc_size);
        for i in 0..opdesc_size {
            opcdesc_data.push(
                *data.get(opdesc_start + i*2).ok_or(UnexpectedEof)?
            );
        }
        Ok(DVLP {
            code: code_data.to_vec(),
            opcdesc: opcdesc_data
        })
    }
}

impl GpuCmdByMut for (VSH,&DVLP) {
    fn cmd_by_mut<A:std::alloc::Allocator>(self, buf: &mut Vec<u32,A>) {
        use ctru_sys::*;
        // buf.push([0,GPUREG_VSH_CODETRANSFER_CONFIG | mask(0xf)]);

    }
}
