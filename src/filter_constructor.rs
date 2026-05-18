use core::ffi::{c_char, c_void};
use core::fmt;
use core::ptr;
use std::ffi::CStr;
use std::mem;

use crate::ffi;
use crate::filter::CIFilter;
use crate::util::string_to_cstring;
use crate::CIError;

type FilterConstructorFn = dyn Fn(&str) -> Option<CIFilter>;

struct FilterConstructorCallback {
    callback: Box<FilterConstructorFn>,
}

unsafe extern "C" fn filter_constructor_invoke(
    context: *mut c_void,
    name: *const c_char,
) -> *mut c_void {
    if context.is_null() || name.is_null() {
        return ptr::null_mut();
    }

    let context = unsafe { &*context.cast::<FilterConstructorCallback>() };
    let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();
    (context.callback)(name.as_ref()).map_or(ptr::null_mut(), |filter| {
        let handle = filter.as_ptr();
        mem::forget(filter);
        handle
    })
}

unsafe extern "C" fn filter_constructor_release(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(context.cast::<FilterConstructorCallback>())) };
}

/// A Rust-backed `CIFilterConstructor` protocol object.
pub struct CIFilterConstructor {
    ptr: *mut c_void,
}

impl Drop for CIFilterConstructor {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIFilterConstructor {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIFilterConstructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIFilterConstructor")
            .field("ptr", &self.ptr)
            .finish_non_exhaustive()
    }
}

impl CIFilterConstructor {
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

/// Calls the `CoreImage` framework counterpart for `new`.
    pub fn new(callback: impl Fn(&str) -> Option<CIFilter> + 'static) -> Self {
        let callback = Box::new(FilterConstructorCallback {
            callback: Box::new(callback),
        });
        let handle = unsafe {
            ffi::ci_filter_constructor_new(
                Box::into_raw(callback).cast(),
                Some(filter_constructor_invoke),
                Some(filter_constructor_release),
            )
        };
        Self::from_non_null(handle, "CIFilterConstructor")
    }

/// Calls the `CoreImage` framework counterpart for `filter_with_name`.
    pub fn filter_with_name(&self, name: &str) -> Option<CIFilter> {
        let name = string_to_cstring(name, "filter name").ok()?;
        let handle = unsafe { ffi::ci_filter_constructor_filter(self.ptr, name.as_ptr()) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { CIFilter::from_raw(handle) })
        }
    }
}

impl CIFilter {
/// Calls the `CoreImage` framework counterpart for `register_filter_name`.
    pub fn register_filter_name(
        name: &str,
        constructor: &CIFilterConstructor,
        display_name: Option<&str>,
    ) -> Result<(), CIError> {
        let name = string_to_cstring(name, "filter name")?;
        let display_name = display_name
            .map(|value| string_to_cstring(value, "display name"))
            .transpose()?;
        unsafe {
            ffi::ci_filter_register_name(
                name.as_ptr(),
                constructor.as_ptr(),
                display_name
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }
}
