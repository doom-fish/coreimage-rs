#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn ci_object_retain(handle: *mut c_void) -> *mut c_void;
    pub fn ci_object_release(handle: *mut c_void);

    pub fn ci_image_from_path(
        path: *const c_char,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_image_from_encoded_data(
        bytes: *const u8,
        len: usize,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_image_from_cg_image(image: *mut c_void) -> *mut c_void;
    pub fn ci_image_from_cv_pixel_buffer(buffer: *mut c_void) -> *mut c_void;
    pub fn ci_image_from_iosurface(surface: *mut c_void) -> *mut c_void;
    pub fn ci_image_from_bitmap_rgba8(
        bytes: *const u8,
        len: usize,
        width: usize,
        height: usize,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_image_extent(
        handle: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn ci_image_properties_json(handle: *mut c_void) -> *mut c_char;
    pub fn ci_image_cropped(
        handle: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    pub fn ci_image_applying_orientation(handle: *mut c_void, exif_orientation: u32)
        -> *mut c_void;
    pub fn ci_image_composited_over(handle: *mut c_void, background: *mut c_void) -> *mut c_void;
    pub fn ci_image_transformed(
        handle: *mut c_void,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        tx: f64,
        ty: f64,
    ) -> *mut c_void;

    pub fn ci_filter_new(name: *const c_char) -> *mut c_void;
    pub fn ci_filter_names_lines(category: *const c_char) -> *mut c_char;
    pub fn ci_filter_input_keys_lines(handle: *mut c_void) -> *mut c_char;
    pub fn ci_filter_output_keys_lines(handle: *mut c_void) -> *mut c_char;
    pub fn ci_filter_attributes_json(handle: *mut c_void) -> *mut c_char;
    pub fn ci_filter_set_image(handle: *mut c_void, key: *const c_char, image: *mut c_void);
    pub fn ci_filter_set_number(handle: *mut c_void, key: *const c_char, value: f64);
    pub fn ci_filter_set_string(handle: *mut c_void, key: *const c_char, value: *const c_char);
    pub fn ci_filter_set_vector(handle: *mut c_void, key: *const c_char, value: *mut c_void);
    pub fn ci_filter_set_color(handle: *mut c_void, key: *const c_char, value: *mut c_void);
    pub fn ci_filter_output_image(handle: *mut c_void) -> *mut c_void;

    pub fn ci_vector_new2(x: f64, y: f64) -> *mut c_void;
    pub fn ci_vector_new3(x: f64, y: f64, z: f64) -> *mut c_void;
    pub fn ci_vector_new4(x: f64, y: f64, z: f64, w: f64) -> *mut c_void;
    pub fn ci_color_new_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> *mut c_void;

    pub fn ci_context_new_default() -> *mut c_void;
    pub fn ci_context_new_cpu() -> *mut c_void;
    pub fn ci_context_new_metal_device(device: *mut c_void) -> *mut c_void;
    pub fn ci_context_new_metal_command_queue(queue: *mut c_void) -> *mut c_void;
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
    pub fn ci_context_write_tiff(
        handle: *mut c_void,
        image: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const NULL_RESULT: i32 = -2;
    pub const FRAMEWORK: i32 = -3;
    pub const IO: i32 = -4;
    pub const UNSUPPORTED: i32 = -5;
}
