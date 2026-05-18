use core::ffi::c_void;
use core::fmt;
use core::ptr;

use crate::ffi;
use crate::util::{status_result, take_owned_string};
use crate::CIError;

/// Barcode descriptor kinds supported by Core Image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIBarcodeDescriptorKind {
/// Mirrors the `CoreImage` framework case `QrCode`.
    QrCode,
/// Mirrors the `CoreImage` framework case `Aztec`.
    Aztec,
/// Mirrors the `CoreImage` framework case `Pdf417`.
    Pdf417,
/// Mirrors the `CoreImage` framework case `DataMatrix`.
    DataMatrix,
/// Mirrors the `CoreImage` framework case `Unknown`.
    Unknown,
}

/// QR code error-correction levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIQRCodeErrorCorrectionLevel {
/// Mirrors the `CoreImage` framework case `L`.
    L,
/// Mirrors the `CoreImage` framework case `M`.
    M,
/// Mirrors the `CoreImage` framework case `Q`.
    Q,
/// Mirrors the `CoreImage` framework case `H`.
    H,
}

impl CIQRCodeErrorCorrectionLevel {
    const fn code(self) -> i32 {
        match self {
            Self::L => 0,
            Self::M => 1,
            Self::Q => 2,
            Self::H => 3,
        }
    }
}

/// Data Matrix ECC versions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIDataMatrixCodeECCVersion {
/// Mirrors the `CoreImage` framework case `V000`.
    V000,
/// Mirrors the `CoreImage` framework case `V050`.
    V050,
/// Mirrors the `CoreImage` framework case `V080`.
    V080,
/// Mirrors the `CoreImage` framework case `V100`.
    V100,
/// Mirrors the `CoreImage` framework case `V140`.
    V140,
/// Mirrors the `CoreImage` framework case `V200`.
    V200,
}

impl CIDataMatrixCodeECCVersion {
    const fn code(self) -> i32 {
        match self {
            Self::V000 => 0,
            Self::V050 => 50,
            Self::V080 => 80,
            Self::V100 => 100,
            Self::V140 => 140,
            Self::V200 => 200,
        }
    }
}

/// A machine-readable barcode descriptor.
pub struct CIBarcodeDescriptor {
    ptr: *mut c_void,
}

impl Drop for CIBarcodeDescriptor {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIBarcodeDescriptor {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIBarcodeDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIBarcodeDescriptor")
            .field("ptr", &self.ptr)
            .field("kind", &self.kind())
            .finish_non_exhaustive()
    }
}

impl CIBarcodeDescriptor {
    pub(crate) const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    fn from_non_null(ptr: *mut c_void, kind: &str) -> Self {
        assert!(!ptr.is_null(), "{kind} returned nil");
        unsafe { Self::from_raw(ptr) }
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

/// Calls the `CoreImage` framework counterpart for `qr_code`.
    pub fn qr_code(
        payload: &[u8],
        symbol_version: isize,
        mask_pattern: u8,
        error_correction_level: CIQRCodeErrorCorrectionLevel,
    ) -> Result<Self, CIError> {
        let mut descriptor = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_barcode_descriptor_new_qr(
                payload.as_ptr(),
                payload.len(),
                symbol_version,
                mask_pattern,
                error_correction_level.code(),
                &mut descriptor,
                &mut error,
            )
        };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(descriptor, "CIQRCodeDescriptor"))
    }

/// Calls the `CoreImage` framework counterpart for `aztec_code`.
    pub fn aztec_code(
        payload: &[u8],
        is_compact: bool,
        layer_count: isize,
        data_codeword_count: isize,
    ) -> Result<Self, CIError> {
        let mut descriptor = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_barcode_descriptor_new_aztec(
                payload.as_ptr(),
                payload.len(),
                is_compact,
                layer_count,
                data_codeword_count,
                &mut descriptor,
                &mut error,
            )
        };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(descriptor, "CIAztecCodeDescriptor"))
    }

/// Calls the `CoreImage` framework counterpart for `pdf417_code`.
    pub fn pdf417_code(
        payload: &[u8],
        is_compact: bool,
        row_count: isize,
        column_count: isize,
    ) -> Result<Self, CIError> {
        let mut descriptor = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_barcode_descriptor_new_pdf417(
                payload.as_ptr(),
                payload.len(),
                is_compact,
                row_count,
                column_count,
                &mut descriptor,
                &mut error,
            )
        };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(descriptor, "CIPDF417CodeDescriptor"))
    }

/// Calls the `CoreImage` framework counterpart for `data_matrix_code`.
    pub fn data_matrix_code(
        payload: &[u8],
        row_count: isize,
        column_count: isize,
        ecc_version: CIDataMatrixCodeECCVersion,
    ) -> Result<Self, CIError> {
        let mut descriptor = ptr::null_mut();
        let mut error = ptr::null_mut();
        let status = unsafe {
            ffi::ci_barcode_descriptor_new_data_matrix(
                payload.as_ptr(),
                payload.len(),
                row_count,
                column_count,
                ecc_version.code(),
                &mut descriptor,
                &mut error,
            )
        };
        unsafe { status_result(status, error)? };
        Ok(Self::from_non_null(
            descriptor,
            "CIDataMatrixCodeDescriptor",
        ))
    }

/// Calls the `CoreImage` framework counterpart for `kind`.
    pub fn kind(&self) -> CIBarcodeDescriptorKind {
        match unsafe { ffi::ci_barcode_descriptor_kind(self.ptr) } {
            0 => CIBarcodeDescriptorKind::QrCode,
            1 => CIBarcodeDescriptorKind::Aztec,
            2 => CIBarcodeDescriptorKind::Pdf417,
            3 => CIBarcodeDescriptorKind::DataMatrix,
            _ => CIBarcodeDescriptorKind::Unknown,
        }
    }

/// Calls the `CoreImage` framework counterpart for `payload_base64`.
    pub fn payload_base64(&self) -> String {
        unsafe { take_owned_string(ffi::ci_barcode_descriptor_payload_base64(self.ptr)) }
            .unwrap_or_default()
    }

/// Calls the `CoreImage` framework counterpart for `details_json`.
    pub fn details_json(&self) -> String {
        unsafe { take_owned_string(ffi::ci_barcode_descriptor_details_json(self.ptr)) }
            .unwrap_or_else(|| "{}".to_string())
    }
}
