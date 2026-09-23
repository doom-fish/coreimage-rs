use crate::ffi;
use crate::image::CIImage;
use crate::kernel::{
    foreign_owned_context, guarded_region, region_of_interest_invoke, RegionOfInterestFn,
    RegionOfInterestSource,
};
use crate::util::{status_result, take_owned_string};
use crate::{CIError, CIFormat};
use apple_cf::cg::CGRect;
use core::ffi::{c_char, c_void};
use core::fmt;
use core::ptr;
use core::slice;
use std::ffi::CString;
use std::sync::Arc;

use doom_fish_utils::callback_context::CallbackContext;

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

type ProcessFn = dyn Fn(&[CIImageProcessorInputBuffer<'_>], &mut CIImageProcessorOutputBuffer<'_>) -> Result<(), String>
    + Send
    + Sync;

#[derive(Clone, Copy)]
struct BufferLayout {
    region: CGRect,
    format: CIFormat,
    bytes_per_row: usize,
    width: usize,
    height: usize,
    row_len: usize,
    len: usize,
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn pixel_count(extent: f64) -> Result<usize, String> {
    const EXACT_LIMIT: f64 = 9_007_199_254_740_992.0;
    if extent.is_finite() && extent >= 0.0 && extent.fract() == 0.0 && extent <= EXACT_LIMIT {
        Ok(extent as usize)
    } else {
        Err(format!("Core Image supplied a region dimension of {extent} pixels"))
    }
}

impl BufferLayout {
    fn new(region: CGRect, format_raw: i32, bytes_per_row: usize) -> Result<Self, String> {
        let format = CIFormat::from_raw(format_raw)
            .ok_or_else(|| format!("Core Image supplied unknown pixel format {format_raw}"))?;
        let width = pixel_count(region.size.width)?;
        let height = pixel_count(region.size.height)?;
        let row_len = width
            .checked_mul(format.bytes_per_pixel())
            .ok_or_else(|| "processor row length overflowed".to_string())?;
        if bytes_per_row < row_len {
            return Err(format!(
                "processor rows of {bytes_per_row} bytes cannot hold {width} {format:?} pixels"
            ));
        }
        let len = if width == 0 || height == 0 {
            0
        } else {
            bytes_per_row
                .checked_mul(height - 1)
                .and_then(|rows| rows.checked_add(row_len))
                .filter(|len| isize::try_from(*len).is_ok())
                .ok_or_else(|| "processor buffer size overflowed".to_string())?
        };
        Ok(Self {
            region,
            format,
            bytes_per_row,
            width,
            height,
            row_len,
            len,
        })
    }

    fn row_range(&self, y: usize) -> Option<core::ops::Range<usize>> {
        if y >= self.height {
            return None;
        }
        let start = y.checked_mul(self.bytes_per_row)?;
        Some(start..start.checked_add(self.row_len)?)
    }
}

impl fmt::Debug for BufferLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIImageProcessorBuffer")
            .field("region", &self.region)
            .field("format", &self.format)
            .field("bytes_per_row", &self.bytes_per_row)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish_non_exhaustive()
    }
}

pub struct CIImageProcessorInputBuffer<'a> {
    bytes: &'a [u8],
    layout: BufferLayout,
}

impl<'a> CIImageProcessorInputBuffer<'a> {
    pub const fn region(&self) -> CGRect {
        self.layout.region
    }

    pub const fn format(&self) -> CIFormat {
        self.layout.format
    }

    pub const fn bytes_per_row(&self) -> usize {
        self.layout.bytes_per_row
    }

    pub const fn width(&self) -> usize {
        self.layout.width
    }

    pub const fn height(&self) -> usize {
        self.layout.height
    }

    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    pub fn row(&self, y: usize) -> Option<&'a [u8]> {
        self.bytes.get(self.layout.row_range(y)?)
    }
}

