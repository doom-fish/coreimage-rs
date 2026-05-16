use core::ffi::c_void;

unsafe extern "C" {
    pub fn ci_object_retain(handle: *mut c_void) -> *mut c_void;
    pub fn ci_object_release(handle: *mut c_void);
    pub fn ci_array_count(handle: *mut c_void) -> usize;
    pub fn ci_array_object_at(handle: *mut c_void, index: usize) -> *mut c_void;
}
