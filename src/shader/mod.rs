type Swizzle = (u8,u8,u8,u8);

pub const S: Swizzle = (0,1,2,3);

#[derive(Clone,Debug)]
pub enum Error {
    NoSuchRegister
}

fn swizzle_to_u64(s: Swizzle) -> u64 {
    (0
    | ((s.3 & 3) << 0)
    | ((s.2 & 3) << 2)
    | ((s.1 & 3) << 4)
    | ((s.0 & 3) << 6)) as u64
}

#[derive(Clone,Copy,Debug,PartialEq, Eq)]
pub struct Mask(pub bool,pub bool,pub bool,pub bool);

impl Mask {
    pub const XYZW: Mask = Mask(true,true,true,true);
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OpDesc {
    /// X, Y, Z, W
    dest: Mask,
    source1_neg: bool,
    source1: Swizzle,
    source2_neg: bool,
    source2: Swizzle,
    source3_neg: bool,
    source3: Swizzle
}

impl Default for OpDesc {
    fn default() -> Self {
        OpDesc {
            dest: Mask::XYZW,
            source1_neg: false,
            source1: S,
            source2_neg: false,
            source2: S,
            source3_neg: false,
            source3: S,
        }
    }
}

impl Into<u64> for OpDesc {
    fn into(self) -> u64 {
        0
        | if self.dest.0      {1 << 0x3} else {0}
        | if self.dest.1      {1 << 0x2} else {0}
        | if self.dest.2      {1 << 0x1} else {0}
        | if self.dest.3      {1 << 0x0} else {0}
        | if self.source1_neg {1 << 0x4} else {0}
        | (swizzle_to_u64(self.source1) << 0x5)
        | if self.source2_neg {1 << 0xD} else {0}
        | (swizzle_to_u64(self.source2) << 0xE)
        | if self.source3_neg {1 << 0x16} else {0}
        | (swizzle_to_u64(self.source3) << 0x17)

    }
}

impl std::hash::Hash for OpDesc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64((*self).into());
    }
}

pub struct Builder {
    prog: Vec<u32>,
    opdesc: Vec<u64>,
    opdesc_map: std::collections::HashMap<OpDesc,u32>,
}

impl Builder {
    pub(crate) fn add_opdesc(&mut self,opdesc:OpDesc) -> u32 {
        if let Some(x) = self.opdesc_map.get(&opdesc) {
            *x
        } else {
            let as_u: u64 = opdesc.into();
            let current = self.opdesc.len() as u32;
            self.opdesc_map.insert(opdesc,current);
            self.opdesc.push(as_u);
            current
        }
    }
}

pub trait AddToBuilder {
    fn add_to_builder(self,b: Builder) -> Builder;
}

impl<T:AddToBuilder> std::ops::Add<T> for Builder {
    type Output = Builder;
    fn add(self, rhs: T) -> Self::Output {
        rhs.add_to_builder(self)
    }
}

pub struct Label<'a>(pub &'a mut Option<u32>);

impl<'a> AddToBuilder for Label<'a> {
    fn add_to_builder(self,b: Builder) -> Builder {
        *self.0 = Some(b.prog.len() as u32);
        b
    }
}

pub struct DstReg { reg: u32, mask: Mask }
pub struct SrcRegLong { reg: u32, neg: bool, swizzle: Swizzle }
pub struct SrcRegShort { reg: u32, neg: bool, swizzle: Swizzle }

pub struct OutReg { reg: u32 }
///Output Register `0..=15`
pub fn o(reg: u32) -> Result<OutReg,Error> {
    if reg < 0x10 {
        Ok(OutReg {reg})
    } else {
        Err(Error::NoSuchRegister)
    }
}
pub struct InReg {reg: u32}
///Input Register `0..=15`
pub fn v(reg: u32) -> Result<InReg,Error> {
    if reg < 0x10 {
        Ok(InReg {reg})
    } else {
        Err(Error::NoSuchRegister)
    }
}
pub struct GeneralReg {reg: u32}
///General Register `0..=15`
pub fn r(reg: u32) -> Result<GeneralReg,Error> {
    if reg < 0x10 {
        Ok(GeneralReg {reg})
    } else {
        Err(Error::NoSuchRegister)
    }
}
pub struct UniformReg {reg: u32}
///Constant/Uniform Register `0..=95`
pub fn c(reg: u32) -> Result<UniformReg,Error> {
    if reg < 96 {
        Ok(UniformReg {reg})
    } else {
        Err(Error::NoSuchRegister)
    }
}
pub struct IntReg {reg: u32}
///Int Constant Register `0..=3`
pub fn i(reg: u32) -> Result<IntReg,Error> {
    if reg < 4 {
        Ok(IntReg{reg})
    } else {
        Err(Error::NoSuchRegister)
    }
}
pub struct BoolReg {reg: u32}
///Bool Constant Register `0..=15`
pub fn b(reg: u32) -> Result<BoolReg,Error> {
    if reg < 0x10 {
        Ok(BoolReg{reg})
    } else {
        Err(Error::NoSuchRegister)
    }
}

