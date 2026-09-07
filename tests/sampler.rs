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

    assert!((sampler.extent().size.width - image.extent().size.width).abs() < f64::EPSILON);
    assert!(
        (sampler.definition_extent().size.height - image.extent().size.height).abs() < f64::EPSILON
    );
}

#[test]
fn sampler_affine_matrix_uses_the_native_six_number_array_contract() {
    let image = common::solid_image();
    let sampler = CISampler::new(
        &image,
        CISamplerOptions {
            affine_transform: Some(CGAffineTransform::translation(10.0, 20.0)),
            ..CISamplerOptions::default()
        },
    );

    assert!((sampler.extent().origin.x - 10.0).abs() < f64::EPSILON);
    assert!((sampler.extent().origin.y - 20.0).abs() < f64::EPSILON);
    assert!((sampler.definition_extent().origin.x - 10.0).abs() < f64::EPSILON);
    assert!((sampler.definition_extent().origin.y - 20.0).abs() < f64::EPSILON);
}
