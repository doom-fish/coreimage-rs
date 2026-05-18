use core::ffi::{c_char, c_void};

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_new`.
    pub fn ci_filter_generator_new() -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_from_path`.
    pub fn ci_filter_generator_from_path(
        path: *const c_char,
        out_generator: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_connect_filter_output`.
    pub fn ci_filter_generator_connect_filter_output(
        handle: *mut c_void,
        source_filter: *mut c_void,
        source_key: *const c_char,
        target_filter: *mut c_void,
        target_key: *const c_char,
    );
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_disconnect_filter_output`.
    pub fn ci_filter_generator_disconnect_filter_output(
        handle: *mut c_void,
        source_filter: *mut c_void,
        source_key: *const c_char,
        target_filter: *mut c_void,
        target_key: *const c_char,
    );
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_connect_image`.
    pub fn ci_filter_generator_connect_image(
        handle: *mut c_void,
        image: *mut c_void,
        target_filter: *mut c_void,
        target_key: *const c_char,
    );
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_export_key`.
    pub fn ci_filter_generator_export_key(
        handle: *mut c_void,
        key: *const c_char,
        target_filter: *mut c_void,
        exported_name: *const c_char,
    );
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_remove_exported_key`.
    pub fn ci_filter_generator_remove_exported_key(
        handle: *mut c_void,
        exported_name: *const c_char,
    );
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_exported_keys_json`.
    pub fn ci_filter_generator_exported_keys_json(handle: *mut c_void) -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_filter`.
    pub fn ci_filter_generator_filter(handle: *mut c_void) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_register_filter_name`.
    pub fn ci_filter_generator_register_filter_name(
        handle: *mut c_void,
        name: *const c_char,
        display_name: *const c_char,
    );
/// Calls the `CoreImage` framework counterpart for `ci_filter_generator_write_to_path`.
    pub fn ci_filter_generator_write_to_path(
        handle: *mut c_void,
        path: *const c_char,
        atomically: bool,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