#[derive(Clone, Copy,PartialEq, Eq)]
#[repr(u32)]
pub enum Addr {
    None,
    X,
    Y,
    Loop
}

impl Into<DstReg> for (OutReg,Mask) {
    fn into(self) -> DstReg {
        DstReg {
            reg: self.0.reg,
            mask: self.1
        }
    }
}

impl Into<DstReg> for OutReg {
    fn into(self) -> DstReg {
        (self,Mask::XYZW).into()
    }
}

impl Into<DstReg> for (GeneralReg,Mask) {
    fn into(self) -> DstReg {
        DstReg {
            reg: self.0.reg + 0x10,
            mask: self.1
        }
    }
}

impl Into<DstReg> for GeneralReg {
    fn into(self) -> DstReg {
        (self,Mask::XYZW).into()
    }
}

impl Into<SrcRegShort> for (InReg,bool,Swizzle) {
    fn into(self) -> SrcRegShort {
        SrcRegShort { reg: self.0.reg, neg: self.1, swizzle: self.2 }
    }
}

impl Into<SrcRegShort> for (GeneralReg,bool,Swizzle) {
    fn into(self) -> SrcRegShort {
        SrcRegShort { reg: self.0.reg+0x10, neg: self.1, swizzle: self.2 }
    }
}

impl Into<SrcRegLong> for (InReg,bool,Swizzle) {
    fn into(self) -> SrcRegLong {
        SrcRegLong { reg: self.0.reg, neg: self.1, swizzle: self.2 }
    }
}

impl Into<SrcRegLong> for (GeneralReg,bool,Swizzle) {
    fn into(self) -> SrcRegLong {
        SrcRegLong { reg: self.0.reg+0x10, neg: self.1, swizzle: self.2 }
    }
}

impl Into<SrcRegLong> for (UniformReg,bool,Swizzle) {
    fn into(self) -> SrcRegLong {
        SrcRegLong { reg: self.0.reg+0x20, neg: self.1, swizzle: self.2 }
    }
}

pub struct Format1u {
    pub(crate) source1: SrcRegLong,
    pub(crate) source1_addr: Addr,
    pub(crate) dest: DstReg,
    pub(crate) opcode: u32
}

impl Format1u {
    pub(crate) fn new(opcode: u32,dest: DstReg,source1:SrcRegLong) -> Format1u {
        Format1u {
            opcode,dest,source1,
            source1_addr: Addr::None
        }
    }
}

impl std::ops::Mul<Addr> for Format1u {
    type Output=Format1u;

    fn mul(mut self, rhs: Addr) -> Self::Output {
        self.source1_addr = rhs;
        self
    }
}

impl AddToBuilder for Format1u {
    fn add_to_builder(self,mut b: Builder) -> Builder {
        let opdesc = OpDesc {
            dest: self.dest.mask,
            source1_neg: self.source1.neg,
            source1: self.source1.swizzle,
            ..Default::default()
        };
        let opdesc = b.add_opdesc(opdesc);
        b.prog.push(
            0 | opdesc
            | (self.source1.reg << 0x7)
            | ((self.source1_addr as u32) << 0x13)
            | (self.dest.reg << 0x15)
            | (self.opcode << 0x1A)
        );
        b
    }
}

pub fn mov(dst: impl Into<DstReg>, src: impl Into<SrcRegLong>) -> Format1u {
    Format1u::new(
        0x13,
        dst.into(),
        src.into()
    )
}
