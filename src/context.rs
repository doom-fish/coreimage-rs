use core::ffi::c_void;
use core::fmt;
use core::ptr;
use std::path::Path;

use apple_cf::cg::{CGImage, CGSize};
use apple_cf::cv::CVPixelBuffer;
use apple_cf::iosurface::IOSurface;

#[cfg(feature = "metal")]
use apple_metal::{CommandQueue, MetalDevice};

use crate::ffi;
use crate::util::{path_to_cstring, status_result, string_to_cstring};
use crate::{CIColorSpace, CIError, CIFormat, CIImage, CIRenderDestination, CIRenderTask};

/// Common Core Image context options that don't require extra framework types.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Default)]
pub struct CIContextOptions {
    pub cache_intermediates: bool,
    pub priority_request_low: bool,
    pub allow_low_power: bool,
    pub output_premultiplied: bool,
    pub high_quality_downsample: bool,
    pub output_color_space: Option<CIColorSpace>,
    pub working_color_space: Option<CIColorSpace>,
    pub working_format: Option<CIFormat>,
    pub memory_limit: Option<f64>,
    pub name: Option<String>,
}

/// A Core Image rendering context backed by the default renderer, CPU, or Metal.
pub struct CIContext {
    ptr: *mut c_void,
}

impl Drop for CIContext {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIContext {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIContext")
            .field("ptr", &self.ptr)
            .field("working_format", &self.working_format())
            .finish_non_exhaustive()
    }
}

impl CIContext {
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

    pub fn new_default() -> Self {
        Self::from_non_null(unsafe { ffi::ci_context_new_default() }, "CIContext()")
    }

