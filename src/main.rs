#![feature(allocator_api)]

pub mod buffer;
pub mod gpucmd;
pub mod queue;
pub mod texture;
pub mod vram;

use ctru::prelude::*;

fn main() {
    let mut soc = Soc::new().expect("No Soc");
    let _ = soc.redirect_to_3dslink(true, true);
    //Step 1: Gfx initialises Gsp
    let gfx = unsafe {
        Gfx::with_formats_vram(
            ctru::services::gspgpu::FramebufferFormat::Bgr8,
            ctru::services::gspgpu::FramebufferFormat::Bgr8,
        )
    }
    .expect("No Gfx");
    let some_command = {
        use gpucmd::{Root, alpha, cull_face, depth_map, shader_outmap};
        Root + depth_map::EnabledScaleOffset(1.0, 0.0)
            + cull_face::BackCCW
            + alpha::Color(0)
            + alpha::Blend::new(
                alpha::Add,
                alpha::Add,
                alpha::SrcAlpha,
                alpha::OneMinusSrcAlpha,
                alpha::SrcAlpha,
                alpha::OneMinusSrcAlpha,
            )
            + shader_outmap::reset()
    };
    println!("Hello, world!");
}
