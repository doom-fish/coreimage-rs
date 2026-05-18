use core::ffi::{c_char, c_void};

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_new_bitmap_data`.
    pub fn ci_render_destination_new_bitmap_data(
        data: *mut c_void,
        len: usize,
        width: usize,
        height: usize,
        bytes_per_row: usize,
        format_code: i32,
        use_color_space: bool,
        color_space_code: i32,
    ) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_width`.
    pub fn ci_render_destination_width(handle: *mut c_void) -> usize;
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_height`.
    pub fn ci_render_destination_height(handle: *mut c_void) -> usize;
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_alpha_mode`.
    pub fn ci_render_destination_alpha_mode(handle: *mut c_void) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_set_alpha_mode`.
    pub fn ci_render_destination_set_alpha_mode(handle: *mut c_void, mode: i32);
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_is_flipped`.
    pub fn ci_render_destination_is_flipped(handle: *mut c_void) -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_set_flipped`.
    pub fn ci_render_destination_set_flipped(handle: *mut c_void, flipped: bool);
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_is_dithered`.
    pub fn ci_render_destination_is_dithered(handle: *mut c_void) -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_set_dithered`.
    pub fn ci_render_destination_set_dithered(handle: *mut c_void, dithered: bool);
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_is_clamped`.
    pub fn ci_render_destination_is_clamped(handle: *mut c_void) -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_render_destination_set_clamped`.
    pub fn ci_render_destination_set_clamped(handle: *mut c_void, clamped: bool);
/// Calls the `CoreImage` framework counterpart for `ci_context_start_render_task`.
    pub fn ci_context_start_render_task(
        handle: *mut c_void,
        image: *mut c_void,
        destination: *mut c_void,
        out_task: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_context_prepare_render`.
    pub fn ci_context_prepare_render(
        handle: *mut c_void,
        image: *mut c_void,
        destination: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_context_start_clear_task`.
    pub fn ci_context_start_clear_task(
        handle: *mut c_void,
        destination: *mut c_void,
        out_task: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_render_task_wait_until_completed`.
    pub fn ci_render_task_wait_until_completed(
        handle: *mut c_void,
        out_info: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_render_info_kernel_execution_time`.
    pub fn ci_render_info_kernel_execution_time(handle: *mut c_void) -> f64;
/// Calls the `CoreImage` framework counterpart for `ci_render_info_kernel_compile_time`.
    pub fn ci_render_info_kernel_compile_time(handle: *mut c_void) -> f64;
/// Calls the `CoreImage` framework counterpart for `ci_render_info_pass_count`.
    pub fn ci_render_info_pass_count(handle: *mut c_void) -> usize;
/// Calls the `CoreImage` framework counterpart for `ci_render_info_pixels_processed`.
    pub fn ci_render_info_pixels_processed(handle: *mut c_void) -> usize;
}
