use ctru_sys::*;

use super::{GpuCmd, impl_gpucmd, mask};

#[derive(Clone, Copy)]
pub struct No;

#[derive(Clone, Copy)]
pub struct FrontCCW;

#[derive(Clone, Copy)]
pub struct BackCCW;

impl_gpucmd!(No, |_this: No| 0, GPUREG_FACECULLING_CONFIG);
impl_gpucmd!(FrontCCW, |_this: FrontCCW| 1, GPUREG_FACECULLING_CONFIG);
impl_gpucmd!(BackCCW, |_this: BackCCW| 2, GPUREG_FACECULLING_CONFIG);
