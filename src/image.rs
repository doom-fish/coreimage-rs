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
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIImage {
    fn clone(&self) -> Self {
        Self {
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

    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut image = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe { ffi::ci_image_from_path(path.as_ptr(), &mut image, &mut error) };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(image, "CIImage(contentsOf:)"))
    }

    pub fn from_encoded_data(data: &[u8]) -> Result<Self, CIError> {
        if data.is_empty() {
            return Err(CIError::InvalidArgument(
                "encoded image data must not be empty".to_string(),
            ));
        }
        let mut image = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_image_from_encoded_data(data.as_ptr(), data.len(), &mut image, &mut error)
        };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(image, "CIImage(data:)"))
    }

    pub fn from_cg_image(image: &CGImage) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_from_cg_image(image.as_ptr()) },
            "CIImage(cgImage:)",
        )
    }

    pub fn from_cv_pixel_buffer(buffer: &CVPixelBuffer) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_from_cv_pixel_buffer(buffer.as_ptr()) },
            "CIImage(cvPixelBuffer:)",
        )
    }

    pub fn from_iosurface(surface: &IOSurface) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_from_iosurface(surface.as_ptr()) },
            "CIImage(ioSurface:)",
        )
    }

    pub fn from_color(color: &CIColor) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_from_color(color.as_ptr()) },
            "CIImage(color:)",
        )
    }

    pub fn empty() -> Self {
        Self::from_non_null(unsafe { ffi::ci_image_empty() }, "CIImage.emptyImage")
    }

    pub fn named_color(color: crate::color::CIColorName) -> Self {
        Self::from_color(&CIColor::named(color))
    }

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
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(image, "CIImage(bitmapData:)"))
    }

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

    pub fn extent(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_image_extent(self.ptr, &mut x, &mut y, &mut width, &mut height) };
        CGRect::new(x, y, width, height)
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { ffi::ci_image_is_opaque(self.ptr) }
    }

    pub fn properties_json(&self) -> String {
        unsafe { take_owned_string(ffi::ci_image_properties_json(self.ptr)) }
            .unwrap_or_else(|| "{}".to_string())
    }

    pub fn cropped_to(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_cropped(self.ptr, rect.x, rect.y, rect.width, rect.height) },
            "CIImage.cropped(to:)",
        )
    }

    pub fn clamped_to_extent(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_clamped_to_extent(self.ptr) },
            "CIImage.clampedToExtent()",
        )
    }

    pub fn clamped_to_rect(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_image_clamped_to_rect(self.ptr, rect.x, rect.y, rect.width, rect.height)
            },
            "CIImage.clamped(to:)",
        )
    }

    pub fn applying_orientation(&self, exif_orientation: u32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_applying_orientation(self.ptr, exif_orientation) },
            "CIImage.oriented(forExifOrientation:)",
        )
    }

    pub fn oriented(&self, exif_orientation: u32) -> Self {
        self.applying_orientation(exif_orientation)
    }

    pub fn composited_over(&self, background: &Self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_composited_over(self.ptr, background.ptr) },
            "CIImage.composited(over:)",
        )
    }

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

    pub fn translated(&self, dx: f64, dy: f64) -> Self {
        self.transformed(CGAffineTransform::translation(dx, dy))
    }

    pub fn scaled(&self, sx: f64, sy: f64) -> Self {
        self.transformed(CGAffineTransform::scale(sx, sy))
    }

    pub fn applying_filter(&self, filter_name: &str) -> Option<Self> {
        let filter_name = string_to_cstring(filter_name, "filter name").ok()?;
        let handle = unsafe { ffi::ci_image_apply_filter_name(self.ptr, filter_name.as_ptr()) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(handle) })
        }
    }

    pub fn premultiplying_alpha(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_premultiplying_alpha(self.ptr) },
            "CIImage.premultiplyingAlpha()",
        )
    }

    pub fn unpremultiplying_alpha(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_unpremultiplying_alpha(self.ptr) },
            "CIImage.unpremultiplyingAlpha()",
        )
    }

    pub fn setting_alpha_one_in_extent(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_image_setting_alpha_one_in_extent(
                    self.ptr,
                    rect.x,
                    rect.y,
                    rect.width,
                    rect.height,
                )
            },
            "CIImage.settingAlphaOne(in:)",
        )
    }

    pub fn gaussian_blurred(&self, sigma: f64) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_gaussian_blur(self.ptr, sigma) },
            "CIImage.applyingGaussianBlur(sigma:)",
        )
    }

    pub fn sampling_linear(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_sampling_linear(self.ptr) },
            "CIImage.samplingLinear()",
        )
    }

    pub fn sampling_nearest(&self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_sampling_nearest(self.ptr) },
            "CIImage.samplingNearest()",
        )
    }

    pub fn inserting_intermediate(&self, cache: bool) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_insert_intermediate(self.ptr, cache) },
            "CIImage.insertingIntermediate(cache:)",
        )
    }

    pub fn applying_gain_map(&self, gain_map: &Self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_apply_gain_map(self.ptr, gain_map.ptr) },
            "CIImage.applyingGainMap(_:)",
        )
    }

    pub fn applying_gain_map_with_headroom(&self, gain_map: &Self, headroom: f32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_apply_gain_map_headroom(self.ptr, gain_map.ptr, headroom) },
            "CIImage.applyingGainMap(_:headroom:)",
        )
    }

    pub fn content_headroom(&self) -> f32 {
        unsafe { ffi::ci_image_content_headroom(self.ptr) }
    }

    pub fn content_average_light_level(&self) -> f32 {
        unsafe { ffi::ci_image_content_average_light_level(self.ptr) }
    }

    pub fn setting_content_headroom(&self, headroom: f32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_setting_content_headroom(self.ptr, headroom) },
            "CIImage.settingContentHeadroom(_:)",
        )
    }

    pub fn setting_content_average_light_level(&self, average: f32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_image_setting_content_average_light_level(self.ptr, average) },
            "CIImage.settingContentAverageLightLevel(_:)",
        )
    }

    pub fn region_of_interest_for_image(&self, image: &Self, rect: CGRect) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe {
            ffi::ci_image_region_of_interest_for_image(
                self.ptr,
                image.ptr,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                &mut x,
                &mut y,
                &mut width,
                &mut height,
            );
        };
        CGRect::new(x, y, width, height)
    }

    pub fn apply_filter(&self, filter: &mut CIFilter) -> Result<Self, CIError> {
        filter.set_input_image(self);
        filter
            .output_image()
            .ok_or_else(|| CIError::NullResult("CIFilter.outputImage returned nil".to_string()))
    }
}
