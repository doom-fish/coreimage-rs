use core::ffi::c_char;

unsafe extern "C" {
    pub fn ci_image_format_value(code: i32) -> i32;
    pub fn ci_context_option_name(code: i32) -> *mut c_char;
    pub fn ci_image_option_name(code: i32) -> *mut c_char;
    pub fn ci_image_auto_adjust_option_name(code: i32) -> *mut c_char;
    pub fn ci_image_representation_option_name(code: i32) -> *mut c_char;
    pub fn ci_raw_decoder_version_name(code: i32) -> *mut c_char;
}
