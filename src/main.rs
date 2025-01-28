#![feature(allocator_api)]
#![feature(generic_const_exprs)]
#![feature(slice_split_once)]

pub mod buffer;
pub mod gpucmd;
pub mod queue;
pub mod shader;
pub mod shader_unfun;
pub mod texture;
pub mod vram;
pub mod floater;

use ctru::{prelude::*, services::gfx::Screen};

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
    let inpos = shader::v(0).unwrap();
    let inclr = shader::v(1).unwrap();
    let outpos = shader::o(0).unwrap();
    let outclr = shader::o(1).unwrap();
    let outmap = {
        use gpucmd::{Root,shader_outmap};
        Root + shader_outmap::OutMapTotal(2)
        + shader_outmap::OutMap(0,shader_outmap::PositionX,shader_outmap::PositionY,shader_outmap::PositionZ,shader_outmap::PositionW)
        + shader_outmap::OutMap(1,shader_outmap::ColorR,shader_outmap::ColorG,shader_outmap::ColorB,shader_outmap::ColorA)
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
    let some_shader = {
        use shader::*;
        Builder::new() + mov(outpos, inpos) + mov(outclr, inclr) + end()
    };
    let q = queue::Queue{};
    let some_command = {
        use gpucmd::{CommandEncoder, alpha, cull_face, depth_map, misc, primitive,Finish};
        CommandEncoder::new_with_capacity(512) + depth_map::EnabledScaleOffset(1.0, 0.0)
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
            + primitive::Config {outmap_total_minus_1: 1,primitive_mode: primitive::Mode::Triangles}
            + misc::NumAttr(2)
            + Finish
    };
    q.submit(&some_command);
    let some_other_command = {
        use gpucmd::{CommandEncoder,misc,fixed_attrib,Finish};
        use floater::f32x4tof24x4;
        CommandEncoder::new_with_capacity(256)
          + misc::NumVertices(3)
          + misc::DrawingMode
          + fixed_attrib::Index(0xF)
          + fixed_attrib::Data(f32x4tof24x4([0.0,0.0,-1.0,1.0]))
          + fixed_attrib::Data(f32x4tof24x4([1.0,0.0,0.0,1.0]))
          + fixed_attrib::Data(f32x4tof24x4([1.0,0.0,-1.0,1.0]))
          + fixed_attrib::Data(f32x4tof24x4([0.0,1.0,0.0,1.0]))
          + fixed_attrib::Data(f32x4tof24x4([0.0,1.0,-1.0,1.0]))
          + fixed_attrib::Data(f32x4tof24x4([0.0,0.0,1.0,1.0]))
          + misc::ConfigurationMode
          + misc::ClearPostVertexCache
          + misc::FlushFramebuffer
          + Finish
    };
    println!("Hello, world!");
}
