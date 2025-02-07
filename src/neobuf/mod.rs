// There are functions which take Aligned Slices.
// We also sometimes can take `Linear | VRAM`, other times it's only one or the other.
// To solve this, we need very good buffers which can have an align at compile time.

use std::ops::RangeBounds;

pub trait NeoSlice<'a> {
    type T;
    type Slice: NeoSliceCons<'a, T = Self::T>;
    fn ptr(&'a self) -> *mut Self::T;
    fn len(&'a self) -> usize;
    fn slice_internal<S: RangeBounds<usize>>(&'a self, bounds: S) -> Option<(*mut Self::T, usize)> {
        let ptr = self.ptr();
        let len = self.len();
        let start = match bounds.start_bound() {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match bounds.end_bound() {
            std::ops::Bound::Included(&x) => x + 1,
            std::ops::Bound::Excluded(&x) => x,
            std::ops::Bound::Unbounded => len,
        };
        if !((0..len).contains(&start)) {
            return None;
        }
        if !((0..=len).contains(&end)) {
            return None;
        }
        Some((unsafe { ptr.add(start) }, end - start))
    }
    fn slice_aligned_8_internal<S: RangeBounds<usize>>(
        &'a self,
        bounds: S,
    ) -> Option<(*mut Self::T, usize)> {
        let sliced = self.slice_internal(bounds)?;
        if ((sliced.0 as u32) & 0b111) != 0 {
            return None;
        }
        if (unsafe { sliced.0.add(sliced.1) as u32 } & 0b111) != 0 {
            return None;
        }
        Some(sliced)
    }
    fn slice<S: RangeBounds<usize>>(&'a self, bounds: S) -> Option<Self::Slice> {
        let sliced = self.slice_internal(bounds)?;
        Some(Self::Slice::neo_slice_cons(sliced.0, sliced.1))
    }
    fn slice_aligned_8<S: RangeBounds<usize>>(&'a self, bounds: S) -> Option<Self::Slice> {
        let sliced = self.slice_aligned_8_internal(bounds)?;
        Some(Self::Slice::neo_slice_cons(sliced.0, sliced.1))
    }
}

pub trait NeoSliceMut<'a>: NeoSlice<'a> {
    type SliceMut: NeoSliceCons<'a, T = Self::T>;
    fn slice_mut<S: RangeBounds<usize>>(&'a mut self, bounds: S) -> Option<Self::SliceMut> {
        let sliced = self.slice_internal(bounds)?;
        Some(Self::SliceMut::neo_slice_cons(sliced.0, sliced.1))
    }
    fn slice_aligned_8_mut<S: RangeBounds<usize>>(
        &'a mut self,
        bounds: S,
    ) -> Option<Self::SliceMut> {
        let sliced = self.slice_aligned_8_internal(bounds)?;
        Some(Self::SliceMut::neo_slice_cons(sliced.0, sliced.1))
    }
}

pub trait NeoSliceCons<'a>: NeoSlice<'a> {
    fn neo_slice_cons(ptr: *mut Self::T, len: usize) -> Self;
}

pub struct LinearBuf<T> {
    ptr: *mut T,
    len: usize,
}
pub struct VramBuf<T> {
    ptr: *mut T,
    len: usize,
}

impl<'a, T: 'a> NeoSlice<'a> for LinearBuf<T> {
    type T = T;
    type Slice = LinearSlice<'a, T>;
    fn len(&self) -> usize {
        self.len
    }
    fn ptr(&self) -> *mut Self::T {
        self.ptr
    }
}

impl<'a, T: 'a> NeoSlice<'a> for VramBuf<T> {
    type T = T;
    type Slice = VramSlice<'a, T>;
    fn len(&self) -> usize {
        self.len
    }
    fn ptr(&self) -> *mut Self::T {
        self.ptr
    }
}

impl<'a, T: 'a> NeoSliceMut<'a> for LinearBuf<T> {
    type SliceMut = LinearSliceMut<'a, T>;
}

impl<'a, T: 'a> NeoSliceMut<'a> for VramBuf<T> {
    type SliceMut = VramSliceMut<'a, T>;
}

impl<T> Drop for LinearBuf<T> {
    fn drop(&mut self) {
        unsafe {
            ctru_sys::linearFree(self.ptr.cast());
        }
    }
}
impl<T> Drop for VramBuf<T> {
    fn drop(&mut self) {
        unsafe {
            ctru_sys::vramFree(self.ptr.cast());
        }
    }
}

#[derive(Clone, Copy)]
pub struct LinearSlice<'a, T> {
    ptr: *const T,
    len: usize,
    _life: std::marker::PhantomData<&'a T>,
}

