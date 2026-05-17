import CoreGraphics
import CoreImage
import Foundation

public func ci_color_space(from code: Int32) -> CGColorSpace? {
    switch code {
    case 0:
        return nil
    case 1:
        return CGColorSpace(name: CGColorSpace.sRGB)
    case 2:
        return CGColorSpace(name: CGColorSpace.extendedSRGB)
    case 3:
        return CGColorSpace(name: CGColorSpace.displayP3)
    case 4:
        return CGColorSpace(name: CGColorSpace.genericGrayGamma2_2)
    default:
        return nil
    }
}

public func ci_image_format(from code: Int32) -> CIFormat? {
    CIFormat(rawValue: code)
}

@inline(__always)
private func ci_named_string(_ code: Int32, values: [String]) -> UnsafeMutablePointer<CChar>? {
    let index = Int(code)
    guard values.indices.contains(index) else {
        return ci_string("")
    }
    return ci_string(values[index])
}

@_cdecl("ci_image_format_value")
public func ci_image_format_value(_ code: Int32) -> Int32 {
    switch code {
    case 0: return 265
    case 1: return 266
    case 2: return 264
    case 3: return 268
    case 4: return 267
    case 5: return 2056
    case 6: return 1800
    case 7: return 2312
    case 8: return 1804
    case 9: return 2060
    case 10: return 2316
    case 11: return 775
    case 12: return 257
    case 13: return 1793
    case 14: return 2049
    case 15: return 2305
    case 16: return 261
    case 17: return 1797
    case 18: return 2053
    case 19: return 2309
    case 20: return 262
    case 21: return 1798
    case 22: return 2054
    case 23: return 2310
    case 24: return 259
    case 25: return 1795
    case 26: return 2051
    case 27: return 2307
    case 28: return 260
    case 29: return 1796
    case 30: return 2052
    case 31: return 2308
    default: return 0
    }
}

@_cdecl("ci_context_option_name")
public func ci_context_option_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    let value: String
    switch code {
    case 0: value = "kCIContextOutputColorSpace"
    case 1: value = "kCIContextWorkingColorSpace"
    case 2: value = "kCIContextWorkingFormat"
    case 3: value = "kCIContextHighQualityDownsample"
    case 4: value = "kCIContextOutputPremultiplied"
    case 5: value = "kCIContextCacheIntermediates"
    case 6: value = "kCIContextUseSoftwareRenderer"
    case 7: value = "kCIContextPriorityRequestLow"
    case 8: value = "kCIContextAllowLowPower"
    case 9: value = "kCIContextName"
    case 10: value = "kCIContextCVMetalTextureCache"
    case 11: value = "kCIContextMemoryLimit"
    default: value = ""
    }
    return ci_string(value)
}

@_cdecl("ci_image_option_name")
public func ci_image_option_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    let value: String
    switch code {
    case 0: value = "kCIImageColorSpace"
    case 1: value = "kCIImageApplyCleanAperture"
    case 2: value = "kCIImageToneMapHDRtoSDR"
    case 3: value = "kCIImageExpandToHDR"
    case 4: value = "kCIImageContentHeadroom"
    case 5: value = "kCIImageContentAverageLightLevel"
    case 6: value = "kCIImageNearestSampling"
    case 7: value = "kCIImageCacheImmediately"
    case 8: value = "kCIImageProperties"
    case 9: value = "kCIImageApplyOrientationProperty"
    case 10: value = "kCIImageAuxiliaryDepth"
    case 11: value = "kCIImageAuxiliaryDisparity"
    case 12: value = "kCIImageAuxiliaryPortraitEffectsMatte"
    case 13: value = "kCIImageAuxiliarySemanticSegmentationSkinMatte"
    case 14: value = "kCIImageAuxiliarySemanticSegmentationHairMatte"
    case 15: value = "kCIImageAuxiliarySemanticSegmentationTeethMatte"
    case 16: value = "kCIImageAuxiliarySemanticSegmentationGlassesMatte"
    case 17: value = "kCIImageAuxiliarySemanticSegmentationSkyMatte"
    case 18: value = "kCIImageAuxiliaryHDRGainMap"
    default: value = ""
    }
    return ci_string(value)
}

@_cdecl("ci_image_auto_adjust_option_name")
public func ci_image_auto_adjust_option_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    let value: String
    switch code {
    case 0: value = "kCIImageAutoAdjustEnhance"
    case 1: value = "kCIImageAutoAdjustRedEye"
    case 2: value = "kCIImageAutoAdjustFeatures"
    case 3: value = "kCIImageAutoAdjustCrop"
    case 4: value = "kCIImageAutoAdjustLevel"
    default: value = ""
    }
    return ci_string(value)
}

