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

pub fn gaussian_blur(input: &CIImage, radius: f64) -> Option<CIImage> {
    image_filter_output("CIGaussianBlur", input, |filter| {
        filter.set_input_number("inputRadius", radius);
    })
}

pub fn box_blur(input: &CIImage, radius: f64) -> Option<CIImage> {
    image_filter_output("CIBoxBlur", input, |filter| {
        filter.set_input_number("inputRadius", radius);
    })
}

pub fn disc_blur(input: &CIImage, radius: f64) -> Option<CIImage> {
    image_filter_output("CIDiscBlur", input, |filter| {
        filter.set_input_number("inputRadius", radius);
    })
}

pub fn motion_blur(input: &CIImage, radius: f64, angle: f64) -> Option<CIImage> {
    image_filter_output("CIMotionBlur", input, |filter| {
        filter.set_input_number("inputRadius", radius);
        filter.set_input_number("inputAngle", angle);
    })
}

pub fn zoom_blur(input: &CIImage, center: (f64, f64), amount: f64) -> Option<CIImage> {
    let center = CIVector::new(center.0, center.1);
    image_filter_output("CIZoomBlur", input, |filter| {
        filter.set_input_vector("inputCenter", &center);
        filter.set_input_number("inputAmount", amount);
    })
}

pub fn sharpen_luminance(input: &CIImage, sharpness: f64, radius: f64) -> Option<CIImage> {
    image_filter_output("CISharpenLuminance", input, |filter| {
        filter.set_input_number("inputSharpness", sharpness);
        filter.set_input_number("inputRadius", radius);
    })
}

pub fn unsharp_mask(input: &CIImage, intensity: f64, radius: f64) -> Option<CIImage> {
    image_filter_output("CIUnsharpMask", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
        filter.set_input_number("inputRadius", radius);
    })
}

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

pub fn exposure_adjust(input: &CIImage, ev: f64) -> Option<CIImage> {
    image_filter_output("CIExposureAdjust", input, |filter| {
        filter.set_input_number("inputEV", ev);
    })
}

pub fn gamma_adjust(input: &CIImage, power: f64) -> Option<CIImage> {
    image_filter_output("CIGammaAdjust", input, |filter| {
        filter.set_input_number("inputPower", power);
    })
}

pub fn hue_adjust(input: &CIImage, angle: f64) -> Option<CIImage> {
    image_filter_output("CIHueAdjust", input, |filter| {
        filter.set_input_number("inputAngle", angle);
    })
}

pub fn vibrance(input: &CIImage, amount: f64) -> Option<CIImage> {
    image_filter_output("CIVibrance", input, |filter| {
        filter.set_input_number("inputAmount", amount);
    })
}

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

pub fn white_point_adjust(input: &CIImage, color: &CIColor) -> Option<CIImage> {
    image_filter_output("CIWhitePointAdjust", input, |filter| {
        filter.set_input_color("inputColor", color);
    })
}

pub fn sepia_tone(input: &CIImage, intensity: f64) -> Option<CIImage> {
    image_filter_output("CISepiaTone", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
    })
}

pub fn color_invert(input: &CIImage) -> Option<CIImage> {
    image_filter_output("CIColorInvert", input, |_| {})
}

pub fn color_monochrome(input: &CIImage, color: &CIColor, intensity: f64) -> Option<CIImage> {
    image_filter_output("CIColorMonochrome", input, |filter| {
        filter.set_input_color("inputColor", color);
        filter.set_input_number("inputIntensity", intensity);
    })
}

pub fn false_color(input: &CIImage, color0: &CIColor, color1: &CIColor) -> Option<CIImage> {
    image_filter_output("CIFalseColor", input, |filter| {
        filter.set_input_color("inputColor0", color0);
        filter.set_input_color("inputColor1", color1);
    })
}

pub fn vignette(input: &CIImage, intensity: f64, radius: f64) -> Option<CIImage> {
    image_filter_output("CIVignette", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
        filter.set_input_number("inputRadius", radius);
    })
}

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

pub fn edges(input: &CIImage, intensity: f64) -> Option<CIImage> {
    image_filter_output("CIEdges", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
    })
}

pub fn edge_work(input: &CIImage, radius: f64) -> Option<CIImage> {
    image_filter_output("CIEdgeWork", input, |filter| {
        filter.set_input_number("inputRadius", radius);
    })
}

pub fn bloom(input: &CIImage, intensity: f64, radius: f64) -> Option<CIImage> {
    image_filter_output("CIBloom", input, |filter| {
        filter.set_input_number("inputIntensity", intensity);
        filter.set_input_number("inputRadius", radius);
    })
}

pub fn pixellate(input: &CIImage, center: (f64, f64), scale: f64) -> Option<CIImage> {
    let center = CIVector::new(center.0, center.1);
    image_filter_output("CIPixellate", input, |filter| {
        filter.set_input_vector("inputCenter", &center);
        filter.set_input_number("inputScale", scale);
    })
}

pub fn comic_effect(input: &CIImage) -> Option<CIImage> {
    image_filter_output("CIComicEffect", input, |_| {})
}

pub fn crystallize(input: &CIImage, center: (f64, f64), radius: f64) -> Option<CIImage> {
    let center = CIVector::new(center.0, center.1);
    image_filter_output("CICrystallize", input, |filter| {
        filter.set_input_vector("inputCenter", &center);
        filter.set_input_number("inputRadius", radius);
    })
}

pub fn straighten(input: &CIImage, angle: f64) -> Option<CIImage> {
    image_filter_output("CIStraightenFilter", input, |filter| {
        filter.set_input_number("inputAngle", angle);
    })
}

pub fn lanczos_scale_transform(input: &CIImage, scale: f64, aspect_ratio: f64) -> Option<CIImage> {
    image_filter_output("CILanczosScaleTransform", input, |filter| {
        filter.set_input_number("inputScale", scale);
        filter.set_input_number("inputAspectRatio", aspect_ratio);
    })
}

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

pub fn crop(input: &CIImage, rect: CGRect) -> Option<CIImage> {
    Some(input.cropped_to(rect))
}

pub fn source_over_compositing(foreground: &CIImage, background: &CIImage) -> Option<CIImage> {
    filter_output("CISourceOverCompositing", |filter| {
        filter.set_input_image(foreground);
        filter.set_input_image_for_key("inputBackgroundImage", background);
    })
}

pub fn multiply_compositing(foreground: &CIImage, background: &CIImage) -> Option<CIImage> {
    filter_output("CIMultiplyCompositing", |filter| {
        filter.set_input_image(foreground);
        filter.set_input_image_for_key("inputBackgroundImage", background);
    })
}

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

pub fn constant_color(width: usize, height: usize, color: &CIColor) -> Option<CIImage> {
    let image = filter_output("CIConstantColorGenerator", |filter| {
        filter.set_input_color("inputColor", color);
    })?;
    Some(image.cropped_to(CGRect::new(0.0, 0.0, width as f64, height as f64)))
}

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