impl fmt::Debug for CIImageProcessorInputBuffer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.layout.fmt(f)
    }
}

pub struct CIImageProcessorOutputBuffer<'a> {
    bytes: &'a mut [u8],
    layout: BufferLayout,
}

impl CIImageProcessorOutputBuffer<'_> {
    pub const fn region(&self) -> CGRect {
        self.layout.region
    }

    pub const fn format(&self) -> CIFormat {
        self.layout.format
    }

    pub const fn bytes_per_row(&self) -> usize {
        self.layout.bytes_per_row
    }

    pub const fn width(&self) -> usize {
        self.layout.width
    }

    pub const fn height(&self) -> usize {
        self.layout.height
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        self.bytes
    }

    pub fn row(&self, y: usize) -> Option<&[u8]> {
        self.bytes.get(self.layout.row_range(y)?)
    }

    pub fn row_mut(&mut self, y: usize) -> Option<&mut [u8]> {
        let range = self.layout.row_range(y)?;
        self.bytes.get_mut(range)
    }
}

impl fmt::Debug for CIImageProcessorOutputBuffer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.layout.fmt(f)
    }
}

struct ProcessorInvocation {
    process: Arc<ProcessFn>,
    region_of_interest: Option<Arc<RegionOfInterestFn>>,
    input_extents: Vec<CGRect>,
}

impl RegionOfInterestSource for ProcessorInvocation {
    const SITE: &'static str = "CIImageProcessorKernel region-of-interest callback";

    fn region(&self, input_index: usize, destination: CGRect) -> CGRect {
        self.region_of_interest
            .as_deref()
            .map_or(destination, |callback| {
                guarded_region(
                    Self::SITE,
                    callback,
                    &self.input_extents,
                    input_index,
                    destination,
                )
            })
    }
}

const PROCESS_SITE: &str = "CIImageProcessorKernel process closure";

fn ranges_overlap(first: (usize, usize), second: (usize, usize)) -> bool {
    let (first_start, first_len) = first;
    let (second_start, second_len) = second;
    if first_len == 0 || second_len == 0 {
        return false;
    }
    match (first_start.checked_add(first_len), second_start.checked_add(second_len)) {
        (Some(first_end), Some(second_end)) => first_start < second_end && second_start < first_end,
        _ => true,
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn run_processor(
    context: *mut c_void,
    input_count: usize,
    input_bases: *const *const u8,
    input_bytes_per_row: *const usize,
    input_formats: *const i32,
    input_regions: *const f64,
    output_base: *mut u8,
    output_layout: BufferLayout,
) -> Result<(), String> {
    if output_layout.len > 0 && output_base.is_null() {
        return Err("Core Image supplied no output memory".to_string());
    }
    let region_count = input_count
        .checked_mul(4)
        .ok_or_else(|| "too many processor inputs".to_string())?;
    if input_count > 0
        && (input_bases.is_null()
            || input_bytes_per_row.is_null()
            || input_formats.is_null()
            || input_regions.is_null())
    {
        return Err("Core Image supplied no input descriptions".to_string());
    }
    let (bases, strides, formats, regions): (&[*const u8], &[usize], &[i32], &[f64]) =
        if input_count == 0 {
            (&[], &[], &[], &[])
        } else {
            unsafe {
                (
                    slice::from_raw_parts(input_bases, input_count),
                    slice::from_raw_parts(input_bytes_per_row, input_count),
                    slice::from_raw_parts(input_formats, input_count),
                    slice::from_raw_parts(input_regions, region_count),
                )
            }
        };
    let output_range = (output_base as usize, output_layout.len);
    let mut inputs = Vec::with_capacity(input_count);
    for index in 0..input_count {
        let region = CGRect::new(
            regions[index * 4],
            regions[index * 4 + 1],
            regions[index * 4 + 2],
            regions[index * 4 + 3],
        );
        let layout = BufferLayout::new(region, formats[index], strides[index])?;
        let base = bases[index];
        if layout.len > 0 && base.is_null() {
            return Err(format!("Core Image supplied no memory for input {index}"));
        }
        if ranges_overlap((base as usize, layout.len), output_range) {
            return Err(format!("input {index} overlaps the output memory"));
        }
        let bytes: &[u8] = if layout.len == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(base, layout.len) }
        };
        inputs.push(CIImageProcessorInputBuffer { bytes, layout });
    }
    let bytes: &mut [u8] = if output_layout.len == 0 {
        Default::default()
    } else {
        unsafe { slice::from_raw_parts_mut(output_base, output_layout.len) }
    };
    let mut output = CIImageProcessorOutputBuffer {
        bytes,
        layout: output_layout,
    };
    let outcome = unsafe {
        CallbackContext::<ProcessorInvocation>::with(context, PROCESS_SITE, |invocation| {
            (invocation.process)(&inputs, &mut output)
        })
    };
    match outcome {
        Some(Ok(())) => Ok(()),
        Some(Err(message)) => {
            output.bytes.fill(0);
            Err(message)
        }
        None => {
            output.bytes.fill(0);
            Err("the image processor closure panicked".to_string())
        }
    }
}

