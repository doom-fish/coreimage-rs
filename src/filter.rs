use core::ffi::c_void;
use core::fmt;
use core::ptr;

use crate::barcode_descriptor::CIBarcodeDescriptor;
use crate::color::CIColor;
use crate::constants::{CIFilterCategory, CIInputKey, CIOutputKey};
use crate::ffi;
use crate::image::CIImage;
use crate::util::{split_lines, string_to_cstring, take_owned_string};
use crate::vector::CIVector;

/// A mutable Core Image filter instance.
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
            .field("name", &self.name())
            .field("input_keys", &self.input_keys())
            .field("output_keys", &self.output_keys())
            .finish()
    }
}

impl CIFilter {
    pub(crate) const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

/// Calls the `CoreImage` framework counterpart for `new`.
    pub fn new(name: &str) -> Option<Self> {
        let name = string_to_cstring(name, "filter name").ok()?;
        let filter = unsafe { ffi::ci_filter_new(name.as_ptr()) };
        if filter.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(filter) })
        }
    }

/// Calls the `CoreImage` framework counterpart for `name`.
    pub fn name(&self) -> String {
        unsafe { take_owned_string(ffi::ci_filter_name(self.ptr)) }.unwrap_or_default()
    }

/// Calls the `CoreImage` framework counterpart for `set_name`.
    pub fn set_name(&mut self, name: &str) {
        let Ok(name) = string_to_cstring(name, "filter name") else {
            return;
        };
        unsafe { ffi::ci_filter_set_name(self.ptr, name.as_ptr()) };
    }

