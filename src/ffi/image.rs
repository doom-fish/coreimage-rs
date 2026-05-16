use core::ffi::{c_char, c_void};

unsafe extern "C" {
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
    pub fn ci_image_from_color(color: *mut c_void) -> *mut c_void;
    pub fn ci_image_empty() -> *mut c_void;
    pub fn ci_image_from_bitmap(
        bytes: *const u8,
        len: usize,
        width: usize,
        height: usize,
        bytes_per_row: usize,
        format_code: i32,
        use_color_space: bool,
        color_space_code: i32,
        out_image: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
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
    pub fn ci_image_is_opaque(handle: *mut c_void) -> bool;
    pub fn ci_image_properties_json(handle: *mut c_void) -> *mut c_char;
    pub fn ci_image_cropped(
        handle: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    pub fn ci_image_clamped_to_extent(handle: *mut c_void) -> *mut c_void;
    pub fn ci_image_clamped_to_rect(
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
    pub fn ci_image_apply_filter_name(handle: *mut c_void, name: *const c_char) -> *mut c_void;
    pub fn ci_image_premultiplying_alpha(handle: *mut c_void) -> *mut c_void;
    pub fn ci_image_unpremultiplying_alpha(handle: *mut c_void) -> *mut c_void;
    pub fn ci_image_setting_alpha_one_in_extent(
        handle: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    pub fn ci_image_gaussian_blur(handle: *mut c_void, sigma: f64) -> *mut c_void;
    pub fn ci_image_sampling_linear(handle: *mut c_void) -> *mut c_void;
    pub fn ci_image_sampling_nearest(handle: *mut c_void) -> *mut c_void;
    pub fn ci_image_insert_intermediate(handle: *mut c_void, cache: bool) -> *mut c_void;
    pub fn ci_image_apply_gain_map(handle: *mut c_void, gain_map: *mut c_void) -> *mut c_void;
    pub fn ci_image_apply_gain_map_headroom(
        handle: *mut c_void,
        gain_map: *mut c_void,
        headroom: f32,
    ) -> *mut c_void;
    pub fn ci_image_content_headroom(handle: *mut c_void) -> f32;
    pub fn ci_image_content_average_light_level(handle: *mut c_void) -> f32;
    pub fn ci_image_setting_content_headroom(handle: *mut c_void, headroom: f32) -> *mut c_void;
    pub fn ci_image_setting_content_average_light_level(
        handle: *mut c_void,
        average: f32,
    ) -> *mut c_void;
    pub fn ci_image_region_of_interest_for_image(
        handle: *mut c_void,
        image: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
}
