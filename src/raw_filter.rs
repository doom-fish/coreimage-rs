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
/// Mirrors the `CoreImage` framework case `None`.
    None,
/// Mirrors the `CoreImage` framework case `Version9`.
    Version9,
/// Mirrors the `CoreImage` framework case `Version9Dng`.
    Version9Dng,
/// Mirrors the `CoreImage` framework case `Version8`.
    Version8,
/// Mirrors the `CoreImage` framework case `Version8Dng`.
    Version8Dng,
/// Mirrors the `CoreImage` framework case `Version7`.
    Version7,
/// Mirrors the `CoreImage` framework case `Version7Dng`.
    Version7Dng,
/// Mirrors the `CoreImage` framework case `Version6`.
    Version6,
/// Mirrors the `CoreImage` framework case `Version6Dng`.
    Version6Dng,
}

impl CIRAWDecoderVersion {
    fn values() -> &'static Vec<String> {
        static VALUES: OnceLock<Vec<String>> = OnceLock::new();
        VALUES.get_or_init(|| {
            (0..9)
                .map(|index| unsafe {
                    take_owned_string(ffi::ci_raw_decoder_version_name(index)).unwrap_or_default()
                })
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

/// Calls the `CoreImage` framework counterpart for `as_str`.
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

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

/// Calls the `CoreImage` framework counterpart for `supported_camera_models`.
    pub fn supported_camera_models() -> Vec<String> {
        let joined =
            unsafe { take_owned_string(ffi::ci_raw_filter_supported_camera_models_lines()) };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

/// Calls the `CoreImage` framework counterpart for `from_path`.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, CIError> {
        let path = crate::util::path_to_cstring(path.as_ref())?;
        let mut filter = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_raw_filter_new_from_path(path.as_ptr(), &mut filter, &mut error) };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(filter, "CIRAWFilter(imageURL:)"))
    }

/// Calls the `CoreImage` framework counterpart for `from_data`.
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
                identifier_hint
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
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

/// Calls the `CoreImage` framework counterpart for `supported_decoder_versions`.
    pub fn supported_decoder_versions(&self) -> Vec<String> {
        let joined = unsafe {
            take_owned_string(ffi::ci_raw_filter_supported_decoder_versions_lines(
                self.ptr,
            ))
        };
        joined.map_or_else(Vec::new, |text| split_lines(&text))
    }

/// Calls the `CoreImage` framework counterpart for `native_size`.
    pub fn native_size(&self) -> CGSize {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_raw_filter_native_size(self.ptr, &mut width, &mut height) };
        CGSize::new(width, height)
    }

/// Calls the `CoreImage` framework counterpart for `properties_json`.
    pub fn properties_json(&self) -> String {
        unsafe { take_owned_string(ffi::ci_raw_filter_properties_json(self.ptr)) }
            .unwrap_or_else(|| "{}".to_string())
    }

/// Calls the `CoreImage` framework counterpart for `orientation`.
    pub fn orientation(&self) -> u32 {
        unsafe { ffi::ci_raw_filter_orientation(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `set_orientation`.
    pub fn set_orientation(&mut self, orientation: u32) {
        unsafe { ffi::ci_raw_filter_set_orientation(self.ptr, orientation) };
    }

/// Calls the `CoreImage` framework counterpart for `is_draft_mode_enabled`.
    pub fn is_draft_mode_enabled(&self) -> bool {
        unsafe { ffi::ci_raw_filter_is_draft_mode_enabled(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `set_draft_mode_enabled`.
    pub fn set_draft_mode_enabled(&mut self, enabled: bool) {
        unsafe { ffi::ci_raw_filter_set_draft_mode_enabled(self.ptr, enabled) };
    }

/// Calls the `CoreImage` framework counterpart for `decoder_version`.
    pub fn decoder_version(&self) -> String {
        unsafe { take_owned_string(ffi::ci_raw_filter_decoder_version(self.ptr)) }
            .unwrap_or_default()
    }

/// Calls the `CoreImage` framework counterpart for `set_decoder_version`.
    pub fn set_decoder_version(&mut self, version: CIRAWDecoderVersion) {
        let value = version.as_str();
        let Ok(value) = string_to_cstring(value, "RAW decoder version") else {
            return;
        };
        unsafe { ffi::ci_raw_filter_set_decoder_version(self.ptr, value.as_ptr()) };
    }

/// Calls the `CoreImage` framework counterpart for `scale_factor`.
    pub fn scale_factor(&self) -> f32 {
        unsafe { ffi::ci_raw_filter_scale_factor(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `set_scale_factor`.
    pub fn set_scale_factor(&mut self, value: f32) {
        unsafe { ffi::ci_raw_filter_set_scale_factor(self.ptr, value) };
    }

/// Calls the `CoreImage` framework counterpart for `exposure`.
    pub fn exposure(&self) -> f32 {
        unsafe { ffi::ci_raw_filter_exposure(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `set_exposure`.
    pub fn set_exposure(&mut self, value: f32) {
        unsafe { ffi::ci_raw_filter_set_exposure(self.ptr, value) };
    }

/// Calls the `CoreImage` framework counterpart for `preview_image`.
    pub fn preview_image(&self) -> Option<CIImage> {
        let handle = unsafe { ffi::ci_raw_filter_preview_image(self.ptr) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { CIImage::from_raw(handle) })
        }
    }

/// Calls the `CoreImage` framework counterpart for `output_image`.
    pub fn output_image(&self) -> Option<CIImage> {
        let handle = unsafe { ffi::ci_filter_output_image(self.ptr) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { CIImage::from_raw(handle) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoder_version_indexes_are_stable() {
        let cases = [
            (CIRAWDecoderVersion::None, 0),
            (CIRAWDecoderVersion::Version9, 1),
            (CIRAWDecoderVersion::Version9Dng, 2),
            (CIRAWDecoderVersion::Version8, 3),
            (CIRAWDecoderVersion::Version8Dng, 4),
            (CIRAWDecoderVersion::Version7, 5),
            (CIRAWDecoderVersion::Version7Dng, 6),
            (CIRAWDecoderVersion::Version6, 7),
            (CIRAWDecoderVersion::Version6Dng, 8),
        ];

        for (version, expected_index) in cases {
            assert_eq!(version.index(), expected_index);
        }
    }

    #[test]
    fn decoder_version_strings_match_expected_values() {
        assert_eq!(CIRAWDecoderVersion::None.as_str(), "None");
        assert_eq!(CIRAWDecoderVersion::Version9.as_str(), "9");
        assert_eq!(CIRAWDecoderVersion::Version9Dng.as_str(), "9.dng");
        assert_eq!(CIRAWDecoderVersion::Version6Dng.as_str(), "6.dng");
    }

    #[test]
    fn from_data_rejects_empty_buffers() {
        let error = CIRAWFilter::from_data(&[], None).expect_err("empty buffers must be rejected");
        assert_eq!(
            error,
            CIError::InvalidArgument("RAW image data must not be empty".to_string())
        );
    }
}
