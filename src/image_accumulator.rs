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

    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn new(extent: CGRect, format: CIFormat) -> Option<Self> {
        Self::new_inner(extent, format, None)
    }

    pub fn with_color_space(extent: CGRect, format: CIFormat, color_space: CIColorSpace) -> Option<Self> {
        Self::new_inner(extent, format, Some(color_space))
    }

    fn new_inner(extent: CGRect, format: CIFormat, color_space: Option<CIColorSpace>) -> Option<Self> {
        let handle = unsafe {
            ffi::ci_image_accumulator_new(
                extent.x,
                extent.y,
                extent.width,
                extent.height,
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

    pub fn format(&self) -> Option<CIFormat> {
        CIFormat::from_raw(unsafe { ffi::ci_image_accumulator_format(self.ptr) })
    }

    pub fn image(&self) -> Option<CIImage> {
        let handle = unsafe { ffi::ci_image_accumulator_image(self.ptr) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { CIImage::from_raw(handle) })
        }
    }

    pub fn set_image(&mut self, image: &CIImage) {
        unsafe { ffi::ci_image_accumulator_set_image(self.ptr, image.as_ptr()) };
    }

    pub fn set_image_dirty_rect(&mut self, image: &CIImage, dirty_rect: CGRect) {
        unsafe {
            ffi::ci_image_accumulator_set_image_dirty_rect(
                self.ptr,
                image.as_ptr(),
                dirty_rect.x,
                dirty_rect.y,
                dirty_rect.width,
                dirty_rect.height,
            );
        };
    }

    pub fn clear(&mut self) {
        unsafe { ffi::ci_image_accumulator_clear(self.ptr) };
    }
}
