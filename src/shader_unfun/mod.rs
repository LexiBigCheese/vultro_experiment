use derive_more::derive::From;

pub mod dvlb;
pub mod dvlp;
pub mod dvle;

#[derive(From,Debug)]
pub enum Error {
    BytemuckPodcast(bytemuck::PodCastError),
    UnexpectedEof,
    BadGshMode,
    Utf8Error(std::str::Utf8Error)
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Kind {
    Vertex,
    Geometry,
}

#[derive(Clone,Copy,Debug)]
pub enum GshMode {
    Point,
    VariablePrim,
    FixedPrim
}

impl TryFrom<u32> for GshMode {
    type Error=Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use GshMode::*;
        match value {
            0 => Ok(Point),
            1 => Ok(VariablePrim),
            2 => Ok(FixedPrim),
            _ => Err(Error::BadGshMode)
        }
    }
}
