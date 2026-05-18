use core::ffi::c_void;

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_image_accumulator_new`.
    pub fn ci_image_accumulator_new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        format_code: i32,
        use_color_space: bool,
        color_space_code: i32,
    ) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_image_accumulator_extent`.
    pub fn ci_image_accumulator_extent(
        handle: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
/// Calls the `CoreImage` framework counterpart for `ci_image_accumulator_format`.
    pub fn ci_image_accumulator_format(handle: *mut c_void) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_image_accumulator_image`.
    pub fn ci_image_accumulator_image(handle: *mut c_void) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_image_accumulator_set_image`.
    pub fn ci_image_accumulator_set_image(handle: *mut c_void, image: *mut c_void);
/// Calls the `CoreImage` framework counterpart for `ci_image_accumulator_set_image_dirty_rect`.
    pub fn ci_image_accumulator_set_image_dirty_rect(
        handle: *mut c_void,
        image: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );
/// Calls the `CoreImage` framework counterpart for `ci_image_accumulator_clear`.
    pub fn ci_image_accumulator_clear(handle: *mut c_void);
}
