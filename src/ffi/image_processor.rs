use core::ffi::{c_char, c_void};

use super::kernel::{RustContextReleaseCallback, RustRegionOfInterestCallback};

pub type RustProcessorCallback = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        input_count: usize,
        input_bases: *const *const u8,
        input_bytes_per_row: *const usize,
        input_formats: *const i32,
        input_regions: *const f64,
        output_base: *mut u8,
        output_bytes_per_row: usize,
        output_format: i32,
        output_x: f64,
        output_y: f64,
        output_width: f64,
        output_height: f64,
        out_error: *mut *mut c_char,
    ) -> bool,
>;

unsafe extern "C" {
    pub fn ci_image_processor_apply_closure(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        inputs: *const *mut c_void,
        input_count: usize,
        format: i32,
        context: *mut c_void,
        process: RustProcessorCallback,
        region_of_interest: RustRegionOfInterestCallback,
        release: RustContextReleaseCallback,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
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
