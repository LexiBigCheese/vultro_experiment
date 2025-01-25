use std::{
    alloc::{AllocError, Allocator},
    ptr::NonNull,
};

#[derive(Copy, Clone, Default, Debug)]
pub struct VramAllocator;

impl VramAllocator {
    pub fn free_space() -> u32 {
        unsafe { ctru_sys::vramSpaceFree() }
    }
}

unsafe impl Allocator for VramAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let pointer = unsafe { ctru_sys::vramMemAlign(layout.size(), layout.align()) };
        NonNull::new(pointer.cast())
            .map(|ptr| NonNull::slice_from_raw_parts(ptr, layout.size()))
            .ok_or(AllocError)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: std::alloc::Layout) {
        unsafe {
            ctru_sys::vramFree(ptr.as_ptr().cast());
        }
    }
}