@_cdecl("ci_image_representation_option_name")
public func ci_image_representation_option_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    let value: String
    switch code {
    case 0: value = "kCIImageRepresentationAVDepthData"
    case 1: value = "kCIImageRepresentationDepthImage"
    case 2: value = "kCIImageRepresentationDisparityImage"
    case 3: value = "kCIImageRepresentationAVPortraitEffectsMatte"
    case 4: value = "kCIImageRepresentationPortraitEffectsMatteImage"
    case 5: value = "kCIImageRepresentationAVSemanticSegmentationMattes"
    case 6: value = "kCIImageRepresentationSemanticSegmentationSkinMatteImage"
    case 7: value = "kCIImageRepresentationSemanticSegmentationHairMatteImage"
    case 8: value = "kCIImageRepresentationSemanticSegmentationTeethMatteImage"
    case 9: value = "kCIImageRepresentationSemanticSegmentationGlassesMatteImage"
    case 10: value = "kCIImageRepresentationSemanticSegmentationSkyMatteImage"
    case 11: value = "kCIImageRepresentationHDRImage"
    case 12: value = "kCIImageRepresentationHDRGainMapImage"
    case 13: value = "kCIImageRepresentationHDRGainMapAsRGB"
    default: value = ""
    }
    return ci_string(value)
}

@_cdecl("ci_apply_option_name")
public func ci_apply_option_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCIApplyOptionExtent",
        "kCIApplyOptionDefinition",
        "kCIApplyOptionUserInfo",
        "kCIApplyOptionColorSpace",
    ])
}

@_cdecl("ci_attribute_key_name")
public func ci_attribute_key_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCIAttributeFilterName",
        "kCIAttributeFilterDisplayName",
        "kCIAttributeDescription",
        "kCIAttributeFilterAvailable_Mac",
        "kCIAttributeFilterAvailable_iOS",
        "kCIAttributeReferenceDocumentation",
        "kCIAttributeFilterCategories",
        "kCIAttributeClass",
        "kCIAttributeType",
        "kCIAttributeMin",
        "kCIAttributeMax",
        "kCIAttributeSliderMin",
        "kCIAttributeSliderMax",
        "kCIAttributeDefault",
        "kCIAttributeIdentity",
        "kCIAttributeName",
        "kCIAttributeDisplayName",
    ])
}

@_cdecl("ci_attribute_type_name")
public func ci_attribute_type_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCIAttributeTypeTime",
        "kCIAttributeTypeScalar",
        "kCIAttributeTypeDistance",
        "kCIAttributeTypeAngle",
        "kCIAttributeTypeBoolean",
        "kCIAttributeTypeInteger",
        "kCIAttributeTypeCount",
        "kCIAttributeTypePosition",
        "kCIAttributeTypeOffset",
        "kCIAttributeTypePosition3",
        "kCIAttributeTypeRectangle",
        "kCIAttributeTypeOpaqueColor",
        "kCIAttributeTypeColor",
        "kCIAttributeTypeGradient",
        "kCIAttributeTypeImage",
        "kCIAttributeTypeTransform",
    ])
}

@_cdecl("ci_filter_category_name")
public func ci_filter_category_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCICategoryDistortionEffect",
        "kCICategoryGeometryAdjustment",
        "kCICategoryCompositeOperation",
        "kCICategoryHalftoneEffect",
        "kCICategoryColorAdjustment",
        "kCICategoryColorEffect",
        "kCICategoryTransition",
        "kCICategoryTileEffect",
        "kCICategoryGenerator",
        "kCICategoryReduction",
        "kCICategoryGradient",
        "kCICategoryStylize",
        "kCICategorySharpen",
        "kCICategoryBlur",
        "kCICategoryVideo",
        "kCICategoryStillImage",
        "kCICategoryInterlaced",
        "kCICategoryNonSquarePixels",
        "kCICategoryHighDynamicRange",
        "kCICategoryBuiltIn",
        "kCICategoryFilterGenerator",
    ])
}

@_cdecl("ci_filter_category_value")
public func ci_filter_category_value(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "CICategoryDistortionEffect",
        "CICategoryGeometryAdjustment",
        "CICategoryCompositeOperation",
        "CICategoryHalftoneEffect",
        "CICategoryColorAdjustment",
        "CICategoryColorEffect",
        "CICategoryTransition",
        "CICategoryTileEffect",
        "CICategoryGenerator",
        "CICategoryReduction",
        "CICategoryGradient",
        "CICategoryStylize",
        "CICategorySharpen",
        "CICategoryBlur",
        "CICategoryVideo",
        "CICategoryStillImage",
        "CICategoryInterlaced",
        "CICategoryNonSquarePixels",
        "CICategoryHighDynamicRange",
        "CICategoryBuiltIn",
        "CICategoryFilterGenerator",
    ])
}

@_cdecl("ci_dynamic_range_name")
public func ci_dynamic_range_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCIDynamicRangeStandard",
        "kCIDynamicRangeHigh",
        "kCIDynamicRangeConstrainedHigh",
    ])
}

