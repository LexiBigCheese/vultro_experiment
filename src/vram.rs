use std::ffi::c_void;

pub fn free_space() -> u32 {
    unsafe { ctru_sys::vramSpaceFree() }
}

pub struct VramAllocation {
    ptr: *mut c_void,
}

impl VramAllocation {
    ///Will align to 0x80 bytes
    pub fn new(size: usize) -> Option<VramAllocation> {
        let ptr = unsafe { ctru_sys::vramAlloc(size) };
        if ptr.is_null() {
            None
        } else {
            Some(VramAllocation { ptr })
        }
    }
}

impl<T> Into<*mut T> for &VramAllocation {
    fn into(self) -> *mut T {
        self.ptr.cast()
    }
}

impl<T> Into<*const T> for &VramAllocation {
    fn into(self) -> *const T {
        self.ptr.cast_const().cast()
    }
}

impl Drop for VramAllocation {
    fn drop(&mut self) {
        unsafe {
            ctru_sys::vramFree(self.ptr);
        }
    }
}
