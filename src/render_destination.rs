use core::cell::UnsafeCell;
use core::ffi::c_void;
use core::fmt;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};
use std::mem;
use std::sync::Arc;

use crate::ffi;
use crate::{CIColorSpace, CIError, CIFormat};

/// Alpha handling for a `CIRenderDestination`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CIRenderDestinationAlphaMode {
/// Mirrors the `CoreImage` framework case `None`.
    #[default]
    None,
/// Mirrors the `CoreImage` framework case `Premultiplied`.
    Premultiplied,
/// Mirrors the `CoreImage` framework case `Unpremultiplied`.
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
///
/// Bitmap storage is shared with each task and remains inaccessible until native work completes.
pub struct CIRenderDestination {
    ptr: *mut c_void,
    backing: Arc<RenderDestinationBacking>,
    bytes_per_row: usize,
    format: CIFormat,
}

struct RenderDestinationBacking {
    bytes: UnsafeCell<Box<[u8]>>,
    in_flight: AtomicBool,
}

// Native writes are bracketed by the in-flight state and task completion barrier; Rust only
// exposes shared or exclusive byte references while that state is idle.
unsafe impl Send for RenderDestinationBacking {}
unsafe impl Sync for RenderDestinationBacking {}

pub(crate) struct CIRenderDestinationReservation {
    destination: *mut c_void,
    backing: Option<Arc<RenderDestinationBacking>>,
}

impl Drop for CIRenderDestinationReservation {
    fn drop(&mut self) {
        if !self.destination.is_null() {
            unsafe { ffi::ci_object_release(self.destination) };
            self.destination = ptr::null_mut();
        }
        if let Some(backing) = self.backing.take() {
            backing.in_flight.store(false, Ordering::Release);
        }
    }
}

impl CIRenderDestinationReservation {
    pub(crate) fn into_task(mut self, task: *mut c_void) -> CIRenderTask {
        CIRenderTask {
            ptr: task,
            destination: mem::replace(&mut self.destination, ptr::null_mut()),
            backing: self.backing.take(),
        }
    }
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
/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

/// Calls the `CoreImage` framework counterpart for `bitmap`.
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
        let backing = Arc::new(RenderDestinationBacking {
            bytes: UnsafeCell::new(vec![0_u8; len].into_boxed_slice()),
            in_flight: AtomicBool::new(false),
        });
        let data = unsafe { (&mut *backing.bytes.get()).as_mut_ptr() };
        let handle = unsafe {
            ffi::ci_render_destination_new_bitmap_data(
                data.cast(),
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
            backing,
            bytes_per_row,
            format,
        })
    }

/// Calls the `CoreImage` framework counterpart for `bitmap_rgba8`.
    pub fn bitmap_rgba8(width: usize, height: usize) -> Result<Self, CIError> {
        Self::bitmap(width, height, CIFormat::Rgba8, Some(CIColorSpace::Srgb))
    }

