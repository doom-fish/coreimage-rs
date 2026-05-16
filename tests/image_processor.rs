mod common;

use std::error::Error;

use coreimage::prelude::*;

#[test]
fn passthrough_processor_preserves_extent() -> Result<(), Box<dyn Error>> {
    let image = common::solid_image();
    let output = CIImageProcessor::apply_passthrough(&image)?;

    assert!((output.extent().width - image.extent().width).abs() < f64::EPSILON);
    assert!(CIImageProcessor::last_invocation_json().starts_with('{'));
    Ok(())
}
