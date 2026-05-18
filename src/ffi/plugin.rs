use core::ffi::{c_char, c_void};

/// Mirrors the `CoreImage` framework counterpart for `RustPlugInRegistrationCallback`.
pub type RustPlugInRegistrationCallback =
    Option<unsafe extern "C" fn(context: *mut c_void, host: *mut c_void) -> bool>;
/// Mirrors the `CoreImage` framework counterpart for `RustPlugInRegistrationReleaseCallback`.
pub type RustPlugInRegistrationReleaseCallback = Option<unsafe extern "C" fn(context: *mut c_void)>;

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_plugin_registration_new`.
    pub fn ci_plugin_registration_new(
        context: *mut c_void,
        callback: RustPlugInRegistrationCallback,
        release_callback: RustPlugInRegistrationReleaseCallback,
    ) -> *mut c_void;
/// Calls the `CoreImage` framework counterpart for `ci_plugin_registration_load`.
    pub fn ci_plugin_registration_load(handle: *mut c_void, host: *mut c_void) -> bool;
/// Calls the `CoreImage` framework counterpart for `ci_plugin_load_all_plugins`.
    pub fn ci_plugin_load_all_plugins();
/// Calls the `CoreImage` framework counterpart for `ci_plugin_load_non_executable_plugins`.
    pub fn ci_plugin_load_non_executable_plugins();
/// Calls the `CoreImage` framework counterpart for `ci_plugin_load_plugin`.
    pub fn ci_plugin_load_plugin(path: *const c_char, allow_executable_code: bool);
/// Calls the `CoreImage` framework counterpart for `ci_plugin_load_non_executable_plugin`.
    pub fn ci_plugin_load_non_executable_plugin(path: *const c_char);
}
