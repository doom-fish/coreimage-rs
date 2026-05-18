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

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

/// Calls the `CoreImage` framework counterpart for `new`.
    pub fn new(rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_filter_shape_new(rect.origin.x, rect.origin.y, rect.size.width, rect.size.height) },
            "CIFilterShape(rect:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `extent`.
    pub fn extent(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_filter_shape_extent(self.ptr, &mut x, &mut y, &mut width, &mut height) };
        CGRect::new(x, y, width, height)
    }

/// Calls the `CoreImage` framework counterpart for `transform`.
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

/// Calls the `CoreImage` framework counterpart for `inset`.
    pub fn inset(&self, dx: i32, dy: i32) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_filter_shape_inset(self.ptr, dx, dy) },
            "CIFilterShape.insetBy(x:y:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `union`.
    pub fn union(&self, other: &Self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_filter_shape_union(self.ptr, other.ptr) },
            "CIFilterShape.union(with:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `union_rect`.
    pub fn union_rect(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_filter_shape_union_rect(self.ptr, rect.origin.x, rect.origin.y, rect.size.width, rect.size.height)
            },
            "CIFilterShape.union(with:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `intersection`.
    pub fn intersection(&self, other: &Self) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_filter_shape_intersect(self.ptr, other.ptr) },
            "CIFilterShape.intersect(with:)",
        )
    }

/// Calls the `CoreImage` framework counterpart for `intersection_rect`.
    pub fn intersection_rect(&self, rect: CGRect) -> Self {
        Self::from_non_null(
            unsafe {
                ffi::ci_filter_shape_intersect_rect(
                    self.ptr,
                    rect.origin.x,
                    rect.origin.y,
                    rect.size.width,
                    rect.size.height,
                )
            },
            "CIFilterShape.intersect(with:)",
        )
    }
}
