use std::error::Error;

use coreimage::prelude::*;

fn qr_image() -> CIImage {
    let mut filter = CIFilter::new("CIQRCodeGenerator").expect("CIQRCodeGenerator should exist");
    filter.set_input_bytes("inputMessage", b"coreimage-rs");
    filter.set_input_string("inputCorrectionLevel", "M");
    filter
        .output_image()
        .expect("CIQRCodeGenerator should produce an image")
        .scaled(8.0, 8.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let detector = CIDetector::new(CIDetectorType::QrCode, None, &CIDetectorOptions::default())?;
    let feature = detector
        .features_in_image(&qr_image(), &CIDetectionOptions::default())?
        .into_iter()
        .next()
        .ok_or("expected at least one QR feature")?;

    println!(
        "feature kind: {:?}, message: {:?}",
        feature.kind(),
        feature.message_string()
    );
    Ok(())
}
