use core::ffi::{c_char, c_void};
use std::any::Any;
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::panic::{catch_unwind, AssertUnwindSafe};
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

pub(crate) fn catch_callback_panic<R>(site: &str, fallback: R, callback: impl FnOnce() -> R) -> R {
    catch_callback_panic_result(site, callback).unwrap_or(fallback)
}

pub(crate) fn catch_callback_panic_with_cleanup<S, R, F, C>(
    site: &str,
    mut state: S,
    mut callback: F,
    mut cleanup: C,
) -> Option<R>
where
    F: FnMut(&mut S) -> R,
    C: FnMut(&mut S),
{
    let callback_result = catch_callback_panic_result(site, || callback(&mut state));
    let cleanup_succeeded =
        catch_callback_panic_result(site, || cleanup(&mut state)).is_some();
    let callback_drop_succeeded =
        catch_callback_panic_result(site, || drop(callback)).is_some();
    let cleanup_drop_succeeded =
        catch_callback_panic_result(site, || drop(cleanup)).is_some();
    let state_drop_succeeded = catch_callback_panic_result(site, || drop(state)).is_some();

    if callback_drop_succeeded
        && cleanup_succeeded
        && cleanup_drop_succeeded
        && state_drop_succeeded
    {
        callback_result
    } else {
        if let Some(result) = callback_result {
            let _ = catch_callback_panic_result(site, || drop(result));
        }
        None
    }
}

fn catch_callback_panic_result<R>(site: &str, callback: impl FnOnce() -> R) -> Option<R> {
    let boundary_result = catch_unwind(AssertUnwindSafe(|| {
        match catch_unwind(AssertUnwindSafe(callback)) {
            Ok(result) => Some(result),
            Err(payload) => {
                log_callback_panic(site, payload.as_ref());
                drop(payload);
                None
            }
        }
    }));

    match boundary_result {
        Ok(result) => result,
        Err(payload) => {
            log_callback_panic(site, payload.as_ref());
            drop_payload_best_effort(payload);
            None
        }
    }
}

fn log_callback_panic(site: &str, payload: &(dyn Any + Send)) {
    if let Err(payload) = catch_unwind(AssertUnwindSafe(|| {
        let message = payload.downcast_ref::<&'static str>().map_or_else(
            || {
                payload
                    .downcast_ref::<String>()
                    .map_or("<non-string panic payload>", String::as_str)
            },
            |message| *message,
        );
        eprintln!("coreimage: panic in {site} caught at C ABI boundary: {message}");
    })) {
        drop_payload_best_effort(payload);
    }
}

fn drop_payload_best_effort(payload: Box<dyn Any + Send>) {
    if let Err(payload) = catch_unwind(AssertUnwindSafe(|| drop(payload))) {
        std::mem::forget(payload);
    }
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