    pub fn new_cpu() -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_context_new_cpu() },
            "CIContext(useSoftwareRenderer:)",
        )
    }

    pub fn with_options(options: &CIContextOptions) -> Result<Self, CIError> {
        let name = options
            .name
            .as_deref()
            .map(|name| string_to_cstring(name, "context name"))
            .transpose()?;
        Ok(Self::from_non_null(
            unsafe {
                ffi::ci_context_new_with_options(
                    options.cache_intermediates,
                    options.priority_request_low,
                    options.allow_low_power,
                    options.output_premultiplied,
                    options.high_quality_downsample,
                    options.output_color_space.is_some(),
                    options.output_color_space.map_or(0, CIColorSpace::code),
                    options.working_color_space.is_some(),
                    options.working_color_space.map_or(0, CIColorSpace::code),
                    options.working_format.is_some(),
                    options.working_format.map_or(0, CIFormat::raw_value),
                    options.memory_limit.is_some(),
                    options.memory_limit.unwrap_or_default(),
                    name.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                )
            },
            "CIContext(options:)",
        ))
    }

    #[cfg(feature = "metal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "metal")))]
    pub fn new_metal(device: &MetalDevice) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_context_new_metal_device(device.as_ptr()) },
            "CIContext(mtlDevice:)",
        )
    }

    #[cfg(feature = "metal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "metal")))]
    pub fn new_command_queue(queue: &CommandQueue) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_context_new_metal_command_queue(queue.as_ptr()) },
            "CIContext(mtlCommandQueue:)",
        )
    }

    pub fn working_format(&self) -> i32 {
        unsafe { ffi::ci_context_working_format(self.ptr) }
    }

    pub fn working_pixel_format(&self) -> Option<CIFormat> {
        CIFormat::from_raw(self.working_format())
    }

    pub fn render_to_cg_image(&self, image: &CIImage) -> Result<CGImage, CIError> {
        let rendered = unsafe { ffi::ci_context_create_cg_image(self.ptr, image.as_ptr()) };
        if rendered.is_null() {
            Err(CIError::NullResult(
                "CIContext.createCGImage returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CGImage::from_raw(rendered) })
        }
    }

    pub fn render_to_cv_pixel_buffer(
        &self,
        image: &CIImage,
        buffer: &CVPixelBuffer,
    ) -> Result<(), CIError> {
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_render_to_cv_pixel_buffer(
                self.ptr,
                image.as_ptr(),
                buffer.as_ptr(),
                &mut error,
            )
        };
        unsafe { status_result(status, error) }
    }

    pub fn render_to_iosurface(&self, image: &CIImage, surface: &IOSurface) -> Result<(), CIError> {
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_render_to_iosurface(
                self.ptr,
                image.as_ptr(),
                surface.as_ptr(),
                &mut error,
            )
        };
        unsafe { status_result(status, error) }
    }

    pub fn start_render_task(
        &self,
        image: &CIImage,
        destination: &CIRenderDestination,
    ) -> Result<CIRenderTask, CIError> {
        let mut task = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_start_render_task(
                self.ptr,
                image.as_ptr(),
                destination.as_ptr(),
                &mut task,
                &mut error,
            )
        };
        unsafe { status_result(status, error)? };
        if task.is_null() {
            Err(CIError::NullResult(
                "CIContext.startTask(toRender:to:) returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIRenderTask::from_raw(task) })
        }
    }

    pub fn prepare_render(
        &self,
        image: &CIImage,
        destination: &CIRenderDestination,
    ) -> Result<(), CIError> {
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_prepare_render(
                self.ptr,
                image.as_ptr(),
                destination.as_ptr(),
                &mut error,
            )
        };
        unsafe { status_result(status, error) }
    }

    pub fn start_clear_task(
        &self,
        destination: &CIRenderDestination,
    ) -> Result<CIRenderTask, CIError> {
        let mut task = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_start_clear_task(self.ptr, destination.as_ptr(), &mut task, &mut error)
        };
        unsafe { status_result(status, error)? };
        if task.is_null() {
            Err(CIError::NullResult(
                "CIContext.startTask(toClear:) returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIRenderTask::from_raw(task) })
        }
    }

    pub fn reclaim_resources(&self) {
        unsafe { ffi::ci_context_reclaim_resources(self.ptr) };
    }

    pub fn clear_caches(&self) {
        unsafe { ffi::ci_context_clear_caches(self.ptr) };
    }

    pub fn input_image_maximum_size(&self) -> CGSize {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_context_input_image_maximum_size(self.ptr, &mut width, &mut height) };
        CGSize::new(width, height)
    }

    pub fn output_image_maximum_size(&self) -> CGSize {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_context_output_image_maximum_size(self.ptr, &mut width, &mut height) };
        CGSize::new(width, height)
    }

    pub fn write_png(&self, image: &CIImage, path: impl AsRef<Path>) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_write_png(self.ptr, image.as_ptr(), path.as_ptr(), &mut error)
        };
        unsafe { status_result(status, error) }
    }

    pub fn write_jpeg(
        &self,
        image: &CIImage,
        path: impl AsRef<Path>,
        quality: f64,
    ) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_write_jpeg(self.ptr, image.as_ptr(), path.as_ptr(), quality, &mut error)
        };
        unsafe { status_result(status, error) }
    }

    pub fn write_heif(
        &self,
        image: &CIImage,
        path: impl AsRef<Path>,
        quality: f64,
    ) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_write_heif(self.ptr, image.as_ptr(), path.as_ptr(), quality, &mut error)
        };
        unsafe { status_result(status, error) }
    }

    pub fn write_heif10(
        &self,
        image: &CIImage,
        path: impl AsRef<Path>,
        quality: f64,
    ) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_write_heif10(
                self.ptr,
                image.as_ptr(),
                path.as_ptr(),
                quality,
                &mut error,
            )
        };
        unsafe { status_result(status, error) }
    }

    pub fn write_tiff(&self, image: &CIImage, path: impl AsRef<Path>) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_write_tiff(self.ptr, image.as_ptr(), path.as_ptr(), &mut error)
        };
        unsafe { status_result(status, error) }
    }

    pub fn write_openexr(&self, image: &CIImage, path: impl AsRef<Path>) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_write_openexr(self.ptr, image.as_ptr(), path.as_ptr(), &mut error)
        };
        unsafe { status_result(status, error) }
    }
}
