use core::ffi::c_void;

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_object_retain`.
    pub fn ci_object_retain(handle: *mut c_void) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_object_release`.
    pub fn ci_object_release(handle: *mut c_void);
/// Calls the `CoreImage` framework counterpart for `ci_array_count`.
    pub fn ci_array_count(handle: *mut c_void) -> usize;
/// Calls the `CoreImage` framework counterpart for `ci_array_object_at`.
    pub fn ci_array_object_at(handle: *mut c_void, index: usize) -> *mut c_void;
}
