use core::ffi::c_void;
use core::fmt;
use core::ptr;

use apple_cf::cg::CGRect;

use crate::color::CIColor;
use crate::ffi;
use crate::image::CIImage;
use crate::util::{status_result, string_to_cstring, take_owned_string};
use crate::vector::CIVector;
use crate::CIError;

/// Built-in Core Image blend kernels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIBlendKernelKind {
    ComponentAdd,
    ComponentMultiply,
    ComponentMin,
    ComponentMax,
    Clear,
    Source,
    Destination,
    SourceOver,
    DestinationOver,
    SourceIn,
    DestinationIn,
    SourceOut,
    DestinationOut,
    SourceAtop,
    DestinationAtop,
    ExclusiveOr,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    Subtract,
    Divide,
    LinearBurn,
    LinearDodge,
    VividLight,
    LinearLight,
    PinLight,
    HardMix,
    DarkerColor,
    LighterColor,
}

impl CIBlendKernelKind {
    const fn code(self) -> i32 {
        match self {
            Self::ComponentAdd => 0,
            Self::ComponentMultiply => 1,
            Self::ComponentMin => 2,
            Self::ComponentMax => 3,
            Self::Clear => 4,
            Self::Source => 5,
            Self::Destination => 6,
            Self::SourceOver => 7,
            Self::DestinationOver => 8,
            Self::SourceIn => 9,
            Self::DestinationIn => 10,
            Self::SourceOut => 11,
            Self::DestinationOut => 12,
            Self::SourceAtop => 13,
            Self::DestinationAtop => 14,
            Self::ExclusiveOr => 15,
            Self::Multiply => 16,
            Self::Screen => 17,
            Self::Overlay => 18,
            Self::Darken => 19,
            Self::Lighten => 20,
            Self::ColorDodge => 21,
            Self::ColorBurn => 22,
            Self::HardLight => 23,
            Self::SoftLight => 24,
            Self::Difference => 25,
            Self::Exclusion => 26,
            Self::Hue => 27,
            Self::Saturation => 28,
            Self::Color => 29,
            Self::Luminosity => 30,
            Self::Subtract => 31,
            Self::Divide => 32,
            Self::LinearBurn => 33,
            Self::LinearDodge => 34,
            Self::VividLight => 35,
            Self::LinearLight => 36,
            Self::PinLight => 37,
            Self::HardMix => 38,
            Self::DarkerColor => 39,
            Self::LighterColor => 40,
        }
    }
}

/// A Core Image color kernel compiled from source.
pub struct CIColorKernel {
    ptr: *mut c_void,
}

/// A Core Image warp kernel compiled from source.
pub struct CIWarpKernel {
    ptr: *mut c_void,
}

/// A built-in or custom Core Image blend kernel.
pub struct CIBlendKernel {
    ptr: *mut c_void,
}

macro_rules! impl_kernel_handle {
    ($name:ident) => {
        impl Drop for $name {
            fn drop(&mut self) {
                if !self.ptr.is_null() {
                    unsafe { ffi::ci_object_release(self.ptr) };
                    self.ptr = ptr::null_mut();
                }
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self {
                    ptr: unsafe { ffi::ci_object_retain(self.ptr) },
                }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ptr", &self.ptr)
                    .field("name", &self.name())
                    .finish_non_exhaustive()
            }
        }

        impl $name {
            const unsafe fn from_raw(ptr: *mut c_void) -> Self {
                Self { ptr }
            }

            fn from_non_null(ptr: *mut c_void, kind: &str) -> Self {
                assert!(!ptr.is_null(), "{kind} returned nil");
                unsafe { Self::from_raw(ptr) }
            }

            pub const fn as_ptr(&self) -> *mut c_void {
                self.ptr
            }

            pub fn name(&self) -> String {
                unsafe { take_owned_string(ffi::ci_kernel_name(self.ptr)) }.unwrap_or_default()
            }
        }
    };
}

impl_kernel_handle!(CIColorKernel);
impl_kernel_handle!(CIWarpKernel);
impl_kernel_handle!(CIBlendKernel);

impl CIColorKernel {
    pub fn from_source(source: &str) -> Result<Self, CIError> {
        let source = string_to_cstring(source, "kernel source")?;
        let mut kernel = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_color_kernel_new_source(source.as_ptr(), &mut kernel, &mut error) };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(kernel, "CIColorKernel(source:)"))
    }

    pub fn apply_image_scalar(
        &self,
        image: &CIImage,
        value: f64,
        extent: CGRect,
    ) -> Result<CIImage, CIError> {
        let handle = unsafe {
            ffi::ci_color_kernel_apply_image_scalar(
                self.ptr,
                image.as_ptr(),
                value,
                extent.x,
                extent.y,
                extent.width,
                extent.height,
            )
        };
        if handle.is_null() {
            Err(CIError::NullResult(
                "CIColorKernel.apply returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIImage::from_raw(handle) })
        }
    }

    pub fn apply_image_color(
        &self,
        image: &CIImage,
        color: &CIColor,
        extent: CGRect,
    ) -> Result<CIImage, CIError> {
        let handle = unsafe {
            ffi::ci_color_kernel_apply_image_color(
                self.ptr,
                image.as_ptr(),
                color.as_ptr(),
                extent.x,
                extent.y,
                extent.width,
                extent.height,
            )
        };
        if handle.is_null() {
            Err(CIError::NullResult(
                "CIColorKernel.apply returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIImage::from_raw(handle) })
        }
    }

    pub fn apply_image_vector(
        &self,
        image: &CIImage,
        vector: &CIVector,
        extent: CGRect,
    ) -> Result<CIImage, CIError> {
        let handle = unsafe {
            ffi::ci_color_kernel_apply_image_vector(
                self.ptr,
                image.as_ptr(),
                vector.as_ptr(),
                extent.x,
                extent.y,
                extent.width,
                extent.height,
            )
        };
        if handle.is_null() {
            Err(CIError::NullResult(
                "CIColorKernel.apply returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIImage::from_raw(handle) })
        }
    }
}

impl CIWarpKernel {
    pub fn from_source(source: &str) -> Result<Self, CIError> {
        let source = string_to_cstring(source, "kernel source")?;
        let mut kernel = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_warp_kernel_new_source(source.as_ptr(), &mut kernel, &mut error) };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(kernel, "CIWarpKernel(source:)"))
    }

    pub fn apply_image_scalar(
        &self,
        image: &CIImage,
        value: f64,
        extent: CGRect,
    ) -> Result<CIImage, CIError> {
        let handle = unsafe {
            ffi::ci_warp_kernel_apply_image_scalar(
                self.ptr,
                image.as_ptr(),
                value,
                extent.x,
                extent.y,
                extent.width,
                extent.height,
            )
        };
        if handle.is_null() {
            Err(CIError::NullResult(
                "CIWarpKernel.apply returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIImage::from_raw(handle) })
        }
    }
}

impl CIBlendKernel {
    pub fn built_in(kind: CIBlendKernelKind) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_blend_kernel_builtin(kind.code()) },
            "CIBlendKernel",
        )
    }

    pub fn apply(&self, foreground: &CIImage, background: &CIImage) -> Result<CIImage, CIError> {
        let handle = unsafe {
            ffi::ci_blend_kernel_apply(self.ptr, foreground.as_ptr(), background.as_ptr())
        };
        if handle.is_null() {
            Err(CIError::NullResult(
                "CIBlendKernel.apply returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIImage::from_raw(handle) })
        }
    }
}
