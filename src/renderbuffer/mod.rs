use crate::{gpucmd::{GpuCmdByMut,mask}, vram::VramAllocation};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ColorFormat {
    RGBA8,
    ///DO NOT USE RGB8 AS A RENDERBUFFER. I REPEAT. DO NOT USE RGB8 AS A RENDERBUFFER
    ///TODO: Properly codify the difference between Textures and RenderBuffers, as well as where they unify
    RGB8,
    RGBA5551,
    RGB565,
    RGBA4
}

impl ColorFormat {
    pub fn bytes_per_pixel(self) -> usize {
        use ColorFormat::*;
        match self {
            RGBA8 => 4,
            RGB8 => 3,
            RGBA5551 => 2,
            RGB565 => 2,
            RGBA4 => 2,
        }
    }
    pub fn gpureg_param(self) -> u32 {
        use ColorFormat::*;
        match self {
            RGBA8 => 2,
            RGB8 => panic!("Whoops"),
            RGBA5551 => 2,
            RGB565 => 2,
            RGBA4 => 2,
        }
    }
}

pub struct ColorBuffer {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) format: ColorFormat,
    pub(crate) allocation: VramAllocation
}

impl ColorBuffer {
    pub fn new(width: u32, height: u32, format: ColorFormat) -> Option<ColorBuffer> {
        let size = (width as usize) * (height as usize) * format.bytes_per_pixel();
        let allocation = VramAllocation::new(size)?;
        Some(ColorBuffer {
            width,
            height,
            format,
            allocation
        })
    }
    pub(crate) fn dim(&self) -> u32 {
        dim(self.width,self.height)
    }
}

impl GpuCmdByMut for &ColorBuffer {
    fn cmd_by_mut<A:std::alloc::Allocator>(self, buf: &mut Vec<u32,A>) {
        use ctru_sys::*;
        buf.extend_from_slice(&[
            self.format.gpureg_param(),
            GPUREG_COLORBUFFER_FORMAT | mask(0xF),
            unsafe {osConvertVirtToPhys((&self.allocation).into()) >> 3},
            GPUREG_COLORBUFFER_LOC | mask(0xF),
            0xF,
            GPUREG_COLORBUFFER_READ | mask(0xF),
            0xF,
            GPUREG_COLORBUFFER_WRITE | mask(0xF),
            self.width | ((self.height - 1) << 12),
            GPUREG_FRAMEBUFFER_DIM | mask(0xF)
        ]);
    }
}

pub(crate) fn dim(width: u32,height: u32) -> u32 {
    (height << 16) | (width & 0xFFFF)
}
