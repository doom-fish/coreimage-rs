use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ci_image_processor_apply_passthrough(
        image: *mut c_void,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_image_processor_last_invocation_json() -> *mut c_char;
}