impl<'a, T> LinearSlice<'a, T> {
    pub unsafe fn map(self) -> &'a [T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}
#[derive(Clone, Copy)]
pub struct VramSlice<'a, T> {
    ptr: *const T,
    len: usize,
    _life: std::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> NeoSlice<'a> for LinearSlice<'a, T> {
    type T = T;
    type Slice = LinearSlice<'a, T>;
    fn len(&self) -> usize {
        self.len
    }
    fn ptr(&self) -> *mut Self::T {
        self.ptr.cast_mut()
    }
}

impl<'a, T: 'a> NeoSliceCons<'a> for LinearSlice<'a, T> {
    fn neo_slice_cons(ptr: *mut Self::T, len: usize) -> Self {
        LinearSlice {
            ptr: ptr.cast_const(),
            len,
            _life: Default::default(),
        }
    }
}

impl<'a, T: 'a> NeoSlice<'a> for VramSlice<'a, T> {
    type T = T;
    type Slice = VramSlice<'a, T>;
    fn len(&self) -> usize {
        self.len
    }
    fn ptr(&self) -> *mut Self::T {
        self.ptr.cast_mut()
    }
}

impl<'a, T: 'a> NeoSliceCons<'a> for VramSlice<'a, T> {
    fn neo_slice_cons(ptr: *mut Self::T, len: usize) -> Self {
        VramSlice {
            ptr: ptr.cast_const(),
            len,
            _life: Default::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct LinearSliceMut<'a, T> {
    ptr: *mut T,
    len: usize,
    _life: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> LinearSliceMut<'a, T> {
    pub unsafe fn map(self) -> &'a [T] {
        unsafe { std::slice::from_raw_parts(self.ptr.cast_const(), self.len) }
    }
}

impl<'a, T: 'a> NeoSlice<'a> for LinearSliceMut<'a, T> {
    type T = T;
    type Slice = LinearSlice<'a, T>;
    fn len(&self) -> usize {
        self.len
    }
    fn ptr(&self) -> *mut Self::T {
        self.ptr
    }
}

impl<'a, T: 'a> NeoSliceMut<'a> for LinearSliceMut<'a, T> {
    type SliceMut = LinearSliceMut<'a, T>;
}

impl<'a, T: 'a> NeoSliceCons<'a> for LinearSliceMut<'a, T> {
    fn neo_slice_cons(ptr: *mut Self::T, len: usize) -> Self {
        LinearSliceMut {
            ptr,
            len,
            _life: Default::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct VramSliceMut<'a, T> {
    ptr: *mut T,
    len: usize,
    _life: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T: 'a> NeoSlice<'a> for VramSliceMut<'a, T> {
    type T = T;
    type Slice = VramSlice<'a, T>;
    fn len(&self) -> usize {
        self.len
    }
    fn ptr(&self) -> *mut Self::T {
        self.ptr
    }
}

impl<'a, T: 'a> NeoSliceMut<'a> for VramSliceMut<'a, T> {
    type SliceMut = VramSliceMut<'a, T>;
}

impl<'a, T: 'a> NeoSliceCons<'a> for VramSliceMut<'a, T> {
    fn neo_slice_cons(ptr: *mut Self::T, len: usize) -> Self {
        VramSliceMut {
            ptr,
            len,
            _life: Default::default(),
        }
    }
}

pub trait LinearOrVramMut<'a>: NeoSliceMut<'a> {}

impl<'a,T:'a> LinearOrVramMut<'a> for LinearBuf<T> {}
impl<'a,T:'a> LinearOrVramMut<'a> for VramBuf<T> {}
impl<'a,T:'a> LinearOrVramMut<'a> for LinearSliceMut<'a,T> {}
impl<'a,T:'a> LinearOrVramMut<'a> for VramSliceMut<'a,T> {}

pub trait LinearOrVram<'a>: NeoSlice<'a> {}

impl<'a,T:'a> LinearOrVram<'a> for LinearBuf<T> {}
impl<'a,T:'a> LinearOrVram<'a> for VramBuf<T> {}
impl<'a,T:'a> LinearOrVram<'a> for LinearSliceMut<'a,T> {}
impl<'a,T:'a> LinearOrVram<'a> for VramSliceMut<'a,T> {}
impl<'a,T:'a> LinearOrVram<'a> for LinearSlice<'a,T> {}
impl<'a,T:'a> LinearOrVram<'a> for VramSlice<'a,T> {}

pub trait LinearMut<'a>: NeoSliceMut<'a> {}

impl<'a,T:'a> LinearMut<'a> for LinearBuf<T> {}
impl<'a,T:'a> LinearMut<'a> for LinearSliceMut<'a,T> {}

pub trait Linear<'a>: NeoSlice<'a> {}

impl<'a,T:'a> Linear<'a> for LinearBuf<T> {}
impl<'a,T:'a> Linear<'a> for LinearSliceMut<'a,T> {}
impl<'a,T:'a> Linear<'a> for LinearSlice<'a,T> {}

pub trait VramMut<'a>: NeoSliceMut<'a> {}

impl<'a,T:'a> VramMut<'a> for VramBuf<T> {}
impl<'a,T:'a> VramMut<'a> for VramSliceMut<'a,T> {}

pub trait Vram<'a>: NeoSlice<'a> {}

impl<'a,T:'a> Vram<'a> for VramBuf<T> {}
impl<'a,T:'a> Vram<'a> for VramSliceMut<'a,T> {}
impl<'a,T:'a> Vram<'a> for VramSlice<'a,T> {}
