use std::{ffi::c_void, ops::RangeBounds};

use ctru::linear::LinearAllocator;

pub enum Buffer {
    Linear(Box<[u8],AlignedLinearAllocator>),
    ///Note: DO NOT MODIFY UNLESS YOU KNOW WHAT YOU ARE DOING
    Vram{addr: *mut c_void,size: usize}
}

impl Buffer {
    pub fn new(size: usize,vram: bool) -> Self {
        if vram {
            let addr = unsafe { ctru_sys::vramMemAlign(size,8) };
            Self::Vram { addr, size }
        } else {
            Self::Linear(unsafe {Box::new_uninit_slice_in(size,AlignedLinearAllocator).assume_init()})
        }
    }
    pub fn slice<S: RangeBounds<usize> + std::slice::SliceIndex<[u8], Output = [u8]>>(&mut self, bounds: S) -> BufferSlice {
        match self {
            Buffer::Linear(x) => BufferSlice::Linear(&mut x[bounds]),
            Buffer::Vram { addr, size } => {
                let start = match bounds.start_bound() {
                    std::ops::Bound::Included(&x) => x,
                    std::ops::Bound::Excluded(&x) => x + 1,
                    std::ops::Bound::Unbounded => 0,
                };
                let end = match bounds.end_bound() {
                    std::ops::Bound::Included(&x) => x + 1,
                    std::ops::Bound::Excluded(&x) => x,
                    std::ops::Bound::Unbounded => *size,
                };
                assert!((0..(*size)).contains(&start));
                assert!((0..(*size)).contains(&end));
                BufferSlice::Vram { addr: unsafe{ (*addr).byte_add(start) }, size: (end-start) }
            },
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if let Buffer::Vram { addr, .. } = self {
            unsafe {
                ctru_sys::vramFree(*addr);
            }
        }
    }
}

pub enum BufferSlice<'a> {
    Linear(&'a mut [u8]),
    ///Note: DO NOT MODIFY UNLESS YOU KNOW WHAT YOU ARE DOING
    Vram{addr: *mut c_void,size: usize}
}

impl<'a> BufferSlice<'a> {
    pub fn map(&self) -> Option<&[u8]> {
        match self {
            BufferSlice::Linear(x) => Some(*x),
            BufferSlice::Vram { .. } => None,
        }
    }
    pub fn map_mut(&mut self) -> Option<&mut [u8]> {
        match self {
            BufferSlice::Linear(x) => Some(*x),
            BufferSlice::Vram { .. } => None,
        }
    }
    pub(crate) fn start_addr(&mut self) -> *mut c_void {
        match self {
            BufferSlice::Linear(x) => x.as_mut_ptr().cast(),
            BufferSlice::Vram { addr, .. } => *addr,
        }
    }
    pub(crate) fn end_addr(&self) -> *const c_void {
        match self {
            BufferSlice::Linear(x) => x.as_ptr_range().end.cast(),
            BufferSlice::Vram { addr, size } => unsafe {(*addr).byte_add(*size)},
        }
    }
    pub fn size(&self) -> usize {
        match self {
            BufferSlice::Linear(x) => x.len(),
            BufferSlice::Vram { size, .. } => *size,
        }
    }
}

pub struct AlignedLinearAllocator;

unsafe impl std::alloc::Allocator for AlignedLinearAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        let layout = layout.align_to(8).expect("Could not 8 Byte align Texture");
        LinearAllocator.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
        let layout = layout.align_to(8).expect("Could not 8 Byte align Texture");
        unsafe {LinearAllocator.deallocate(ptr,layout);}
    }
}
