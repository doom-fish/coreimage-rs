use std::error::Error;

use coreimage::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let descriptor = CIBarcodeDescriptor::qr_code(b"A", 1, 0, CIQRCodeErrorCorrectionLevel::M)?;
    println!(
        "descriptor kind: {:?}, payload: {}",
        descriptor.kind(),
        descriptor.payload_base64()
    );
    Ok(())
}
