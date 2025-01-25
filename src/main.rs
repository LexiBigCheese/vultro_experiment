#![feature(allocator_api)]

pub mod gpucmd;
pub mod queue;
pub mod texture;
pub mod vram;
pub mod buffer;

use ctru::prelude::*;

fn main() {
    let mut soc = Soc::new().expect("No Soc");
    let _ = soc.redirect_to_3dslink(true, true);
    //Step 1: Gfx initialises Gsp
    let gfx = unsafe {Gfx::with_formats_vram(
        ctru::services::gspgpu::FramebufferFormat::Bgr8,
        ctru::services::gspgpu::FramebufferFormat::Bgr8,
    )}.expect("No Gfx");
    unsafe {

    }
    println!("Hello, world!");
}
