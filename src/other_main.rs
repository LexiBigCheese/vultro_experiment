#![feature(allocator_api)]
#![feature(generic_const_exprs)]
#![feature(slice_split_once)]

use super::buffer;
use super::floater;
use super::gpucmd;
use super::queue;
use super::renderbuffer;
use super::shader;

use ctru::{
    prelude::*,
    services::gfx::{Flush, Screen, Swap},
};
use ctru_sys::{gspWaitForAnyEvent, GSPGPU_TriggerCmdReqQueue};

fn other_main() {
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
    let inpos = shader::v(0).unwrap();
    let inclr = shader::v(1).unwrap();
    let outpos = shader::o(0).unwrap();
    let outclr = shader::o(1).unwrap();
    let outmap = {
        use gpucmd::{Root, shader_outmap};
        Root + shader_outmap::OutMapTotal(2)
            + shader_outmap::OutMap(
                0,
                shader_outmap::PositionX,
                shader_outmap::PositionY,
                shader_outmap::PositionZ,
                shader_outmap::PositionW,
            )
            + shader_outmap::OutMap(
                1,
                shader_outmap::ColorR,
                shader_outmap::ColorG,
                shader_outmap::ColorB,
                shader_outmap::ColorA,
            )
            - shader_outmap::UseTextureCoordinates
            + shader_outmap::Clock {
                position_z: true,
                color: true,
                texcoord0: false,
                texcoord1: false,
                texcoord2: false,
                texcoord0w: false,
                normquat_or_view: false,
            }
    };
    let mut shader_entrypoint = None;
    let some_shader = {
        use shader::*;
        Builder::new()
            + Label(&mut shader_entrypoint)
            + mov(outpos, inpos)
            + mov(outclr, inclr)
            + end()
    };
    let shader_entrypoint = shader_entrypoint.unwrap();
    let q = queue::Queue {};
    let some_command = {
        use gpucmd::{CommandEncoder, Finish, alpha, cull_face, depth_map, misc, primitive};
        CommandEncoder::new_with_capacity(512)
            + depth_map::EnabledScaleOffset(1.0, 0.0)
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
            + (some_shader, shader::VSH)
            + outmap
            + misc::VshEntrypoint(shader_entrypoint)
            + primitive::Config {
                outmap_total_minus_1: 1,
                primitive_mode: primitive::Mode::Triangles,
            }
            + misc::NumAttr(2)
            + &bottom_color_buffer
            + Finish
    };
    println!("About to submit command");
    q.submit(&some_command)
        .expect("Could not submit command buffer to queue");
    unsafe {
        GSPGPU_TriggerCmdReqQueue();
    }
    println!("About to wait for event");
    // unsafe {gspWaitForAnyEvent()};
    ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::P3D, false);
    //TODO: Create Render Buffer
    let some_other_command = {
        use floater::f32x4tof24x4;
        use gpucmd::{CommandEncoder, Finish, fixed_attrib, misc};
        CommandEncoder::new_with_capacity(256)
            + misc::NumVertices(3)
            + misc::NumAttr(2)
            + misc::DrawingMode
            + fixed_attrib::Index(0xF)
            + fixed_attrib::Data(f32x4tof24x4([0.0, 0.0, -1.0, 1.0]))
            + fixed_attrib::Data(f32x4tof24x4([1.0, 0.0, 0.0, 1.0]))
            + fixed_attrib::Data(f32x4tof24x4([1.0, 0.0, -1.0, 1.0]))
            + fixed_attrib::Data(f32x4tof24x4([0.0, 1.0, 0.0, 1.0]))
            + fixed_attrib::Data(f32x4tof24x4([0.0, 1.0, -1.0, 1.0]))
            + fixed_attrib::Data(f32x4tof24x4([0.0, 0.0, 1.0, 1.0]))
            + misc::ConfigurationMode
            + misc::ClearPostVertexCache
            + misc::FlushFramebuffer
            + Finish
    };
    println!("About to enter main loop");
    println!("Know your rights: {}", unsafe {
        ctru_sys::gspHasGpuRight()
    });
    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        println!("Loop");
        ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::VBlank1, false);
        if !hid.keys_down().contains(KeyPad::A) {
            continue;
        }
        let _ = q.submit(&some_other_command);
        // gfx.wait_for_vblank();
        // unsafe {while gspWaitForAnyEvent() != ctru_sys::GSPGPU_EVENT_P3D {}};
        ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::P3D, false);
        let _ = q.transfer_colorbuffer_to_framebuffer(
            &bottom_color_buffer,
            bottom_screen.raw_framebuffer(),
            queue::TransferFlags {
                flip_vert: false,
                tiled_out: false,
                output_width_less_than_input_width: false,
                texture_copy: false,
                tiled_to_tiled: false,
                input_color_format: queue::TransferFormat::RGBA8,
                output_color_format: queue::TransferFormat::RGBA8,
                block_tiling_mode: false,
                scale_down_filter: queue::ScaleDownFilter::None,
            },
        );
        // unsafe {gspWaitForAnyEvent()};
        std::thread::yield_now();
        println!("A");
        unsafe {
            GSPGPU_TriggerCmdReqQueue();
        }
        std::thread::yield_now();
        ctru::services::gspgpu::wait_for_event(ctru::services::gspgpu::Event::PPF, false);
        println!("B");
        bottom_screen.flush_buffers();
        println!("C");
        bottom_screen.swap_buffers();
        println!("Loop End");
    }
    println!("Hello, world!");
}
