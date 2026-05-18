use core::ffi::c_void;
use core::fmt;
use core::ptr;
use std::path::Path;

use apple_cf::cg::{CGAffineTransform, CGImage, CGRect};
use apple_cf::cv::CVPixelBuffer;
use apple_cf::iosurface::IOSurface;

use crate::color::CIColor;
use crate::ffi;
use crate::filter::CIFilter;
use crate::util::{path_to_cstring, status_result, string_to_cstring, take_owned_string};
use crate::{CIColorSpace, CIError, CIFormat};

/// An immutable Core Image image.
pub struct CIImage {
    ptr: *mut c_void,
}

impl Drop for CIImage {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: `self.ptr` is a valid Objective-C handle returned by the FFI layer.
            // `ci_object_release` decrements the retain count exactly once and is called
            // exactly once (guaranteed by Drop semantics and the null check above).
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIImage {
    fn clone(&self) -> Self {
        Self {
            // SAFETY: `self.ptr` is a valid Objective-C handle returned by the FFI layer.
            // `ci_object_retain` increments the retain count and is thread-safe. Each clone
            // gets its own independent ownership of the object (managed by Drop).
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIImage")
            .field("ptr", &self.ptr)
            .field("extent", &self.extent())
            .field("opaque", &self.is_opaque())
            .finish()
    }
}

impl CIImage {
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

/// Calls the `CoreImage` framework counterpart for `from_path`.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut image = ptr::null_mut();
        let mut error = ptr::null_mut();
        // SAFETY: `path.as_ptr()` is a valid nul-terminated C string from CString.
        // The FFI function populates `image` with a valid Objective-C handle (or null on error).
        let status = unsafe { ffi::ci_image_from_path(path.as_ptr(), &mut image, &mut error) };
        // SAFETY: `error` is either null or a valid C string pointer allocated by the FFI layer.
        // `status_result` takes ownership and frees the error if necessary.
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(image, "CIImage(contentsOf:)"))
    }

/// Calls the `CoreImage` framework counterpart for `from_encoded_data`.
    pub fn from_encoded_data(data: &[u8]) -> Result<Self, CIError> {
        if data.is_empty() {
            return Err(CIError::InvalidArgument(
                "encoded image data must not be empty".to_string(),
            ));
        }
        let mut image = ptr::null_mut();
        let mut error = ptr::null_mut();
        // SAFETY: `data.as_ptr()` is a valid byte slice pointer. `data.len()` is the correct length.
        // The FFI function populates `image` with a valid handle (or null on error).
        let status = unsafe {
            ffi::ci_image_from_encoded_data(data.as_ptr(), data.len(), &mut image, &mut error)
        };
        // SAFETY: `error` is either null or a valid C string allocated by the FFI layer.
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(image, "CIImage(data:)"))
    }

