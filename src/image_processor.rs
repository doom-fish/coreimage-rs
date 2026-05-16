use crate::ffi;
use crate::image::CIImage;
use crate::util::{status_result, take_owned_string};
use crate::CIError;
use core::ptr;

/// Helpers for running the built-in bridge image processor kernels.
pub struct CIImageProcessor;

impl CIImageProcessor {
    pub fn apply_passthrough(image: &CIImage) -> Result<CIImage, CIError> {
        let mut output = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_image_processor_apply_passthrough(image.as_ptr(), &mut output, &mut error)
        };
        unsafe { status_result(status, error)? };
        Ok(unsafe { CIImage::from_raw(output) })
    }

    pub fn last_invocation_json() -> String {
        unsafe { take_owned_string(ffi::ci_image_processor_last_invocation_json()) }
            .unwrap_or_else(|| "{}".to_string())
    }
}