unsafe extern "C" fn processor_invoke(
    context: *mut c_void,
    input_count: usize,
    input_bases: *const *const u8,
    input_bytes_per_row: *const usize,
    input_formats: *const i32,
    input_regions: *const f64,
    output_base: *mut u8,
    output_bytes_per_row: usize,
    output_format: i32,
    output_x: f64,
    output_y: f64,
    output_width: f64,
    output_height: f64,
    out_error: *mut *mut c_char,
) -> bool {
    let outcome = BufferLayout::new(
        CGRect::new(output_x, output_y, output_width, output_height),
        output_format,
        output_bytes_per_row,
    )
    .and_then(|output_layout| unsafe {
        run_processor(
            context,
            input_count,
            input_bases,
            input_bytes_per_row,
            input_formats,
            input_regions,
            output_base,
            output_layout,
        )
    });
    match outcome {
        Ok(()) => true,
        Err(message) => {
            if !out_error.is_null() {
                let message = CString::new(message.replace('\0', " ")).unwrap_or_default();
                unsafe { *out_error = libc::strdup(message.as_ptr()) };
            }
            false
        }
    }
}

#[derive(Clone)]
pub struct CIImageProcessorKernel {
    process: Arc<ProcessFn>,
    region_of_interest: Option<Arc<RegionOfInterestFn>>,
    format: Option<CIFormat>,
}

impl fmt::Debug for CIImageProcessorKernel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIImageProcessorKernel")
            .field("format", &self.format)
            .field("region_of_interest", &self.region_of_interest.is_some())
            .finish_non_exhaustive()
    }
}

