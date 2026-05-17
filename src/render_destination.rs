use core::ffi::c_void;
use core::fmt;
use core::ptr;

use crate::ffi;
use crate::{CIColorSpace, CIError, CIFormat};

/// Alpha handling for a `CIRenderDestination`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CIRenderDestinationAlphaMode {
    #[default]
    None,
    Premultiplied,
    Unpremultiplied,
}

impl CIRenderDestinationAlphaMode {
    const fn code(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Premultiplied => 1,
            Self::Unpremultiplied => 2,
        }
    }

    const fn from_code(code: i32) -> Self {
        match code {
            1 => Self::Premultiplied,
            2 => Self::Unpremultiplied,
            _ => Self::None,
        }
    }
}

/// A render target for asynchronous Core Image rendering.
pub struct CIRenderDestination {
    ptr: *mut c_void,
    bytes: Vec<u8>,
    bytes_per_row: usize,
    format: CIFormat,
}

impl Drop for CIRenderDestination {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl fmt::Debug for CIRenderDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIRenderDestination")
            .field("ptr", &self.ptr)
            .field("width", &self.width())
            .field("height", &self.height())
            .field("alpha_mode", &self.alpha_mode())
            .finish_non_exhaustive()
    }
}

impl CIRenderDestination {
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn bitmap(
        width: usize,
        height: usize,
        format: CIFormat,
        color_space: Option<CIColorSpace>,
    ) -> Result<Self, CIError> {
        let bytes_per_row = width.checked_mul(format.bytes_per_pixel()).ok_or_else(|| {
            CIError::InvalidArgument("render destination row stride overflowed".to_string())
        })?;
        let len = bytes_per_row.checked_mul(height).ok_or_else(|| {
            CIError::InvalidArgument("render destination buffer size overflowed".to_string())
        })?;
        let mut bytes = vec![0_u8; len];
        let handle = unsafe {
            ffi::ci_render_destination_new_bitmap_data(
                bytes.as_mut_ptr().cast(),
                len,
                width,
                height,
                bytes_per_row,
                format.raw_value(),
                color_space.is_some(),
                color_space.map_or(0, CIColorSpace::code),
            )
        };
        if handle.is_null() {
            return Err(CIError::NullResult(
                "CIRenderDestination(bitmapData:width:height:bytesPerRow:format:) returned nil"
                    .to_string(),
            ));
        }
        Ok(Self {
            ptr: handle,
            bytes,
            bytes_per_row,
            format,
        })
    }

    pub fn bitmap_rgba8(width: usize, height: usize) -> Result<Self, CIError> {
        Self::bitmap(width, height, CIFormat::Rgba8, Some(CIColorSpace::Srgb))
    }

    pub fn width(&self) -> usize {
        unsafe { ffi::ci_render_destination_width(self.ptr) }
    }

    pub fn height(&self) -> usize {
        unsafe { ffi::ci_render_destination_height(self.ptr) }
    }

    pub fn alpha_mode(&self) -> CIRenderDestinationAlphaMode {
        CIRenderDestinationAlphaMode::from_code(unsafe {
            ffi::ci_render_destination_alpha_mode(self.ptr)
        })
    }

    pub fn set_alpha_mode(&mut self, alpha_mode: CIRenderDestinationAlphaMode) {
        unsafe { ffi::ci_render_destination_set_alpha_mode(self.ptr, alpha_mode.code()) };
    }

    pub fn is_flipped(&self) -> bool {
        unsafe { ffi::ci_render_destination_is_flipped(self.ptr) }
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        unsafe { ffi::ci_render_destination_set_flipped(self.ptr, flipped) };
    }

    pub fn is_dithered(&self) -> bool {
        unsafe { ffi::ci_render_destination_is_dithered(self.ptr) }
    }

    pub fn set_dithered(&mut self, dithered: bool) {
        unsafe { ffi::ci_render_destination_set_dithered(self.ptr, dithered) };
    }

    pub fn is_clamped(&self) -> bool {
        unsafe { ffi::ci_render_destination_is_clamped(self.ptr) }
    }

    pub fn set_clamped(&mut self, clamped: bool) {
        unsafe { ffi::ci_render_destination_set_clamped(self.ptr, clamped) };
    }

    pub fn bitmap_data(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn bitmap_data_mut(&mut self) -> &mut [u8] {
        self.bytes.as_mut_slice()
    }

    pub const fn bytes_per_row(&self) -> usize {
        self.bytes_per_row
    }

    pub const fn format(&self) -> CIFormat {
        self.format
    }
}

/// Timing information for a completed Core Image render task.
pub struct CIRenderInfo {
    ptr: *mut c_void,
}

impl Drop for CIRenderInfo {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl fmt::Debug for CIRenderInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIRenderInfo")
            .field("kernel_execution_time", &self.kernel_execution_time())
            .field("kernel_compile_time", &self.kernel_compile_time())
            .field("pass_count", &self.pass_count())
            .field("pixels_processed", &self.pixels_processed())
            .finish_non_exhaustive()
    }
}

impl CIRenderInfo {
    pub(crate) const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    pub fn kernel_execution_time(&self) -> f64 {
        unsafe { ffi::ci_render_info_kernel_execution_time(self.ptr) }
    }

    pub fn kernel_compile_time(&self) -> f64 {
        unsafe { ffi::ci_render_info_kernel_compile_time(self.ptr) }
    }

    pub fn pass_count(&self) -> usize {
        unsafe { ffi::ci_render_info_pass_count(self.ptr) }
    }

    pub fn pixels_processed(&self) -> usize {
        unsafe { ffi::ci_render_info_pixels_processed(self.ptr) }
    }
}

/// A handle for an in-flight Core Image render.
pub struct CIRenderTask {
    ptr: *mut c_void,
}

impl Drop for CIRenderTask {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl fmt::Debug for CIRenderTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIRenderTask")
            .field("ptr", &self.ptr)
            .finish_non_exhaustive()
    }
}

impl CIRenderTask {
    pub(crate) const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    pub fn wait_until_completed(self) -> Result<CIRenderInfo, CIError> {
        let mut info = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_render_task_wait_until_completed(self.ptr, &mut info, &mut error) };
        unsafe { crate::util::status_result(status, error)? };
        if info.is_null() {
            Err(CIError::NullResult(
                "CIRenderTask.waitUntilCompleted() returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIRenderInfo::from_raw(info) })
        }
    }
}
