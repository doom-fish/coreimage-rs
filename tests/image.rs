mod common;

use std::error::Error;

use coreimage::prelude::*;

#[test]
fn image_from_bitmap_tracks_extent() -> Result<(), Box<dyn Error>> {
    let pixels = [
        255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
    ];
    let image = CIImage::from_bitmap_rgba8(&pixels, 2, 2)?;
    let shifted = image.translated(2.0, 3.0);
    let extent = shifted.extent();

    assert!((extent.size.width - 2.0).abs() < f64::EPSILON);
    assert!((extent.size.height - 2.0).abs() < f64::EPSILON);
    assert!(shifted.properties_json().starts_with('{'));
    Ok(())
}
