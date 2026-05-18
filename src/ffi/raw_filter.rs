use core::ffi::{c_char, c_void};

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_supported_camera_models_lines`.
    pub fn ci_raw_filter_supported_camera_models_lines() -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_new_from_path`.
    pub fn ci_raw_filter_new_from_path(
        path: *const c_char,
        out_filter: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_new_from_data`.
    pub fn ci_raw_filter_new_from_data(
        bytes: *const u8,
        len: usize,
        identifier_hint: *const c_char,
        out_filter: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_supported_decoder_versions_lines`.
    pub fn ci_raw_filter_supported_decoder_versions_lines(handle: *mut c_void) -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_native_size`.
    pub fn ci_raw_filter_native_size(
        handle: *mut c_void,
        out_width: *mut f64,
        out_height: *mut f64,
    );
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_properties_json`.
    pub fn ci_raw_filter_properties_json(handle: *mut c_void) -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_orientation`.
    pub fn ci_raw_filter_orientation(handle: *mut c_void) -> u32;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_set_orientation`.
    pub fn ci_raw_filter_set_orientation(handle: *mut c_void, orientation: u32);
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_is_draft_mode_enabled`.
    pub fn ci_raw_filter_is_draft_mode_enabled(handle: *mut c_void) -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_set_draft_mode_enabled`.
    pub fn ci_raw_filter_set_draft_mode_enabled(handle: *mut c_void, enabled: bool);
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_decoder_version`.
    pub fn ci_raw_filter_decoder_version(handle: *mut c_void) -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_set_decoder_version`.
    pub fn ci_raw_filter_set_decoder_version(handle: *mut c_void, value: *const c_char);
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_scale_factor`.
    pub fn ci_raw_filter_scale_factor(handle: *mut c_void) -> f32;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_set_scale_factor`.
    pub fn ci_raw_filter_set_scale_factor(handle: *mut c_void, value: f32);
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_exposure`.
    pub fn ci_raw_filter_exposure(handle: *mut c_void) -> f32;
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_set_exposure`.
    pub fn ci_raw_filter_set_exposure(handle: *mut c_void, value: f32);
/// Calls the `CoreImage` framework counterpart for `ci_raw_filter_preview_image`.
    pub fn ci_raw_filter_preview_image(handle: *mut c_void) -> *mut c_void;
}