/// Calls the `CoreImage` framework counterpart for `from_cg_image`.
    pub fn from_cg_image(image: &CGImage) -> Self {
        Self::from_non_null(
            // SAFETY: `image.as_ptr()` is a valid CoreGraphics handle. The FFI function wraps it
            // in a CIImage and returns a valid Objective-C handle.
            unsafe { ffi::ci_image_from_cg_image(image.as_ptr()) },
            "CIImage(cgImage:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `from_cv_pixel_buffer`.
    pub fn from_cv_pixel_buffer(buffer: &CVPixelBuffer) -> Self {
        Self::from_non_null(
            // SAFETY: `buffer.as_ptr()` is a valid CoreVideo handle. The FFI function wraps it
            // in a CIImage and returns a valid Objective-C handle.
            unsafe { ffi::ci_image_from_cv_pixel_buffer(buffer.as_ptr()) },
            "CIImage(cvPixelBuffer:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `from_iosurface`.
    pub fn from_iosurface(surface: &IOSurface) -> Self {
        Self::from_non_null(
            // SAFETY: `surface.as_ptr()` is a valid IOSurface handle. The FFI function wraps it
            // in a CIImage and returns a valid Objective-C handle.
            unsafe { ffi::ci_image_from_iosurface(surface.as_ptr()) },
            "CIImage(ioSurface:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `from_color`.
    pub fn from_color(color: &CIColor) -> Self {
        Self::from_non_null(
            // SAFETY: `color.as_ptr()` is a valid CIColor handle. The FFI function returns
            // a valid Objective-C handle.
            unsafe { ffi::ci_image_from_color(color.as_ptr()) },
            "CIImage(color:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `empty`.
    pub fn empty() -> Self {
        Self::from_non_null(
            // SAFETY: `ci_image_empty` returns a valid Objective-C image handle.
            unsafe { ffi::ci_image_empty() },
            "CIImage.emptyImage"
        )
    }

/// Calls the `CoreImage` framework counterpart for `named_color`.
    pub fn named_color(color: crate::color::CIColorName) -> Self {
        Self::from_color(&CIColor::named(color))
    }

/// Calls the `CoreImage` framework counterpart for `from_bitmap`.
    pub fn from_bitmap(
        data: &[u8],
        width: usize,
        height: usize,
        bytes_per_row: usize,
        format: CIFormat,
        color_space: Option<CIColorSpace>,
    ) -> Result<Self, CIError> {
        let required_len = bytes_per_row
            .checked_mul(height)
            .ok_or_else(|| CIError::InvalidArgument("bitmap dimensions overflowed".to_string()))?;

        if data.len() < required_len {
            return Err(CIError::InvalidArgument(format!(
                "expected at least {required_len} bytes for a {width}x{height} bitmap with {bytes_per_row} bytes per row, got {}",
                data.len()
            )));
        }

        let mut image = ptr::null_mut();
        let mut error = ptr::null_mut();
        // SAFETY: `data.as_ptr()` is valid and points to `data.len()` bytes.
        // `bytes_per_row` is validated to safely multiply with height. The FFI function
        // returns a valid handle (or null on error).
        let status = unsafe {
            ffi::ci_image_from_bitmap(
                data.as_ptr(),
                data.len(),
                width,
                height,
                bytes_per_row,
                format.raw_value(),
                color_space.is_some(),
                color_space.map_or(0, CIColorSpace::code),
                &mut image,
                &mut error,
            )
        };
        // SAFETY: `error` is either null or a valid C string allocated by the FFI layer.
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(image, "CIImage(bitmapData:)"))
    }

/// Calls the `CoreImage` framework counterpart for `from_bitmap_rgba8`.
    pub fn from_bitmap_rgba8(data: &[u8], width: usize, height: usize) -> Result<Self, CIError> {
        let expected_len = width
            .checked_mul(height)
            .and_then(|pixel_count| pixel_count.checked_mul(4))
            .ok_or_else(|| CIError::InvalidArgument("bitmap dimensions overflowed".to_string()))?;

        if data.len() != expected_len {
            return Err(CIError::InvalidArgument(format!(
                "expected {expected_len} RGBA bytes for a {width}x{height} image, got {}",
                data.len()
            )));
        }

        Self::from_bitmap(
            data,
            width,
            height,
            width * 4,
            CIFormat::Rgba8,
            Some(CIColorSpace::Srgb),
        )
    }

/// Calls the `CoreImage` framework counterpart for `extent`.
    pub fn extent(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        // SAFETY: `self.ptr` is a valid CIImage handle. The FFI function writes to the provided
        // mutable references, which are stack-allocated and valid.
        unsafe { ffi::ci_image_extent(self.ptr, &mut x, &mut y, &mut width, &mut height) };
        CGRect::new(x, y, width, height)
    }

/// Calls the `CoreImage` framework counterpart for `is_opaque`.
    pub fn is_opaque(&self) -> bool {
        unsafe { ffi::ci_image_is_opaque(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `properties_json`.
    pub fn properties_json(&self) -> String {
        unsafe { take_owned_string(ffi::ci_image_properties_json(self.ptr)) }
            .unwrap_or_else(|| "{}".to_string())
    }

/// Calls the `CoreImage` framework counterpart for `cropped_to`.
    pub fn cropped_to(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_cropped(self.ptr, rect.origin.x, rect.origin.y, rect.size.width, rect.size.height) },
            "CIImage.cropped(to:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `clamped_to_extent`.
    pub fn clamped_to_extent(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_clamped_to_extent(self.ptr) },
            "CIImage.clampedToExtent()",
        )
    }

/// Calls the `CoreImage` framework counterpart for `clamped_to_rect`.
    pub fn clamped_to_rect(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_image_clamped_to_rect(self.ptr, rect.origin.x, rect.origin.y, rect.size.width, rect.size.height)
            },
            "CIImage.clamped(to:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `applying_orientation`.
    pub fn applying_orientation(&self, exif_orientation: u32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_applying_orientation(self.ptr, exif_orientation) },
            "CIImage.oriented(forExifOrientation:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `oriented`.
    pub fn oriented(&self, exif_orientation: u32) -> Self {
        self.applying_orientation(exif_orientation)
    }

/// Calls the `CoreImage` framework counterpart for `composited_over`.
    pub fn composited_over(&self, background: &Self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_composited_over(self.ptr, background.ptr) },
            "CIImage.composited(over:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `transformed`.
    pub fn transformed(&self, transform: CGAffineTransform) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_image_transformed(
                    self.ptr,
                    transform.a,
                    transform.b,
                    transform.c,
                    transform.d,
                    transform.tx,
                    transform.ty,
                )
            },
            "CIImage.transformed(by:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `translated`.
    pub fn translated(&self, dx: f64, dy: f64) -> Self {
        self.transformed(CGAffineTransform::translation(dx, dy))
    }

/// Calls the `CoreImage` framework counterpart for `scaled`.
    pub fn scaled(&self, sx: f64, sy: f64) -> Self {
        self.transformed(CGAffineTransform::scale(sx, sy))
    }

/// Calls the `CoreImage` framework counterpart for `applying_filter`.
    pub fn applying_filter(&self, filter_name: &str) -> Option<Self> {
        let filter_name = string_to_cstring(filter_name, "filter name").ok()?;
        let handle = unsafe { ffi::ci_image_apply_filter_name(self.ptr, filter_name.as_ptr()) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(handle) })
        }
    }

/// Calls the `CoreImage` framework counterpart for `premultiplying_alpha`.
    pub fn premultiplying_alpha(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_premultiplying_alpha(self.ptr) },
            "CIImage.premultiplyingAlpha()",
        )
    }

/// Calls the `CoreImage` framework counterpart for `unpremultiplying_alpha`.
    pub fn unpremultiplying_alpha(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_unpremultiplying_alpha(self.ptr) },
            "CIImage.unpremultiplyingAlpha()",
        )
    }

/// Calls the `CoreImage` framework counterpart for `setting_alpha_one_in_extent`.
    pub fn setting_alpha_one_in_extent(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_image_setting_alpha_one_in_extent(
                    self.ptr,
                    rect.origin.x,
                    rect.origin.y,
                    rect.size.width,
                    rect.size.height,
                )
            },
            "CIImage.settingAlphaOne(in:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `gaussian_blurred`.
    pub fn gaussian_blurred(&self, sigma: f64) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_gaussian_blur(self.ptr, sigma) },
            "CIImage.applyingGaussianBlur(sigma:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `sampling_linear`.
    pub fn sampling_linear(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_sampling_linear(self.ptr) },
            "CIImage.samplingLinear()",
        )
    }

