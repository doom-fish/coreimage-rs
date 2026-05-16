use coreimage::prelude::*;

#[test]
fn raw_filter_reports_supported_models_and_versions() {
    let camera_models = CIRAWFilter::supported_camera_models();
    assert!(!camera_models.is_empty());
    assert_eq!(CIRAWDecoderVersion::None.as_str(), "None");
    assert_eq!(CIRAWDecoderVersion::Version9Dng.as_str(), "9.dng");
}
