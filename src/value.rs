use core::ffi::c_void;
use core::fmt;
use core::ptr;

use apple_cf::cg::CGPoint;

use crate::ffi;

/// A CoreImage vector value used for filter inputs such as centers, extents, and corners.
pub struct CIVector {
    ptr: *mut c_void,
}

impl Drop for CIVector {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIVector {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIVector")
            .field("ptr", &self.ptr)
            .finish_non_exhaustive()
    }
}

impl CIVector {
    const fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    fn from_non_null(ptr: *mut c_void, kind: &str) -> Self {
        assert!(!ptr.is_null(), "{kind} returned nil");
        Self::from_raw(ptr)
    }

    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn new(x: f64, y: f64) -> Self {
        Self::from_non_null(unsafe { ffi::ci_vector_new2(x, y) }, "CIVector")
    }

    pub fn new3(x: f64, y: f64, z: f64) -> Self {
        Self::from_non_null(unsafe { ffi::ci_vector_new3(x, y, z) }, "CIVector")
    }

    pub fn new4(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self::from_non_null(unsafe { ffi::ci_vector_new4(x, y, z, w) }, "CIVector")
    }

    pub fn from_point(point: CGPoint) -> Self {
        Self::new(point.x, point.y)
    }
}

impl From<(f64, f64)> for CIVector {
    fn from(value: (f64, f64)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(f64, f64, f64)> for CIVector {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::new3(value.0, value.1, value.2)
    }
}

impl From<(f64, f64, f64, f64)> for CIVector {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self::new4(value.0, value.1, value.2, value.3)
    }
}

/// A CoreImage RGBA color value used for filter inputs and generators.
pub struct CIColor {
    ptr: *mut c_void,
}

impl Drop for CIColor {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIColor {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIColor")
            .field("ptr", &self.ptr)
            .finish_non_exhaustive()
    }
}

impl CIColor {
    const fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    fn from_non_null(ptr: *mut c_void, kind: &str) -> Self {
        assert!(!ptr.is_null(), "{kind} returned nil");
        Self::from_raw(ptr)
    }

    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_color_new_rgba(red, green, blue, alpha) },
            "CIColor",
        )
    }

    pub fn rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::rgba(red, green, blue, 1.0)
    }

    pub fn white(intensity: f64, alpha: f64) -> Self {
        Self::rgba(intensity, intensity, intensity, alpha)
    }
}
