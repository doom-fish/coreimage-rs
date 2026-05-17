use core::ffi::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use crate::error::CIError;
use crate::ffi;

pub(crate) fn path_to_cstring(path: &Path) -> Result<CString, CIError> {
    CString::new(path.as_os_str().as_bytes())
        .map_err(|_| CIError::InvalidArgument("path contains an interior NUL byte".to_string()))
}

pub(crate) fn string_to_cstring(value: &str, name: &str) -> Result<CString, CIError> {
    CString::new(value)
        .map_err(|_| CIError::InvalidArgument(format!("{name} contains an interior NUL byte")))
}

pub(crate) unsafe fn status_result(status: i32, error_str: *mut c_char) -> Result<(), CIError> {
    // SAFETY: Caller must guarantee `error_str` is either null or a valid C string allocated
    // by the FFI layer that must be freed exactly once.
    if status == ffi::status::OK {
        if !error_str.is_null() {
            libc::free(error_str.cast());
        }
        Ok(())
    } else {
        Err(CIError::from_swift(status, error_str))
    }
}

pub(crate) unsafe fn take_owned_string(ptr: *mut c_char) -> Option<String> {
    // SAFETY: Caller must guarantee `ptr` is either null or a valid C string allocated
    // by the FFI layer. This function takes ownership and frees it exactly once.
    if ptr.is_null() {
        None
    } else {
        let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        libc::free(ptr.cast());
        Some(string)
    }
}

pub(crate) unsafe fn take_array_objects(array_handle: *mut c_void) -> Vec<*mut c_void> {
    // SAFETY: Caller must guarantee `array_handle` is either null or a valid Objective-C array
    // handle allocated by the FFI layer. This function takes ownership and releases it exactly once.
    if array_handle.is_null() {
        return Vec::new();
    }
    let count = ffi::ci_array_count(array_handle);
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        let value = ffi::ci_array_object_at(array_handle, index);
        if !value.is_null() {
            values.push(value);
        }
    }
    ffi::ci_object_release(array_handle);
    values
}

pub(crate) fn split_lines(text: &str) -> Vec<String> {
    if text.is_empty() {
        Vec::new()
    } else {
        text.lines().map(ToOwned::to_owned).collect()
    }
}
