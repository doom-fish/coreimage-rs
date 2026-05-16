use core::ffi::c_void;
use core::fmt;
use core::ptr;
use std::ffi::CString;

use crate::ffi;
use crate::image::CIImage;
use crate::util::{split_lines, take_owned_string};
use crate::{CIColor, CIVector};

/// A mutable CoreImage filter instance.
pub struct CIFilter {
    ptr: *mut c_void,
}

impl Drop for CIFilter {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIFilter {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIFilter")
            .field("ptr", &self.ptr)
            .field("input_keys", &self.input_keys())
            .field("output_keys", &self.output_keys())
            .finish()
    }
}

impl CIFilter {
    const fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn new(name: &str) -> Option<Self> {
        let name = CString::new(name).ok()?;
        let filter = unsafe { ffi::ci_filter_new(name.as_ptr()) };
        if filter.is_null() {
            None
        } else {
            Some(Self::from_raw(filter))
        }
    }

    pub fn all_names() -> Vec<String> {
        let joined = unsafe { take_owned_string(ffi::ci_filter_names_lines(ptr::null())) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

    pub fn names_in_category(category: &str) -> Vec<String> {
        let Ok(category) = CString::new(category) else {
            return Vec::new();
        };
        let joined = unsafe { take_owned_string(ffi::ci_filter_names_lines(category.as_ptr())) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

    pub fn input_keys(&self) -> Vec<String> {
        let joined = unsafe { take_owned_string(ffi::ci_filter_input_keys_lines(self.ptr)) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

    pub fn output_keys(&self) -> Vec<String> {
        let joined = unsafe { take_owned_string(ffi::ci_filter_output_keys_lines(self.ptr)) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

    pub fn attributes_json(&self) -> String {
        unsafe { take_owned_string(ffi::ci_filter_attributes_json(self.ptr)) }
            .unwrap_or_else(|| "{}".to_string())
    }

    pub fn set_input_image(&mut self, image: &CIImage) {
        self.set_input_image_for_key("inputImage", image);
    }

    pub fn set_input_image_for_key(&mut self, key: &str, image: &CIImage) {
        let Ok(key) = CString::new(key) else {
            return;
        };
        unsafe { ffi::ci_filter_set_image(self.ptr, key.as_ptr(), image.as_ptr()) };
    }

    pub fn set_input_number(&mut self, key: &str, value: f64) {
        let Ok(key) = CString::new(key) else {
            return;
        };
        unsafe { ffi::ci_filter_set_number(self.ptr, key.as_ptr(), value) };
    }

    pub fn set_input_string(&mut self, key: &str, value: &str) {
        let Ok(key) = CString::new(key) else {
            return;
        };
        let Ok(value) = CString::new(value) else {
            return;
        };
        unsafe { ffi::ci_filter_set_string(self.ptr, key.as_ptr(), value.as_ptr()) };
    }

    pub fn set_input_vector(&mut self, key: &str, value: &CIVector) {
        let Ok(key) = CString::new(key) else {
            return;
        };
        unsafe { ffi::ci_filter_set_vector(self.ptr, key.as_ptr(), value.as_ptr()) };
    }

    pub fn set_input_color(&mut self, key: &str, value: &CIColor) {
        let Ok(key) = CString::new(key) else {
            return;
        };
        unsafe { ffi::ci_filter_set_color(self.ptr, key.as_ptr(), value.as_ptr()) };
    }

    pub fn output_image(&self) -> Option<CIImage> {
        let image = unsafe { ffi::ci_filter_output_image(self.ptr) };
        if image.is_null() {
            None
        } else {
            Some(unsafe { CIImage::from_raw(image) })
        }
    }
}
