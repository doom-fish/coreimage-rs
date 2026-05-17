use std::error::Error;

use coreimage::prelude::*;

#[test]
fn typed_constant_families_expose_expected_values() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        CIContextOptionKey::MemoryLimit.as_str(),
        "kCIContextMemoryLimit"
    );
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
    assert_eq!(
        CIApplyOptionKey::ColorSpace.as_str(),
        "kCIApplyOptionColorSpace"
    );
    assert_eq!(
        CIAttributeKey::FilterName.as_str(),
        "kCIAttributeFilterName"
    );
    assert_eq!(
        CIAttributeType::Transform.as_str(),
        "kCIAttributeTypeTransform"
    );
    assert_eq!(CIFilterCategory::Blur.as_str(), "kCICategoryBlur");
    assert_eq!(
        CIDynamicRange::ConstrainedHigh.as_str(),
        "kCIDynamicRangeConstrainedHigh"
    );
    assert_eq!(CIInputKey::Intensity.as_str(), "kCIInputIntensityKey");
    assert_eq!(CIOutputKey::Image.as_str(), "kCIOutputImageKey");
    assert_eq!(
        CIUIParameterSetKey::ParameterSet.as_str(),
        "kCIUIParameterSet"
    );
    assert_eq!(CIUIParameterSet::Advanced.as_str(), "kCIUISetAdvanced");
    assert_eq!(
        CIFilterGeneratorExportedKey::TargetObject.as_str(),
        "kCIFilterGeneratorExportedKeyTargetObject"
    );
    assert_eq!(
        CIImageProviderOptionKey::TileSize.as_str(),
        "kCIImageProviderTileSize"
    );
    assert_eq!(
        CISamplerOptionKey::ColorSpace.as_str(),
        "kCISamplerColorSpace"
    );
    assert_eq!(CIFormat::Rgba8.bytes_per_pixel(), 4);
    assert_eq!(
        CIFormat::from_raw(CIFormat::Rgba8.raw_value()),
        Some(CIFormat::Rgba8)
    );
    assert!(CIFilter::names_in_category_key(CIFilterCategory::Blur)
        .iter()
        .any(|name| name == "CIGaussianBlur"));

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
