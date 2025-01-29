use crate::buffer::Buffer;

pub struct Texture {
    data: Buffer,
    descriptor: TextureDescriptor
}

#[derive(Clone, Copy)]
pub struct TextureDescriptor {
    pub width: u16,
    pub height: u16,
    pub max_mip: u8,
    pub format: Format,
    pub mode: Mode,
    pub vram: bool
}

impl TextureDescriptor {
    pub fn bytesize_mip0(&self) -> usize {
        // >>3 = /8
        (self.format.bitsize() * self.width as usize * self.height as usize) >> 3
    }
    pub fn bytesize_mip0_to_mipn(bytesize: usize,mip_level: usize) -> usize {
        bytesize >> (2*mip_level)
    }
    pub fn bytesize_total(&self) -> usize {
        let size = self.bytesize_mip0();
        (size - Self::bytesize_mip0_to_mipn(size, self.max_mip as usize + 1)) * 4 / 3
    }
}

pub enum Error {
    ModeNotSupported,
    IncorrectSideLength,
}

impl Texture {
    pub fn new(descriptor: TextureDescriptor) -> Result<Self,Error> {
        if descriptor.mode != Mode::Ordinary {
            return Err(Error::ModeNotSupported);
        }
        if !Texture::valid_size(descriptor.width) || !Texture::valid_size(descriptor.height) {
            return Err(Error::IncorrectSideLength);
        }
        let size = descriptor.bytesize_total();
        let data = Buffer::new(size,descriptor.vram);
        Ok(Texture {
            data,
            descriptor
        })
    }
    pub fn valid_size(size: u16) -> bool {
        size >= 8 && size <= 1024 && size.is_power_of_two()
    }
    pub fn slice(&self, rect: Rect, mip_level: u8, face: Face) -> TextureSlice {
        TextureSlice { texture: self, rect, mip_level, face }
    }
    pub fn descriptor(&self) -> &TextureDescriptor {
        &self.descriptor
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub struct TextureSlice<'a> {
    texture: &'a Texture,
    rect: Rect,
    mip_level: u8,
    face: Face
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Format {
    RGBA8,
    RGB8,
    RGBA5551,
    RGB565,
    RGBA4,
    LA8,
    HILO8,
    L8,
    A8,
    LA4,
    L4,
    A4,
    ETC1,
    ETC1A4,
}

impl Format {
    pub fn bitsize(self) -> usize {
        use Format::*;
        match self {
            RGBA8 => 32,
            RGB8 => 24,
            RGBA5551 | RGB565 | RGBA4 | LA8 | HILO8 => 16,
            L8 | A8 | LA4 | ETC1A4 => 8,
            L4 | A4 | ETC1 => 4,
        }
    }
}

#[derive(Copy,Clone,Eq,PartialEq)]
#[repr(u8)]
pub enum Mode {
    ///2D texture
    Ordinary,
    ///Cube Map
    CubeMap,
    ///2D Shadow Texture
    Shadow2d,
    ///Projection
    Projection,
    ///Shadow Cube Map
    ShadowCube,
    ///Disabled
    Disabled
}

#[derive(Copy,Clone,Eq,PartialEq)]
#[repr(u8)]
pub enum Face {
    D2
}