@_cdecl("ci_input_key_name")
public func ci_input_key_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCIInputBackgroundImageKey",
        "kCIInputImageKey",
        "kCIInputDepthImageKey",
        "kCIInputDisparityImageKey",
        "kCIInputAmountKey",
        "kCIInputCountKey",
        "kCIInputThresholdKey",
        "kCIInputTimeKey",
        "kCIInputTransformKey",
        "kCIInputScaleKey",
        "kCIInputAspectRatioKey",
        "kCIInputCenterKey",
        "kCIInputRadiusKey",
        "kCIInputRadius0Key",
        "kCIInputRadius1Key",
        "kCIInputAngleKey",
        "kCIInputRefractionKey",
        "kCIInputWidthKey",
        "kCIInputSharpnessKey",
        "kCIInputIntensityKey",
        "kCIInputEVKey",
        "kCIInputSaturationKey",
        "kCIInputColorKey",
        "kCIInputColor0Key",
        "kCIInputColor1Key",
        "kCIInputColorSpaceKey",
        "kCIInputBrightnessKey",
        "kCIInputContrastKey",
        "kCIInputExtrapolateKey",
        "kCIInputPerceptualKey",
        "kCIInputBiasKey",
        "kCIInputBiasVectorKey",
        "kCIInputGradientImageKey",
        "kCIInputMaskImageKey",
        "kCIInputMatteImageKey",
        "kCIInputBacksideImageKey",
        "kCIInputShadingImageKey",
        "kCIInputTargetImageKey",
        "kCIInputPaletteImageKey",
        "kCIInputExtentKey",
        "kCIInputVersionKey",
        "kCIInputPoint0Key",
        "kCIInputPoint1Key",
        "kCIInputWeightsKey",
    ])
}

@_cdecl("ci_input_key_value")
public func ci_input_key_value(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "inputBackgroundImage",
        "inputImage",
        "inputDepthImage",
        "inputDisparityImage",
        "inputAmount",
        "inputCount",
        "inputThreshold",
        "inputTime",
        "inputTransform",
        "inputScale",
        "inputAspectRatio",
        "inputCenter",
        "inputRadius",
        "inputRadius0",
        "inputRadius1",
        "inputAngle",
        "inputRefraction",
        "inputWidth",
        "inputSharpness",
        "inputIntensity",
        "inputEV",
        "inputSaturation",
        "inputColor",
        "inputColor0",
        "inputColor1",
        "inputColorSpace",
        "inputBrightness",
        "inputContrast",
        "inputExtrapolate",
        "inputPerceptual",
        "inputBias",
        "inputBiasVector",
        "inputGradientImage",
        "inputMaskImage",
        "inputMatteImage",
        "inputBacksideImage",
        "inputShadingImage",
        "inputTargetImage",
        "inputPaletteImage",
        "inputExtent",
        "inputVersion",
        "inputPoint0",
        "inputPoint1",
        "inputWeights",
    ])
}

@_cdecl("ci_output_key_name")
public func ci_output_key_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: ["kCIOutputImageKey"])
}

@_cdecl("ci_output_key_value")
public func ci_output_key_value(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: ["outputImage"])
}

@_cdecl("ci_ui_parameter_set_key_name")
public func ci_ui_parameter_set_key_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: ["kCIUIParameterSet"])
}

@_cdecl("ci_ui_parameter_set_name")
public func ci_ui_parameter_set_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCIUISetBasic",
        "kCIUISetIntermediate",
        "kCIUISetAdvanced",
        "kCIUISetDevelopment",
    ])
}

@_cdecl("ci_filter_generator_exported_key_name")
public func ci_filter_generator_exported_key_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCIFilterGeneratorExportedKey",
        "kCIFilterGeneratorExportedKeyName",
        "kCIFilterGeneratorExportedKeyTargetObject",
    ])
}

@_cdecl("ci_image_provider_option_name")
public func ci_image_provider_option_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCIImageProviderTileSize",
        "kCIImageProviderUserInfo",
    ])
}

@_cdecl("ci_sampler_option_name")
public func ci_sampler_option_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    ci_named_string(code, values: [
        "kCISamplerAffineMatrix",
        "kCISamplerFilterMode",
        "kCISamplerWrapMode",
        "kCISamplerColorSpace",
    ])
}

@_cdecl("ci_raw_decoder_version_name")
public func ci_raw_decoder_version_name(_ code: Int32) -> UnsafeMutablePointer<CChar>? {
    let value: String
    switch code {
    case 0: value = "None"
    case 1: value = "9"
    case 2: value = "9.dng"
    case 3: value = "8"
    case 4: value = "8.dng"
    case 5: value = "7"
    case 6: value = "7.dng"
    case 7: value = "6"
    case 8: value = "6.dng"
    default: value = ""
    }
    return ci_string(value)
}
