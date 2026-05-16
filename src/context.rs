use core::ffi::c_void;
use core::fmt;
use core::ptr;
use std::path::Path;

use apple_cf::cg::CGImage;
use apple_cf::cv::CVPixelBuffer;
use apple_cf::iosurface::IOSurface;

use crate::ffi;
use crate::image::CIImage;
use crate::util::{path_to_cstring, status_result};
use crate::CIError;

#[cfg(feature = "metal")]
use apple_metal::{CommandQueue, MetalDevice};

/// A CoreImage rendering context backed by the default renderer, CPU, or Metal.
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
            .finish_non_exhaustive()
    }
}

impl CIContext {
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

    pub fn new_default() -> Self {
        Self::from_non_null(unsafe { ffi::ci_context_new_default() }, "CIContext()")
    }

    pub fn new_cpu() -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_context_new_cpu() },
            "CIContext(useSoftwareRenderer:)",
        )
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

    pub fn write_tiff(&self, image: &CIImage, path: impl AsRef<Path>) -> Result<(), CIError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_context_write_tiff(self.ptr, image.as_ptr(), path.as_ptr(), &mut error)
        };
        unsafe { status_result(status, error) }
    }
}
