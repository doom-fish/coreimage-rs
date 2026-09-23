use core::ffi::c_void;
use core::fmt;
use core::ptr;

use apple_cf::cg::CGRect;
use doom_fish_utils::panic_safe::{catch_user_panic_result, catch_user_panic_result_with_cleanup};

use crate::color::CIColor;
use crate::ffi;
use crate::image::CIImage;
use crate::util::{status_result, string_to_cstring, take_owned_string};
use crate::vector::CIVector;
use crate::CIError;

type WarpRegionOfInterestFn = dyn Fn(usize, CGRect) -> CGRect + Send + Sync;

struct WarpRegionOfInterestCallback {
    callback: Box<WarpRegionOfInterestFn>,
    fallback: CGRect,
}

unsafe extern "C" fn warp_region_of_interest_invoke(
    context: *mut c_void,
    input_index: i32,
    destination_x: f64,
    destination_y: f64,
    destination_width: f64,
    destination_height: f64,
    out_x: *mut f64,
    out_y: *mut f64,
    out_width: *mut f64,
    out_height: *mut f64,
) {
    if context.is_null()
        || out_x.is_null()
        || out_y.is_null()
        || out_width.is_null()
        || out_height.is_null()
    {
        return;
    }
    let context = unsafe { &*context.cast::<WarpRegionOfInterestCallback>() };
    let destination = CGRect::new(
        destination_x,
        destination_y,
        destination_width,
        destination_height,
    );
    let region = catch_user_panic_result("CIWarpKernel region-of-interest callback", || {
        let input_index = usize::try_from(input_index).unwrap_or_default();
        (context.callback)(input_index, destination)
    })
    .unwrap_or(context.fallback);
    unsafe {
        *out_x = region.origin.x;
        *out_y = region.origin.y;
        *out_width = region.size.width;
        *out_height = region.size.height;
    }
}

unsafe extern "C" fn warp_region_of_interest_release(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    let state = unsafe { Some(Box::from_raw(context.cast::<WarpRegionOfInterestCallback>())) };
    let _ = catch_user_panic_result_with_cleanup(
        "CIWarpKernel region-of-interest release",
        state,
        |_| (),
        |state| drop(state.take()),
    );
}

/// Built-in Core Image blend kernels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIBlendKernelKind {
/// Mirrors the `CoreImage` framework case `ComponentAdd`.
    ComponentAdd,
/// Mirrors the `CoreImage` framework case `ComponentMultiply`.
    ComponentMultiply,
/// Mirrors the `CoreImage` framework case `ComponentMin`.
    ComponentMin,
/// Mirrors the `CoreImage` framework case `ComponentMax`.
    ComponentMax,
/// Mirrors the `CoreImage` framework case `Clear`.
    Clear,
/// Mirrors the `CoreImage` framework case `Source`.
    Source,
/// Mirrors the `CoreImage` framework case `Destination`.
    Destination,
/// Mirrors the `CoreImage` framework case `SourceOver`.
    SourceOver,
/// Mirrors the `CoreImage` framework case `DestinationOver`.
    DestinationOver,
/// Mirrors the `CoreImage` framework case `SourceIn`.
    SourceIn,
/// Mirrors the `CoreImage` framework case `DestinationIn`.
    DestinationIn,
/// Mirrors the `CoreImage` framework case `SourceOut`.
    SourceOut,
/// Mirrors the `CoreImage` framework case `DestinationOut`.
    DestinationOut,
/// Mirrors the `CoreImage` framework case `SourceAtop`.
    SourceAtop,
/// Mirrors the `CoreImage` framework case `DestinationAtop`.
    DestinationAtop,
/// Mirrors the `CoreImage` framework case `ExclusiveOr`.
    ExclusiveOr,
/// Mirrors the `CoreImage` framework case `Multiply`.
    Multiply,
/// Mirrors the `CoreImage` framework case `Screen`.
    Screen,
/// Mirrors the `CoreImage` framework case `Overlay`.
    Overlay,
/// Mirrors the `CoreImage` framework case `Darken`.
    Darken,
/// Mirrors the `CoreImage` framework case `Lighten`.
    Lighten,
/// Mirrors the `CoreImage` framework case `ColorDodge`.
    ColorDodge,
/// Mirrors the `CoreImage` framework case `ColorBurn`.
    ColorBurn,
/// Mirrors the `CoreImage` framework case `HardLight`.
    HardLight,
/// Mirrors the `CoreImage` framework case `SoftLight`.
    SoftLight,
/// Mirrors the `CoreImage` framework case `Difference`.
    Difference,
/// Mirrors the `CoreImage` framework case `Exclusion`.
    Exclusion,
/// Mirrors the `CoreImage` framework case `Hue`.
    Hue,
/// Mirrors the `CoreImage` framework case `Saturation`.
    Saturation,
/// Mirrors the `CoreImage` framework case `Color`.
    Color,
/// Mirrors the `CoreImage` framework case `Luminosity`.
    Luminosity,
/// Mirrors the `CoreImage` framework case `Subtract`.
    Subtract,
/// Mirrors the `CoreImage` framework case `Divide`.
    Divide,
/// Mirrors the `CoreImage` framework case `LinearBurn`.
    LinearBurn,
