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
}
