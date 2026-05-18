use core::ffi::{c_char, c_void};

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_feature_type_code`.
    pub fn ci_feature_type_code(handle: *mut c_void) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_feature_bounds`.
    pub fn ci_feature_bounds(
        handle: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
/// Calls the `CoreImage` framework counterpart for `ci_feature_details_json`.
    pub fn ci_feature_details_json(handle: *mut c_void) -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_feature_message_string`.
    pub fn ci_feature_message_string(handle: *mut c_void) -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_feature_symbol_descriptor`.
    pub fn ci_feature_symbol_descriptor(handle: *mut c_void) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_feature_subfeatures`.
    pub fn ci_feature_subfeatures(handle: *mut c_void) -> *mut c_void;
}
