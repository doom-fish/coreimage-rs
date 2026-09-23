use core::ffi::{c_char, c_void};

pub type RustRegionOfInterestCallback = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        input_index: i32,
        destination_x: f64,
        destination_y: f64,
        destination_width: f64,
        destination_height: f64,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    ),
>;
pub type RustContextReleaseCallback = Option<unsafe extern "C" fn(context: *mut c_void)>;

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_color_kernel_new_source`.
    pub fn ci_color_kernel_new_source(
        source: *const c_char,
        out_kernel: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_warp_kernel_new_source`.
    pub fn ci_warp_kernel_new_source(
        source: *const c_char,
        out_kernel: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_blend_kernel_builtin`.
    pub fn ci_blend_kernel_builtin(kind: i32) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_kernel_name`.
    pub fn ci_kernel_name(handle: *mut c_void) -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_color_kernel_apply_image_scalar`.
    pub fn ci_color_kernel_apply_image_scalar(
        handle: *mut c_void,
        image: *mut c_void,
        value: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_color_kernel_apply_image_color`.
    pub fn ci_color_kernel_apply_image_color(
        handle: *mut c_void,
        image: *mut c_void,
        color: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_color_kernel_apply_image_vector`.
    pub fn ci_color_kernel_apply_image_vector(
        handle: *mut c_void,
        image: *mut c_void,
        vector: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_warp_kernel_apply_image_scalar`.
    pub fn ci_warp_kernel_apply_image_scalar(
        handle: *mut c_void,
        image: *mut c_void,
        value: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        use_destination_rect: bool,
    ) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_warp_kernel_apply_image_scalar_with_roi`.
    pub fn ci_warp_kernel_apply_image_scalar_with_roi(
        handle: *mut c_void,
        image: *mut c_void,
        value: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        context: *mut c_void,
        callback: RustRegionOfInterestCallback,
        release_callback: RustContextReleaseCallback,
    ) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_blend_kernel_apply`.
    pub fn ci_blend_kernel_apply(
        handle: *mut c_void,
        foreground: *mut c_void,
        background: *mut c_void,
    ) -> *mut c_void;
    pub fn ci_kernel_new_metal_library(
        kind: i32,
        function_name: *const c_char,
        data: *const u8,
        len: usize,
        has_output_format: bool,
        output_format: i32,
        out_kernel: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_kernel_names_metal_library(data: *const u8, len: usize) -> *mut c_char;
    pub fn ci_kernels_new_metal_source(
        source: *const c_char,
        out_kernels: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_kernel_is_kind(handle: *mut c_void, kind: i32) -> bool;
    pub fn ci_kernel_apply_arguments(
        handle: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        argument_kinds: *const i32,
        argument_scalars: *const f64,
        argument_objects: *const *mut c_void,
        argument_count: usize,
        context: *mut c_void,
        callback: RustRegionOfInterestCallback,
        release_callback: RustContextReleaseCallback,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_color_kernel_apply_arguments(
        handle: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        argument_kinds: *const i32,
        argument_scalars: *const f64,
        argument_objects: *const *mut c_void,
        argument_count: usize,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_warp_kernel_apply_arguments(
        handle: *mut c_void,
        image: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        argument_kinds: *const i32,
        argument_scalars: *const f64,
        argument_objects: *const *mut c_void,
        argument_count: usize,
        context: *mut c_void,
        callback: RustRegionOfInterestCallback,
        release_callback: RustContextReleaseCallback,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
