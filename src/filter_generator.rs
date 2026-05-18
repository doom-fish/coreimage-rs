use core::ffi::c_void;
use core::fmt;
use core::ptr;
use std::path::Path;

use crate::ffi;
use crate::filter::CIFilter;
use crate::image::CIImage;
use crate::util::{path_to_cstring, status_result, string_to_cstring, take_owned_string};
use crate::CIError;

/// A reusable Core Image filter graph.
pub struct CIFilterGenerator {
    ptr: *mut c_void,
}

impl Drop for CIFilterGenerator {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIFilterGenerator {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIFilterGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIFilterGenerator")
            .field("ptr", &self.ptr)
            .field("exported_keys_json", &self.exported_keys_json())
            .finish_non_exhaustive()
    }
}

impl Default for CIFilterGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CIFilterGenerator {
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
    pub fn new() -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_filter_generator_new() },
            "CIFilterGenerator",
        )
    }

/// Calls the `CoreImage` framework counterpart for `from_path`.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut generator = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_filter_generator_from_path(path.as_ptr(), &mut generator, &mut error)
        };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(
            generator,
            "CIFilterGenerator(contentsOf:)",
        ))
    }

/// Calls the `CoreImage` framework counterpart for `connect_filter_output`.
    pub fn connect_filter_output(
        &mut self,
        source_filter: &CIFilter,
        source_key: Option<&str>,
        target_filter: &CIFilter,
        target_key: &str,
    ) -> Result<(), CIError> {
        let source_key = source_key
            .map(|value| string_to_cstring(value, "source key"))
            .transpose()?;
        let target_key = string_to_cstring(target_key, "target key")?;
        unsafe {
            ffi::ci_filter_generator_connect_filter_output(
                self.ptr,
                source_filter.as_ptr(),
                source_key
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                target_filter.as_ptr(),
                target_key.as_ptr(),
            );
        };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `disconnect_filter_output`.
    pub fn disconnect_filter_output(
        &mut self,
        source_filter: &CIFilter,
        source_key: &str,
        target_filter: &CIFilter,
        target_key: &str,
    ) -> Result<(), CIError> {
        let source_key = string_to_cstring(source_key, "source key")?;
        let target_key = string_to_cstring(target_key, "target key")?;
        unsafe {
            ffi::ci_filter_generator_disconnect_filter_output(
                self.ptr,
                source_filter.as_ptr(),
                source_key.as_ptr(),
                target_filter.as_ptr(),
                target_key.as_ptr(),
            );
        };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `connect_image`.
    pub fn connect_image(
        &mut self,
        image: &CIImage,
        target_filter: &CIFilter,
        target_key: &str,
    ) -> Result<(), CIError> {
        let target_key = string_to_cstring(target_key, "target key")?;
        unsafe {
            ffi::ci_filter_generator_connect_image(
                self.ptr,
                image.as_ptr(),
                target_filter.as_ptr(),
                target_key.as_ptr(),
            );
        };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `export_key`.
    pub fn export_key(
        &mut self,
        key: &str,
        target_filter: &CIFilter,
        exported_name: Option<&str>,
    ) -> Result<(), CIError> {
        let key = string_to_cstring(key, "key")?;
        let exported_name = exported_name
            .map(|value| string_to_cstring(value, "exported name"))
            .transpose()?;
        unsafe {
            ffi::ci_filter_generator_export_key(
                self.ptr,
                key.as_ptr(),
                target_filter.as_ptr(),
                exported_name
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `remove_exported_key`.
    pub fn remove_exported_key(&mut self, exported_name: &str) -> Result<(), CIError> {
        let exported_name = string_to_cstring(exported_name, "exported name")?;
        unsafe { ffi::ci_filter_generator_remove_exported_key(self.ptr, exported_name.as_ptr()) };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `exported_keys_json`.
    pub fn exported_keys_json(&self) -> String {
        unsafe { take_owned_string(ffi::ci_filter_generator_exported_keys_json(self.ptr)) }
            .unwrap_or_else(|| "{}".to_string())
    }

/// Calls the `CoreImage` framework counterpart for `filter`.
    pub fn filter(&self) -> Option<CIFilter> {
        let handle = unsafe { ffi::ci_filter_generator_filter(self.ptr) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { CIFilter::from_raw(handle) })
        }
    }

/// Calls the `CoreImage` framework counterpart for `register_filter_name`.
    pub fn register_filter_name(
        &mut self,
        name: &str,
        display_name: Option<&str>,
    ) -> Result<(), CIError> {
        let name = string_to_cstring(name, "filter name")?;
        let display_name = display_name
            .map(|value| string_to_cstring(value, "display name"))
            .transpose()?;
        unsafe {
            ffi::ci_filter_generator_register_filter_name(
                self.ptr,
                name.as_ptr(),
                display_name
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `write_to_path`.
    pub fn write_to_path(&self, path: impl AsRef<Path>, atomically: bool) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_filter_generator_write_to_path(self.ptr, path.as_ptr(), atomically, &mut error)
        };
        unsafe { status_result(status, error) }
    }
}
