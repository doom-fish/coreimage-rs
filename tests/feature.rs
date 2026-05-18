mod common;

use std::error::Error;

use coreimage::prelude::*;

#[test]
fn qr_features_expose_message_and_descriptor() -> Result<(), Box<dyn Error>> {
    let detector = CIDetector::new(CIDetectorType::QrCode, None, &CIDetectorOptions::default())?;
    let features =
        detector.features_in_image(&common::qr_image(), &CIDetectionOptions::default())?;
    let feature = features
        .into_iter()
        .find(|feature| feature.kind() == CIFeatureKind::QrCode)
        .expect("expected a QR feature");

    assert_eq!(feature.message_string().as_deref(), Some("coreimage-rs"));
    assert_eq!(
        feature
            .symbol_descriptor()
            .map(|descriptor| descriptor.kind()),
        Some(CIBarcodeDescriptorKind::QrCode)
    );
    assert!(feature.bounds().size.width > 0.0);
    Ok(())
}
