mod common;

use coreimage::prelude::*;

#[test]
fn sampler_reports_image_extents() {
    let image = common::solid_image();
    let sampler = CISampler::new(
        &image,
        CISamplerOptions {
            color_space: Some(CIColorSpace::DisplayP3),
            ..CISamplerOptions::default()
        },
    );

    assert!((sampler.extent().width - image.extent().width).abs() < f64::EPSILON);
    assert!((sampler.definition_extent().height - image.extent().height).abs() < f64::EPSILON);
}
