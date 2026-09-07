use core::ffi::c_void;
use core::fmt;
use core::ptr::{self, NonNull};
use std::path::Path;

use crate::ffi;
use crate::util::{
    catch_callback_panic, catch_callback_panic_with_cleanup, path_to_cstring,
};
use crate::CIError;

type PlugInRegistrationFn = dyn Fn(*mut c_void) -> bool + Send + Sync;

struct PlugInRegistrationCallback {
    callback: Box<PlugInRegistrationFn>,
}

unsafe extern "C" fn plugin_registration_invoke(context: *mut c_void, host: *mut c_void) -> bool {
    if context.is_null() {
        return false;
    }

    catch_callback_panic("CIPlugInRegistration callback", false, || {
        let context = unsafe { &*context.cast::<PlugInRegistrationCallback>() };
        (context.callback)(host)
    })
}

unsafe extern "C" fn plugin_registration_release(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    let state = unsafe { Some(Box::from_raw(context.cast::<PlugInRegistrationCallback>())) };
    let _ = catch_callback_panic_with_cleanup(
        "CIPlugInRegistration release",
        state,
        |_| (),
        |state| drop(state.take()),
    );
}

/// A Rust-backed `CIPlugInRegistration` protocol object.
pub struct CIPlugInRegistration {
    ptr: *mut c_void,
}

impl Drop for CIPlugInRegistration {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIPlugInRegistration {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIPlugInRegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIPlugInRegistration")
            .field("ptr", &self.ptr)
            .finish_non_exhaustive()
    }
}

impl CIPlugInRegistration {
    pub(crate) const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    fn from_non_null(ptr: *mut c_void, kind: &str) -> Self {
        assert!(!ptr.is_null(), "{kind} returned nil");
        unsafe { Self::from_raw(ptr) }
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

/// Creates a registration callback that may be invoked and released on any thread.
///
/// Callback-body panics return `false`. Captured values must implement non-panicking `Drop`;
/// Rust cannot recover from multiple destructor panics in one closure aggregate.
    pub fn new(callback: impl Fn(*mut c_void) -> bool + Send + Sync + 'static) -> Self {
        let callback = Box::new(PlugInRegistrationCallback {
            callback: Box::new(callback),
        });
        let handle = unsafe {
            ffi::ci_plugin_registration_new(
                Box::into_raw(callback).cast(),
                Some(plugin_registration_invoke),
                Some(plugin_registration_release),
            )
        };
        Self::from_non_null(handle, "CIPlugInRegistration")
    }

/// Calls the `CoreImage` framework counterpart for `load`.
    pub fn load(&self, host: Option<NonNull<c_void>>) -> bool {
        unsafe {
            ffi::ci_plugin_registration_load(
                self.ptr,
                host.map_or(ptr::null_mut(), NonNull::as_ptr),
            )
        }
    }
}

/// Plug-in loading helpers for Core Image image units on macOS.
pub struct CIPlugIn;

impl CIPlugIn {
/// Calls the `CoreImage` framework counterpart for `load_all_plugins`.
    pub fn load_all_plugins() {
        unsafe { ffi::ci_plugin_load_all_plugins() };
    }

/// Calls the `CoreImage` framework counterpart for `load_non_executable_plugins`.
    pub fn load_non_executable_plugins() {
        unsafe { ffi::ci_plugin_load_non_executable_plugins() };
    }

/// Calls the `CoreImage` framework counterpart for `load_plugin`.
    pub fn load_plugin(path: impl AsRef<Path>, allow_executable_code: bool) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        unsafe { ffi::ci_plugin_load_plugin(path.as_ptr(), allow_executable_code) };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `load_non_executable_plugin`.
    pub fn load_non_executable_plugin(path: impl AsRef<Path>) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        unsafe { ffi::ci_plugin_load_non_executable_plugin(path.as_ptr()) };
        Ok(())
    }
}
