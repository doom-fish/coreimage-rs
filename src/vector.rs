use core::ffi::c_void;
use core::fmt;
use core::ptr;

use apple_cf::cg::{CGAffineTransform, CGPoint, CGRect};

use crate::ffi;
use crate::util::{string_to_cstring, take_owned_string};

/// A Core Image vector value used for filter inputs such as centers, extents, and corners.
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
            .field("values", &self.values())
            .finish_non_exhaustive()
    }
}

impl CIVector {
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

    pub fn new1(x: f64) -> Self {
        Self::from_non_null(unsafe { ffi::ci_vector_new1(x) }, "CIVector")
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

    pub fn from_rect(rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_vector_from_rect(rect.origin.x, rect.origin.y, rect.size.width, rect.size.height) },
            "CIVector",
        )
    }

    pub fn from_transform(transform: CGAffineTransform) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_vector_from_transform(
                    transform.a,
                    transform.b,
                    transform.c,
                    transform.d,
                    transform.tx,
                    transform.ty,
                )
            },
            "CIVector",
        )
    }

    pub fn from_string(representation: &str) -> Option<Self> {
        let representation = string_to_cstring(representation, "representation").ok()?;
        let handle = unsafe { ffi::ci_vector_from_string(representation.as_ptr()) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(handle) })
        }
    }

    pub fn count(&self) -> usize {
        unsafe { ffi::ci_vector_count(self.ptr) }
    }

    pub fn value_at(&self, index: usize) -> f64 {
        unsafe { ffi::ci_vector_value_at(self.ptr, index) }
    }

    pub fn values(&self) -> Vec<f64> {
        (0..self.count())
            .map(|index| self.value_at(index))
            .collect()
    }

    pub fn x(&self) -> f64 {
        unsafe { ffi::ci_vector_x(self.ptr) }
    }

    pub fn y(&self) -> f64 {
        unsafe { ffi::ci_vector_y(self.ptr) }
    }

    pub fn z(&self) -> f64 {
        unsafe { ffi::ci_vector_z(self.ptr) }
    }

    pub fn w(&self) -> f64 {
        unsafe { ffi::ci_vector_w(self.ptr) }
    }

    pub fn point_value(&self) -> CGPoint {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { ffi::ci_vector_point(self.ptr, &mut x, &mut y) };
        CGPoint { x, y }
    }

    pub fn rect_value(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_vector_rect(self.ptr, &mut x, &mut y, &mut width, &mut height) };
        CGRect::new(x, y, width, height)
    }

    pub fn affine_transform_value(&self) -> CGAffineTransform {
        let mut a = 0.0;
        let mut b = 0.0;
        let mut c = 0.0;
        let mut d = 0.0;
        let mut tx = 0.0;
        let mut ty = 0.0;
        unsafe {
            ffi::ci_vector_transform(self.ptr, &mut a, &mut b, &mut c, &mut d, &mut tx, &mut ty);
        };
        CGAffineTransform { a, b, c, d, tx, ty }
    }

    pub fn string_representation(&self) -> String {
        unsafe { take_owned_string(ffi::ci_vector_string_representation(self.ptr)) }
            .unwrap_or_default()
    }
}

impl From<(f64,)> for CIVector {
    fn from(value: (f64,)) -> Self {
        Self::new1(value.0)
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
