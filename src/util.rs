use core::ffi::c_char;
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use crate::error::CIError;
use crate::ffi;

pub(crate) fn path_to_cstring(path: &Path) -> Result<CString, CIError> {
    CString::new(path.as_os_str().as_bytes())
        .map_err(|_| CIError::InvalidArgument("path contains an interior NUL byte".to_string()))
}

pub(crate) unsafe fn status_result(status: i32, error_str: *mut c_char) -> Result<(), CIError> {
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
    if ptr.is_null() {
        None
    } else {
        let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        libc::free(ptr.cast());
        Some(string)
    }
}

pub(crate) fn split_lines(text: &str) -> Vec<String> {
    if text.is_empty() {
        Vec::new()
    } else {
        text.lines().map(ToOwned::to_owned).collect()
    }
}
