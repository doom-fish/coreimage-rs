#![allow(clippy::too_many_arguments, clippy::too_many_lines)]

use apple_cf::cg::CGRect;

use crate::{CIColor, CIFilter, CIImage, CIVector};

fn filter_output(name: &str, configure: impl FnOnce(&mut CIFilter)) -> Option<CIImage> {
    let mut filter = CIFilter::new(name)?;
    configure(&mut filter);
    filter.output_image()
}

fn image_filter_output(
    name: &str,
    input: &CIImage,
    configure: impl FnOnce(&mut CIFilter),
) -> Option<CIImage> {
    filter_output(name, |filter| {
        filter.set_input_image(input);
        configure(filter);
    })
}

/// Calls the `CoreImage` framework counterpart for `gaussian_blur`.
pub fn gaussian_blur(input: &CIImage, radius: f64) -> Option<CIImage> {
    image_filter_output("CIGaussianBlur", input, |filter| {
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `box_blur`.
pub fn box_blur(input: &CIImage, radius: f64) -> Option<CIImage> {
    image_filter_output("CIBoxBlur", input, |filter| {
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `disc_blur`.
pub fn disc_blur(input: &CIImage, radius: f64) -> Option<CIImage> {
    image_filter_output("CIDiscBlur", input, |filter| {
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `motion_blur`.
pub fn motion_blur(input: &CIImage, radius: f64, angle: f64) -> Option<CIImage> {
    image_filter_output("CIMotionBlur", input, |filter| {
        filter.set_input_number("inputRadius", radius);
        filter.set_input_number("inputAngle", angle);
    })
}

/// Calls the `CoreImage` framework counterpart for `zoom_blur`.
pub fn zoom_blur(input: &CIImage, center: (f64, f64), amount: f64) -> Option<CIImage> {
    let center = CIVector::new(center.0, center.1);
    image_filter_output("CIZoomBlur", input, |filter| {
        filter.set_input_vector("inputCenter", &center);
        filter.set_input_number("inputAmount", amount);
    })
}

/// Calls the `CoreImage` framework counterpart for `sharpen_luminance`.
pub fn sharpen_luminance(input: &CIImage, sharpness: f64, radius: f64) -> Option<CIImage> {
    image_filter_output("CISharpenLuminance", input, |filter| {
        filter.set_input_number("inputSharpness", sharpness);
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `unsharp_mask`.
pub fn unsharp_mask(input: &CIImage, intensity: f64, radius: f64) -> Option<CIImage> {
    image_filter_output("CIUnsharpMask", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `color_controls`.
pub fn color_controls(
    input: &CIImage,
    brightness: f64,
    contrast: f64,
    saturation: f64,
) -> Option<CIImage> {
    image_filter_output("CIColorControls", input, |filter| {
        filter.set_input_number("inputBrightness", brightness);
        filter.set_input_number("inputContrast", contrast);
        filter.set_input_number("inputSaturation", saturation);
    })
}

/// Calls the `CoreImage` framework counterpart for `exposure_adjust`.
pub fn exposure_adjust(input: &CIImage, ev: f64) -> Option<CIImage> {
    image_filter_output("CIExposureAdjust", input, |filter| {
        filter.set_input_number("inputEV", ev);
    })
}

/// Calls the `CoreImage` framework counterpart for `gamma_adjust`.
pub fn gamma_adjust(input: &CIImage, power: f64) -> Option<CIImage> {
    image_filter_output("CIGammaAdjust", input, |filter| {
        filter.set_input_number("inputPower", power);
    })
}

/// Calls the `CoreImage` framework counterpart for `hue_adjust`.
pub fn hue_adjust(input: &CIImage, angle: f64) -> Option<CIImage> {
    image_filter_output("CIHueAdjust", input, |filter| {
        filter.set_input_number("inputAngle", angle);
    })
}

/// Calls the `CoreImage` framework counterpart for `vibrance`.
pub fn vibrance(input: &CIImage, amount: f64) -> Option<CIImage> {
    image_filter_output("CIVibrance", input, |filter| {
        filter.set_input_number("inputAmount", amount);
    })
}

/// Calls the `CoreImage` framework counterpart for `temperature_and_tint`.
pub fn temperature_and_tint(
    input: &CIImage,
    neutral: (f64, f64),
    target_neutral: (f64, f64),
) -> Option<CIImage> {
    let neutral = CIVector::new(neutral.0, neutral.1);
    let target = CIVector::new(target_neutral.0, target_neutral.1);
    image_filter_output("CITemperatureAndTint", input, |filter| {
        filter.set_input_vector("inputNeutral", &neutral);
        filter.set_input_vector("inputTargetNeutral", &target);
    })
}

/// Calls the `CoreImage` framework counterpart for `white_point_adjust`.
pub fn white_point_adjust(input: &CIImage, color: &CIColor) -> Option<CIImage> {
    image_filter_output("CIWhitePointAdjust", input, |filter| {
        filter.set_input_color("inputColor", color);
    })
}

/// Calls the `CoreImage` framework counterpart for `sepia_tone`.
pub fn sepia_tone(input: &CIImage, intensity: f64) -> Option<CIImage> {
    image_filter_output("CISepiaTone", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
    })
}

/// Calls the `CoreImage` framework counterpart for `color_invert`.
pub fn color_invert(input: &CIImage) -> Option<CIImage> {
    image_filter_output("CIColorInvert", input, |_| {})
}

/// Calls the `CoreImage` framework counterpart for `color_monochrome`.
pub fn color_monochrome(input: &CIImage, color: &CIColor, intensity: f64) -> Option<CIImage> {
    image_filter_output("CIColorMonochrome", input, |filter| {
        filter.set_input_color("inputColor", color);
        filter.set_input_number("inputIntensity", intensity);
    })
}

/// Calls the `CoreImage` framework counterpart for `false_color`.
pub fn false_color(input: &CIImage, color0: &CIColor, color1: &CIColor) -> Option<CIImage> {
    image_filter_output("CIFalseColor", input, |filter| {
        filter.set_input_color("inputColor0", color0);
        filter.set_input_color("inputColor1", color1);
    })
}

/// Calls the `CoreImage` framework counterpart for `vignette`.
pub fn vignette(input: &CIImage, intensity: f64, radius: f64) -> Option<CIImage> {
    image_filter_output("CIVignette", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `vignette_effect`.
pub fn vignette_effect(
    input: &CIImage,
    center: (f64, f64),
    intensity: f64,
    radius: f64,
) -> Option<CIImage> {
    let center = CIVector::new(center.0, center.1);
    image_filter_output("CIVignetteEffect", input, |filter| {
        filter.set_input_vector("inputCenter", &center);
        filter.set_input_number("inputIntensity", intensity);
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `edges`.
pub fn edges(input: &CIImage, intensity: f64) -> Option<CIImage> {
    image_filter_output("CIEdges", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
    })
}

/// Calls the `CoreImage` framework counterpart for `edge_work`.
pub fn edge_work(input: &CIImage, radius: f64) -> Option<CIImage> {
    image_filter_output("CIEdgeWork", input, |filter| {
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `bloom`.
pub fn bloom(input: &CIImage, intensity: f64, radius: f64) -> Option<CIImage> {
    image_filter_output("CIBloom", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `pixellate`.
pub fn pixellate(input: &CIImage, center: (f64, f64), scale: f64) -> Option<CIImage> {
    let center = CIVector::new(center.0, center.1);
    image_filter_output("CIPixellate", input, |filter| {
        filter.set_input_vector("inputCenter", &center);
        filter.set_input_number("inputScale", scale);
    })
}

/// Calls the `CoreImage` framework counterpart for `comic_effect`.
pub fn comic_effect(input: &CIImage) -> Option<CIImage> {
    image_filter_output("CIComicEffect", input, |_| {})
}

/// Calls the `CoreImage` framework counterpart for `crystallize`.
pub fn crystallize(input: &CIImage, center: (f64, f64), radius: f64) -> Option<CIImage> {
    let center = CIVector::new(center.0, center.1);
    image_filter_output("CICrystallize", input, |filter| {
        filter.set_input_vector("inputCenter", &center);
        filter.set_input_number("inputRadius", radius);
    })
}

/// Calls the `CoreImage` framework counterpart for `straighten`.
pub fn straighten(input: &CIImage, angle: f64) -> Option<CIImage> {
    image_filter_output("CIStraightenFilter", input, |filter| {
        filter.set_input_number("inputAngle", angle);
    })
}

/// Calls the `CoreImage` framework counterpart for `lanczos_scale_transform`.
pub fn lanczos_scale_transform(input: &CIImage, scale: f64, aspect_ratio: f64) -> Option<CIImage> {
    image_filter_output("CILanczosScaleTransform", input, |filter| {
        filter.set_input_number("inputScale", scale);
        filter.set_input_number("inputAspectRatio", aspect_ratio);
    })
}

/// Calls the `CoreImage` framework counterpart for `perspective_correction`.
pub fn perspective_correction(
    input: &CIImage,
    top_left: (f64, f64),
    top_right: (f64, f64),
    bottom_left: (f64, f64),
    bottom_right: (f64, f64),
) -> Option<CIImage> {
    let top_left = CIVector::new(top_left.0, top_left.1);
    let top_right = CIVector::new(top_right.0, top_right.1);
    let bottom_left = CIVector::new(bottom_left.0, bottom_left.1);
    let bottom_right = CIVector::new(bottom_right.0, bottom_right.1);
    image_filter_output("CIPerspectiveCorrection", input, |filter| {
        filter.set_input_vector("inputTopLeft", &top_left);
        filter.set_input_vector("inputTopRight", &top_right);
        filter.set_input_vector("inputBottomLeft", &bottom_left);
        filter.set_input_vector("inputBottomRight", &bottom_right);
    })
}

/// Calls the `CoreImage` framework counterpart for `perspective_transform`.
pub fn perspective_transform(
    input: &CIImage,
    top_left: (f64, f64),
    top_right: (f64, f64),
    bottom_left: (f64, f64),
    bottom_right: (f64, f64),
) -> Option<CIImage> {
    let top_left = CIVector::new(top_left.0, top_left.1);
    let top_right = CIVector::new(top_right.0, top_right.1);
    let bottom_left = CIVector::new(bottom_left.0, bottom_left.1);
    let bottom_right = CIVector::new(bottom_right.0, bottom_right.1);
    image_filter_output("CIPerspectiveTransform", input, |filter| {
        filter.set_input_vector("inputTopLeft", &top_left);
        filter.set_input_vector("inputTopRight", &top_right);
        filter.set_input_vector("inputBottomLeft", &bottom_left);
        filter.set_input_vector("inputBottomRight", &bottom_right);
    })
}

/// Calls the `CoreImage` framework counterpart for `crop`.
pub fn crop(input: &CIImage, rect: CGRect) -> Option<CIImage> {
    Some(input.cropped_to(rect))
}

/// Calls the `CoreImage` framework counterpart for `source_over_compositing`.
pub fn source_over_compositing(foreground: &CIImage, background: &CIImage) -> Option<CIImage> {
    filter_output("CISourceOverCompositing", |filter| {
        filter.set_input_image(foreground);
        filter.set_input_image_for_key("inputBackgroundImage", background);
    })
}

/// Calls the `CoreImage` framework counterpart for `multiply_compositing`.
pub fn multiply_compositing(foreground: &CIImage, background: &CIImage) -> Option<CIImage> {
    filter_output("CIMultiplyCompositing", |filter| {
        filter.set_input_image(foreground);
        filter.set_input_image_for_key("inputBackgroundImage", background);
    })
}

/// Calls the `CoreImage` framework counterpart for `blend_with_mask`.
pub fn blend_with_mask(
    foreground: &CIImage,
    background: &CIImage,
    mask: &CIImage,
) -> Option<CIImage> {
    filter_output("CIBlendWithMask", |filter| {
        filter.set_input_image(foreground);
        filter.set_input_image_for_key("inputBackgroundImage", background);
        filter.set_input_image_for_key("inputMaskImage", mask);
    })
}

/// Calls the `CoreImage` framework counterpart for `constant_color`.
pub fn constant_color(width: usize, height: usize, color: &CIColor) -> Option<CIImage> {
    let image = filter_output("CIConstantColorGenerator", |filter| {
        filter.set_input_color("inputColor", color);
    })?;
    Some(image.cropped_to(CGRect::new(0.0, 0.0, width as f64, height as f64)))
}

/// Calls the `CoreImage` framework counterpart for `checkerboard`.
pub fn checkerboard(
    width: usize,
    height: usize,
    center: (f64, f64),
    color0: &CIColor,
    color1: &CIColor,
    square_width: f64,
    sharpness: f64,
) -> Option<CIImage> {
    let center = CIVector::new(center.0, center.1);
    let image = filter_output("CICheckerboardGenerator", |filter| {
        filter.set_input_vector("inputCenter", &center);
        filter.set_input_color("inputColor0", color0);
        filter.set_input_color("inputColor1", color1);
        filter.set_input_number("inputWidth", square_width);
        filter.set_input_number("inputSharpness", sharpness);
    })?;
    Some(image.cropped_to(CGRect::new(0.0, 0.0, width as f64, height as f64)))
}

/// Calls the `CoreImage` framework counterpart for `linear_gradient`.
pub fn linear_gradient(
    width: usize,
    height: usize,
    point0: (f64, f64),
    point1: (f64, f64),
    color0: &CIColor,
    color1: &CIColor,
) -> Option<CIImage> {
    let point0 = CIVector::new(point0.0, point0.1);
    let point1 = CIVector::new(point1.0, point1.1);
    let image = filter_output("CILinearGradient", |filter| {
        filter.set_input_vector("inputPoint0", &point0);
        filter.set_input_vector("inputPoint1", &point1);
        filter.set_input_color("inputColor0", color0);
        filter.set_input_color("inputColor1", color1);
    })?;
    Some(image.cropped_to(CGRect::new(0.0, 0.0, width as f64, height as f64)))
}

/// Calls the `CoreImage` framework counterpart for `radial_gradient`.
pub fn radial_gradient(
    width: usize,
    height: usize,
    center: (f64, f64),
    radius0: f64,
    radius1: f64,
    color0: &CIColor,
    color1: &CIColor,
) -> Option<CIImage> {
    let center = CIVector::new(center.0, center.1);
    let image = filter_output("CIRadialGradient", |filter| {
        filter.set_input_vector("inputCenter", &center);
        filter.set_input_number("inputRadius0", radius0);
        filter.set_input_number("inputRadius1", radius1);
        filter.set_input_color("inputColor0", color0);
        filter.set_input_color("inputColor1", color1);
    })?;
    Some(image.cropped_to(CGRect::new(0.0, 0.0, width as f64, height as f64)))
}

fn builtin_filter(name: &str) -> Option<CIFilter> {
    CIFilter::new(name)
}

/// Mirrors the `CoreImage` framework counterpart for `CICompositeOperationKind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CICompositeOperationKind {
/// Mirrors the `CoreImage` framework case `Addition`.
    Addition,
/// Mirrors the `CoreImage` framework case `ColorBlendMode`.
    ColorBlendMode,
/// Mirrors the `CoreImage` framework case `ColorBurnBlendMode`.
    ColorBurnBlendMode,
/// Mirrors the `CoreImage` framework case `ColorDodgeBlendMode`.
    ColorDodgeBlendMode,
/// Mirrors the `CoreImage` framework case `DarkenBlendMode`.
    DarkenBlendMode,
/// Mirrors the `CoreImage` framework case `DifferenceBlendMode`.
    DifferenceBlendMode,
/// Mirrors the `CoreImage` framework case `DivideBlendMode`.
    DivideBlendMode,
/// Mirrors the `CoreImage` framework case `ExclusionBlendMode`.
    ExclusionBlendMode,
/// Mirrors the `CoreImage` framework case `HardLightBlendMode`.
    HardLightBlendMode,
/// Mirrors the `CoreImage` framework case `HueBlendMode`.
    HueBlendMode,
/// Mirrors the `CoreImage` framework case `LightenBlendMode`.
    LightenBlendMode,
/// Mirrors the `CoreImage` framework case `LinearBurnBlendMode`.
    LinearBurnBlendMode,
/// Mirrors the `CoreImage` framework case `LinearDodgeBlendMode`.
    LinearDodgeBlendMode,
/// Mirrors the `CoreImage` framework case `LinearLightBlendMode`.
    LinearLightBlendMode,
/// Mirrors the `CoreImage` framework case `LuminosityBlendMode`.
    LuminosityBlendMode,
/// Mirrors the `CoreImage` framework case `MaximumCompositing`.
    MaximumCompositing,
/// Mirrors the `CoreImage` framework case `MinimumCompositing`.
    MinimumCompositing,
/// Mirrors the `CoreImage` framework case `MultiplyBlendMode`.
    MultiplyBlendMode,
/// Mirrors the `CoreImage` framework case `MultiplyCompositing`.
    MultiplyCompositing,
/// Mirrors the `CoreImage` framework case `OverlayBlendMode`.
    OverlayBlendMode,
/// Mirrors the `CoreImage` framework case `PinLightBlendMode`.
    PinLightBlendMode,
/// Mirrors the `CoreImage` framework case `SaturationBlendMode`.
    SaturationBlendMode,
/// Mirrors the `CoreImage` framework case `ScreenBlendMode`.
    ScreenBlendMode,
/// Mirrors the `CoreImage` framework case `SoftLightBlendMode`.
    SoftLightBlendMode,
/// Mirrors the `CoreImage` framework case `SourceAtopCompositing`.
    SourceAtopCompositing,
/// Mirrors the `CoreImage` framework case `SourceInCompositing`.
    SourceInCompositing,
/// Mirrors the `CoreImage` framework case `SourceOutCompositing`.
    SourceOutCompositing,
/// Mirrors the `CoreImage` framework case `SourceOverCompositing`.
    SourceOverCompositing,
/// Mirrors the `CoreImage` framework case `SubtractBlendMode`.
    SubtractBlendMode,
/// Mirrors the `CoreImage` framework case `VividLightBlendMode`.
    VividLightBlendMode,
}

/// Calls the `CoreImage` framework counterpart for `composite_operation`.
pub fn composite_operation(kind: CICompositeOperationKind) -> Option<CIFilter> {
    builtin_filter(match kind {
        CICompositeOperationKind::Addition => "CIAdditionCompositing",
        CICompositeOperationKind::ColorBlendMode => "CIColorBlendMode",
        CICompositeOperationKind::ColorBurnBlendMode => "CIColorBurnBlendMode",
        CICompositeOperationKind::ColorDodgeBlendMode => "CIColorDodgeBlendMode",
        CICompositeOperationKind::DarkenBlendMode => "CIDarkenBlendMode",
        CICompositeOperationKind::DifferenceBlendMode => "CIDifferenceBlendMode",
        CICompositeOperationKind::DivideBlendMode => "CIDivideBlendMode",
        CICompositeOperationKind::ExclusionBlendMode => "CIExclusionBlendMode",
        CICompositeOperationKind::HardLightBlendMode => "CIHardLightBlendMode",
        CICompositeOperationKind::HueBlendMode => "CIHueBlendMode",
        CICompositeOperationKind::LightenBlendMode => "CILightenBlendMode",
        CICompositeOperationKind::LinearBurnBlendMode => "CILinearBurnBlendMode",
        CICompositeOperationKind::LinearDodgeBlendMode => "CILinearDodgeBlendMode",
        CICompositeOperationKind::LinearLightBlendMode => "CILinearLightBlendMode",
        CICompositeOperationKind::LuminosityBlendMode => "CILuminosityBlendMode",
        CICompositeOperationKind::MaximumCompositing => "CIMaximumCompositing",
        CICompositeOperationKind::MinimumCompositing => "CIMinimumCompositing",
        CICompositeOperationKind::MultiplyBlendMode => "CIMultiplyBlendMode",
        CICompositeOperationKind::MultiplyCompositing => "CIMultiplyCompositing",
        CICompositeOperationKind::OverlayBlendMode => "CIOverlayBlendMode",
        CICompositeOperationKind::PinLightBlendMode => "CIPinLightBlendMode",
        CICompositeOperationKind::SaturationBlendMode => "CISaturationBlendMode",
        CICompositeOperationKind::ScreenBlendMode => "CIScreenBlendMode",
        CICompositeOperationKind::SoftLightBlendMode => "CISoftLightBlendMode",
        CICompositeOperationKind::SourceAtopCompositing => "CISourceAtopCompositing",
        CICompositeOperationKind::SourceInCompositing => "CISourceInCompositing",
        CICompositeOperationKind::SourceOutCompositing => "CISourceOutCompositing",
        CICompositeOperationKind::SourceOverCompositing => "CISourceOverCompositing",
        CICompositeOperationKind::SubtractBlendMode => "CISubtractBlendMode",
        CICompositeOperationKind::VividLightBlendMode => "CIVividLightBlendMode",
    })
}

/// Mirrors the `CoreImage` framework counterpart for `CIFourCoordinateGeometryFilterKind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIFourCoordinateGeometryFilterKind {
/// Mirrors the `CoreImage` framework case `KeystoneCorrectionCombined`.
    KeystoneCorrectionCombined,
/// Mirrors the `CoreImage` framework case `KeystoneCorrectionHorizontal`.
    KeystoneCorrectionHorizontal,
/// Mirrors the `CoreImage` framework case `KeystoneCorrectionVertical`.
    KeystoneCorrectionVertical,
/// Mirrors the `CoreImage` framework case `PerspectiveCorrection`.
    PerspectiveCorrection,
/// Mirrors the `CoreImage` framework case `PerspectiveTransform`.
    PerspectiveTransform,
/// Mirrors the `CoreImage` framework case `PerspectiveTransformWithExtent`.
    PerspectiveTransformWithExtent,
}

/// Calls the `CoreImage` framework counterpart for `four_coordinate_geometry_filter`.
pub fn four_coordinate_geometry_filter(
    kind: CIFourCoordinateGeometryFilterKind,
) -> Option<CIFilter> {
    builtin_filter(match kind {
        CIFourCoordinateGeometryFilterKind::KeystoneCorrectionCombined => {
            "CIKeystoneCorrectionCombined"
        }
        CIFourCoordinateGeometryFilterKind::KeystoneCorrectionHorizontal => {
            "CIKeystoneCorrectionHorizontal"
        }
        CIFourCoordinateGeometryFilterKind::KeystoneCorrectionVertical => {
            "CIKeystoneCorrectionVertical"
        }
        CIFourCoordinateGeometryFilterKind::PerspectiveCorrection => "CIPerspectiveCorrection",
        CIFourCoordinateGeometryFilterKind::PerspectiveTransform => "CIPerspectiveTransform",
        CIFourCoordinateGeometryFilterKind::PerspectiveTransformWithExtent => {
            "CIPerspectiveTransformWithExtent"
        }
    })
}

/// Mirrors the `CoreImage` framework counterpart for `CITransitionFilterKind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CITransitionFilterKind {
/// Mirrors the `CoreImage` framework case `AccordionFold`.
    AccordionFold,
/// Mirrors the `CoreImage` framework case `BarsSwipe`.
    BarsSwipe,
/// Mirrors the `CoreImage` framework case `CopyMachine`.
    CopyMachine,
/// Mirrors the `CoreImage` framework case `DisintegrateWithMask`.
    DisintegrateWithMask,
/// Mirrors the `CoreImage` framework case `Dissolve`.
    Dissolve,
/// Mirrors the `CoreImage` framework case `Flash`.
    Flash,
/// Mirrors the `CoreImage` framework case `Mod`.
    Mod,
/// Mirrors the `CoreImage` framework case `PageCurl`.
    PageCurl,
/// Mirrors the `CoreImage` framework case `PageCurlWithShadow`.
    PageCurlWithShadow,
/// Mirrors the `CoreImage` framework case `Ripple`.
    Ripple,
/// Mirrors the `CoreImage` framework case `Swipe`.
    Swipe,
}

/// Calls the `CoreImage` framework counterpart for `transition_filter`.
pub fn transition_filter(kind: CITransitionFilterKind) -> Option<CIFilter> {
    builtin_filter(match kind {
        CITransitionFilterKind::AccordionFold => "CIAccordionFoldTransition",
        CITransitionFilterKind::BarsSwipe => "CIBarsSwipeTransition",
        CITransitionFilterKind::CopyMachine => "CICopyMachineTransition",
        CITransitionFilterKind::DisintegrateWithMask => "CIDisintegrateWithMaskTransition",
        CITransitionFilterKind::Dissolve => "CIDissolveTransition",
        CITransitionFilterKind::Flash => "CIFlashTransition",
        CITransitionFilterKind::Mod => "CIModTransition",
        CITransitionFilterKind::PageCurl => "CIPageCurlTransition",
        CITransitionFilterKind::PageCurlWithShadow => "CIPageCurlWithShadowTransition",
        CITransitionFilterKind::Ripple => "CIRippleTransition",
        CITransitionFilterKind::Swipe => "CISwipeTransition",
    })
}

macro_rules! builtin_filter_constructors {
    ($(($function:ident, $name:literal)),+ $(,)?) => {
        $(
            #[doc = concat!("Constructs the `", $name, "` built-in filter.")]
            pub fn $function() -> Option<CIFilter> {
                builtin_filter($name)
            }
        )+
    };
}

builtin_filter_constructors! {
    (accordion_fold_transition, "CIAccordionFoldTransition"),
    (affine_clamp, "CIAffineClamp"),
    (affine_tile, "CIAffineTile"),
    (area_average, "CIAreaAverage"),
    (area_average_maximum_red, "CIAreaAverageMaximumRed"),
    (area_bounds_red, "CIAreaBoundsRed"),
    (area_histogram, "CIAreaHistogram"),
    (area_logarithmic_histogram, "CIAreaLogarithmicHistogram"),
    (area_maximum, "CIAreaMaximum"),
    (area_maximum_alpha, "CIAreaMaximumAlpha"),
    (area_min_max, "CIAreaMinMax"),
    (area_min_max_red, "CIAreaMinMaxRed"),
    (area_minimum, "CIAreaMinimum"),
    (area_minimum_alpha, "CIAreaMinimumAlpha"),
    (area_reduction_filter, "CIAreaReductionFilter"),
    (attributed_text_image_generator, "CIAttributedTextImageGenerator"),
    (aztec_code_generator, "CIAztecCodeGenerator"),
    (barcode_generator, "CIBarcodeGenerator"),
    (bars_swipe_transition, "CIBarsSwipeTransition"),
    (bicubic_scale_transform, "CIBicubicScaleTransform"),
    (blurred_rectangle_generator, "CIBlurredRectangleGenerator"),
    (convert_lab_to_rgb, "CIConvertLabToRGB"),
    (convert_rgb_to_lab, "CIConvertRGBtoLab"),
    (blurred_rounded_rectangle_generator, "CIBlurredRoundedRectangleGenerator"),
    (bokeh_blur, "CIBokehBlur"),
    (bump_distortion, "CIBumpDistortion"),
    (bump_distortion_linear, "CIBumpDistortionLinear"),
    (cmyk_halftone, "CICMYKHalftone"),
    (canny_edge_detector, "CICannyEdgeDetector"),
    (circle_splash_distortion, "CICircleSplashDistortion"),
    (circular_screen, "CICircularScreen"),
    (circular_wrap, "CICircularWrap"),
    (code128_barcode_generator, "CICode128BarcodeGenerator"),
    (color_absolute_difference, "CIColorAbsoluteDifference"),
    (color_clamp, "CIColorClamp"),
    (color_cross_polynomial, "CIColorCrossPolynomial"),
    (color_cube, "CIColorCube"),
    (color_cube_with_color_space, "CIColorCubeWithColorSpace"),
    (color_cubes_mixed_with_mask, "CIColorCubesMixedWithMask"),
    (color_curves, "CIColorCurves"),
    (color_map, "CIColorMap"),
    (color_matrix, "CIColorMatrix"),
    (color_polynomial, "CIColorPolynomial"),
    (color_posterize, "CIColorPosterize"),
    (color_threshold, "CIColorThreshold"),
    (color_threshold_otsu, "CIColorThresholdOtsu"),
    (column_average, "CIColumnAverage"),
    (convolution, "CIConvolution"),
    (copy_machine_transition, "CICopyMachineTransition"),
    (core_ml_model, "CICoreMLModelFilter"),
    (depth_of_field, "CIDepthOfField"),
    (edge_preserve_upsample, "CIEdgePreserveUpsampleFilter"),
    (depth_to_disparity, "CIDepthToDisparity"),
    (disintegrate_with_mask_transition, "CIDisintegrateWithMaskTransition"),
    (disparity_to_depth, "CIDisparityToDepth"),
    (displacement_distortion, "CIDisplacementDistortion"),
    (dissolve_transition, "CIDissolveTransition"),
    (distance_gradient_from_red_mask, "CIDistanceGradientFromRedMask"),
    (dither, "CIDither"),
    (document_enhancer, "CIDocumentEnhancer"),
    (dot_screen, "CIDotScreen"),
    (droste, "CIDroste"),
    (eightfold_reflected_tile, "CIEightfoldReflectedTile"),
    (flash_transition, "CIFlashTransition"),
    (fourfold_reflected_tile, "CIFourfoldReflectedTile"),
    (fourfold_rotated_tile, "CIFourfoldRotatedTile"),
    (fourfold_translated_tile, "CIFourfoldTranslatedTile"),
    (gabor_gradients, "CIGaborGradients"),
    (gaussian_gradient, "CIGaussianGradient"),
    (histogram_display, "CIHistogramDisplayFilter"),
    (glass_distortion, "CIGlassDistortion"),
    (glass_lozenge, "CIGlassLozenge"),
    (glide_reflected_tile, "CIGlideReflectedTile"),
    (gloom, "CIGloom"),
    (hatched_screen, "CIHatchedScreen"),
    (height_field_from_mask, "CIHeightFieldFromMask"),
    (hexagonal_pixellate, "CIHexagonalPixellate"),
    (highlight_shadow_adjust, "CIHighlightShadowAdjust"),
    (hole_distortion, "CIHoleDistortion"),
    (hue_saturation_value_gradient, "CIHueSaturationValueGradient"),
    (k_means, "CIKMeans"),
    (kaleidoscope, "CIKaleidoscope"),
    (keystone_correction_combined, "CIKeystoneCorrectionCombined"),
    (keystone_correction_horizontal, "CIKeystoneCorrectionHorizontal"),
    (keystone_correction_vertical, "CIKeystoneCorrectionVertical"),
    (lab_delta_e, "CILabDeltaE"),
    (lenticular_halo_generator, "CILenticularHaloGenerator"),
    (light_tunnel, "CILightTunnel"),
    (line_overlay, "CILineOverlay"),
    (line_screen, "CILineScreen"),
    (linear_to_srgb_tone_curve, "CILinearToSRGBToneCurve"),
    (mask_to_alpha, "CIMaskToAlpha"),
    (masked_variable_blur, "CIMaskedVariableBlur"),
    (maximum_component, "CIMaximumComponent"),
    (median, "CIMedianFilter"),
    (maximum_scale_transform, "CIMaximumScaleTransform"),
    (mesh_generator, "CIMeshGenerator"),
    (minimum_component, "CIMinimumComponent"),
    (mix, "CIMix"),
    (mod_transition, "CIModTransition"),
    (morphology_gradient, "CIMorphologyGradient"),
    (morphology_maximum, "CIMorphologyMaximum"),
    (morphology_minimum, "CIMorphologyMinimum"),
    (morphology_rectangle_maximum, "CIMorphologyRectangleMaximum"),
    (morphology_rectangle_minimum, "CIMorphologyRectangleMinimum"),
    (nine_part_stretched, "CINinePartStretched"),
    (nine_part_tiled, "CINinePartTiled"),
    (noise_reduction, "CINoiseReduction"),
    (op_tile, "CIOpTile"),
    (pdf417_barcode_generator, "CIPDF417BarcodeGenerator"),
    (page_curl_transition, "CIPageCurlTransition"),
    (page_curl_with_shadow_transition, "CIPageCurlWithShadowTransition"),
    (palette_centroid, "CIPaletteCentroid"),
    (palettize, "CIPalettize"),
    (parallelogram_tile, "CIParallelogramTile"),
    (person_segmentation, "CIPersonSegmentation"),
    (perspective_rotate, "CIPerspectiveRotate"),
    (perspective_tile, "CIPerspectiveTile"),
    (perspective_transform_with_extent, "CIPerspectiveTransformWithExtent"),
    (photo_effect, "CIPhotoEffect"),
    (pinch_distortion, "CIPinchDistortion"),
    (pointillize, "CIPointillize"),
    (qr_code_generator, "CIQRCodeGenerator"),
    (random_generator, "CIRandomGenerator"),
    (ripple_transition, "CIRippleTransition"),
    (rounded_qr_code_generator, "CIRoundedQRCodeGenerator"),
    (rounded_rectangle_generator, "CIRoundedRectangleGenerator"),
    (rounded_rectangle_stroke_generator, "CIRoundedRectangleStrokeGenerator"),
    (row_average, "CIRowAverage"),
    (saliency_map, "CISaliencyMapFilter"),
    (srgb_tone_curve_to_linear, "CISRGBToneCurveToLinear"),
    (shaded_material, "CIShadedMaterial"),
    (signed_distance_gradient_from_red_mask, "CISignedDistanceGradientFromRedMask"),
    (sixfold_reflected_tile, "CISixfoldReflectedTile"),
    (sixfold_rotated_tile, "CISixfoldRotatedTile"),
    (smooth_linear_gradient, "CISmoothLinearGradient"),
    (sobel_gradients, "CISobelGradients"),
    (spot_color, "CISpotColor"),
    (spot_light, "CISpotLight"),
    (star_shine_generator, "CIStarShineGenerator"),
    (stretch_crop, "CIStretchCrop"),
    (stripes_generator, "CIStripesGenerator"),
    (sunbeams_generator, "CISunbeamsGenerator"),
    (swipe_transition, "CISwipeTransition"),
    (system_tone_map, "CISystemToneMap"),
    (text_image_generator, "CITextImageGenerator"),
    (thermal, "CIThermal"),
    (tone_curve, "CIToneCurve"),
    (tone_map_headroom, "CIToneMapHeadroom"),
    (torus_lens_distortion, "CITorusLensDistortion"),
    (triangle_kaleidoscope, "CITriangleKaleidoscope"),
    (triangle_tile, "CITriangleTile"),
    (twelvefold_reflected_tile, "CITwelvefoldReflectedTile"),
    (twirl_distortion, "CITwirlDistortion"),
    (vortex_distortion, "CIVortexDistortion"),
    (x_ray, "CIXRay"),
}
