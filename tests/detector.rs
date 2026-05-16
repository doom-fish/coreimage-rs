mod common;

use std::error::Error;

use coreimage::prelude::*;

#[test]
fn qr_detector_finds_features() -> Result<(), Box<dyn Error>> {
    let detector = CIDetector::new(
        CIDetectorType::QrCode,
        None,
        &CIDetectorOptions {
            accuracy: CIDetectorAccuracy::High,
            ..CIDetectorOptions::default()
        },
    )?;
    let features =
        detector.features_in_image(&common::qr_image(), &CIDetectionOptions::default())?;

    assert!(!features.is_empty());
    Ok(())
}
