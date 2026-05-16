mod common;

use coreimage::prelude::*;

#[test]
fn monochrome_filter_produces_output() {
    let mut filter = CIFilter::new("CIColorMonochrome").expect("CIColorMonochrome should exist");
    filter.set_input_image(&common::solid_image());
    filter.set_input_color("inputColor", &CIColor::named(CIColorName::Blue));
    filter.set_input_number("inputIntensity", 0.5);

    let output = filter.output_image().expect("filter should produce output");
    assert!((output.extent().width - 64.0).abs() < f64::EPSILON);
    assert!(filter.input_keys().iter().any(|key| key == "inputImage"));
}
