use core::ffi::{c_char, c_void};

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_apply_passthrough`.
    pub fn ci_image_processor_apply_passthrough(
        image: *mut c_void,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_invocation_json`.
    pub fn ci_image_processor_last_invocation_json() -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_invocation_input_count`.
    pub fn ci_image_processor_last_invocation_input_count() -> usize;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_invocation_has_input`.
    pub fn ci_image_processor_last_invocation_has_input() -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_input_region`.
    pub fn ci_image_processor_last_input_region(
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_output_region`.
    pub fn ci_image_processor_last_output_region(
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_input_bytes_per_row`.
    pub fn ci_image_processor_last_input_bytes_per_row() -> usize;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_output_bytes_per_row`.
    pub fn ci_image_processor_last_output_bytes_per_row() -> usize;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_input_format`.
    pub fn ci_image_processor_last_input_format() -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_output_format`.
    pub fn ci_image_processor_last_output_format() -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_input_has_pixel_buffer`.
    pub fn ci_image_processor_last_input_has_pixel_buffer() -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_output_has_pixel_buffer`.
    pub fn ci_image_processor_last_output_has_pixel_buffer() -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_input_has_metal_texture`.
    pub fn ci_image_processor_last_input_has_metal_texture() -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_output_has_metal_texture`.
    pub fn ci_image_processor_last_output_has_metal_texture() -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_input_digest`.
    pub fn ci_image_processor_last_input_digest() -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_output_digest`.
    pub fn ci_image_processor_last_output_digest() -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_input_roi_tile_index`.
    pub fn ci_image_processor_last_input_roi_tile_index() -> i64;
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_last_input_roi_tile_count`.
    pub fn ci_image_processor_last_input_roi_tile_count() -> i64;
}
