use core::ffi::{c_char, c_void};
use core::fmt;
use core::mem::ManuallyDrop;
use core::ptr;

use apple_cf::cg::CGRect;
use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::panic_safe::catch_user_panic_result;

use crate::color::CIColor;
use crate::ffi;
use crate::image::CIImage;
use crate::util::{
    split_lines, status_result, string_to_cstring, take_array_objects, take_owned_string,
};
use crate::vector::CIVector;
use crate::{CIError, CIFormat};

pub(crate) type RegionOfInterestFn = dyn Fn(usize, CGRect) -> CGRect + Send + Sync;

pub(crate) trait RegionOfInterestSource: Send + Sync + 'static {
    const SITE: &'static str;

    fn region(&self, input_index: usize, destination: CGRect) -> CGRect;
}

pub(crate) fn guarded_region(
    site: &str,
    callback: &RegionOfInterestFn,
    fallbacks: &[CGRect],
    input_index: usize,
    destination: CGRect,
) -> CGRect {
    catch_user_panic_result(site, || callback(input_index, destination))
        .unwrap_or_else(|| fallbacks.get(input_index).copied().unwrap_or(destination))
}

pub(crate) fn foreign_owned_context<T: Send + Sync + 'static>(value: T) -> *mut c_void {
    ManuallyDrop::new(CallbackContext::new(value)).as_ptr()
}

pub(crate) unsafe extern "C" fn region_of_interest_invoke<T: RegionOfInterestSource>(
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
    if out_x.is_null() || out_y.is_null() || out_width.is_null() || out_height.is_null() {
        return;
    }
    let destination = CGRect::new(
        destination_x,
        destination_y,
        destination_width,
        destination_height,
    );
    let input_index = usize::try_from(input_index).unwrap_or_default();
    let region = unsafe {
        CallbackContext::<T>::with(context, T::SITE, |source| {
            source.region(input_index, destination)
        })
    }
    .unwrap_or(destination);
    unsafe {
        *out_x = region.origin.x;
        *out_y = region.origin.y;
        *out_width = region.size.width;
        *out_height = region.size.height;
    }
}

struct KernelRegionOfInterest {
    callback: Box<RegionOfInterestFn>,
    fallbacks: Vec<CGRect>,
}

impl RegionOfInterestSource for KernelRegionOfInterest {
    const SITE: &'static str = "CIKernel region-of-interest callback";

    fn region(&self, input_index: usize, destination: CGRect) -> CGRect {
        guarded_region(
            Self::SITE,
            self.callback.as_ref(),
            &self.fallbacks,
            input_index,
            destination,
        )
    }
}

impl KernelRegionOfInterest {
    fn into_foreign(
        callback: impl Fn(usize, CGRect) -> CGRect + Send + Sync + 'static,
        fallbacks: Vec<CGRect>,
    ) -> *mut c_void {
        foreign_owned_context(Self {
            callback: Box::new(callback),
            fallbacks,
        })
    }
}

const REGION_OF_INTEREST_INVOKE: ffi::RustRegionOfInterestCallback =
    Some(region_of_interest_invoke::<KernelRegionOfInterest>);
const REGION_OF_INTEREST_RELEASE: ffi::RustContextReleaseCallback =
    Some(CallbackContext::<KernelRegionOfInterest>::RELEASE);

#[derive(Clone, Copy, Debug)]
pub enum CIKernelArgument<'a> {
    Image(&'a CIImage),
    Scalar(f64),
    Vector(&'a CIVector),
    Color(&'a CIColor),
}

impl<'a> From<&'a CIImage> for CIKernelArgument<'a> {
    fn from(image: &'a CIImage) -> Self {
        Self::Image(image)
    }
}

impl From<f64> for CIKernelArgument<'_> {
    fn from(value: f64) -> Self {
        Self::Scalar(value)
    }
}

impl<'a> From<&'a CIVector> for CIKernelArgument<'a> {
    fn from(vector: &'a CIVector) -> Self {
        Self::Vector(vector)
    }
}

impl<'a> From<&'a CIColor> for CIKernelArgument<'a> {
    fn from(color: &'a CIColor) -> Self {
        Self::Color(color)
    }
}

struct KernelArguments {
    kinds: Vec<i32>,
    scalars: Vec<f64>,
    objects: Vec<*mut c_void>,
}

