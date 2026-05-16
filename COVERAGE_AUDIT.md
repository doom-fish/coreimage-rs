# coreimage coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 499
VERIFIED: 99
GAPS: 367
EXEMPT: 33
COVERAGE_PCT: 21.24%

> Method note: this is a symbol-level audit. Generic stringly-typed escape hatches such as `CIFilter::new(name)`, raw input-key strings, and JSON metadata blobs are **not** counted as symbol coverage unless the crate exposes a dedicated typed helper for that SDK symbol.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CIAztecCodeDescriptor | interface | CIBarcodeDescriptor.h | CIBarcodeDescriptor::aztec_code |
| CIBarcodeDescriptor | interface | CIBarcodeDescriptor.h | CIBarcodeDescriptor |
| CIDataMatrixCodeDescriptor | interface | CIBarcodeDescriptor.h | CIBarcodeDescriptor::data_matrix_code |
| CIPDF417CodeDescriptor | interface | CIBarcodeDescriptor.h | CIBarcodeDescriptor::pdf417_code |
| CIQRCodeDescriptor | interface | CIBarcodeDescriptor.h | CIBarcodeDescriptor::qr_code |
| CIDataMatrixCodeECCVersion | typedef | CIBarcodeDescriptor.h | CIDataMatrixCodeECCVersion |
| CIQRCodeErrorCorrectionLevel | typedef | CIBarcodeDescriptor.h | CIQRCodeErrorCorrectionLevel |
| CIColor | interface | CIColor.h | CIColor |
| kCIContextAllowLowPower | const | CIContext.h | CIContextOptions::allow_low_power |
| kCIContextCacheIntermediates | const | CIContext.h | CIContextOptions::cache_intermediates |
| kCIContextName | const | CIContext.h | CIContextOptions::name |
| kCIContextOutputPremultiplied | const | CIContext.h | CIContextOptions::output_premultiplied |
| kCIContextPriorityRequestLow | const | CIContext.h | CIContextOptions::priority_request_low |
| kCIContextUseSoftwareRenderer | const | CIContext.h | CIContext::new_cpu |
| CIContext | interface | CIContext.h | CIContext |
| CIDetectorAccuracy | const | CIDetector.h | CIDetectorOptions::accuracy |
| CIDetectorAccuracyHigh | const | CIDetector.h | CIDetectorAccuracy::High |
| CIDetectorAccuracyLow | const | CIDetector.h | CIDetectorAccuracy::Low |
| CIDetectorAspectRatio | const | CIDetector.h | CIDetectionOptions::aspect_ratio |
| CIDetectorEyeBlink | const | CIDetector.h | CIDetectionOptions::eye_blink |
| CIDetectorFocalLength | const | CIDetector.h | CIDetectionOptions::focal_length |
| CIDetectorImageOrientation | const | CIDetector.h | CIDetectionOptions::image_orientation |
| CIDetectorMaxFeatureCount | const | CIDetector.h | CIDetectorOptions::max_feature_count |
| CIDetectorMinFeatureSize | const | CIDetector.h | CIDetectorOptions::min_feature_size |
| CIDetectorNumberOfAngles | const | CIDetector.h | CIDetectorOptions::number_of_angles |
| CIDetectorReturnSubFeatures | const | CIDetector.h | CIDetectionOptions::return_sub_features |
| CIDetectorSmile | const | CIDetector.h | CIDetectionOptions::smile |
| CIDetectorTracking | const | CIDetector.h | CIDetectorOptions::tracking |
| CIDetectorTypeFace | const | CIDetector.h | CIDetectorType::Face |
| CIDetectorTypeQRCode | const | CIDetector.h | CIDetectorType::QrCode |
| CIDetectorTypeRectangle | const | CIDetector.h | CIDetectorType::Rectangle |
| CIDetectorTypeText | const | CIDetector.h | CIDetectorType::Text |
| CIDetector | interface | CIDetector.h | CIDetector |
| CIFeatureTypeFace | const | CIFeature.h | CIFeatureKind::Face |
| CIFeatureTypeQRCode | const | CIFeature.h | CIFeatureKind::QrCode |
| CIFeatureTypeRectangle | const | CIFeature.h | CIFeatureKind::Rectangle |
| CIFeatureTypeText | const | CIFeature.h | CIFeatureKind::Text |
| CIFaceFeature | interface | CIFeature.h | CIFeature + CIFeatureKind::Face |
| CIFeature | interface | CIFeature.h | CIFeature |
| CIQRCodeFeature | interface | CIFeature.h | CIFeature + CIFeatureKind::QrCode |
| CIRectangleFeature | interface | CIFeature.h | CIFeature + CIFeatureKind::Rectangle |
| CITextFeature | interface | CIFeature.h | CIFeature + CIFeatureKind::Text |
| kCIAttributeFilterCategories | const | CIFilter.h | CIFilterGenerator::register_filter_name |
| kCIAttributeFilterDisplayName | const | CIFilter.h | CIFilterGenerator::register_filter_name |
| kCICategoryBuiltIn | const | CIFilter.h | CIFilterGenerator::register_filter_name |
| kCICategoryGenerator | const | CIFilter.h | CIFilterGenerator::register_filter_name |
| kCICategoryStillImage | const | CIFilter.h | CIFilterGenerator::register_filter_name |
| CIFilter | interface | CIFilter.h | CIFilter |
| CIFilter | protocol | CIFilter.h | CIFilter |
| CIBlendWithMask | protocol | CIFilterBuiltins.h | filters::blend_with_mask |
| CIBloom | protocol | CIFilterBuiltins.h | filters::bloom |
| CIBoxBlur | protocol | CIFilterBuiltins.h | filters::box_blur |
| CICheckerboardGenerator | protocol | CIFilterBuiltins.h | filters::checkerboard |
| CIColorControls | protocol | CIFilterBuiltins.h | filters::color_controls |
| CIColorInvert | protocol | CIFilterBuiltins.h | filters::color_invert |
| CIColorMonochrome | protocol | CIFilterBuiltins.h | filters::color_monochrome |
| CIComicEffect | protocol | CIFilterBuiltins.h | filters::comic_effect |
| CICrystallize | protocol | CIFilterBuiltins.h | filters::crystallize |
| CIDiscBlur | protocol | CIFilterBuiltins.h | filters::disc_blur |
| CIEdgeWork | protocol | CIFilterBuiltins.h | filters::edge_work |
| CIEdges | protocol | CIFilterBuiltins.h | filters::edges |
| CIExposureAdjust | protocol | CIFilterBuiltins.h | filters::exposure_adjust |
| CIFalseColor | protocol | CIFilterBuiltins.h | filters::false_color |
| CIGammaAdjust | protocol | CIFilterBuiltins.h | filters::gamma_adjust |
| CIGaussianBlur | protocol | CIFilterBuiltins.h | filters::gaussian_blur / CIImage::gaussian_blurred |
| CIHueAdjust | protocol | CIFilterBuiltins.h | filters::hue_adjust |
| CILanczosScaleTransform | protocol | CIFilterBuiltins.h | filters::lanczos_scale_transform |
| CILinearGradient | protocol | CIFilterBuiltins.h | filters::linear_gradient |
| CIMotionBlur | protocol | CIFilterBuiltins.h | filters::motion_blur |
| CIPerspectiveCorrection | protocol | CIFilterBuiltins.h | filters::perspective_correction |
| CIPerspectiveTransform | protocol | CIFilterBuiltins.h | filters::perspective_transform |
| CIPixellate | protocol | CIFilterBuiltins.h | filters::pixellate |
| CIRadialGradient | protocol | CIFilterBuiltins.h | filters::radial_gradient |
| CISepiaTone | protocol | CIFilterBuiltins.h | filters::sepia_tone |
| CISharpenLuminance | protocol | CIFilterBuiltins.h | filters::sharpen_luminance |
| CIStraighten | protocol | CIFilterBuiltins.h | filters::straighten |
| CITemperatureAndTint | protocol | CIFilterBuiltins.h | filters::temperature_and_tint |
| CIUnsharpMask | protocol | CIFilterBuiltins.h | filters::unsharp_mask |
| CIVibrance | protocol | CIFilterBuiltins.h | filters::vibrance |
| CIVignette | protocol | CIFilterBuiltins.h | filters::vignette |
| CIVignetteEffect | protocol | CIFilterBuiltins.h | filters::vignette_effect |
| CIWhitePointAdjust | protocol | CIFilterBuiltins.h | filters::white_point_adjust |
| CIZoomBlur | protocol | CIFilterBuiltins.h | filters::zoom_blur |
| CIFilterGenerator | interface | CIFilterGenerator.h | CIFilterGenerator |
| kCIFormatRGBA8 | const | CIImage.h | CIImage::from_bitmap_rgba8 |
| CIImage | interface | CIImage.h | CIImage |
| CIImageProcessorKernel | interface | CIImageProcessor.h | CIImageProcessor::apply_passthrough |
| CIBlendKernel | interface | CIKernel.h | CIBlendKernel |
| CIColorKernel | interface | CIKernel.h | CIColorKernel |
| CIWarpKernel | interface | CIKernel.h | CIWarpKernel |
| kCISamplerAffineMatrix | const | CISampler.h | CISamplerOptions::affine_transform |
| kCISamplerFilterLinear | const | CISampler.h | CISamplerFilterMode::Linear |
| kCISamplerFilterMode | const | CISampler.h | CISamplerOptions::filter_mode |
| kCISamplerFilterNearest | const | CISampler.h | CISamplerFilterMode::Nearest |
| kCISamplerWrapBlack | const | CISampler.h | CISamplerWrapMode::Black |
| kCISamplerWrapClamp | const | CISampler.h | CISamplerWrapMode::Clamp |
| kCISamplerWrapMode | const | CISampler.h | CISamplerOptions::wrap_mode |
| CISampler | interface | CISampler.h | CISampler |
| CIVector | interface | CIVector.h | CIVector |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| kCIContextCVMetalTextureCache | const | CIContext.h | No CIContextOptions field or render-option wrapper for this key. |
| kCIContextHighQualityDownsample | const | CIContext.h | No CIContextOptions field or render-option wrapper for this key. |
| kCIContextMemoryLimit | const | CIContext.h | No CIContextOptions field or render-option wrapper for this key. |
| kCIContextOutputColorSpace | const | CIContext.h | No CIContextOptions field or render-option wrapper for this key. |
| kCIContextWorkingColorSpace | const | CIContext.h | No CIContextOptions field or render-option wrapper for this key. |
| kCIContextWorkingFormat | const | CIContext.h | CIContext::working_format returns a raw i32, but this option key is not exposed. |
| kCIImageRepresentationAVDepthData | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationAVPortraitEffectsMatte | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationAVSemanticSegmentationMattes | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationDepthImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationDisparityImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationHDRGainMapAsRGB | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationHDRGainMapImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationHDRImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationPortraitEffectsMatteImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationSemanticSegmentationGlassesMatteImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationSemanticSegmentationHairMatteImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationSemanticSegmentationSkinMatteImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationSemanticSegmentationSkyMatteImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIImageRepresentationSemanticSegmentationTeethMatteImage | const | CIContext.h | CIContext::write_* helpers do not expose CIImageRepresentation option dictionaries. |
| kCIApplyOptionColorSpace | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIApplyOptionDefinition | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIApplyOptionExtent | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIApplyOptionUserInfo | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeClass | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeDefault | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeDescription | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeDisplayName | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeFilterAvailable_Mac | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeFilterAvailable_iOS | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeFilterName | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeIdentity | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeMax | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeMin | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeName | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeReferenceDocumentation | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeSliderMax | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeSliderMin | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeType | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeAngle | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeBoolean | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeColor | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeCount | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeDistance | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeGradient | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeImage | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeInteger | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeOffset | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeOpaqueColor | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypePosition | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypePosition3 | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeRectangle | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeScalar | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeTime | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIAttributeTypeTransform | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCICategoryBlur | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryColorAdjustment | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryColorEffect | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryCompositeOperation | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryDistortionEffect | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryFilterGenerator | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryGeometryAdjustment | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryGradient | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryHalftoneEffect | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryHighDynamicRange | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryInterlaced | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryNonSquarePixels | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryReduction | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategorySharpen | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryStylize | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryTileEffect | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryTransition | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCICategoryVideo | const | CIFilter.h | Pass raw category strings to CIFilter::names_in_category; the constant itself is not exported. |
| kCIDynamicRangeConstrainedHigh | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIDynamicRangeHigh | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIDynamicRangeStandard | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputAmountKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputAngleKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputAspectRatioKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputBackgroundImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputBacksideImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputBiasKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputBiasVectorKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputBrightnessKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputCenterKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputColor0Key | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputColor1Key | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputColorKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputColorSpaceKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputContrastKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputCountKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputDepthImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputDisparityImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputEVKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputExtentKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputExtrapolateKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputGradientImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputIntensityKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputMaskImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputMatteImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputPaletteImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputPerceptualKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputPoint0Key | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputPoint1Key | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputRadius0Key | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputRadius1Key | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputRadiusKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputRefractionKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputSaturationKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputScaleKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputShadingImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputSharpnessKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputTargetImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputThresholdKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputTimeKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputTransformKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputVersionKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputWeightsKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIInputWidthKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIOutputImageKey | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIUIParameterSet | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIUISetAdvanced | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIUISetBasic | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIUISetDevelopment | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| kCIUISetIntermediate | const | CIFilter.h | CIFilter uses raw string keys / JSON metadata here; the constant itself is not wrapped. |
| CIAccordionFoldTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAffineClamp | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAffineTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaAverage | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaAverageMaximumRed | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaBoundsRed | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaHistogram | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaLogarithmicHistogram | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaMaximum | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaMaximumAlpha | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaMinMax | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaMinMaxRed | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaMinimum | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaMinimumAlpha | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAreaReductionFilter | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAttributedTextImageGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIAztecCodeGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIBarcodeGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIBarsSwipeTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIBicubicScaleTransform | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIBlurredRectangleGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIBlurredRoundedRectangleGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIBokehBlur | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIBumpDistortion | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIBumpDistortionLinear | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CICMYKHalftone | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CICannyEdgeDetector | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CICircleSplashDistortion | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CICircularScreen | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CICircularWrap | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CICode128BarcodeGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorAbsoluteDifference | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorClamp | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorCrossPolynomial | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorCube | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorCubeWithColorSpace | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorCubesMixedWithMask | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorCurves | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorMap | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorMatrix | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorPolynomial | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorPosterize | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorThreshold | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColorThresholdOtsu | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIColumnAverage | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CICompositeOperation | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIConvertLab | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIConvolution | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CICopyMachineTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CICoreMLModel | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDepthOfField | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDepthToDisparity | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDisintegrateWithMaskTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDisparityToDepth | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDisplacementDistortion | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDissolveTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDistanceGradientFromRedMask | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDither | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDocumentEnhancer | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDotScreen | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIDroste | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIEdgePreserveUpsample | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIEightfoldReflectedTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIFlashTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIFourCoordinateGeometryFilter | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIFourfoldReflectedTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIFourfoldRotatedTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIFourfoldTranslatedTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIGaborGradients | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIGaussianGradient | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIGlassDistortion | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIGlassLozenge | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIGlideReflectedTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIGloom | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIHatchedScreen | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIHeightFieldFromMask | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIHexagonalPixellate | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIHighlightShadowAdjust | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIHistogramDisplay | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIHoleDistortion | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIHueSaturationValueGradient | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIKMeans | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIKaleidoscope | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIKeystoneCorrectionCombined | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIKeystoneCorrectionHorizontal | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIKeystoneCorrectionVertical | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CILabDeltaE | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CILenticularHaloGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CILightTunnel | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CILineOverlay | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CILineScreen | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CILinearToSRGBToneCurve | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMaskToAlpha | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMaskedVariableBlur | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMaximumComponent | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMaximumScaleTransform | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMedian | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMeshGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMinimumComponent | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMix | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIModTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMorphologyGradient | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMorphologyMaximum | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMorphologyMinimum | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMorphologyRectangleMaximum | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIMorphologyRectangleMinimum | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CINinePartStretched | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CINinePartTiled | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CINoiseReduction | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIOpTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPDF417BarcodeGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPageCurlTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPageCurlWithShadowTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPaletteCentroid | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPalettize | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIParallelogramTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPersonSegmentation | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPerspectiveRotate | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPerspectiveTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPerspectiveTransformWithExtent | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPhotoEffect | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPinchDistortion | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIPointillize | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIQRCodeGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIRandomGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIRippleTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIRoundedQRCodeGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIRoundedRectangleGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIRoundedRectangleStrokeGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIRowAverage | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISRGBToneCurveToLinear | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISaliencyMap | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIShadedMaterial | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISignedDistanceGradientFromRedMask | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISixfoldReflectedTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISixfoldRotatedTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISmoothLinearGradient | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISobelGradients | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISpotColor | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISpotLight | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIStarShineGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIStretchCrop | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIStripesGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISunbeamsGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISwipeTransition | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CISystemToneMap | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CITextImageGenerator | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIThermal | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIToneCurve | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIToneMapHeadroom | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CITorusLensDistortion | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CITransitionFilter | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CITriangleKaleidoscope | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CITriangleTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CITwelvefoldReflectedTile | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CITwirlDistortion | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIVortexDistortion | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIXRay | protocol | CIFilterBuiltins.h | No typed Rust helper; generic CIFilter::new(name) is the fallback. |
| CIFilterConstructor | protocol | CIFilterConstructor.h | No Rust bridge for custom filter-constructor protocol. |
| kCIFilterGeneratorExportedKey | const | CIFilterGenerator.h | CIFilterGenerator wrapper omits exported-key constants. |
| kCIFilterGeneratorExportedKeyName | const | CIFilterGenerator.h | CIFilterGenerator wrapper omits exported-key constants. |
| kCIFilterGeneratorExportedKeyTargetObject | const | CIFilterGenerator.h | CIFilterGenerator wrapper omits exported-key constants. |
| CIFilterShape | interface | CIFilterShape.h | CIFilterShape is not wrapped. |
| kCIFormatA16 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatA8 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatABGR8 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatARGB8 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatAf | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatAh | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatBGRA8 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatL16 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatL8 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatLA16 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatLA8 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatLAf | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatLAh | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatLf | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatLh | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatR16 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatR8 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRG16 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRG8 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGB10 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGBA16 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGBAf | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGBAh | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGBX16 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGBX8 | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGBXf | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGBXh | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGf | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRGh | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRf | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIFormatRh | const | CIImage.h | Crate does not expose CIFormat constants beyond the RGBA8 helper. |
| kCIImageApplyCleanAperture | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageApplyOrientationProperty | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageAutoAdjustCrop | const | CIImage.h | No auto-adjust option/filter wrapper. |
| kCIImageAutoAdjustEnhance | const | CIImage.h | No auto-adjust option/filter wrapper. |
| kCIImageAutoAdjustFeatures | const | CIImage.h | No auto-adjust option/filter wrapper. |
| kCIImageAutoAdjustLevel | const | CIImage.h | No auto-adjust option/filter wrapper. |
| kCIImageAutoAdjustRedEye | const | CIImage.h | No auto-adjust option/filter wrapper. |
| kCIImageAuxiliaryDepth | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageAuxiliaryDisparity | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageAuxiliaryHDRGainMap | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageAuxiliaryPortraitEffectsMatte | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageAuxiliarySemanticSegmentationGlassesMatte | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageAuxiliarySemanticSegmentationHairMatte | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageAuxiliarySemanticSegmentationSkinMatte | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageAuxiliarySemanticSegmentationSkyMatte | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageAuxiliarySemanticSegmentationTeethMatte | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageCacheImmediately | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageColorSpace | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageContentAverageLightLevel | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageContentHeadroom | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageExpandToHDR | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageNearestSampling | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageProperties | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| kCIImageToneMapHDRtoSDR | const | CIImage.h | No CIImageOption wrapper for this symbol. |
| CIImageAccumulator | interface | CIImageAccumulator.h | CIImageAccumulator is not wrapped. |
| CIImageProcessorInput | protocol | CIImageProcessor.h | The passthrough CIImageProcessor helper does not surface these protocol objects. |
| CIImageProcessorOutput | protocol | CIImageProcessor.h | The passthrough CIImageProcessor helper does not surface these protocol objects. |
| kCIImageProviderTileSize | const | CIImageProvider.h | No public Rust wrapper. |
| kCIImageProviderUserInfo | const | CIImageProvider.h | No public Rust wrapper. |
| CIKernel | interface | CIKernel.h | Only CIColorKernel/CIWarpKernel/CIBlendKernel wrappers are exposed. |
| CIPlugIn | interface | CIPlugIn.h | Plug-in APIs are not wrapped. |
| CIPlugInRegistration | protocol | CIPlugInInterface.h | Plug-in registration protocol is not wrapped. |
| CIRAWDecoderVersion6 | const | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRAWDecoderVersion6DNG | const | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRAWDecoderVersion7 | const | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRAWDecoderVersion7DNG | const | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRAWDecoderVersion8 | const | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRAWDecoderVersion8DNG | const | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRAWDecoderVersion9 | const | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRAWDecoderVersion9DNG | const | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRAWDecoderVersionNone | const | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRAWFilter | interface | CIRAWFilter.h | RAW filter APIs are not wrapped. |
| CIRenderDestination | interface | CIRenderDestination.h | Render-destination/task APIs are not wrapped. |
| CIRenderInfo | interface | CIRenderDestination.h | Render-destination/task APIs are not wrapped. |
| CIRenderTask | interface | CIRenderDestination.h | Render-destination/task APIs are not wrapped. |
| CIRenderDestinationAlphaMode | typedef | CIRenderDestination.h | Render-destination/task APIs are not wrapped. |
| kCISamplerColorSpace | const | CISampler.h | No public Rust wrapper. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| kCIImageTextureFormat | const | CIImage.h | Deprecated OpenGL image-option constant | CI_GL_DEPRECATED_MAC(10_9,10_14) |
| kCIImageTextureTarget | const | CIImage.h | Deprecated OpenGL image-option constant | CI_GL_DEPRECATED_MAC(10_9,10_14) |
| kCIActiveKeys | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputAllowDraftModeKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputBaselineExposureKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputBoostKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputBoostShadowAmountKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputColorNoiseReductionAmountKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputDecoderVersionKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputDisableGamutMapKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputEnableChromaticNoiseTrackingKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputEnableEDRModeKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputEnableSharpeningKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputEnableVendorLensCorrectionKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputIgnoreImageOrientationKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputImageOrientationKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputLinearSpaceFilter | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputLocalToneMapAmountKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputLuminanceNoiseReductionAmountKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputMoireAmountKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputNeutralChromaticityXKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputNeutralChromaticityYKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputNeutralLocationKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputNeutralTemperatureKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputNeutralTintKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputNoiseReductionAmountKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputNoiseReductionContrastAmountKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputNoiseReductionDetailAmountKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputNoiseReductionSharpnessAmountKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIInputScaleFactorKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIOutputNativeSizeKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCIPropertiesKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
| kCISupportedDecoderVersionsKey | const | CIRAWFilter_Deprecated.h | Deprecated RAW filter option constant | CIRAWFilter_Deprecated.h |
