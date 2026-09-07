use core::ffi::{c_char, c_void};

pub type RustWarpRegionOfInterestCallback = Option<
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
pub type RustWarpRegionOfInterestReleaseCallback =
    Option<unsafe extern "C" fn(context: *mut c_void)>;

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
        callback: RustWarpRegionOfInterestCallback,
        release_callback: RustWarpRegionOfInterestReleaseCallback,
    ) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_blend_kernel_apply`.
    pub fn ci_blend_kernel_apply(
        handle: *mut c_void,
        foreground: *mut c_void,
        background: *mut c_void,
    ) -> *mut c_void;
}
