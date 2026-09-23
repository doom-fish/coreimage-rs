use crate::ffi;
use crate::image::CIImage;
use crate::util::{status_result, take_owned_string};
use crate::{CIError, CIFormat};
use apple_cf::cg::CGRect;
use core::ffi::c_void;
use core::ptr;

/// Mirrors the `CoreImage` framework counterpart for `CIImageProcessorInput`.
#[derive(Clone, Debug)]
pub struct CIImageProcessorInput {
    region: CGRect,
    bytes_per_row: usize,
    format_raw: i32,
    has_pixel_buffer: bool,
    has_metal_texture: bool,
    digest: Option<String>,
    roi_tile_index: Option<usize>,
    roi_tile_count: Option<usize>,
}

impl CIImageProcessorInput {
/// Calls the `CoreImage` framework counterpart for `region`.
    pub fn region(&self) -> CGRect {
        self.region
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn bytes_per_row(&self) -> usize {
        self.bytes_per_row
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn format_raw(&self) -> i32 {
        self.format_raw
    }

/// Calls the `CoreImage` framework counterpart for `format`.
    pub fn format(&self) -> Option<CIFormat> {
        CIFormat::from_raw(self.format_raw)
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn has_pixel_buffer(&self) -> bool {
        self.has_pixel_buffer
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn has_metal_texture(&self) -> bool {
        self.has_metal_texture
    }

/// Calls the `CoreImage` framework counterpart for `digest`.
    pub fn digest(&self) -> Option<&str> {
        self.digest.as_deref()
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn roi_tile_index(&self) -> Option<usize> {
        self.roi_tile_index
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn roi_tile_count(&self) -> Option<usize> {
        self.roi_tile_count
    }
}

/// Mirrors the `CoreImage` framework counterpart for `CIImageProcessorOutput`.
#[derive(Clone, Debug)]
pub struct CIImageProcessorOutput {
    region: CGRect,
    bytes_per_row: usize,
    format_raw: i32,
    has_pixel_buffer: bool,
    has_metal_texture: bool,
    digest: Option<String>,
}

impl CIImageProcessorOutput {
/// Calls the `CoreImage` framework counterpart for `region`.
    pub fn region(&self) -> CGRect {
        self.region
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn bytes_per_row(&self) -> usize {
        self.bytes_per_row
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn format_raw(&self) -> i32 {
        self.format_raw
    }

/// Calls the `CoreImage` framework counterpart for `format`.
    pub fn format(&self) -> Option<CIFormat> {
        CIFormat::from_raw(self.format_raw)
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn has_pixel_buffer(&self) -> bool {
        self.has_pixel_buffer
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn has_metal_texture(&self) -> bool {
        self.has_metal_texture
    }

/// Calls the `CoreImage` framework counterpart for `digest`.
    pub fn digest(&self) -> Option<&str> {
        self.digest.as_deref()
    }
}

/// Mirrors the `CoreImage` framework counterpart for `CIImageProcessorInvocation`.
#[derive(Clone, Debug)]
pub struct CIImageProcessorInvocation {
    input_count: usize,
    input: Option<CIImageProcessorInput>,
    output: CIImageProcessorOutput,
}

impl CIImageProcessorInvocation {
/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn input_count(&self) -> usize {
        self.input_count
    }

/// Calls the `CoreImage` framework counterpart for `input`.
    pub fn input(&self) -> Option<&CIImageProcessorInput> {
        self.input.as_ref()
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn output(&self) -> &CIImageProcessorOutput {
        &self.output
    }
}

struct InvocationSnapshot {
    ptr: *mut c_void,
}

impl InvocationSnapshot {
    fn acquire() -> Self {
        let ptr = unsafe { ffi::ci_image_processor_invocation_snapshot_new() };
        assert!(!ptr.is_null(), "CIImageProcessor invocation snapshot returned nil");
        Self { ptr }
    }
}

impl Drop for InvocationSnapshot {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

fn read_region(
    snapshot: *mut c_void,
    read: unsafe extern "C" fn(*mut c_void, *mut f64, *mut f64, *mut f64, *mut f64),
) -> CGRect {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut width = 0.0;
    let mut height = 0.0;
    unsafe { read(snapshot, &raw mut x, &raw mut y, &raw mut width, &raw mut height) };
    CGRect::new(x, y, width, height)
}

fn read_digest(
    snapshot: *mut c_void,
    read: unsafe extern "C" fn(*mut c_void) -> *mut core::ffi::c_char,
) -> Option<String> {
    unsafe { take_owned_string(read(snapshot)) }.filter(|value| !value.is_empty())
}

/// Helpers for running the built-in bridge image processor kernels.
pub struct CIImageProcessor;

impl CIImageProcessor {
/// Calls the `CoreImage` framework counterpart for `apply_passthrough`.
    pub fn apply_passthrough(image: &CIImage) -> Result<CIImage, CIError> {
        let mut output = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_image_processor_apply_passthrough(image.as_ptr(), &raw mut output, &raw mut error)
        };
        unsafe { status_result(status, error)? };
        Ok(unsafe { CIImage::from_raw(output) })
    }

/// Returns one owned invocation snapshot captured atomically under the bridge lock.
    pub fn last_invocation() -> CIImageProcessorInvocation {
        let snapshot = InvocationSnapshot::acquire();
        let input = if unsafe {
            ffi::ci_image_processor_invocation_snapshot_has_input(snapshot.ptr)
        } {
            Some(CIImageProcessorInput {
                region: read_region(
                    snapshot.ptr,
                    ffi::ci_image_processor_invocation_snapshot_input_region,
                ),
                bytes_per_row: unsafe {
                    ffi::ci_image_processor_invocation_snapshot_input_bytes_per_row(snapshot.ptr)
                },
                format_raw: unsafe {
                    ffi::ci_image_processor_invocation_snapshot_input_format(snapshot.ptr)
                },
                has_pixel_buffer: unsafe {
                    ffi::ci_image_processor_invocation_snapshot_input_has_pixel_buffer(snapshot.ptr)
                },
                has_metal_texture: unsafe {
                    ffi::ci_image_processor_invocation_snapshot_input_has_metal_texture(snapshot.ptr)
                },
                digest: read_digest(
                    snapshot.ptr,
                    ffi::ci_image_processor_invocation_snapshot_input_digest,
                ),
                roi_tile_index: usize::try_from(unsafe {
                    ffi::ci_image_processor_invocation_snapshot_input_roi_tile_index(snapshot.ptr)
                })
                .ok(),
                roi_tile_count: usize::try_from(unsafe {
                    ffi::ci_image_processor_invocation_snapshot_input_roi_tile_count(snapshot.ptr)
                })
                .ok(),
            })
        } else {
            None
        };

        let output = CIImageProcessorOutput {
            region: read_region(
                snapshot.ptr,
                ffi::ci_image_processor_invocation_snapshot_output_region,
            ),
            bytes_per_row: unsafe {
                ffi::ci_image_processor_invocation_snapshot_output_bytes_per_row(snapshot.ptr)
            },
            format_raw: unsafe {
                ffi::ci_image_processor_invocation_snapshot_output_format(snapshot.ptr)
            },
            has_pixel_buffer: unsafe {
                ffi::ci_image_processor_invocation_snapshot_output_has_pixel_buffer(snapshot.ptr)
            },
            has_metal_texture: unsafe {
                ffi::ci_image_processor_invocation_snapshot_output_has_metal_texture(snapshot.ptr)
            },
            digest: read_digest(
                snapshot.ptr,
                ffi::ci_image_processor_invocation_snapshot_output_digest,
            ),
        };

        CIImageProcessorInvocation {
            input_count: unsafe {
                ffi::ci_image_processor_invocation_snapshot_input_count(snapshot.ptr)
            },
            input,
            output,
        }
    }

/// Returns JSON for one owned invocation snapshot captured atomically under the bridge lock.
    pub fn last_invocation_json() -> String {
        let snapshot = InvocationSnapshot::acquire();
        unsafe {
            take_owned_string(ffi::ci_image_processor_invocation_snapshot_json(
                snapshot.ptr,
            ))
        }
        .unwrap_or_else(|| "{}".to_string())
    }
}
