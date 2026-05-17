# coreimage coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 499
VERIFIED: 466
GAPS: 0
EXEMPT: 33
COVERAGE_PCT: 100.00%

> Methodology: comprehensive symbol-level audit of CoreImage SDK (macOS 26.2) against coreimage-rs wrapper. Enumerates all 499 public SDK symbols (30 classes, 200+ filter protocols, 12+ typedefs, 271 constants) by parsing framework headers. All EXEMPT entries (33 deprecated=/unavailable symbols) re-validated in macOS 26.2 SDK. Generic stringly-typed escape hatches such as `CIFilter::new(name)`, raw input-key strings, and JSON metadata blobs are **not** counted as symbol coverage unless the crate exposes a dedicated typed helper for that SDK symbol.

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
| kCIContextCVMetalTextureCache | const | CIContext.h | CIContextOptionKey::CvMetalTextureCache |
| kCIContextHighQualityDownsample | const | CIContext.h | CIContextOptionKey::HighQualityDownsample + CIContextOptions::high_quality_downsample |
| kCIContextMemoryLimit | const | CIContext.h | CIContextOptionKey::MemoryLimit + CIContextOptions::memory_limit |
| kCIContextOutputColorSpace | const | CIContext.h | CIContextOptionKey::OutputColorSpace + CIContextOptions::output_color_space |
| kCIContextWorkingColorSpace | const | CIContext.h | CIContextOptionKey::WorkingColorSpace + CIContextOptions::working_color_space |
| kCIContextWorkingFormat | const | CIContext.h | CIContextOptionKey::WorkingFormat + CIContextOptions::working_format |
| kCIImageRepresentationAVDepthData | const | CIContext.h | CIImageRepresentationOptionKey::AvDepthData |
| kCIImageRepresentationAVPortraitEffectsMatte | const | CIContext.h | CIImageRepresentationOptionKey::AvPortraitEffectsMatte |
| kCIImageRepresentationAVSemanticSegmentationMattes | const | CIContext.h | CIImageRepresentationOptionKey::AvSemanticSegmentationMattes |
| kCIImageRepresentationDepthImage | const | CIContext.h | CIImageRepresentationOptionKey::DepthImage |
| kCIImageRepresentationDisparityImage | const | CIContext.h | CIImageRepresentationOptionKey::DisparityImage |
| kCIImageRepresentationHDRGainMapAsRGB | const | CIContext.h | CIImageRepresentationOptionKey::HdrGainMapAsRgb |
| kCIImageRepresentationHDRGainMapImage | const | CIContext.h | CIImageRepresentationOptionKey::HdrGainMapImage |
| kCIImageRepresentationHDRImage | const | CIContext.h | CIImageRepresentationOptionKey::HdrImage |
| kCIImageRepresentationPortraitEffectsMatteImage | const | CIContext.h | CIImageRepresentationOptionKey::PortraitEffectsMatteImage |
| kCIImageRepresentationSemanticSegmentationGlassesMatteImage | const | CIContext.h | CIImageRepresentationOptionKey::SemanticSegmentationGlassesMatteImage |
| kCIImageRepresentationSemanticSegmentationHairMatteImage | const | CIContext.h | CIImageRepresentationOptionKey::SemanticSegmentationHairMatteImage |
| kCIImageRepresentationSemanticSegmentationSkinMatteImage | const | CIContext.h | CIImageRepresentationOptionKey::SemanticSegmentationSkinMatteImage |
| kCIImageRepresentationSemanticSegmentationSkyMatteImage | const | CIContext.h | CIImageRepresentationOptionKey::SemanticSegmentationSkyMatteImage |
| kCIImageRepresentationSemanticSegmentationTeethMatteImage | const | CIContext.h | CIImageRepresentationOptionKey::SemanticSegmentationTeethMatteImage |
| CIAccordionFoldTransition | protocol | CIFilterBuiltins.h | filters::accordion_fold_transition |
| CIAffineClamp | protocol | CIFilterBuiltins.h | filters::affine_clamp |
| CIAffineTile | protocol | CIFilterBuiltins.h | filters::affine_tile |
| CIAreaAverage | protocol | CIFilterBuiltins.h | filters::area_average |
| CIAreaAverageMaximumRed | protocol | CIFilterBuiltins.h | filters::area_average_maximum_red |
| CIAreaBoundsRed | protocol | CIFilterBuiltins.h | filters::area_bounds_red |
| CIAreaHistogram | protocol | CIFilterBuiltins.h | filters::area_histogram |
| CIAreaLogarithmicHistogram | protocol | CIFilterBuiltins.h | filters::area_logarithmic_histogram |
| CIAreaMaximum | protocol | CIFilterBuiltins.h | filters::area_maximum |
| CIAreaMaximumAlpha | protocol | CIFilterBuiltins.h | filters::area_maximum_alpha |
| CIAreaMinMax | protocol | CIFilterBuiltins.h | filters::area_min_max |
| CIAreaMinMaxRed | protocol | CIFilterBuiltins.h | filters::area_min_max_red |
| CIAreaMinimum | protocol | CIFilterBuiltins.h | filters::area_minimum |
| CIAreaMinimumAlpha | protocol | CIFilterBuiltins.h | filters::area_minimum_alpha |
| CIAreaReductionFilter | protocol | CIFilterBuiltins.h | filters::area_reduction_filter |
| CIAttributedTextImageGenerator | protocol | CIFilterBuiltins.h | filters::attributed_text_image_generator |
| CIAztecCodeGenerator | protocol | CIFilterBuiltins.h | filters::aztec_code_generator |
| CIBarcodeGenerator | protocol | CIFilterBuiltins.h | filters::barcode_generator |
| CIBarsSwipeTransition | protocol | CIFilterBuiltins.h | filters::bars_swipe_transition |
| CIBicubicScaleTransform | protocol | CIFilterBuiltins.h | filters::bicubic_scale_transform |
| CIBlurredRectangleGenerator | protocol | CIFilterBuiltins.h | filters::blurred_rectangle_generator |
| CIBlurredRoundedRectangleGenerator | protocol | CIFilterBuiltins.h | filters::blurred_rounded_rectangle_generator |
| CIBokehBlur | protocol | CIFilterBuiltins.h | filters::bokeh_blur |
| CIBumpDistortion | protocol | CIFilterBuiltins.h | filters::bump_distortion |
| CIBumpDistortionLinear | protocol | CIFilterBuiltins.h | filters::bump_distortion_linear |
| CICMYKHalftone | protocol | CIFilterBuiltins.h | filters::cmyk_halftone |
| CICannyEdgeDetector | protocol | CIFilterBuiltins.h | filters::canny_edge_detector |
| CICircleSplashDistortion | protocol | CIFilterBuiltins.h | filters::circle_splash_distortion |
| CICircularScreen | protocol | CIFilterBuiltins.h | filters::circular_screen |
| CICircularWrap | protocol | CIFilterBuiltins.h | filters::circular_wrap |
| CICode128BarcodeGenerator | protocol | CIFilterBuiltins.h | filters::code128_barcode_generator |
| CIColorAbsoluteDifference | protocol | CIFilterBuiltins.h | filters::color_absolute_difference |
| CIColorClamp | protocol | CIFilterBuiltins.h | filters::color_clamp |
| CIColorCrossPolynomial | protocol | CIFilterBuiltins.h | filters::color_cross_polynomial |
| CIColorCube | protocol | CIFilterBuiltins.h | filters::color_cube |
| CIColorCubeWithColorSpace | protocol | CIFilterBuiltins.h | filters::color_cube_with_color_space |
| CIColorCubesMixedWithMask | protocol | CIFilterBuiltins.h | filters::color_cubes_mixed_with_mask |
| CIColorCurves | protocol | CIFilterBuiltins.h | filters::color_curves |
| CIColorMap | protocol | CIFilterBuiltins.h | filters::color_map |
| CIColorMatrix | protocol | CIFilterBuiltins.h | filters::color_matrix |
| CIColorPolynomial | protocol | CIFilterBuiltins.h | filters::color_polynomial |
| CIColorPosterize | protocol | CIFilterBuiltins.h | filters::color_posterize |
| CIColorThreshold | protocol | CIFilterBuiltins.h | filters::color_threshold |
| CIColorThresholdOtsu | protocol | CIFilterBuiltins.h | filters::color_threshold_otsu |
| CIColumnAverage | protocol | CIFilterBuiltins.h | filters::column_average |
| CIConvolution | protocol | CIFilterBuiltins.h | filters::convolution |
| CICopyMachineTransition | protocol | CIFilterBuiltins.h | filters::copy_machine_transition |
| CIDepthOfField | protocol | CIFilterBuiltins.h | filters::depth_of_field |
| CIDepthToDisparity | protocol | CIFilterBuiltins.h | filters::depth_to_disparity |
| CIDisintegrateWithMaskTransition | protocol | CIFilterBuiltins.h | filters::disintegrate_with_mask_transition |
| CIDisparityToDepth | protocol | CIFilterBuiltins.h | filters::disparity_to_depth |
| CIDisplacementDistortion | protocol | CIFilterBuiltins.h | filters::displacement_distortion |
| CIDissolveTransition | protocol | CIFilterBuiltins.h | filters::dissolve_transition |
| CIDistanceGradientFromRedMask | protocol | CIFilterBuiltins.h | filters::distance_gradient_from_red_mask |
| CIDither | protocol | CIFilterBuiltins.h | filters::dither |
| CIDocumentEnhancer | protocol | CIFilterBuiltins.h | filters::document_enhancer |
| CIDotScreen | protocol | CIFilterBuiltins.h | filters::dot_screen |
| CIDroste | protocol | CIFilterBuiltins.h | filters::droste |
| CIEightfoldReflectedTile | protocol | CIFilterBuiltins.h | filters::eightfold_reflected_tile |
| CIFlashTransition | protocol | CIFilterBuiltins.h | filters::flash_transition |
| CIFourfoldReflectedTile | protocol | CIFilterBuiltins.h | filters::fourfold_reflected_tile |
| CIFourfoldRotatedTile | protocol | CIFilterBuiltins.h | filters::fourfold_rotated_tile |
| CIFourfoldTranslatedTile | protocol | CIFilterBuiltins.h | filters::fourfold_translated_tile |
| CIGaborGradients | protocol | CIFilterBuiltins.h | filters::gabor_gradients |
| CIGaussianGradient | protocol | CIFilterBuiltins.h | filters::gaussian_gradient |
| CIGlassDistortion | protocol | CIFilterBuiltins.h | filters::glass_distortion |
| CIGlassLozenge | protocol | CIFilterBuiltins.h | filters::glass_lozenge |
| CIGlideReflectedTile | protocol | CIFilterBuiltins.h | filters::glide_reflected_tile |
| CIGloom | protocol | CIFilterBuiltins.h | filters::gloom |
| CIHatchedScreen | protocol | CIFilterBuiltins.h | filters::hatched_screen |
| CIHeightFieldFromMask | protocol | CIFilterBuiltins.h | filters::height_field_from_mask |
| CIHexagonalPixellate | protocol | CIFilterBuiltins.h | filters::hexagonal_pixellate |
| CIHighlightShadowAdjust | protocol | CIFilterBuiltins.h | filters::highlight_shadow_adjust |
| CIHoleDistortion | protocol | CIFilterBuiltins.h | filters::hole_distortion |
| CIHueSaturationValueGradient | protocol | CIFilterBuiltins.h | filters::hue_saturation_value_gradient |
| CIKMeans | protocol | CIFilterBuiltins.h | filters::k_means |
| CIKaleidoscope | protocol | CIFilterBuiltins.h | filters::kaleidoscope |
| CIKeystoneCorrectionCombined | protocol | CIFilterBuiltins.h | filters::keystone_correction_combined |
| CIKeystoneCorrectionHorizontal | protocol | CIFilterBuiltins.h | filters::keystone_correction_horizontal |
| CIKeystoneCorrectionVertical | protocol | CIFilterBuiltins.h | filters::keystone_correction_vertical |
| CILabDeltaE | protocol | CIFilterBuiltins.h | filters::lab_delta_e |
| CILenticularHaloGenerator | protocol | CIFilterBuiltins.h | filters::lenticular_halo_generator |
| CILightTunnel | protocol | CIFilterBuiltins.h | filters::light_tunnel |
| CILineOverlay | protocol | CIFilterBuiltins.h | filters::line_overlay |
| CILineScreen | protocol | CIFilterBuiltins.h | filters::line_screen |
| CILinearToSRGBToneCurve | protocol | CIFilterBuiltins.h | filters::linear_to_srgb_tone_curve |
| CIMaskToAlpha | protocol | CIFilterBuiltins.h | filters::mask_to_alpha |
| CIMaskedVariableBlur | protocol | CIFilterBuiltins.h | filters::masked_variable_blur |
| CIMaximumComponent | protocol | CIFilterBuiltins.h | filters::maximum_component |
| CIMaximumScaleTransform | protocol | CIFilterBuiltins.h | filters::maximum_scale_transform |
| CIMeshGenerator | protocol | CIFilterBuiltins.h | filters::mesh_generator |
| CIMinimumComponent | protocol | CIFilterBuiltins.h | filters::minimum_component |
| CIMix | protocol | CIFilterBuiltins.h | filters::mix |
| CIModTransition | protocol | CIFilterBuiltins.h | filters::mod_transition |
| CIMorphologyGradient | protocol | CIFilterBuiltins.h | filters::morphology_gradient |
| CIMorphologyMaximum | protocol | CIFilterBuiltins.h | filters::morphology_maximum |
| CIMorphologyMinimum | protocol | CIFilterBuiltins.h | filters::morphology_minimum |
| CIMorphologyRectangleMaximum | protocol | CIFilterBuiltins.h | filters::morphology_rectangle_maximum |
| CIMorphologyRectangleMinimum | protocol | CIFilterBuiltins.h | filters::morphology_rectangle_minimum |
| CINinePartStretched | protocol | CIFilterBuiltins.h | filters::nine_part_stretched |
| CINinePartTiled | protocol | CIFilterBuiltins.h | filters::nine_part_tiled |
| CINoiseReduction | protocol | CIFilterBuiltins.h | filters::noise_reduction |
| CIOpTile | protocol | CIFilterBuiltins.h | filters::op_tile |
| CIPDF417BarcodeGenerator | protocol | CIFilterBuiltins.h | filters::pdf417_barcode_generator |
| CIPageCurlTransition | protocol | CIFilterBuiltins.h | filters::page_curl_transition |
| CIPageCurlWithShadowTransition | protocol | CIFilterBuiltins.h | filters::page_curl_with_shadow_transition |
| CIPaletteCentroid | protocol | CIFilterBuiltins.h | filters::palette_centroid |
| CIPalettize | protocol | CIFilterBuiltins.h | filters::palettize |
| CIParallelogramTile | protocol | CIFilterBuiltins.h | filters::parallelogram_tile |
| CIPersonSegmentation | protocol | CIFilterBuiltins.h | filters::person_segmentation |
| CIPerspectiveRotate | protocol | CIFilterBuiltins.h | filters::perspective_rotate |
| CIPerspectiveTile | protocol | CIFilterBuiltins.h | filters::perspective_tile |
| CIPerspectiveTransformWithExtent | protocol | CIFilterBuiltins.h | filters::perspective_transform_with_extent |
| CIPhotoEffect | protocol | CIFilterBuiltins.h | filters::photo_effect |
| CIPinchDistortion | protocol | CIFilterBuiltins.h | filters::pinch_distortion |
| CIPointillize | protocol | CIFilterBuiltins.h | filters::pointillize |
| CIQRCodeGenerator | protocol | CIFilterBuiltins.h | filters::qr_code_generator |
| CIRandomGenerator | protocol | CIFilterBuiltins.h | filters::random_generator |
| CIRippleTransition | protocol | CIFilterBuiltins.h | filters::ripple_transition |
| CIRoundedQRCodeGenerator | protocol | CIFilterBuiltins.h | filters::rounded_qr_code_generator |
| CIRoundedRectangleGenerator | protocol | CIFilterBuiltins.h | filters::rounded_rectangle_generator |
| CIRoundedRectangleStrokeGenerator | protocol | CIFilterBuiltins.h | filters::rounded_rectangle_stroke_generator |
| CIRowAverage | protocol | CIFilterBuiltins.h | filters::row_average |
| CISRGBToneCurveToLinear | protocol | CIFilterBuiltins.h | filters::srgb_tone_curve_to_linear |
| CIShadedMaterial | protocol | CIFilterBuiltins.h | filters::shaded_material |
| CISignedDistanceGradientFromRedMask | protocol | CIFilterBuiltins.h | filters::signed_distance_gradient_from_red_mask |
| CISixfoldReflectedTile | protocol | CIFilterBuiltins.h | filters::sixfold_reflected_tile |
| CISixfoldRotatedTile | protocol | CIFilterBuiltins.h | filters::sixfold_rotated_tile |
| CISmoothLinearGradient | protocol | CIFilterBuiltins.h | filters::smooth_linear_gradient |
| CISobelGradients | protocol | CIFilterBuiltins.h | filters::sobel_gradients |
| CISpotColor | protocol | CIFilterBuiltins.h | filters::spot_color |
| CISpotLight | protocol | CIFilterBuiltins.h | filters::spot_light |
| CIStarShineGenerator | protocol | CIFilterBuiltins.h | filters::star_shine_generator |
| CIStretchCrop | protocol | CIFilterBuiltins.h | filters::stretch_crop |
| CIStripesGenerator | protocol | CIFilterBuiltins.h | filters::stripes_generator |
| CISunbeamsGenerator | protocol | CIFilterBuiltins.h | filters::sunbeams_generator |
| CISwipeTransition | protocol | CIFilterBuiltins.h | filters::swipe_transition |
| CISystemToneMap | protocol | CIFilterBuiltins.h | filters::system_tone_map |
| CITextImageGenerator | protocol | CIFilterBuiltins.h | filters::text_image_generator |
| CIThermal | protocol | CIFilterBuiltins.h | filters::thermal |
| CIToneCurve | protocol | CIFilterBuiltins.h | filters::tone_curve |
| CIToneMapHeadroom | protocol | CIFilterBuiltins.h | filters::tone_map_headroom |
| CITorusLensDistortion | protocol | CIFilterBuiltins.h | filters::torus_lens_distortion |
| CITriangleKaleidoscope | protocol | CIFilterBuiltins.h | filters::triangle_kaleidoscope |
| CITriangleTile | protocol | CIFilterBuiltins.h | filters::triangle_tile |
| CITwelvefoldReflectedTile | protocol | CIFilterBuiltins.h | filters::twelvefold_reflected_tile |
| CITwirlDistortion | protocol | CIFilterBuiltins.h | filters::twirl_distortion |
| CIVortexDistortion | protocol | CIFilterBuiltins.h | filters::vortex_distortion |
| CIXRay | protocol | CIFilterBuiltins.h | filters::x_ray |
| CIFilterShape | interface | CIFilterShape.h | CIFilterShape |
| kCIFormatA16 | const | CIImage.h | CIFormat::A16 |
| kCIFormatA8 | const | CIImage.h | CIFormat::A8 |
| kCIFormatABGR8 | const | CIImage.h | CIFormat::Abgr8 |
| kCIFormatARGB8 | const | CIImage.h | CIFormat::Argb8 |
| kCIFormatAf | const | CIImage.h | CIFormat::AF |
| kCIFormatAh | const | CIImage.h | CIFormat::AH |
| kCIFormatBGRA8 | const | CIImage.h | CIFormat::Bgra8 |
| kCIFormatL16 | const | CIImage.h | CIFormat::L16 |
| kCIFormatL8 | const | CIImage.h | CIFormat::L8 |
| kCIFormatLA16 | const | CIImage.h | CIFormat::La16 |
| kCIFormatLA8 | const | CIImage.h | CIFormat::La8 |
| kCIFormatLAf | const | CIImage.h | CIFormat::LaF |
| kCIFormatLAh | const | CIImage.h | CIFormat::LaH |
| kCIFormatLf | const | CIImage.h | CIFormat::LF |
| kCIFormatLh | const | CIImage.h | CIFormat::LH |
| kCIFormatR16 | const | CIImage.h | CIFormat::R16 |
| kCIFormatR8 | const | CIImage.h | CIFormat::R8 |
| kCIFormatRG16 | const | CIImage.h | CIFormat::Rg16 |
| kCIFormatRG8 | const | CIImage.h | CIFormat::Rg8 |
| kCIFormatRGB10 | const | CIImage.h | CIFormat::Rgb10 |
| kCIFormatRGBA16 | const | CIImage.h | CIFormat::Rgba16 |
| kCIFormatRGBAf | const | CIImage.h | CIFormat::RgbaF |
| kCIFormatRGBAh | const | CIImage.h | CIFormat::RgbaH |
| kCIFormatRGBX16 | const | CIImage.h | CIFormat::Rgbx16 |
| kCIFormatRGBX8 | const | CIImage.h | CIFormat::Rgbx8 |
| kCIFormatRGBXf | const | CIImage.h | CIFormat::RgbxF |
| kCIFormatRGBXh | const | CIImage.h | CIFormat::RgbxH |
| kCIFormatRGf | const | CIImage.h | CIFormat::RgF |
| kCIFormatRGh | const | CIImage.h | CIFormat::RgH |
| kCIFormatRf | const | CIImage.h | CIFormat::RF |
| kCIFormatRh | const | CIImage.h | CIFormat::RH |
| kCIImageApplyCleanAperture | const | CIImage.h | CIImageOptionKey::ApplyCleanAperture |
| kCIImageApplyOrientationProperty | const | CIImage.h | CIImageOptionKey::ApplyOrientationProperty |
| kCIImageAutoAdjustCrop | const | CIImage.h | CIImageAutoAdjustmentOptionKey::Crop |
| kCIImageAutoAdjustEnhance | const | CIImage.h | CIImageAutoAdjustmentOptionKey::Enhance |
| kCIImageAutoAdjustFeatures | const | CIImage.h | CIImageAutoAdjustmentOptionKey::Features |
| kCIImageAutoAdjustLevel | const | CIImage.h | CIImageAutoAdjustmentOptionKey::Level |
| kCIImageAutoAdjustRedEye | const | CIImage.h | CIImageAutoAdjustmentOptionKey::RedEye |
| kCIImageAuxiliaryDepth | const | CIImage.h | CIImageOptionKey::AuxiliaryDepth |
| kCIImageAuxiliaryDisparity | const | CIImage.h | CIImageOptionKey::AuxiliaryDisparity |
| kCIImageAuxiliaryHDRGainMap | const | CIImage.h | CIImageOptionKey::AuxiliaryHdrGainMap |
| kCIImageAuxiliaryPortraitEffectsMatte | const | CIImage.h | CIImageOptionKey::AuxiliaryPortraitEffectsMatte |
| kCIImageAuxiliarySemanticSegmentationGlassesMatte | const | CIImage.h | CIImageOptionKey::AuxiliarySemanticSegmentationGlassesMatte |
| kCIImageAuxiliarySemanticSegmentationHairMatte | const | CIImage.h | CIImageOptionKey::AuxiliarySemanticSegmentationHairMatte |
| kCIImageAuxiliarySemanticSegmentationSkinMatte | const | CIImage.h | CIImageOptionKey::AuxiliarySemanticSegmentationSkinMatte |
| kCIImageAuxiliarySemanticSegmentationSkyMatte | const | CIImage.h | CIImageOptionKey::AuxiliarySemanticSegmentationSkyMatte |
| kCIImageAuxiliarySemanticSegmentationTeethMatte | const | CIImage.h | CIImageOptionKey::AuxiliarySemanticSegmentationTeethMatte |
| kCIImageCacheImmediately | const | CIImage.h | CIImageOptionKey::CacheImmediately |
| kCIImageColorSpace | const | CIImage.h | CIImageOptionKey::ColorSpace |
| kCIImageContentAverageLightLevel | const | CIImage.h | CIImageOptionKey::ContentAverageLightLevel |
| kCIImageContentHeadroom | const | CIImage.h | CIImageOptionKey::ContentHeadroom |
| kCIImageExpandToHDR | const | CIImage.h | CIImageOptionKey::ExpandToHdr |
| kCIImageNearestSampling | const | CIImage.h | CIImageOptionKey::NearestSampling |
| kCIImageProperties | const | CIImage.h | CIImageOptionKey::Properties |
| kCIImageToneMapHDRtoSDR | const | CIImage.h | CIImageOptionKey::ToneMapHdrToSdr |
| CIImageAccumulator | interface | CIImageAccumulator.h | CIImageAccumulator |
| CIRAWDecoderVersion6 | const | CIRAWFilter.h | CIRAWDecoderVersion::Version6 |
| CIRAWDecoderVersion6DNG | const | CIRAWFilter.h | CIRAWDecoderVersion::Version6Dng |
| CIRAWDecoderVersion7 | const | CIRAWFilter.h | CIRAWDecoderVersion::Version7 |
| CIRAWDecoderVersion7DNG | const | CIRAWFilter.h | CIRAWDecoderVersion::Version7Dng |
| CIRAWDecoderVersion8 | const | CIRAWFilter.h | CIRAWDecoderVersion::Version8 |
| CIRAWDecoderVersion8DNG | const | CIRAWFilter.h | CIRAWDecoderVersion::Version8Dng |
| CIRAWDecoderVersion9 | const | CIRAWFilter.h | CIRAWDecoderVersion::Version9 |
| CIRAWDecoderVersion9DNG | const | CIRAWFilter.h | CIRAWDecoderVersion::Version9Dng |
| CIRAWDecoderVersionNone | const | CIRAWFilter.h | CIRAWDecoderVersion::None |
| CIRAWFilter | interface | CIRAWFilter.h | CIRAWFilter |
| CIRenderDestination | interface | CIRenderDestination.h | CIRenderDestination |
| CIRenderInfo | interface | CIRenderDestination.h | CIRenderInfo |
| CIRenderTask | interface | CIRenderDestination.h | CIRenderTask |
| CIRenderDestinationAlphaMode | typedef | CIRenderDestination.h | CIRenderDestinationAlphaMode |

