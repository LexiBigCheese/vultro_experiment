// There are functions which take Aligned Slices.
// We also sometimes can take `Linear | VRAM`, other times it's only one or the other.
// To solve this, we need very good buffers which can have an align at compile time.

pub struct LinearBuf<T> {
    ptr: *mut T,
    len: usize
}
pub struct VramBuf<T> {
    ptr: *mut T,
    len: usize
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
pub struct LinearSliceMut<'a,T> {
    ptr: *mut T,
    len: usize,
    _life: std::marker::PhantomData<&'a mut T>
}

impl<'a,T> LinearSliceMut<'a,T> {
    pub unsafe fn map(self) -> &'a [T] {
        unsafe {std::slice::from_raw_parts(self.ptr.cast_const(), self.len)}
    }
}
#[derive(Clone, Copy)]
pub struct VramSliceMut<'a,T> {
    ptr: *mut T,
    len: usize,
    _life: std::marker::PhantomData<&'a mut T>
}

pub trait LinearOrVram {
    type Stored;
    fn ptr(self) -> *mut Self::Stored;
    fn len(self) -> usize;
}

impl<'a,T> LinearOrVram for LinearSliceMut<'a,T> {
    type Stored = T;
    fn ptr(self) -> *mut Self::Stored {
        self.ptr
    }
    fn len(self) -> usize {
        self.len
    }
}

impl<'a,T> LinearOrVram for VramSliceMut<'a,T> {
    type Stored = T;
    fn ptr(self) -> *mut Self::Stored {
        self.ptr
    }
    fn len(self) -> usize {
        self.len
    }
}
