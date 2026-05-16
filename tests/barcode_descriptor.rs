use std::error::Error;

use coreimage::prelude::*;

#[test]
fn qr_barcode_descriptor_reports_kind() -> Result<(), Box<dyn Error>> {
    let descriptor = CIBarcodeDescriptor::qr_code(b"A", 1, 0, CIQRCodeErrorCorrectionLevel::M)?;

    assert_eq!(descriptor.kind(), CIBarcodeDescriptorKind::QrCode);
    assert!(!descriptor.payload_base64().is_empty());
    Ok(())
}
