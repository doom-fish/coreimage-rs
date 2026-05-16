use core::ffi::c_void;
use core::fmt;
use core::ptr;
use std::sync::OnceLock;

use apple_cf::cg::CGSize;

use crate::ffi;
use crate::image::CIImage;
use crate::util::{split_lines, status_result, string_to_cstring, take_owned_string};
use crate::CIError;

/// Supported RAW decoder versions exposed by `CIRAWFilter`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRAWDecoderVersion {
    None,
    Version9,
    Version9Dng,
    Version8,
    Version8Dng,
    Version7,
    Version7Dng,
    Version6,
    Version6Dng,
}

impl CIRAWDecoderVersion {
    fn values() -> &'static Vec<String> {
        static VALUES: OnceLock<Vec<String>> = OnceLock::new();
        VALUES.get_or_init(|| {
            (0..9)
                .map(|index| unsafe { take_owned_string(ffi::ci_raw_decoder_version_name(index)).unwrap_or_default() })
                .collect()
        })
    }

    const fn index(self) -> usize {
        match self {
            Self::None => 0,
            Self::Version9 => 1,
            Self::Version9Dng => 2,
            Self::Version8 => 3,
            Self::Version8Dng => 4,
            Self::Version7 => 5,
            Self::Version7Dng => 6,
            Self::Version6 => 7,
            Self::Version6Dng => 8,
        }
    }

    pub fn as_str(self) -> &'static str {
        Self::values()[self.index()].as_str()
    }
}

/// A live Core Image RAW pipeline.
pub struct CIRAWFilter {
    ptr: *mut c_void,
}

impl Drop for CIRAWFilter {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIRAWFilter {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIRAWFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIRAWFilter")
            .field("ptr", &self.ptr)
            .field("native_size", &self.native_size())
            .field("decoder_version", &self.decoder_version())
            .finish_non_exhaustive()
    }
}

impl CIRAWFilter {
    pub(crate) const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    fn from_non_null(ptr: *mut c_void, kind: &str) -> Self {
        assert!(!ptr.is_null(), "{kind} returned nil");
        unsafe { Self::from_raw(ptr) }
    }

    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn supported_camera_models() -> Vec<String> {
        let joined = unsafe { take_owned_string(ffi::ci_raw_filter_supported_camera_models_lines()) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, CIError> {
        let path = crate::util::path_to_cstring(path.as_ref())?;
        let mut filter = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe { ffi::ci_raw_filter_new_from_path(path.as_ptr(), &mut filter, &mut error) };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(filter, "CIRAWFilter(imageURL:)"))
    }

    pub fn from_data(data: &[u8], identifier_hint: Option<&str>) -> Result<Self, CIError> {
        if data.is_empty() {
            return Err(CIError::InvalidArgument(
                "RAW image data must not be empty".to_string(),
            ));
        }
        let identifier_hint = identifier_hint
            .map(|value| string_to_cstring(value, "RAW identifier hint"))
            .transpose()?;
        let mut filter = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_raw_filter_new_from_data(
                data.as_ptr(),
                data.len(),
                identifier_hint.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                &mut filter,
                &mut error,
            )
        };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(
            filter,
            "CIRAWFilter(imageData:identifierHint:)",
        ))
    }

    pub fn supported_decoder_versions(&self) -> Vec<String> {
        let joined = unsafe { take_owned_string(ffi::ci_raw_filter_supported_decoder_versions_lines(self.ptr)) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

    pub fn native_size(&self) -> CGSize {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_raw_filter_native_size(self.ptr, &mut width, &mut height) };
        CGSize::new(width, height)
    }

    pub fn properties_json(&self) -> String {
        unsafe { take_owned_string(ffi::ci_raw_filter_properties_json(self.ptr)) }
            .unwrap_or_else(|| "{}".to_string())
    }

    pub fn orientation(&self) -> u32 {
        unsafe { ffi::ci_raw_filter_orientation(self.ptr) }
    }

    pub fn set_orientation(&mut self, orientation: u32) {
        unsafe { ffi::ci_raw_filter_set_orientation(self.ptr, orientation) };
    }

    pub fn is_draft_mode_enabled(&self) -> bool {
        unsafe { ffi::ci_raw_filter_is_draft_mode_enabled(self.ptr) }
    }

    pub fn set_draft_mode_enabled(&mut self, enabled: bool) {
        unsafe { ffi::ci_raw_filter_set_draft_mode_enabled(self.ptr, enabled) };
    }

    pub fn decoder_version(&self) -> String {
        unsafe { take_owned_string(ffi::ci_raw_filter_decoder_version(self.ptr)) }.unwrap_or_default()
    }

    pub fn set_decoder_version(&mut self, version: CIRAWDecoderVersion) {
        let value = version.as_str();
        let Ok(value) = string_to_cstring(value, "RAW decoder version") else {
            return;
        };
        unsafe { ffi::ci_raw_filter_set_decoder_version(self.ptr, value.as_ptr()) };
    }

    pub fn scale_factor(&self) -> f32 {
        unsafe { ffi::ci_raw_filter_scale_factor(self.ptr) }
    }

    pub fn set_scale_factor(&mut self, value: f32) {
        unsafe { ffi::ci_raw_filter_set_scale_factor(self.ptr, value) };
    }

    pub fn exposure(&self) -> f32 {
        unsafe { ffi::ci_raw_filter_exposure(self.ptr) }
    }

    pub fn set_exposure(&mut self, value: f32) {
        unsafe { ffi::ci_raw_filter_set_exposure(self.ptr, value) };
    }

    pub fn preview_image(&self) -> Option<CIImage> {
        let handle = unsafe { ffi::ci_raw_filter_preview_image(self.ptr) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { CIImage::from_raw(handle) })
        }
    }

    pub fn output_image(&self) -> Option<CIImage> {
        let handle = unsafe { ffi::ci_filter_output_image(self.ptr) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { CIImage::from_raw(handle) })
        }
    }
}
