use core::ffi::c_void;
use core::fmt;
use core::ptr;

use apple_cf::cg::{CGAffineTransform, CGRect};

use crate::ffi;
use crate::image::CIImage;
use crate::CIColorSpace;

/// Wrap behavior when sampling outside the image extent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CISamplerWrapMode {
/// Mirrors the `CoreImage` framework case `Black`.
    #[default]
    Black,
/// Mirrors the `CoreImage` framework case `Clamp`.
    Clamp,
}

impl CISamplerWrapMode {
    const fn code(self) -> i32 {
        match self {
            Self::Black => 0,
            Self::Clamp => 1,
        }
    }
}

/// Sampling filter mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CISamplerFilterMode {
/// Mirrors the `CoreImage` framework case `Linear`.
    #[default]
    Linear,
/// Mirrors the `CoreImage` framework case `Nearest`.
    Nearest,
}

impl CISamplerFilterMode {
    const fn code(self) -> i32 {
        match self {
            Self::Linear => 0,
            Self::Nearest => 1,
        }
    }
}

/// Options when creating a `CISampler`.
#[derive(Clone, Copy, Debug, Default)]
pub struct CISamplerOptions {
/// Mirrors the `CoreImage` framework property for `wrap_mode`.
    pub wrap_mode: CISamplerWrapMode,
/// Mirrors the `CoreImage` framework property for `filter_mode`.
    pub filter_mode: CISamplerFilterMode,
/// Mirrors the `CoreImage` framework property for `affine_transform`.
    pub affine_transform: Option<CGAffineTransform>,
/// Mirrors the `CoreImage` framework property for `color_space`.
    pub color_space: Option<CIColorSpace>,
}

/// A Core Image sampler object.
pub struct CISampler {
    ptr: *mut c_void,
}

impl Drop for CISampler {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CISampler {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CISampler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CISampler")
            .field("ptr", &self.ptr)
            .field("extent", &self.extent())
            .finish_non_exhaustive()
    }
}

impl CISampler {
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
    pub fn new(image: &CIImage, options: CISamplerOptions) -> Self {
        let transform = options
            .affine_transform
            .unwrap_or(CGAffineTransform::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0));
        Self::from_non_null(
            unsafe {
                ffi::ci_sampler_new(
                    image.as_ptr(),
                    options.wrap_mode.code(),
                    options.filter_mode.code(),
                    options.affine_transform.is_some(),
                    transform.a,
                    transform.b,
                    transform.c,
                    transform.d,
                    transform.tx,
                    transform.ty,
                    options.color_space.is_some(),
                    options.color_space.map_or(0, CIColorSpace::code),
                )
            },
            "CISampler",
        )
    }

/// Calls the `CoreImage` framework counterpart for `extent`.
    pub fn extent(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_sampler_extent(self.ptr, &mut x, &mut y, &mut width, &mut height) };
        CGRect::new(x, y, width, height)
    }

/// Calls the `CoreImage` framework counterpart for `definition_extent`.
    pub fn definition_extent(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe {
            ffi::ci_sampler_definition_extent(self.ptr, &mut x, &mut y, &mut width, &mut height);
        };
        CGRect::new(x, y, width, height)
    }
}
