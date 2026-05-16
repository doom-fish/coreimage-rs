use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ci_raw_filter_supported_camera_models_lines() -> *mut c_char;
    pub fn ci_raw_filter_new_from_path(
        path: *const c_char,
        out_filter: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_raw_filter_new_from_data(
        bytes: *const u8,
        len: usize,
        identifier_hint: *const c_char,
        out_filter: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_raw_filter_supported_decoder_versions_lines(handle: *mut c_void) -> *mut c_char;
    pub fn ci_raw_filter_native_size(handle: *mut c_void, out_width: *mut f64, out_height: *mut f64);
    pub fn ci_raw_filter_properties_json(handle: *mut c_void) -> *mut c_char;
    pub fn ci_raw_filter_orientation(handle: *mut c_void) -> u32;
    pub fn ci_raw_filter_set_orientation(handle: *mut c_void, orientation: u32);
    pub fn ci_raw_filter_is_draft_mode_enabled(handle: *mut c_void) -> bool;
    pub fn ci_raw_filter_set_draft_mode_enabled(handle: *mut c_void, enabled: bool);
    pub fn ci_raw_filter_decoder_version(handle: *mut c_void) -> *mut c_char;
    pub fn ci_raw_filter_set_decoder_version(handle: *mut c_void, value: *const c_char);
    pub fn ci_raw_filter_scale_factor(handle: *mut c_void) -> f32;
    pub fn ci_raw_filter_set_scale_factor(handle: *mut c_void, value: f32);
    pub fn ci_raw_filter_exposure(handle: *mut c_void) -> f32;
    pub fn ci_raw_filter_set_exposure(handle: *mut c_void, value: f32);
    pub fn ci_raw_filter_preview_image(handle: *mut c_void) -> *mut c_void;
}
