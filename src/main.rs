#![feature(allocator_api)]
#![feature(generic_const_exprs)]
#![feature(slice_split_once)]

pub mod buffer;
pub mod floater;
pub mod gpucmd;
pub mod queue;
pub mod renderbuffer;
pub mod shader;
pub mod shader_unfun;
pub mod texture;
pub mod vram;
pub mod other_main;
pub mod neobuf;

use ctru::{
    prelude::*,
    services::gfx::{Flush, Screen, Swap},
};
use ctru_sys::{gspWaitForAnyEvent, GSPGPU_TriggerCmdReqQueue};

fn main() {
    let mut soc = Soc::new().expect("No Soc");
    let _ = soc.redirect_to_3dslink(true, true);
    //Step 1: Gfx initialises Gsp
    let apt = Apt::new().expect("No Apt");
    let mut hid = Hid::new().expect("No Hid");
    let gfx = unsafe {
        Gfx::with_formats_vram(
            ctru::services::gspgpu::FramebufferFormat::Rgba8,
            ctru::services::gspgpu::FramebufferFormat::Rgba8,
        )
    }
    .expect("No Gfx");
    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    bottom_screen.set_double_buffering(true);
    let bottom_color_buffer =
        renderbuffer::ColorBuffer::new(240, 320, renderbuffer::ColorFormat::RGBA8)
            .expect("No ColorBuffer");
    let q = queue::Queue {};
    let mut some_buf = buffer::Buffer::new(64,true);
    q.fill_buffer(some_buf.slice(..), queue::FillValue::N32(0x12ABCDEF)).expect("couldn't memfill");
    ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::Psc0, false);
    let mut other_buf = buffer::Buffer::new(64,false);
    q.copy_buffer(some_buf.slice(..), other_buf.slice(..), true).expect("couldn't memcpy");
    ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::DMA, false);
    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        if hid.keys_down().contains(KeyPad::A) {
            let mut whole_buf = other_buf.slice(..);
            unsafe {
                ctru_sys::GSPGPU_FlushDataCache(whole_buf.start_addr(), whole_buf.size() as u32);
                ctru_sys::GSPGPU_InvalidateDataCache(whole_buf.start_addr(), whole_buf.size() as u32);
            }
            let Some(mapped) = whole_buf.map_mut() else {return};
            println!("{:x?}",mapped);
            // for v in mapped {
            //     let the_ptr = v as *mut u8;
            //     println!("{}",unsafe {the_ptr.read_volatile()})
            // }
        }
    }
    println!("Hello, world!");
}
