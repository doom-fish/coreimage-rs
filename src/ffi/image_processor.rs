use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ci_image_processor_apply_passthrough(
        image: *mut c_void,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_image_processor_last_invocation_json() -> *mut c_char;
    pub fn ci_image_processor_last_invocation_input_count() -> usize;
    pub fn ci_image_processor_last_invocation_has_input() -> bool;
    pub fn ci_image_processor_last_input_region(
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn ci_image_processor_last_output_region(
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn ci_image_processor_last_input_bytes_per_row() -> usize;
    pub fn ci_image_processor_last_output_bytes_per_row() -> usize;
    pub fn ci_image_processor_last_input_format() -> i32;
    pub fn ci_image_processor_last_output_format() -> i32;
    pub fn ci_image_processor_last_input_has_pixel_buffer() -> bool;
    pub fn ci_image_processor_last_output_has_pixel_buffer() -> bool;
    pub fn ci_image_processor_last_input_has_metal_texture() -> bool;
    pub fn ci_image_processor_last_output_has_metal_texture() -> bool;
    pub fn ci_image_processor_last_input_digest() -> *mut c_char;
    pub fn ci_image_processor_last_output_digest() -> *mut c_char;
    pub fn ci_image_processor_last_input_roi_tile_index() -> i64;
    pub fn ci_image_processor_last_input_roi_tile_count() -> i64;
}