/// Calls the `CoreImage` framework counterpart for `is_enabled`.
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::ci_filter_is_enabled(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `set_enabled`.
    pub fn set_enabled(&mut self, enabled: bool) {
        unsafe { ffi::ci_filter_set_enabled(self.ptr, enabled) };
    }

/// Calls the `CoreImage` framework counterpart for `set_defaults`.
    pub fn set_defaults(&mut self) {
        unsafe { ffi::ci_filter_set_defaults(self.ptr) };
    }

/// Calls the `CoreImage` framework counterpart for `all_names`.
    pub fn all_names() -> Vec<String> {
        let joined = unsafe { take_owned_string(ffi::ci_filter_names_lines(ptr::null())) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

/// Calls the `CoreImage` framework counterpart for `names_in_category`.
    pub fn names_in_category(category: &str) -> Vec<String> {
        let Ok(category) = string_to_cstring(category, "category") else {
            return Vec::new();
        };
        let joined = unsafe { take_owned_string(ffi::ci_filter_names_lines(category.as_ptr())) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

/// Calls the `CoreImage` framework counterpart for `names_in_category_key`.
    pub fn names_in_category_key(category: CIFilterCategory) -> Vec<String> {
        Self::names_in_category(category.value())
    }

/// Calls the `CoreImage` framework counterpart for `localized_name`.
    pub fn localized_name(filter_name: &str) -> Option<String> {
        let filter_name = string_to_cstring(filter_name, "filter name").ok()?;
        unsafe { take_owned_string(ffi::ci_filter_localized_name(filter_name.as_ptr())) }
    }

/// Calls the `CoreImage` framework counterpart for `localized_description`.
    pub fn localized_description(filter_name: &str) -> Option<String> {
        let filter_name = string_to_cstring(filter_name, "filter name").ok()?;
        unsafe { take_owned_string(ffi::ci_filter_localized_description(filter_name.as_ptr())) }
    }

/// Calls the `CoreImage` framework counterpart for `localized_reference_url`.
    pub fn localized_reference_url(filter_name: &str) -> Option<String> {
        let filter_name = string_to_cstring(filter_name, "filter name").ok()?;
        unsafe { take_owned_string(ffi::ci_filter_localized_reference_url(filter_name.as_ptr())) }
    }

/// Calls the `CoreImage` framework counterpart for `input_keys`.
    pub fn input_keys(&self) -> Vec<String> {
        let joined = unsafe { take_owned_string(ffi::ci_filter_input_keys_lines(self.ptr)) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

/// Calls the `CoreImage` framework counterpart for `output_keys`.
    pub fn output_keys(&self) -> Vec<String> {
        let joined = unsafe { take_owned_string(ffi::ci_filter_output_keys_lines(self.ptr)) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

/// Calls the `CoreImage` framework counterpart for `attributes_json`.
    pub fn attributes_json(&self) -> String {
        unsafe { take_owned_string(ffi::ci_filter_attributes_json(self.ptr)) }
            .unwrap_or_else(|| "{}".to_string())
    }

/// Calls the `CoreImage` framework counterpart for `set_input_image`.
    pub fn set_input_image(&mut self, image: &CIImage) {
        self.set_input_image_for_key("inputImage", image);
    }

/// Calls the `CoreImage` framework counterpart for `set_input_image_for_key`.
    pub fn set_input_image_for_key(&mut self, key: &str, image: &CIImage) {
        let Ok(key) = string_to_cstring(key, "key") else {
            return;
        };
        unsafe { ffi::ci_filter_set_image(self.ptr, key.as_ptr(), image.as_ptr()) };
    }

/// Calls the `CoreImage` framework counterpart for `set_input_image_key`.
    pub fn set_input_image_key(&mut self, key: CIInputKey, image: &CIImage) {
        self.set_input_image_for_key(key.value(), image);
    }

/// Calls the `CoreImage` framework counterpart for `set_input_number`.
    pub fn set_input_number(&mut self, key: &str, value: f64) {
        let Ok(key) = string_to_cstring(key, "key") else {
            return;
        };
        unsafe { ffi::ci_filter_set_number(self.ptr, key.as_ptr(), value) };
    }

/// Calls the `CoreImage` framework counterpart for `set_input_number_key`.
    pub fn set_input_number_key(&mut self, key: CIInputKey, value: f64) {
        self.set_input_number(key.value(), value);
    }

/// Calls the `CoreImage` framework counterpart for `set_input_string`.
    pub fn set_input_string(&mut self, key: &str, value: &str) {
        let Ok(key) = string_to_cstring(key, "key") else {
            return;
        };
        let Ok(value) = string_to_cstring(value, "value") else {
            return;
        };
        unsafe { ffi::ci_filter_set_string(self.ptr, key.as_ptr(), value.as_ptr()) };
    }

/// Calls the `CoreImage` framework counterpart for `set_input_string_key`.
    pub fn set_input_string_key(&mut self, key: CIInputKey, value: &str) {
        self.set_input_string(key.value(), value);
    }

/// Calls the `CoreImage` framework counterpart for `set_input_bytes`.
    pub fn set_input_bytes(&mut self, key: &str, value: &[u8]) {
        let Ok(key) = string_to_cstring(key, "key") else {
            return;
        };
        unsafe { ffi::ci_filter_set_bytes(self.ptr, key.as_ptr(), value.as_ptr(), value.len()) };
    }

/// Calls the `CoreImage` framework counterpart for `set_input_bytes_key`.
    pub fn set_input_bytes_key(&mut self, key: CIInputKey, value: &[u8]) {
        self.set_input_bytes(key.value(), value);
    }

/// Calls the `CoreImage` framework counterpart for `set_input_vector`.
    pub fn set_input_vector(&mut self, key: &str, value: &CIVector) {
        let Ok(key) = string_to_cstring(key, "key") else {
            return;
        };
        unsafe { ffi::ci_filter_set_vector(self.ptr, key.as_ptr(), value.as_ptr()) };
    }

/// Calls the `CoreImage` framework counterpart for `set_input_vector_key`.
    pub fn set_input_vector_key(&mut self, key: CIInputKey, value: &CIVector) {
        self.set_input_vector(key.value(), value);
    }

/// Calls the `CoreImage` framework counterpart for `set_input_color`.
    pub fn set_input_color(&mut self, key: &str, value: &CIColor) {
        let Ok(key) = string_to_cstring(key, "key") else {
            return;
        };
        unsafe { ffi::ci_filter_set_color(self.ptr, key.as_ptr(), value.as_ptr()) };
    }

/// Calls the `CoreImage` framework counterpart for `set_input_color_key`.
    pub fn set_input_color_key(&mut self, key: CIInputKey, value: &CIColor) {
        self.set_input_color(key.value(), value);
    }

/// Calls the `CoreImage` framework counterpart for `set_input_barcode_descriptor`.
    pub fn set_input_barcode_descriptor(&mut self, key: &str, value: &CIBarcodeDescriptor) {
        let Ok(key) = string_to_cstring(key, "key") else {
            return;
        };
        unsafe {
            ffi::ci_filter_set_barcode_descriptor(self.ptr, key.as_ptr(), value.as_ptr());
        };
    }

/// Calls the `CoreImage` framework counterpart for `set_input_barcode_descriptor_key`.
    pub fn set_input_barcode_descriptor_key(
        &mut self,
        key: CIInputKey,
        value: &CIBarcodeDescriptor,
    ) {
        self.set_input_barcode_descriptor(key.value(), value);
    }

/// Calls the `CoreImage` framework counterpart for `output_image`.
    pub fn output_image(&self) -> Option<CIImage> {
        let image = unsafe { ffi::ci_filter_output_image(self.ptr) };
        if image.is_null() {
            None
        } else {
            Some(unsafe { CIImage::from_raw(image) })
        }
    }

/// Calls the `CoreImage` framework counterpart for `output_image_for_key`.
    pub fn output_image_for_key(&self, key: CIOutputKey) -> Option<CIImage> {
        match key {
            CIOutputKey::Image => self.output_image(),
        }
    }
}
