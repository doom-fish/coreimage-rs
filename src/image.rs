use core::ffi::c_void;
use core::fmt;
use core::ptr;
use std::path::Path;

use apple_cf::cg::{CGAffineTransform, CGImage, CGRect};
use apple_cf::cv::CVPixelBuffer;
use apple_cf::iosurface::IOSurface;

use crate::ffi;
use crate::filter::CIFilter;
use crate::util::{path_to_cstring, status_result, take_owned_string};
use crate::CIError;

/// An immutable CoreImage image.
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

        let mut image = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_image_from_bitmap_rgba8(
                data.as_ptr(),
                data.len(),
                width,
                height,
                &mut image,
                &mut error,
            )
        };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(image, "CIImage(bitmapData:)"))
    }

    pub fn extent(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_image_extent(self.ptr, &mut x, &mut y, &mut width, &mut height) };
        CGRect::new(x, y, width, height)
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

    pub fn apply_filter(&self, filter: &mut CIFilter) -> Result<Self, CIError> {
        filter.set_input_image(self);
        filter
            .output_image()
            .ok_or_else(|| CIError::NullResult("CIFilter.outputImage returned nil".to_string()))
    }
}
