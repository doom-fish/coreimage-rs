use crate::ffi;
use crate::image::CIImage;
use crate::util::{status_result, take_owned_string};
use crate::{CIError, CIFormat};
use apple_cf::cg::CGRect;
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

fn read_region(read: unsafe extern "C" fn(*mut f64, *mut f64, *mut f64, *mut f64)) -> CGRect {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut width = 0.0;
    let mut height = 0.0;
    unsafe { read(&mut x, &mut y, &mut width, &mut height) };
    CGRect::new(x, y, width, height)
}

fn read_digest(read: unsafe extern "C" fn() -> *mut core::ffi::c_char) -> Option<String> {
    unsafe { take_owned_string(read()) }.filter(|value| !value.is_empty())
}

/// Helpers for running the built-in bridge image processor kernels.
pub struct CIImageProcessor;

impl CIImageProcessor {
/// Calls the `CoreImage` framework counterpart for `apply_passthrough`.
    pub fn apply_passthrough(image: &CIImage) -> Result<CIImage, CIError> {
        let mut output = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_image_processor_apply_passthrough(image.as_ptr(), &mut output, &mut error)
        };
        unsafe { status_result(status, error)? };
        Ok(unsafe { CIImage::from_raw(output) })
    }

/// Calls the `CoreImage` framework counterpart for `last_invocation`.
    pub fn last_invocation() -> CIImageProcessorInvocation {
        let input = if unsafe { ffi::ci_image_processor_last_invocation_has_input() } {
            Some(CIImageProcessorInput {
                region: read_region(ffi::ci_image_processor_last_input_region),
                bytes_per_row: unsafe { ffi::ci_image_processor_last_input_bytes_per_row() },
                format_raw: unsafe { ffi::ci_image_processor_last_input_format() },
                has_pixel_buffer: unsafe { ffi::ci_image_processor_last_input_has_pixel_buffer() },
                has_metal_texture: unsafe {
                    ffi::ci_image_processor_last_input_has_metal_texture()
                },
                digest: read_digest(ffi::ci_image_processor_last_input_digest),
                roi_tile_index: usize::try_from(unsafe {
                    ffi::ci_image_processor_last_input_roi_tile_index()
                })
                .ok(),
                roi_tile_count: usize::try_from(unsafe {
                    ffi::ci_image_processor_last_input_roi_tile_count()
                })
                .ok(),
            })
        } else {
            None
        };

        let output = CIImageProcessorOutput {
            region: read_region(ffi::ci_image_processor_last_output_region),
            bytes_per_row: unsafe { ffi::ci_image_processor_last_output_bytes_per_row() },
            format_raw: unsafe { ffi::ci_image_processor_last_output_format() },
            has_pixel_buffer: unsafe { ffi::ci_image_processor_last_output_has_pixel_buffer() },
            has_metal_texture: unsafe { ffi::ci_image_processor_last_output_has_metal_texture() },
            digest: read_digest(ffi::ci_image_processor_last_output_digest),
        };

        CIImageProcessorInvocation {
            input_count: unsafe { ffi::ci_image_processor_last_invocation_input_count() },
            input,
            output,
        }
    }

/// Calls the `CoreImage` framework counterpart for `last_invocation_json`.
    pub fn last_invocation_json() -> String {
        unsafe { take_owned_string(ffi::ci_image_processor_last_invocation_json()) }
            .unwrap_or_else(|| "{}".to_string())
    }
}
