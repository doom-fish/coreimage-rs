use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ci_context_new_default() -> *mut c_void;
    pub fn ci_context_new_cpu() -> *mut c_void;
    pub fn ci_context_new_with_options(
        cache_intermediates: bool,
        priority_request_low: bool,
        allow_low_power: bool,
        output_premultiplied: bool,
        name: *const c_char,
    ) -> *mut c_void;
    #[cfg(feature = "metal")]
    pub fn ci_context_new_metal_device(device: *mut c_void) -> *mut c_void;
    #[cfg(feature = "metal")]
    pub fn ci_context_new_metal_command_queue(queue: *mut c_void) -> *mut c_void;
    pub fn ci_context_working_format(handle: *mut c_void) -> i32;
    pub fn ci_context_create_cg_image(handle: *mut c_void, image: *mut c_void) -> *mut c_void;
    pub fn ci_context_render_to_cv_pixel_buffer(
        handle: *mut c_void,
        image: *mut c_void,
        buffer: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_context_render_to_iosurface(
        handle: *mut c_void,
        image: *mut c_void,
        surface: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_context_reclaim_resources(handle: *mut c_void);
    pub fn ci_context_clear_caches(handle: *mut c_void);
    pub fn ci_context_input_image_maximum_size(
        handle: *mut c_void,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn ci_context_output_image_maximum_size(
        handle: *mut c_void,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn ci_context_write_png(
        handle: *mut c_void,
        image: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_context_write_jpeg(
        handle: *mut c_void,
        image: *mut c_void,
        path: *const c_char,
        quality: f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_context_write_heif(
        handle: *mut c_void,
        image: *mut c_void,
        path: *const c_char,
        quality: f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_context_write_heif10(
        handle: *mut c_void,
        image: *mut c_void,
        path: *const c_char,
        quality: f64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_context_write_tiff(
        handle: *mut c_void,
        image: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_context_write_openexr(
        handle: *mut c_void,
        image: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
