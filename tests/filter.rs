mod common;

use std::error::Error;

use coreimage::prelude::*;

#[test]
fn monochrome_filter_produces_output() -> Result<(), Box<dyn Error>> {
    let mut filter = CIFilter::new("CIColorMonochrome").expect("CIColorMonochrome should exist");
    filter.set_input_image(&common::solid_image())?;
    filter.set_color(&CIColor::named(CIColorName::Blue))?;
    filter.set_intensity(0.5)?;

    let output = filter
        .output_image_for_key(CIOutputKey::Image)
        .expect("filter should produce output");
    assert!((output.extent().size.width - 64.0).abs() < f64::EPSILON);
    assert!(filter.input_keys().iter().any(|key| key == "inputImage"));
    Ok(())
}

#[test]
fn dynamic_filter_inputs_reject_unknown_keys_and_wrong_types() {
    let mut filter = CIFilter::new("CIGaussianBlur").expect("CIGaussianBlur should exist");

    assert!(matches!(
        filter.set_input_number("inputNotARealKey", 1.0),
        Err(CIError::InvalidArgument(_))
    ));
    assert!(matches!(
        filter.set_input_string("inputRadius", "not a number"),
        Err(CIError::InvalidArgument(_))
    ));
}
