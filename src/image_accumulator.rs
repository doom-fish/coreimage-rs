use core::ffi::c_void;
use core::fmt;
use core::ptr;

use apple_cf::cg::CGRect;

use crate::ffi;
use crate::{CIColorSpace, CIFormat, CIImage};

/// A mutable image store for incremental Core Image rendering.
pub struct CIImageAccumulator {
    ptr: *mut c_void,
}

impl Drop for CIImageAccumulator {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIImageAccumulator {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIImageAccumulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIImageAccumulator")
            .field("ptr", &self.ptr)
            .field("extent", &self.extent())
            .field("format", &self.format())
            .finish_non_exhaustive()
    }
}

impl CIImageAccumulator {
    pub(crate) const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

/// Calls the `CoreImage` framework counterpart for `new`.
    pub fn new(extent: CGRect, format: CIFormat) -> Option<Self> {
        Self::new_inner(extent, format, None)
    }

/// Calls the `CoreImage` framework counterpart for `with_color_space`.
    pub fn with_color_space(
        extent: CGRect,
        format: CIFormat,
        color_space: CIColorSpace,
    ) -> Option<Self> {
        Self::new_inner(extent, format, Some(color_space))
    }

    fn new_inner(
        extent: CGRect,
        format: CIFormat,
        color_space: Option<CIColorSpace>,
    ) -> Option<Self> {
        let handle = unsafe {
            ffi::ci_image_accumulator_new(
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
                format.raw_value(),
                color_space.is_some(),
                color_space.map_or(0, CIColorSpace::code),
            )
        };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(handle) })
        }
    }

/// Calls the `CoreImage` framework counterpart for `extent`.
    pub fn extent(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe {
            ffi::ci_image_accumulator_extent(self.ptr, &mut x, &mut y, &mut width, &mut height);
        }
        CGRect::new(x, y, width, height)
    }

/// Calls the `CoreImage` framework counterpart for `format`.
    pub fn format(&self) -> Option<CIFormat> {
        CIFormat::from_raw(unsafe { ffi::ci_image_accumulator_format(self.ptr) })
    }

/// Calls the `CoreImage` framework counterpart for `image`.
    pub fn image(&self) -> Option<CIImage> {
        let handle = unsafe { ffi::ci_image_accumulator_image(self.ptr) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { CIImage::from_raw(handle) })
        }
    }

/// Calls the `CoreImage` framework counterpart for `set_image`.
    pub fn set_image(&mut self, image: &CIImage) {
        unsafe { ffi::ci_image_accumulator_set_image(self.ptr, image.as_ptr()) };
    }

/// Calls the `CoreImage` framework counterpart for `set_image_dirty_rect`.
    pub fn set_image_dirty_rect(&mut self, image: &CIImage, dirty_rect: CGRect) {
        unsafe {
            ffi::ci_image_accumulator_set_image_dirty_rect(
                self.ptr,
                image.as_ptr(),
                dirty_rect.origin.x,
                dirty_rect.origin.y,
                dirty_rect.size.width,
                dirty_rect.size.height,
            );
        };
    }

/// Calls the `CoreImage` framework counterpart for `clear`.
    pub fn clear(&mut self) {
        unsafe { ffi::ci_image_accumulator_clear(self.ptr) };
    }
}
