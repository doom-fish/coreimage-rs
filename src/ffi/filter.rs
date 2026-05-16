use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ci_filter_new(name: *const c_char) -> *mut c_void;
    pub fn ci_filter_name(handle: *mut c_void) -> *mut c_char;
    pub fn ci_filter_set_name(handle: *mut c_void, name: *const c_char);
    pub fn ci_filter_is_enabled(handle: *mut c_void) -> bool;
    pub fn ci_filter_set_enabled(handle: *mut c_void, enabled: bool);
    pub fn ci_filter_set_defaults(handle: *mut c_void);
    pub fn ci_filter_names_lines(category: *const c_char) -> *mut c_char;
    pub fn ci_filter_input_keys_lines(handle: *mut c_void) -> *mut c_char;
    pub fn ci_filter_output_keys_lines(handle: *mut c_void) -> *mut c_char;
    pub fn ci_filter_attributes_json(handle: *mut c_void) -> *mut c_char;
    pub fn ci_filter_localized_name(name: *const c_char) -> *mut c_char;
    pub fn ci_filter_localized_description(name: *const c_char) -> *mut c_char;
    pub fn ci_filter_localized_reference_url(name: *const c_char) -> *mut c_char;
    pub fn ci_filter_set_image(handle: *mut c_void, key: *const c_char, image: *mut c_void);
    pub fn ci_filter_set_number(handle: *mut c_void, key: *const c_char, value: f64);
    pub fn ci_filter_set_string(handle: *mut c_void, key: *const c_char, value: *const c_char);
    pub fn ci_filter_set_bytes(
        handle: *mut c_void,
        key: *const c_char,
        bytes: *const u8,
        len: usize,
    );
    pub fn ci_filter_set_vector(handle: *mut c_void, key: *const c_char, value: *mut c_void);
    pub fn ci_filter_set_color(handle: *mut c_void, key: *const c_char, value: *mut c_void);
    pub fn ci_filter_set_barcode_descriptor(
        handle: *mut c_void,
        key: *const c_char,
        value: *mut c_void,
    );
    pub fn ci_filter_output_image(handle: *mut c_void) -> *mut c_void;
}
