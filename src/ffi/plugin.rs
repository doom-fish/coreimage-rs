use core::ffi::{c_char, c_void};

pub type RustPlugInRegistrationCallback =
    Option<unsafe extern "C" fn(context: *mut c_void, host: *mut c_void) -> bool>;
pub type RustPlugInRegistrationReleaseCallback =
    Option<unsafe extern "C" fn(context: *mut c_void)>;

unsafe extern "C" {
    pub fn ci_plugin_registration_new(
        context: *mut c_void,
        callback: RustPlugInRegistrationCallback,
        release_callback: RustPlugInRegistrationReleaseCallback,
    ) -> *mut c_void;
    pub fn ci_plugin_registration_load(handle: *mut c_void, host: *mut c_void) -> bool;
    pub fn ci_plugin_load_all_plugins();
    pub fn ci_plugin_load_non_executable_plugins();
    pub fn ci_plugin_load_plugin(path: *const c_char, allow_executable_code: bool);
    pub fn ci_plugin_load_non_executable_plugin(path: *const c_char);
}