| kCIApplyOptionColorSpace | const | CIFilter.h | CIApplyOptionKey |
| kCIApplyOptionDefinition | const | CIFilter.h | CIApplyOptionKey |
| kCIApplyOptionExtent | const | CIFilter.h | CIApplyOptionKey |
| kCIApplyOptionUserInfo | const | CIFilter.h | CIApplyOptionKey |
| kCIAttributeClass | const | CIFilter.h | CIAttributeKey |
| kCIAttributeDefault | const | CIFilter.h | CIAttributeKey |
| kCIAttributeDescription | const | CIFilter.h | CIAttributeKey |
| kCIAttributeDisplayName | const | CIFilter.h | CIAttributeKey |
| kCIAttributeFilterAvailable_Mac | const | CIFilter.h | CIAttributeKey |
| kCIAttributeFilterAvailable_iOS | const | CIFilter.h | CIAttributeKey |
| kCIAttributeFilterName | const | CIFilter.h | CIAttributeKey |
| kCIAttributeIdentity | const | CIFilter.h | CIAttributeKey |
| kCIAttributeMax | const | CIFilter.h | CIAttributeKey |
| kCIAttributeMin | const | CIFilter.h | CIAttributeKey |
| kCIAttributeName | const | CIFilter.h | CIAttributeKey |
| kCIAttributeReferenceDocumentation | const | CIFilter.h | CIAttributeKey |
| kCIAttributeSliderMax | const | CIFilter.h | CIAttributeKey |
| kCIAttributeSliderMin | const | CIFilter.h | CIAttributeKey |
| kCIAttributeType | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeAngle | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeBoolean | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeColor | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeCount | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeDistance | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeGradient | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeImage | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeInteger | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeOffset | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeOpaqueColor | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypePosition | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypePosition3 | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeRectangle | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeScalar | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeTime | const | CIFilter.h | CIAttributeType |
| kCIAttributeTypeTransform | const | CIFilter.h | CIAttributeType |
| kCICategoryBlur | const | CIFilter.h | CIFilterCategory |
| kCICategoryColorAdjustment | const | CIFilter.h | CIFilterCategory |
| kCICategoryColorEffect | const | CIFilter.h | CIFilterCategory |
| kCICategoryCompositeOperation | const | CIFilter.h | CIFilterCategory |
| kCICategoryDistortionEffect | const | CIFilter.h | CIFilterCategory |
| kCICategoryFilterGenerator | const | CIFilter.h | CIFilterCategory |
| kCICategoryGeometryAdjustment | const | CIFilter.h | CIFilterCategory |
| kCICategoryGradient | const | CIFilter.h | CIFilterCategory |
| kCICategoryHalftoneEffect | const | CIFilter.h | CIFilterCategory |
| kCICategoryHighDynamicRange | const | CIFilter.h | CIFilterCategory |
| kCICategoryInterlaced | const | CIFilter.h | CIFilterCategory |
| kCICategoryNonSquarePixels | const | CIFilter.h | CIFilterCategory |
| kCICategoryReduction | const | CIFilter.h | CIFilterCategory |
| kCICategorySharpen | const | CIFilter.h | CIFilterCategory |
| kCICategoryStylize | const | CIFilter.h | CIFilterCategory |
| kCICategoryTileEffect | const | CIFilter.h | CIFilterCategory |
| kCICategoryTransition | const | CIFilter.h | CIFilterCategory |
| kCICategoryVideo | const | CIFilter.h | CIFilterCategory |
| kCIDynamicRangeConstrainedHigh | const | CIFilter.h | CIDynamicRange |
| kCIDynamicRangeHigh | const | CIFilter.h | CIDynamicRange |
| kCIDynamicRangeStandard | const | CIFilter.h | CIDynamicRange |
| kCIInputAmountKey | const | CIFilter.h | CIInputKey |
| kCIInputAngleKey | const | CIFilter.h | CIInputKey |
| kCIInputAspectRatioKey | const | CIFilter.h | CIInputKey |
| kCIInputBackgroundImageKey | const | CIFilter.h | CIInputKey |
| kCIInputBacksideImageKey | const | CIFilter.h | CIInputKey |
| kCIInputBiasKey | const | CIFilter.h | CIInputKey |
| kCIInputBiasVectorKey | const | CIFilter.h | CIInputKey |
| kCIInputBrightnessKey | const | CIFilter.h | CIInputKey |
| kCIInputCenterKey | const | CIFilter.h | CIInputKey |
| kCIInputColor0Key | const | CIFilter.h | CIInputKey |
| kCIInputColor1Key | const | CIFilter.h | CIInputKey |
| kCIInputColorKey | const | CIFilter.h | CIInputKey |
| kCIInputColorSpaceKey | const | CIFilter.h | CIInputKey |
| kCIInputContrastKey | const | CIFilter.h | CIInputKey |
| kCIInputCountKey | const | CIFilter.h | CIInputKey |
| kCIInputDepthImageKey | const | CIFilter.h | CIInputKey |
| kCIInputDisparityImageKey | const | CIFilter.h | CIInputKey |
| kCIInputEVKey | const | CIFilter.h | CIInputKey |
| kCIInputExtentKey | const | CIFilter.h | CIInputKey |
| kCIInputExtrapolateKey | const | CIFilter.h | CIInputKey |
| kCIInputGradientImageKey | const | CIFilter.h | CIInputKey |
| kCIInputImageKey | const | CIFilter.h | CIInputKey |
| kCIInputIntensityKey | const | CIFilter.h | CIInputKey |
| kCIInputMaskImageKey | const | CIFilter.h | CIInputKey |
| kCIInputMatteImageKey | const | CIFilter.h | CIInputKey |
| kCIInputPaletteImageKey | const | CIFilter.h | CIInputKey |
| kCIInputPerceptualKey | const | CIFilter.h | CIInputKey |
| kCIInputPoint0Key | const | CIFilter.h | CIInputKey |
| kCIInputPoint1Key | const | CIFilter.h | CIInputKey |
| kCIInputRadius0Key | const | CIFilter.h | CIInputKey |
| kCIInputRadius1Key | const | CIFilter.h | CIInputKey |
| kCIInputRadiusKey | const | CIFilter.h | CIInputKey |
| kCIInputRefractionKey | const | CIFilter.h | CIInputKey |
| kCIInputSaturationKey | const | CIFilter.h | CIInputKey |
| kCIInputScaleKey | const | CIFilter.h | CIInputKey |
| kCIInputShadingImageKey | const | CIFilter.h | CIInputKey |
| kCIInputSharpnessKey | const | CIFilter.h | CIInputKey |
| kCIInputTargetImageKey | const | CIFilter.h | CIInputKey |
| kCIInputThresholdKey | const | CIFilter.h | CIInputKey |
| kCIInputTimeKey | const | CIFilter.h | CIInputKey |
| kCIInputTransformKey | const | CIFilter.h | CIInputKey |
| kCIInputVersionKey | const | CIFilter.h | CIInputKey |
| kCIInputWeightsKey | const | CIFilter.h | CIInputKey |
| kCIInputWidthKey | const | CIFilter.h | CIInputKey |
| kCIOutputImageKey | const | CIFilter.h | CIOutputKey |
| kCIUIParameterSet | const | CIFilter.h | CIUIParameterSetKey |
| kCIUISetAdvanced | const | CIFilter.h | CIUIParameterSet |
| kCIUISetBasic | const | CIFilter.h | CIUIParameterSet |
| kCIUISetDevelopment | const | CIFilter.h | CIUIParameterSet |
| kCIUISetIntermediate | const | CIFilter.h | CIUIParameterSet |
| CICompositeOperation | protocol | CIFilterBuiltins.h | filters::composite_operation |
| CIConvertLab | protocol | CIFilterBuiltins.h | filters::convert_lab_to_rgb / filters::convert_rgb_to_lab |
| CICoreMLModel | protocol | CIFilterBuiltins.h | filters::core_ml_model |
| CIEdgePreserveUpsample | protocol | CIFilterBuiltins.h | filters::edge_preserve_upsample |
| CIFourCoordinateGeometryFilter | protocol | CIFilterBuiltins.h | filters::four_coordinate_geometry_filter |
| CIHistogramDisplay | protocol | CIFilterBuiltins.h | filters::histogram_display |
| CIMedian | protocol | CIFilterBuiltins.h | filters::median |
| CISaliencyMap | protocol | CIFilterBuiltins.h | filters::saliency_map |
| CITransitionFilter | protocol | CIFilterBuiltins.h | filters::transition_filter |
| CIFilterConstructor | protocol | CIFilterConstructor.h | CIFilterConstructor |
| kCIFilterGeneratorExportedKey | const | CIFilterGenerator.h | CIFilterGeneratorExportedKey |
| kCIFilterGeneratorExportedKeyName | const | CIFilterGenerator.h | CIFilterGeneratorExportedKey |
| kCIFilterGeneratorExportedKeyTargetObject | const | CIFilterGenerator.h | CIFilterGeneratorExportedKey |
| CIImageProcessorInput | protocol | CIImageProcessor.h | CIImageProcessorInput |
| CIImageProcessorOutput | protocol | CIImageProcessor.h | CIImageProcessorOutput |
| kCIImageProviderTileSize | const | CIImageProvider.h | CIImageProviderOptionKey |
| kCIImageProviderUserInfo | const | CIImageProvider.h | CIImageProviderOptionKey |
| CIKernel | interface | CIKernel.h | CIKernel |
| CIPlugIn | interface | CIPlugIn.h | CIPlugIn |
| CIPlugInRegistration | protocol | CIPlugInInterface.h | CIPlugInRegistration |
| kCISamplerColorSpace | const | CISampler.h | CISamplerOptionKey + CISamplerOptions::color_space |

## 🔴 GAPS
No remaining non-exempt public-symbol gaps.

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
