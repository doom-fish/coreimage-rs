use core::ffi::{c_char, c_void};

unsafe extern "C" {
/// Calls the `CoreImage` framework counterpart for `ci_barcode_descriptor_new_qr`.
    pub fn ci_barcode_descriptor_new_qr(
        bytes: *const u8,
        len: usize,
        symbol_version: isize,
        mask_pattern: u8,
        error_correction_level: i32,
        out_descriptor: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_barcode_descriptor_new_aztec`.
    pub fn ci_barcode_descriptor_new_aztec(
        bytes: *const u8,
        len: usize,
        is_compact: bool,
        layer_count: isize,
        data_codeword_count: isize,
        out_descriptor: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_barcode_descriptor_new_pdf417`.
    pub fn ci_barcode_descriptor_new_pdf417(
        bytes: *const u8,
        len: usize,
        is_compact: bool,
        row_count: isize,
        column_count: isize,
        out_descriptor: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_barcode_descriptor_new_data_matrix`.
    pub fn ci_barcode_descriptor_new_data_matrix(
        bytes: *const u8,
        len: usize,
        row_count: isize,
        column_count: isize,
        ecc_version: i32,
        out_descriptor: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_barcode_descriptor_kind`.
    pub fn ci_barcode_descriptor_kind(handle: *mut c_void) -> i32;
/// Calls the `CoreImage` framework counterpart for `ci_barcode_descriptor_payload_base64`.
    pub fn ci_barcode_descriptor_payload_base64(handle: *mut c_void) -> *mut c_char;
/// Calls the `CoreImage` framework counterpart for `ci_barcode_descriptor_details_json`.
    pub fn ci_barcode_descriptor_details_json(handle: *mut c_void) -> *mut c_char;
}