/// Mirrors the `CoreImage` framework case `LinearDodge`.
    LinearDodge,
/// Mirrors the `CoreImage` framework case `VividLight`.
    VividLight,
/// Mirrors the `CoreImage` framework case `LinearLight`.
    LinearLight,
/// Mirrors the `CoreImage` framework case `PinLight`.
    PinLight,
/// Mirrors the `CoreImage` framework case `HardMix`.
    HardMix,
/// Mirrors the `CoreImage` framework case `DarkerColor`.
    DarkerColor,
/// Mirrors the `CoreImage` framework case `LighterColor`.
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

/// A generic Core Image kernel handle.
pub struct CIKernel {
    ptr: *mut c_void,
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

            #[allow(dead_code)]
            fn from_non_null(ptr: *mut c_void, kind: &str) -> Self {
                assert!(!ptr.is_null(), "{kind} returned nil");
                unsafe { Self::from_raw(ptr) }
            }

/// Mirrors the `CoreImage` framework constant `fn`.
            pub const fn as_ptr(&self) -> *mut c_void {
                self.ptr
            }

/// Calls the `CoreImage` framework counterpart for `name`.
            pub fn name(&self) -> String {
                unsafe { take_owned_string(ffi::ci_kernel_name(self.ptr)) }.unwrap_or_default()
            }
        }
    };
}

impl_kernel_handle!(CIKernel);
impl_kernel_handle!(CIColorKernel);
impl_kernel_handle!(CIWarpKernel);
impl_kernel_handle!(CIBlendKernel);

impl From<&CIColorKernel> for CIKernel {
    fn from(kernel: &CIColorKernel) -> Self {
        unsafe { Self::from_raw(ffi::ci_object_retain(kernel.as_ptr())) }
    }
}

impl From<&CIWarpKernel> for CIKernel {
    fn from(kernel: &CIWarpKernel) -> Self {
        unsafe { Self::from_raw(ffi::ci_object_retain(kernel.as_ptr())) }
    }
}

impl From<&CIBlendKernel> for CIKernel {
    fn from(kernel: &CIBlendKernel) -> Self {
        unsafe { Self::from_raw(ffi::ci_object_retain(kernel.as_ptr())) }
    }
}

impl CIColorKernel {
/// Calls the `CoreImage` framework counterpart for `as_kernel`.
    pub fn as_kernel(&self) -> CIKernel {
        CIKernel::from(self)
    }

/// Calls the `CoreImage` framework counterpart for `from_source`.
    pub fn from_source(source: &str) -> Result<Self, CIError> {
        let source = string_to_cstring(source, "kernel source")?;
        let mut kernel = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_color_kernel_new_source(source.as_ptr(), &raw mut kernel, &raw mut error) };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(kernel, "CIColorKernel(source:)"))
    }

/// Calls the `CoreImage` framework counterpart for `apply_image_scalar`.
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
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
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

/// Calls the `CoreImage` framework counterpart for `apply_image_color`.
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
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
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

/// Calls the `CoreImage` framework counterpart for `apply_image_vector`.
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
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
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
/// Calls the `CoreImage` framework counterpart for `as_kernel`.
    pub fn as_kernel(&self) -> CIKernel {
        CIKernel::from(self)
    }

/// Calls the `CoreImage` framework counterpart for `from_source`.
    pub fn from_source(source: &str) -> Result<Self, CIError> {
        let source = string_to_cstring(source, "kernel source")?;
        let mut kernel = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_warp_kernel_new_source(source.as_ptr(), &raw mut kernel, &raw mut error) };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(kernel, "CIWarpKernel(source:)"))
    }

/// Applies a warp using the full input extent as a conservative source ROI.
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
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
                false,
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

    /// Applies a known local warp whose source ROI equals the requested destination region.
    pub fn apply_image_scalar_with_destination_roi(
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
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
                true,
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

    /// Applies a warp with a caller-provided source region-of-interest callback.
    ///
    /// Callback-body panics use the full input extent. Captured values must implement non-panicking
    /// `Drop`; Rust cannot recover from multiple destructor panics in one closure aggregate.
        pub fn apply_image_scalar_with_roi(
        &self,
        image: &CIImage,
        value: f64,
        extent: CGRect,
        callback: impl Fn(usize, CGRect) -> CGRect + Send + Sync + 'static,
    ) -> Result<CIImage, CIError> {
        let callback = Box::new(WarpRegionOfInterestCallback {
            callback: Box::new(callback),
            fallback: image.extent(),
        });
        let handle = unsafe {
            ffi::ci_warp_kernel_apply_image_scalar_with_roi(
                self.ptr,
                image.as_ptr(),
                value,
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
                Box::into_raw(callback).cast(),
                Some(warp_region_of_interest_invoke),
                Some(warp_region_of_interest_release),
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
/// Calls the `CoreImage` framework counterpart for `as_kernel`.
    pub fn as_kernel(&self) -> CIKernel {
        CIKernel::from(self)
    }

/// Calls the `CoreImage` framework counterpart for `built_in`.
    pub fn built_in(kind: CIBlendKernelKind) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_blend_kernel_builtin(kind.code()) },
            "CIBlendKernel",
        )
    }

/// Calls the `CoreImage` framework counterpart for `apply`.
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
