mod common;

use std::error::Error;

use coreimage::prelude::*;

#[test]
fn passthrough_processor_preserves_extent() -> Result<(), Box<dyn Error>> {
    let image = common::solid_image();
    let output = CIImageProcessor::apply_passthrough(&image)?;
    let invocation = CIImageProcessor::last_invocation();
    let image_extent = image.extent();

    assert!((output.extent().width - image_extent.width).abs() < f64::EPSILON);
    assert!(CIImageProcessor::last_invocation_json().starts_with('{'));
    assert_eq!(invocation.input_count(), 1);
    let input = invocation
        .input()
        .expect("expected image processor input snapshot");
    assert!(input.bytes_per_row() > 0);
    assert!((input.region().width - image_extent.width).abs() < f64::EPSILON);
    assert!((invocation.output().region().height - image_extent.height).abs() < f64::EPSILON);
    Ok(())
}
