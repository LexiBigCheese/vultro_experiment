use std::ffi::c_void;

use ctru_sys::gspSubmitGxCommand;

use crate::buffer::BufferSlice;

pub type GxCommand = [u32; 8];

///Queue of GX Commands
pub struct Queue {}

pub enum Error {
    TooManyCommands,
    Unknown,
}

///Note that addresses are either in LINEAR or in VRAM
///NOTABLE GSP COMMANDS:
///DMA Request SRC, DST, SIZE, _, _, FLUSH
///Process Command List
///Memory Fill ALIGNED

impl Queue {
    pub unsafe fn submit_command(&self, command: impl Into<GxCommand>) -> Result<(), Error> {
        let command: GxCommand = command.into();
        match unsafe { gspSubmitGxCommand(command.as_ptr().cast()) } {
            0 => Ok(()),
            -2 => Err(Error::TooManyCommands),
            _ => Err(Error::Unknown),
        }
    }
    pub fn copy_buffer(&self, mut src: BufferSlice, mut dst: BufferSlice, flush: bool) -> Result<(),Error> {
        let size = src.size().min(dst.size());
        unsafe {
            self.submit_command(gx_dma(src.start_addr(),dst.start_addr(),size,flush))
        }
    }
}

//TODO: Some sort of Buffer wrapper type, then also Buffer::slice and Texture::slice, ways to easily swap between the two, and ways to "map" LINEAR allocated stuff
//(In reality, just check that it's not in vram then return &mut [u8])

pub(crate) fn gx_dma(src: *const c_void, dst: *mut c_void, size: usize, flush: bool) -> GxCommand {
    [
        gx_cmd_head(0, true, true),
        src as u32,
        dst as u32,
        size as u32,
        0,
        0,
        0,
        if flush { 1 } else { 0 },
    ]
}

pub(crate) fn gx_command_list(
    src: *const c_void,
    size: usize,
    update_gas_additive_blend_results: bool,
    flush: bool,
) -> GxCommand {
    [
        gx_cmd_head(1, true, true),
        src as u32,
        size as u32,
        if update_gas_additive_blend_results {
            1
        } else {
            0
        },
        0,
        0,
        0,
        if flush { 1 } else { 0 },
    ]
}

pub(crate) fn gx_memory_fill(
    buf0: *mut c_void,
    buf0_val: u32,
    buf0_end: *const c_void,
    control0: u32,
    buf1: *mut c_void,
    buf1_val: u32,
    buf1_end: *const c_void,
    control1: u32,
) -> GxCommand {
    [
        gx_cmd_head(2, true, true),
        buf0 as u32,
        buf0_val,
        buf0_end as u32,
        buf1 as u32,
        buf1_val,
        buf1_end as u32,
        control0 | (control1 << 16),
    ]
}

pub(crate) fn gx_display_transfer(
    src: *const c_void,
    srcdim: u32,
    dst: *mut c_void,
    dstdim: u32,
    main_screen: bool,
) -> GxCommand {
    [
        gx_cmd_head(3, true, true),
        src as u32,
        dst as u32,
        srcdim,
        dstdim,
        if main_screen { 0x1001000 } else { 0x1000 },
        0,
        0,
    ]
}

//So, it appears there's a thing called the "Transfer Engine"
//It can do texture swizzles, hardware accelerated.
//I'll most likely want to expose it in some form in the future, but for now i can use the swizzle_3ds crate.

pub(crate) fn gx_texture_copy(
    src: *const c_void,
    srcdim: u32,
    dst: *mut c_void,
    dstdim: u32,
    n_bytes: usize,
    flags: u32,
) -> GxCommand {
    [
        gx_cmd_head(4, true, true),
        src as u32,
        dst as u32,
        n_bytes as u32,
        srcdim,
        dstdim,
        flags,
        0,
    ]
}

pub(crate) fn gx_flush_cache_regions(
    reg0: *const c_void,
    reg0_size: usize,
    reg1: *const c_void,
    reg1_size: usize,
    reg2: *const c_void,
    reg2_size: usize,
) -> GxCommand {
    [
        gx_cmd_head(5, true, false),
        reg0 as u32,
        reg0_size as u32,
        reg1 as u32,
        reg1_size as u32,
        reg2 as u32,
        reg2_size as u32,
        0
    ]
}

pub(crate) const fn gx_cmd_head(cmd_id: u8, set_bit0: bool, fail_on_busy: bool) -> u32 {
    (cmd_id as u32) | if set_bit0 { 1 << 16 } else { 0 } | if fail_on_busy { 1 << 24 } else { 0 }
}
