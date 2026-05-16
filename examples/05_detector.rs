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
    let detector = CIDetector::new(
        CIDetectorType::QrCode,
        None,
        &CIDetectorOptions {
            accuracy: CIDetectorAccuracy::High,
            ..CIDetectorOptions::default()
        },
    )?;
    let features = detector.features_in_image(&qr_image(), &CIDetectionOptions::default())?;

    println!("detected {} qr features", features.len());
    Ok(())
}