impl KernelArguments {
    fn new(arguments: &[CIKernelArgument<'_>]) -> Self {
        let mut marshalled = Self {
            kinds: Vec::with_capacity(arguments.len()),
            scalars: Vec::with_capacity(arguments.len()),
            objects: Vec::with_capacity(arguments.len()),
        };
        for argument in arguments {
            let (kind, scalar, object) = match *argument {
                CIKernelArgument::Image(image) => (0, 0.0, image.as_ptr()),
                CIKernelArgument::Scalar(value) => (1, value, ptr::null_mut()),
                CIKernelArgument::Vector(vector) => (2, 0.0, vector.as_ptr()),
                CIKernelArgument::Color(color) => (3, 0.0, color.as_ptr()),
            };
            marshalled.kinds.push(kind);
            marshalled.scalars.push(scalar);
            marshalled.objects.push(object);
        }
        marshalled
    }
}

fn image_extents(arguments: &[CIKernelArgument<'_>]) -> Vec<CGRect> {
    arguments
        .iter()
        .filter_map(|argument| match argument {
            CIKernelArgument::Image(image) => Some(image.extent()),
            _ => None,
        })
        .collect()
}

unsafe fn kernel_output(
    status: i32,
    image: *mut c_void,
    error: *mut c_char,
    method: &str,
) -> Result<CIImage, CIError> {
    unsafe { status_result(status, error)? };
    if image.is_null() {
        Err(CIError::NullResult(format!("{method} returned nil")))
    } else {
        Ok(unsafe { CIImage::from_raw(image) })
    }
}

fn metal_library_kernel(
    kind: i32,
    function_name: &str,
    data: &[u8],
    output_format: Option<CIFormat>,
) -> Result<*mut c_void, CIError> {
    if data.is_empty() {
        return Err(CIError::InvalidArgument(
            "Metal library data must not be empty".to_string(),
        ));
    }
    let function_name = string_to_cstring(function_name, "kernel function name")?;
    let mut kernel = ptr::null_mut();
    let mut error = ptr::null_mut();
    let status = unsafe {
        ffi::ci_kernel_new_metal_library(
            kind,
            function_name.as_ptr(),
            data.as_ptr(),
            data.len(),
            output_format.is_some(),
            output_format.map_or(0, CIFormat::raw_value),
            &raw mut kernel,
            &raw mut error,
        )
    };
    unsafe { status_result(status, error)? };
    if kernel.is_null() {
        Err(CIError::NullResult(
            "CIKernel(functionName:fromMetalLibraryData:) returned nil".to_string(),
        ))
    } else {
        Ok(kernel)
    }
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
    ($name:ident, $kind:expr) => {
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

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

            pub fn from_metal_library_data(
                function_name: &str,
                data: &[u8],
                output_format: Option<CIFormat>,
            ) -> Result<Self, CIError> {
                metal_library_kernel($kind, function_name, data, output_format)
                    .map(|kernel| unsafe { Self::from_raw(kernel) })
            }
        }
    };
}

impl_kernel_handle!(CIKernel, 0);
impl_kernel_handle!(CIColorKernel, 1);
impl_kernel_handle!(CIWarpKernel, 2);
impl_kernel_handle!(CIBlendKernel, 3);

impl CIKernel {
    pub fn kernel_names_from_metal_library_data(data: &[u8]) -> Vec<String> {
        if data.is_empty() {
            return Vec::new();
        }
        unsafe { take_owned_string(ffi::ci_kernel_names_metal_library(data.as_ptr(), data.len())) }
            .map_or_else(Vec::new, |text| split_lines(&text))
    }

    pub fn kernels_from_metal_source(source: &str) -> Result<Vec<Self>, CIError> {
        let source = string_to_cstring(source, "Metal kernel source")?;
        let mut kernels = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_kernels_new_metal_source(source.as_ptr(), &raw mut kernels, &raw mut error)
        };
        unsafe { status_result(status, error)? };
        Ok(unsafe { take_array_objects(kernels) }
            .into_iter()
            .map(|kernel| unsafe { Self::from_raw(kernel) })
            .collect())
    }

    fn is_kind(&self, kind: i32) -> bool {
        unsafe { ffi::ci_kernel_is_kind(self.ptr, kind) }
    }

    pub fn as_color_kernel(&self) -> Option<CIColorKernel> {
        self.is_kind(1)
            .then(|| unsafe { CIColorKernel::from_raw(ffi::ci_object_retain(self.ptr)) })
    }