impl CIImageProcessorKernel {
    pub fn new(
        process: impl Fn(
                &[CIImageProcessorInputBuffer<'_>],
                &mut CIImageProcessorOutputBuffer<'_>,
            ) -> Result<(), String>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self {
            process: Arc::new(process),
            region_of_interest: None,
            format: None,
        }
    }

    pub fn with_format(mut self, format: CIFormat) -> Result<Self, CIError> {
        if !matches!(
            format,
            CIFormat::Bgra8 | CIFormat::RgbaH | CIFormat::RgbaF | CIFormat::R8 | CIFormat::RH | CIFormat::RF
        ) {
            return Err(CIError::InvalidArgument(format!(
                "CIImageProcessorKernel supports Bgra8, RgbaH, RgbaF, R8, RH and RF, not {format:?}"
            )));
        }
        self.format = Some(format);
        Ok(self)
    }

    pub fn with_region_of_interest(
        mut self,
        region_of_interest: impl Fn(usize, CGRect) -> CGRect + Send + Sync + 'static,
    ) -> Self {
        self.region_of_interest = Some(Arc::new(region_of_interest));
        self
    }

    pub const fn format(&self) -> Option<CIFormat> {
        self.format
    }

    pub fn apply(&self, extent: CGRect, inputs: &[&CIImage]) -> Result<CIImage, CIError> {
        let handles = inputs.iter().map(|image| image.as_ptr()).collect::<Vec<_>>();
        let context = foreign_owned_context(ProcessorInvocation {
            process: Arc::clone(&self.process),
            region_of_interest: self.region_of_interest.clone(),
            input_extents: inputs.iter().map(|image| image.extent()).collect(),
        });
        let mut output = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_image_processor_apply_closure(
                extent.origin.x,
                extent.origin.y,
                extent.size.width,
                extent.size.height,
                handles.as_ptr(),
                handles.len(),
                self.format.map_or(0, CIFormat::raw_value),
                context,
                Some(processor_invoke),
                Some(region_of_interest_invoke::<ProcessorInvocation>),
                Some(CallbackContext::<ProcessorInvocation>::RELEASE),
                &raw mut output,
                &raw mut error,
            )
        };
        unsafe { status_result(status, error)? };
        if output.is_null() {
            Err(CIError::NullResult(
                "CIImageProcessorKernel.apply returned nil".to_string(),
            ))
        } else {
            Ok(unsafe { CIImage::from_raw(output) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout(width: f64, height: f64, format: CIFormat, bytes_per_row: usize) -> Result<BufferLayout, String> {
        BufferLayout::new(CGRect::new(0.0, 0.0, width, height), format.raw_value(), bytes_per_row)
    }

    fn invocation(
        process: impl Fn(&[CIImageProcessorInputBuffer<'_>], &mut CIImageProcessorOutputBuffer<'_>) -> Result<(), String>
            + Send
            + Sync
            + 'static,
    ) -> CallbackContext<ProcessorInvocation> {
        CallbackContext::new(ProcessorInvocation {
            process: Arc::new(process),
            region_of_interest: None,
            input_extents: Vec::new(),
        })
    }

    #[test]
    fn layouts_cover_only_the_rows_core_image_describes() {
        let bgra = layout(3.0, 2.0, CIFormat::Bgra8, 16).expect("valid layout");
        assert_eq!(bgra.len, 16 + 12);
        assert_eq!(bgra.row_range(0), Some(0..12));
        assert_eq!(bgra.row_range(1), Some(16..28));
        assert_eq!(bgra.row_range(2), None);
        let empty = layout(0.0, 5.0, CIFormat::RgbaF, 0).expect("empty layouts are valid");
        assert_eq!(empty.len, 0);
        assert_eq!(empty.row_range(0), Some(0..0));
    }

    #[test]
    fn layouts_reject_short_strides_and_unusable_dimensions() {
        assert!(layout(3.0, 2.0, CIFormat::Bgra8, 11).is_err());
        assert!(layout(2.0, 2.0, CIFormat::RgbaF, 31).is_err());
        for dimension in [f64::NAN, f64::INFINITY, -1.0, 2.5, 1e300] {
            assert!(layout(dimension, 2.0, CIFormat::R8, 64).is_err());
            assert!(layout(2.0, dimension, CIFormat::R8, 64).is_err());
        }
        assert!(layout(1.0, 4_503_599_627_370_496.0, CIFormat::R8, usize::MAX / 2).is_err());
        assert!(layout(1.0, 2.0, CIFormat::R8, usize::MAX / 2 + 1).is_err());
        assert!(BufferLayout::new(CGRect::new(0.0, 0.0, 1.0, 1.0), 12345, 64).is_err());
    }

    #[test]
    fn overlap_checks_treat_wrapping_ranges_as_overlapping() {
        assert!(ranges_overlap((0, 10), (5, 10)));
        assert!(ranges_overlap((5, 10), (0, 10)));
        assert!(!ranges_overlap((0, 10), (10, 10)));
        assert!(!ranges_overlap((0, 0), (0, 10)));
        assert!(ranges_overlap((usize::MAX - 1, 10), (0, 1)));
    }

    #[test]
    fn processor_trampoline_hands_out_exact_views() {
        let input = [1_u8; 28];
        let mut output = [0_u8; 28];
        let context = invocation(|inputs, output| {
            let input = inputs.first().ok_or("missing input")?;
            if input.bytes().len() != 28 || input.row(1).map(<[u8]>::len) != Some(12) {
                return Err("unexpected input view".to_string());
            }
            for y in 0..output.height() {
                output.row_mut(y).ok_or("missing output row")?.fill(7);
            }
            Ok(())
        });
        let bases = [input.as_ptr()];
        let strides = [16_usize];
        let formats = [CIFormat::Bgra8.raw_value()];
        let regions = [0.0, 0.0, 3.0, 2.0];
        let output_layout = layout(3.0, 2.0, CIFormat::Bgra8, 16).expect("valid layout");
        let result = unsafe {
            run_processor(
                context.as_ptr(),
                1,
                bases.as_ptr(),
                strides.as_ptr(),
                formats.as_ptr(),
                regions.as_ptr(),
                output.as_mut_ptr(),
                output_layout,
            )
        };
        assert_eq!(result, Ok(()));
        assert_eq!(&output[..12], &[7; 12]);
        assert_eq!(&output[12..16], &[0; 4]);
        assert_eq!(&output[16..], &[7; 12]);
    }

    #[test]
    fn processor_trampoline_refuses_overlapping_or_missing_memory() {
        let mut memory = vec![0_u8; 64];
        let context = invocation(|_, _| Ok(()));
        let output_layout = layout(2.0, 2.0, CIFormat::Bgra8, 8).expect("valid layout");
        let regions = [0.0, 0.0, 2.0, 2.0];
        let strides = [8_usize];
        let formats = [CIFormat::Bgra8.raw_value()];
        let base = memory.as_mut_ptr();
        let run = |bases: &[*const u8], output: *mut u8| unsafe {
            run_processor(
                context.as_ptr(),
                bases.len(),
                bases.as_ptr(),
                strides.as_ptr(),
                formats.as_ptr(),
                regions.as_ptr(),
                output,
                output_layout,
            )
        };
        assert!(run(&[unsafe { base.add(8) }.cast_const()], base).is_err());
        assert!(run(&[base.cast_const()], unsafe { base.add(15) }).is_err());
        assert!(run(&[unsafe { base.add(32) }.cast_const()], base).is_ok());
        assert!(run(&[unsafe { base.add(16) }.cast_const()], base).is_ok());
        assert!(run(&[ptr::null()], base).is_err());
        assert!(run(&[], ptr::null_mut()).is_err());
        let missing = unsafe {
            run_processor(
                context.as_ptr(),
                1,
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                base,
                output_layout,
            )
        };
        assert!(missing.is_err());
    }

    #[test]
    fn processor_trampoline_zeroes_the_output_after_errors_and_panics() {
        let failing = [
            invocation(|_, output| {
                output.bytes_mut().fill(9);
                Err("refused".to_string())
            }),
            invocation(|_, output| {
                output.bytes_mut().fill(9);
                panic!("processor closure panic")
            }),
        ];
        for context in failing {
            let mut output = vec![5_u8; 16];
            let result = unsafe {
                run_processor(
                    context.as_ptr(),
                    0,
                    ptr::null(),
                    ptr::null(),
                    ptr::null(),
                    ptr::null(),
                    output.as_mut_ptr(),
                    layout(2.0, 2.0, CIFormat::Bgra8, 8).expect("valid layout"),
                )
            };
            assert!(result.is_err());
            assert!(output.iter().all(|byte| *byte == 0));
        }
    }
}
