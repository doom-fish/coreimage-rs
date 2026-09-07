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

#[test]
fn exif_orientation_accepts_only_the_documented_integer_domain() -> Result<(), Box<dyn Error>> {
    let pixels = vec![255_u8; 2 * 3 * 4];
    let image = CIImage::from_bitmap_rgba8(&pixels, 2, 3)?;

    for orientation in 1..=8 {
        image.applying_orientation(orientation)?;
    }
    for orientation in [0, 9, u32::MAX] {
        assert!(matches!(
            image.applying_orientation(orientation),
            Err(CIError::InvalidArgument(_))
        ));
    }

    let rotated = image.oriented(6)?;
    assert!((rotated.extent().size.width - 3.0).abs() < f64::EPSILON);
    assert!((rotated.extent().size.height - 2.0).abs() < f64::EPSILON);
    Ok(())
}
