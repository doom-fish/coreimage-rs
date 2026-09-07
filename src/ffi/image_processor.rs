use core::ffi::{c_char, c_void};

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_image_processor_apply_passthrough`.
    pub fn ci_image_processor_apply_passthrough(
        image: *mut c_void,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_new`.
    pub fn ci_image_processor_invocation_snapshot_new() -> *mut c_void;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_json`.
    pub fn ci_image_processor_invocation_snapshot_json(snapshot: *mut c_void) -> *mut c_char;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_input_count`.
    pub fn ci_image_processor_invocation_snapshot_input_count(snapshot: *mut c_void) -> usize;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_has_input`.
    pub fn ci_image_processor_invocation_snapshot_has_input(snapshot: *mut c_void) -> bool;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_input_region`.
    pub fn ci_image_processor_invocation_snapshot_input_region(
        snapshot: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_output_region`.
    pub fn ci_image_processor_invocation_snapshot_output_region(
        snapshot: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_input_bytes_per_row`.
    pub fn ci_image_processor_invocation_snapshot_input_bytes_per_row(
        snapshot: *mut c_void,
    ) -> usize;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_output_bytes_per_row`.
    pub fn ci_image_processor_invocation_snapshot_output_bytes_per_row(
        snapshot: *mut c_void,
    ) -> usize;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_input_format`.
    pub fn ci_image_processor_invocation_snapshot_input_format(snapshot: *mut c_void) -> i32;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_output_format`.
    pub fn ci_image_processor_invocation_snapshot_output_format(snapshot: *mut c_void) -> i32;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_input_has_pixel_buffer`.
    pub fn ci_image_processor_invocation_snapshot_input_has_pixel_buffer(
        snapshot: *mut c_void,
    ) -> bool;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_output_has_pixel_buffer`.
    pub fn ci_image_processor_invocation_snapshot_output_has_pixel_buffer(
        snapshot: *mut c_void,
    ) -> bool;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_input_has_metal_texture`.
    pub fn ci_image_processor_invocation_snapshot_input_has_metal_texture(
        snapshot: *mut c_void,
    ) -> bool;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_output_has_metal_texture`.
    pub fn ci_image_processor_invocation_snapshot_output_has_metal_texture(
        snapshot: *mut c_void,
    ) -> bool;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_input_digest`.
    pub fn ci_image_processor_invocation_snapshot_input_digest(
        snapshot: *mut c_void,
    ) -> *mut c_char;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_output_digest`.
    pub fn ci_image_processor_invocation_snapshot_output_digest(
        snapshot: *mut c_void,
    ) -> *mut c_char;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_input_roi_tile_index`.
    pub fn ci_image_processor_invocation_snapshot_input_roi_tile_index(
        snapshot: *mut c_void,
    ) -> i64;
    /// Calls the `CoreImage` framework counterpart for `ci_image_processor_invocation_snapshot_input_roi_tile_count`.
    pub fn ci_image_processor_invocation_snapshot_input_roi_tile_count(
        snapshot: *mut c_void,
    ) -> i64;
}
