use core::ffi::c_void;
use core::fmt;
use core::ptr;

use apple_cf::cg::{CGAffineTransform, CGRect};

use crate::ffi;

/// A Core Image region shape used by accumulators and render plumbing.
pub struct CIFilterShape {
    ptr: *mut c_void,
}

impl Drop for CIFilterShape {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIFilterShape {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIFilterShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIFilterShape")
            .field("ptr", &self.ptr)
            .field("extent", &self.extent())
            .finish_non_exhaustive()
    }
}

impl CIFilterShape {
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

    pub fn new(rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_filter_shape_new(rect.x, rect.y, rect.width, rect.height) },
            "CIFilterShape(rect:)",
        )
    }

    pub fn extent(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_filter_shape_extent(self.ptr, &mut x, &mut y, &mut width, &mut height) };
        CGRect::new(x, y, width, height)
    }

    pub fn transform(&self, transform: CGAffineTransform, interior: bool) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_filter_shape_transform(
                    self.ptr,
                    transform.a,
                    transform.b,
                    transform.c,
                    transform.d,
                    transform.tx,
                    transform.ty,
                    interior,
                )
            },
            "CIFilterShape.transform(by:interior:)",
        )
    }

    pub fn inset(&self, dx: i32, dy: i32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_filter_shape_inset(self.ptr, dx, dy) },
            "CIFilterShape.insetBy(x:y:)",
        )
    }

    pub fn union(&self, other: &Self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_filter_shape_union(self.ptr, other.ptr) },
            "CIFilterShape.union(with:)",
        )
    }

    pub fn union_rect(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_filter_shape_union_rect(self.ptr, rect.x, rect.y, rect.width, rect.height)
            },
            "CIFilterShape.union(with:)",
        )
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_filter_shape_intersect(self.ptr, other.ptr) },
            "CIFilterShape.intersect(with:)",
        )
    }

    pub fn intersection_rect(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_filter_shape_intersect_rect(
                    self.ptr,
                    rect.x,
                    rect.y,
                    rect.width,
                    rect.height,
                )
            },
            "CIFilterShape.intersect(with:)",
        )
    }
}