/// Calls the `CoreImage` framework counterpart for `sampling_nearest`.
    pub fn sampling_nearest(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_sampling_nearest(self.ptr) },
            "CIImage.samplingNearest()",
        )
    }

/// Calls the `CoreImage` framework counterpart for `inserting_intermediate`.
    pub fn inserting_intermediate(&self, cache: bool) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_insert_intermediate(self.ptr, cache) },
            "CIImage.insertingIntermediate(cache:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `applying_gain_map`.
    pub fn applying_gain_map(&self, gain_map: &Self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_apply_gain_map(self.ptr, gain_map.ptr) },
            "CIImage.applyingGainMap(_:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `applying_gain_map_with_headroom`.
    pub fn applying_gain_map_with_headroom(&self, gain_map: &Self, headroom: f32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_apply_gain_map_headroom(self.ptr, gain_map.ptr, headroom) },
            "CIImage.applyingGainMap(_:headroom:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `content_headroom`.
    pub fn content_headroom(&self) -> f32 {
        unsafe { ffi::ci_image_content_headroom(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `content_average_light_level`.
    pub fn content_average_light_level(&self) -> f32 {
        unsafe { ffi::ci_image_content_average_light_level(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `setting_content_headroom`.
    pub fn setting_content_headroom(&self, headroom: f32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_setting_content_headroom(self.ptr, headroom) },
            "CIImage.settingContentHeadroom(_:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `setting_content_average_light_level`.
    pub fn setting_content_average_light_level(&self, average: f32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_setting_content_average_light_level(self.ptr, average) },
            "CIImage.settingContentAverageLightLevel(_:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `region_of_interest_for_image`.
    pub fn region_of_interest_for_image(&self, image: &Self, rect: CGRect) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe {
            ffi::ci_image_region_of_interest_for_image(
                self.ptr,
                image.ptr,
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
                &mut x,
                &mut y,
                &mut width,
                &mut height,
            );
        };
        CGRect::new(x, y, width, height)
    }

/// Calls the `CoreImage` framework counterpart for `apply_filter`.
    pub fn apply_filter(&self, filter: &mut CIFilter) -> Result<Self, CIError> {
        filter.set_input_image(self);
        filter
            .output_image()
            .ok_or_else(|| CIError::NullResult("CIFilter.outputImage returned nil".to_string()))
    }
}
