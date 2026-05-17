use core::ffi::{c_char, c_void};

pub type RustFilterConstructorCallback =
    Option<unsafe extern "C" fn(context: *mut c_void, name: *const c_char) -> *mut c_void>;
pub type RustFilterConstructorReleaseCallback = Option<unsafe extern "C" fn(context: *mut c_void)>;

unsafe extern "C" {
    pub fn ci_filter_constructor_new(
        context: *mut c_void,
        callback: RustFilterConstructorCallback,
        release_callback: RustFilterConstructorReleaseCallback,
    ) -> *mut c_void;
    pub fn ci_filter_constructor_filter(handle: *mut c_void, name: *const c_char) -> *mut c_void;
    pub fn ci_filter_register_name(
        name: *const c_char,
        constructor: *mut c_void,
        display_name: *const c_char,
    );
}