/// Calls the `CoreImage` framework counterpart for `width`.
    pub fn width(&self) -> usize {
        unsafe { ffi::ci_render_destination_width(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `height`.
    pub fn height(&self) -> usize {
        unsafe { ffi::ci_render_destination_height(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `alpha_mode`.
    pub fn alpha_mode(&self) -> CIRenderDestinationAlphaMode {
        CIRenderDestinationAlphaMode::from_code(unsafe {
            ffi::ci_render_destination_alpha_mode(self.ptr)
        })
    }

/// Calls the `CoreImage` framework counterpart for `set_alpha_mode`.
    pub fn set_alpha_mode(
        &mut self,
        alpha_mode: CIRenderDestinationAlphaMode,
    ) -> Result<(), CIError> {
        self.ensure_idle()?;
        unsafe { ffi::ci_render_destination_set_alpha_mode(self.ptr, alpha_mode.code()) };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `is_flipped`.
    pub fn is_flipped(&self) -> bool {
        unsafe { ffi::ci_render_destination_is_flipped(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `set_flipped`.
    pub fn set_flipped(&mut self, flipped: bool) -> Result<(), CIError> {
        self.ensure_idle()?;
        unsafe { ffi::ci_render_destination_set_flipped(self.ptr, flipped) };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `is_dithered`.
    pub fn is_dithered(&self) -> bool {
        unsafe { ffi::ci_render_destination_is_dithered(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `set_dithered`.
    pub fn set_dithered(&mut self, dithered: bool) -> Result<(), CIError> {
        self.ensure_idle()?;
        unsafe { ffi::ci_render_destination_set_dithered(self.ptr, dithered) };
        Ok(())
    }

/// Calls the `CoreImage` framework counterpart for `is_clamped`.
    pub fn is_clamped(&self) -> bool {
        unsafe { ffi::ci_render_destination_is_clamped(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `set_clamped`.
    pub fn set_clamped(&mut self, clamped: bool) -> Result<(), CIError> {
        self.ensure_idle()?;
        unsafe { ffi::ci_render_destination_set_clamped(self.ptr, clamped) };
        Ok(())
    }

/// Borrows the bitmap bytes after any render task has completed.
    pub fn bitmap_data(&self) -> Result<&[u8], CIError> {
        self.ensure_idle()?;
        Ok(unsafe { (&*self.backing.bytes.get()).as_ref() })
    }

/// Mutably borrows the bitmap bytes after any render task has completed.
    pub fn bitmap_data_mut(&mut self) -> Result<&mut [u8], CIError> {
        self.ensure_idle()?;
        Ok(unsafe { (&mut *self.backing.bytes.get()).as_mut() })
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn bytes_per_row(&self) -> usize {
        self.bytes_per_row
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn format(&self) -> CIFormat {
        self.format
    }

    pub(crate) fn ensure_idle(&self) -> Result<(), CIError> {
        if self.backing.in_flight.load(Ordering::Acquire) {
            Err(CIError::InvalidArgument(
                "render destination has an in-flight task".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn reserve_task(&mut self) -> Result<CIRenderDestinationReservation, CIError> {
        self.backing
            .in_flight
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| {
                CIError::InvalidArgument(
                    "render destination already has an in-flight task".to_string(),
                )
            })?;
        let destination = unsafe { ffi::ci_object_retain(self.ptr) };
        if destination.is_null() {
            self.backing.in_flight.store(false, Ordering::Release);
            return Err(CIError::NullResult(
                "retaining CIRenderDestination returned nil".to_string(),
            ));
        }
        Ok(CIRenderDestinationReservation {
            destination,
            backing: Some(Arc::clone(&self.backing)),
        })
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

/// Calls the `CoreImage` framework counterpart for `kernel_execution_time`.
    pub fn kernel_execution_time(&self) -> f64 {
        unsafe { ffi::ci_render_info_kernel_execution_time(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `kernel_compile_time`.
    pub fn kernel_compile_time(&self) -> f64 {
        unsafe { ffi::ci_render_info_kernel_compile_time(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `pass_count`.
    pub fn pass_count(&self) -> usize {
        unsafe { ffi::ci_render_info_pass_count(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `pixels_processed`.
    pub fn pixels_processed(&self) -> usize {
        unsafe { ffi::ci_render_info_pixels_processed(self.ptr) }
    }
}

/// A handle for an in-flight Core Image render.
///
/// Dropping the task waits for completion. Forgetting it leaks the retained destination and bitmap
/// storage, keeping safe byte access closed rather than exposing memory still owned by native work.
pub struct CIRenderTask {
    ptr: *mut c_void,
    destination: *mut c_void,
    backing: Option<Arc<RenderDestinationBacking>>,
}

impl Drop for CIRenderTask {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            self.release_after_completion();
            return;
        }
        let mut info = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_render_task_wait_until_completed(self.ptr, &raw mut info, &raw mut error) };
        let _ = self.complete_wait(status, info, error);
    }
}

impl fmt::Debug for CIRenderTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIRenderTask")
            .field("ptr", &self.ptr)
            .field("destination", &self.destination)
            .finish_non_exhaustive()
    }
}

impl CIRenderTask {
    fn release_after_completion(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
        if !self.destination.is_null() {
            unsafe { ffi::ci_object_release(self.destination) };
            self.destination = ptr::null_mut();
        }
        if let Some(backing) = self.backing.take() {
            backing.in_flight.store(false, Ordering::Release);
        }
    }

    fn complete_wait(
        &mut self,
        status: i32,
        info: *mut c_void,
        error: *mut core::ffi::c_char,
    ) -> Result<CIRenderInfo, CIError> {
        self.release_after_completion();
        if let Err(error) = unsafe { crate::util::status_result(status, error) } {
            if !info.is_null() {
                unsafe { ffi::ci_object_release(info) };
            }
            return Err(error);
        }
        if info.is_null() {
            Err(CIError::NullResult(
                "CIRenderTask.waitUntilCompleted() returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIRenderInfo::from_raw(info) })
        }
    }

/// Waits for completion, releases the destination reservation, and returns owned render info.
    pub fn wait_until_completed(mut self) -> Result<CIRenderInfo, CIError> {
        let mut info = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status =
            unsafe { ffi::ci_render_task_wait_until_completed(self.ptr, &raw mut info, &raw mut error) };
        self.complete_wait(status, info, error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_errors_release_the_destination_reservation() {
        let mut destination =
            CIRenderDestination::bitmap_rgba8(1, 1).expect("bitmap destination should be valid");
        let reservation = destination
            .reserve_task()
            .expect("destination should be idle");
        let mut task = reservation.into_task(ptr::null_mut());

        let error = task
            .complete_wait(
                ffi::status::FRAMEWORK,
                ptr::null_mut(),
                ptr::null_mut(),
            )
            .expect_err("framework status should fail");

        assert!(matches!(error, CIError::Framework(_)));
        assert!(destination.bitmap_data().is_ok());
    }

    #[test]
    fn missing_render_info_releases_the_destination_reservation() {
        let mut destination =
            CIRenderDestination::bitmap_rgba8(1, 1).expect("bitmap destination should be valid");
        let reservation = destination
            .reserve_task()
            .expect("destination should be idle");
        let mut task = reservation.into_task(ptr::null_mut());

        let error = task
            .complete_wait(ffi::status::OK, ptr::null_mut(), ptr::null_mut())
            .expect_err("missing render info should fail");

        assert!(matches!(error, CIError::NullResult(_)));
        assert!(destination.bitmap_data().is_ok());
    }
}
