use std::error::Error;

use coreimage::prelude::*;

#[test]
fn typed_constant_families_expose_expected_values() -> Result<(), Box<dyn Error>> {
    assert_eq!(CIContextOptionKey::MemoryLimit.as_str(), "kCIContextMemoryLimit");
    assert_eq!(
        CIImageOptionKey::AuxiliaryHdrGainMap.as_str(),
        "kCIImageAuxiliaryHDRGainMap"
    );
    assert_eq!(
        CIImageRepresentationOptionKey::HdrImage.as_str(),
        "kCIImageRepresentationHDRImage"
    );
    assert_eq!(
        CIImageAutoAdjustmentOptionKey::Enhance.as_str(),
        "kCIImageAutoAdjustEnhance"
    );
    assert_eq!(CIRAWDecoderVersion::Version6Dng.as_str(), "6.dng");
    assert_eq!(CIFormat::Rgba8.bytes_per_pixel(), 4);
    assert_eq!(CIFormat::from_raw(CIFormat::Rgba8.raw_value()), Some(CIFormat::Rgba8));

    let options = CIContextOptions {
        cache_intermediates: true,
        priority_request_low: true,
        allow_low_power: true,
        output_premultiplied: true,
        high_quality_downsample: true,
        output_color_space: Some(CIColorSpace::Srgb),
        working_color_space: Some(CIColorSpace::ExtendedSrgb),
        working_format: Some(CIFormat::Rgba8),
        memory_limit: Some(32.0 * 1024.0 * 1024.0),
        name: Some("constants-test".to_string()),
    };
    let context = CIContext::with_options(&options)?;
    assert_eq!(context.working_pixel_format(), Some(CIFormat::Rgba8));
    Ok(())
}
