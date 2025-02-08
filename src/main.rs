#![feature(allocator_api)]
#![feature(generic_const_exprs)]
#![feature(slice_split_once)]

pub mod buffer;
pub mod floater;
pub mod gpucmd;
pub mod neobuf;
pub mod other_main;
pub mod queue;
pub mod renderbuffer;
pub mod shader;
pub mod shader_unfun;
pub mod texture;
pub mod vram;

use ctru::{
    prelude::*,
    services::gfx::{Flush, Screen, Swap},
};
use ctru_sys::{GSPGPU_TriggerCmdReqQueue, gspWaitForAnyEvent};

use neobuf::{NeoSlice, NeoSliceMut};

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
    let mut some_buf = neobuf::VramBuf::<u32>::new(320 * 240);
    q.fill_buffer(
        some_buf.slice_aligned_8_mut(..).expect("Could not slice"),
        queue::FillValue::N32(0x12ABCDEF),
    )
    .expect("couldn't memfill");
    ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::Psc0, false);
    let mut other_buf = neobuf::LinearBuf::<u32>::new(320 * 240);
    q.copy_buffer(
        some_buf.slice(..).expect("Could not slice"),
        other_buf.slice_mut(..).expect("Could not slice"),
        true,
    )
    .expect("couldn't memcpy");
    ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::DMA, false);
    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        if hid.keys_down().contains(KeyPad::A) {
            let mut whole_buf = &other_buf;
            unsafe {
                ctru_sys::GSPGPU_FlushDataCache(
                    whole_buf.ptr().cast_const().cast(),
                    whole_buf.len() as u32,
                );
                ctru_sys::GSPGPU_InvalidateDataCache(
                    whole_buf.ptr().cast_const().cast(),
                    whole_buf.len() as u32,
                );
            }
            let mapped = unsafe { whole_buf.map() };
            q.display_transfer_to_fb(
                some_buf.slice(..).expect("Could not slice"),
                240,
                320,
                bottom_screen.raw_framebuffer(),
                queue::TransferFlags {
                    block_tiling_mode: false,
                    tiled_out: false,
                    flip_vert: false,
                    input_color_format: queue::TransferFormat::RGBA8,
                    output_color_format: queue::TransferFormat::RGBA8,
                    output_width_less_than_input_width: false,
                    scale_down_filter: queue::ScaleDownFilter::None,
                    texture_copy: false,
                    tiled_to_tiled: false,
                },
            )
            .expect("Could not DisplayTransfer");
            ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::PPF, false);
            bottom_screen.flush_buffers();
            bottom_screen.swap_buffers();
            // for v in mapped {
            //     let the_ptr = v as *mut u8;
            //     println!("{}",unsafe {the_ptr.read_volatile()})
            // }
        }
    }
    println!("Hello, world!");
}
