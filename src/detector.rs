use core::ffi::c_void;
use core::fmt;
use core::ptr;

use crate::context::CIContext;
use crate::feature::CIFeature;
use crate::ffi;
use crate::image::CIImage;
use crate::util::{status_result, take_array_objects};
use crate::CIError;

/// A detector class for finding faces, rectangles, QR codes, and text features in images.
pub struct CIDetector {
    ptr: *mut c_void,
}

/// Built-in detector kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIDetectorType {
/// Mirrors the `CoreImage` framework case `Face`.
    Face,
/// Mirrors the `CoreImage` framework case `Rectangle`.
    Rectangle,
/// Mirrors the `CoreImage` framework case `QrCode`.
    QrCode,
/// Mirrors the `CoreImage` framework case `Text`.
    Text,
}

impl CIDetectorType {
    const fn code(self) -> i32 {
        match self {
            Self::Face => 0,
            Self::Rectangle => 1,
            Self::QrCode => 2,
            Self::Text => 3,
        }
    }
}

/// Accuracy / performance trade-offs for detector construction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CIDetectorAccuracy {
/// Mirrors the `CoreImage` framework case `Default`.
    #[default]
    Default,
/// Mirrors the `CoreImage` framework case `Low`.
    Low,
/// Mirrors the `CoreImage` framework case `High`.
    High,
}

impl CIDetectorAccuracy {
    const fn code(self) -> i32 {
        match self {
            Self::Default => 0,
            Self::Low => 1,
            Self::High => 2,
        }
    }
}

/// Configuration for `CIDetector` creation.
#[derive(Clone, Copy, Debug, Default)]
pub struct CIDetectorOptions {
/// Mirrors the `CoreImage` framework property for `accuracy`.
    pub accuracy: CIDetectorAccuracy,
/// Mirrors the `CoreImage` framework property for `tracking`.
    pub tracking: bool,
/// Mirrors the `CoreImage` framework property for `min_feature_size`.
    pub min_feature_size: Option<f64>,
/// Mirrors the `CoreImage` framework property for `max_feature_count`.
    pub max_feature_count: Option<u32>,
/// Mirrors the `CoreImage` framework property for `number_of_angles`.
    pub number_of_angles: Option<u32>,
}

/// Options for a single feature-detection request.
#[derive(Clone, Copy, Debug, Default)]
pub struct CIDetectionOptions {
/// Mirrors the `CoreImage` framework property for `image_orientation`.
    pub image_orientation: Option<u32>,
/// Mirrors the `CoreImage` framework property for `eye_blink`.
    pub eye_blink: bool,
/// Mirrors the `CoreImage` framework property for `smile`.
    pub smile: bool,
/// Mirrors the `CoreImage` framework property for `focal_length`.
    pub focal_length: Option<f64>,
/// Mirrors the `CoreImage` framework property for `aspect_ratio`.
    pub aspect_ratio: Option<f64>,
/// Mirrors the `CoreImage` framework property for `return_sub_features`.
    pub return_sub_features: bool,
}

impl Drop for CIDetector {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIDetector {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIDetector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIDetector")
            .field("ptr", &self.ptr)
            .finish_non_exhaustive()
    }
}

impl CIDetector {
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
    pub fn new(
        detector_type: CIDetectorType,
        context: Option<&CIContext>,
        options: &CIDetectorOptions,
    ) -> Result<Self, CIError> {
        let mut detector = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_detector_new(
                detector_type.code(),
                context.map_or(ptr::null_mut(), CIContext::as_ptr),
                options.accuracy.code(),
                options.tracking,
                options.min_feature_size.unwrap_or(0.0),
                options
                    .max_feature_count
                    .map_or(0, |value| i32::try_from(value).unwrap_or(i32::MAX)),
                options
                    .number_of_angles
                    .map_or(0, |value| i32::try_from(value).unwrap_or(i32::MAX)),
                &mut detector,
                &mut error,
            )
        };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(detector, "CIDetector"))
    }

/// Calls the `CoreImage` framework counterpart for `features_in_image`.
    pub fn features_in_image(
        &self,
        image: &CIImage,
        options: &CIDetectionOptions,
    ) -> Result<Vec<CIFeature>, CIError> {
        let mut features = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_detector_features(
                self.ptr,
                image.as_ptr(),
                options
                    .image_orientation
                    .map_or(0, |value| i32::try_from(value).unwrap_or(i32::MAX)),
                options.eye_blink,
                options.smile,
                options.focal_length.unwrap_or(-1.0),
                options.aspect_ratio.unwrap_or(0.0),
                options.return_sub_features,
                &mut features,
                &mut error,
            )
        };
        unsafe { status_result(status, error)? };
        let handles = unsafe { take_array_objects(features) };
        Ok(handles
            .into_iter()
            .map(|handle| unsafe { CIFeature::from_raw(handle) })
            .collect())
    }
}