    pub fn as_warp_kernel(&self) -> Option<CIWarpKernel> {
        self.is_kind(2)
            .then(|| unsafe { CIWarpKernel::from_raw(ffi::ci_object_retain(self.ptr)) })
    }

    pub fn as_blend_kernel(&self) -> Option<CIBlendKernel> {
        self.is_kind(3)
            .then(|| unsafe { CIBlendKernel::from_raw(ffi::ci_object_retain(self.ptr)) })
    }

    pub fn apply(
        &self,
        extent: CGRect,
        arguments: &[CIKernelArgument<'_>],
        region_of_interest: impl Fn(usize, CGRect) -> CGRect + Send + Sync + 'static,
    ) -> Result<CIImage, CIError> {
        let marshalled = KernelArguments::new(arguments);
        let context =
            KernelRegionOfInterest::into_foreign(region_of_interest, image_extents(arguments));
        let mut image = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_kernel_apply_arguments(
                self.ptr,
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
                marshalled.kinds.as_ptr(),
                marshalled.scalars.as_ptr(),
                marshalled.objects.as_ptr(),
                marshalled.kinds.len(),
                context,
                REGION_OF_INTEREST_INVOKE,
                REGION_OF_INTEREST_RELEASE,
                &raw mut image,
                &raw mut error,
            )
        };
        unsafe { kernel_output(status, image, error, "CIKernel.apply") }
    }
}

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
    #[deprecated(
        since = "0.5.0",
        note = "Core Image Kernel Language is deprecated since macOS 10.14; load a Metal kernel with from_metal_library_data"
    )]
    pub fn from_source(source: &str) -> Result<Self, CIError> {
        let source = string_to_cstring(source, "kernel source")?;
        let mut kernel = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_color_kernel_new_source(source.as_ptr(), &raw mut kernel, &raw mut error) };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(kernel, "CIColorKernel(source:)"))
    }

    pub fn apply(
        &self,
        extent: CGRect,
        arguments: &[CIKernelArgument<'_>],
    ) -> Result<CIImage, CIError> {
        let marshalled = KernelArguments::new(arguments);
        let mut image = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_color_kernel_apply_arguments(
                self.ptr,
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
                marshalled.kinds.as_ptr(),
                marshalled.scalars.as_ptr(),
                marshalled.objects.as_ptr(),
                marshalled.kinds.len(),
                &raw mut image,
                &raw mut error,
            )
        };
        unsafe { kernel_output(status, image, error, "CIColorKernel.apply") }
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
    #[deprecated(
        since = "0.5.0",
        note = "Core Image Kernel Language is deprecated since macOS 10.14; load a Metal kernel with from_metal_library_data"
    )]
    pub fn from_source(source: &str) -> Result<Self, CIError> {
        let source = string_to_cstring(source, "kernel source")?;
        let mut kernel = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_warp_kernel_new_source(source.as_ptr(), &raw mut kernel, &raw mut error) };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(kernel, "CIWarpKernel(source:)"))
    }

    pub fn apply(
        &self,
        extent: CGRect,
        image: &CIImage,
        arguments: &[CIKernelArgument<'_>],
        region_of_interest: impl Fn(usize, CGRect) -> CGRect + Send + Sync + 'static,
    ) -> Result<CIImage, CIError> {
        let marshalled = KernelArguments::new(arguments);
        let context = KernelRegionOfInterest::into_foreign(region_of_interest, vec![image.extent()]);
        let mut output = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_warp_kernel_apply_arguments(
                self.ptr,
                image.as_ptr(),
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
                marshalled.kinds.as_ptr(),
                marshalled.scalars.as_ptr(),
                marshalled.objects.as_ptr(),
                marshalled.kinds.len(),
                context,
                REGION_OF_INTEREST_INVOKE,
                REGION_OF_INTEREST_RELEASE,
                &raw mut output,
                &raw mut error,
            )
        };
        unsafe { kernel_output(status, output, error, "CIWarpKernel.apply") }
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
        let context = KernelRegionOfInterest::into_foreign(callback, vec![image.extent()]);
        let handle = unsafe {
            ffi::ci_warp_kernel_apply_image_scalar_with_roi(
                self.ptr,
                image.as_ptr(),
                value,
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
                context,
                REGION_OF_INTEREST_INVOKE,
                REGION_OF_INTEREST_RELEASE,
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
