use core::ffi::{c_char, c_void};

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_color_new_rgba`.
    pub fn ci_color_new_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_color_from_string`.
    pub fn ci_color_from_string(representation: *const c_char) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_color_named`.
    pub fn ci_color_named(kind: i32) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_color_number_of_components`.
    pub fn ci_color_number_of_components(handle: *mut c_void) -> usize;
/// Calls the `CoreImage` framework counterpart for `ci_color_component_at`.
    pub fn ci_color_component_at(handle: *mut c_void, index: usize) -> f64;
/// Calls the `CoreImage` framework counterpart for `ci_color_alpha`.
    pub fn ci_color_alpha(handle: *mut c_void) -> f64;
/// Calls the `CoreImage` framework counterpart for `ci_color_red`.
    pub fn ci_color_red(handle: *mut c_void) -> f64;
/// Calls the `CoreImage` framework counterpart for `ci_color_green`.
    pub fn ci_color_green(handle: *mut c_void) -> f64;
/// Calls the `CoreImage` framework counterpart for `ci_color_blue`.
    pub fn ci_color_blue(handle: *mut c_void) -> f64;
/// Calls the `CoreImage` framework counterpart for `ci_color_string_representation`.
    pub fn ci_color_string_representation(handle: *mut c_void) -> *mut c_char;
}
