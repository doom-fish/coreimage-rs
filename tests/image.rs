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

#[test]
fn bitmap_rows_must_hold_every_pixel_of_the_declared_width() {
    let tiny = [0_u8; 4];
    let error = CIImage::from_bitmap(&tiny, 1000, 1, 4, CIFormat::Rgba8, None)
        .expect_err("a 4-byte row cannot hold 1000 RGBA8 pixels");
    assert!(matches!(error, CIError::InvalidArgument(ref message) if message.contains("4000")));

    let plenty = vec![0_u8; 4000];
    assert!(matches!(
        CIImage::from_bitmap(&plenty, 1000, 1, 4, CIFormat::Rgba8, None),
        Err(CIError::InvalidArgument(_))
    ));

    let floats = vec![0_u8; 64];
    assert!(matches!(
        CIImage::from_bitmap(&floats, 2, 2, 16, CIFormat::RgbaF, None),
        Err(CIError::InvalidArgument(_))
    ));
    let image = CIImage::from_bitmap(&floats, 2, 2, 32, CIFormat::RgbaF, None)
        .expect("two 16-byte pixels fit a 32-byte row");
    assert!((image.extent().size.width - 2.0).abs() < f64::EPSILON);
}

#[test]
fn bitmap_size_arithmetic_rejects_overflow_and_empty_dimensions() {
    let data = [0_u8; 16];
    for (width, height, bytes_per_row) in [
        (usize::MAX, 1, usize::MAX),
        (usize::MAX / 2, 1, usize::MAX),
        (1, usize::MAX, 4),
        (2, 2, usize::MAX / 2),
        (0, 2, 8),
        (2, 0, 8),
    ] {
        assert!(
            matches!(
                CIImage::from_bitmap(&data, width, height, bytes_per_row, CIFormat::Rgba8, None),
                Err(CIError::InvalidArgument(_))
            ),
            "{width}x{height} with {bytes_per_row} bytes per row must be rejected"
        );
    }
    assert!(matches!(
        CIImage::from_bitmap(&data, 2, 2, 8, CIFormat::Rgba8, None).map(|image| image.extent()),
        Ok(extent) if (extent.size.height - 2.0).abs() < f64::EPSILON
    ));
    assert!(matches!(
        CIImage::from_bitmap(&data[..15], 2, 2, 8, CIFormat::Rgba8, None),
        Err(CIError::InvalidArgument(_))
    ));
}

#[test]
fn padded_bitmap_rows_skip_the_padding_bytes() -> Result<(), Box<dyn Error>> {
    let padding = [0x55_u8; 4];
    let mut data = Vec::new();
    for row in [[[255, 0, 0, 255], [0, 255, 0, 255]], [[0, 0, 255, 255], [255, 255, 255, 255]]] {
        data.extend(row.iter().flatten());
        data.extend(padding);
    }
    let image = CIImage::from_bitmap(&data, 2, 2, 12, CIFormat::Rgba8, Some(CIColorSpace::Srgb))?;

    let mut pixels = common::render_rgba8(&CIContext::new_default(), &image, 2, 2);
    pixels.sort_unstable();
    let mut expected = vec![[0, 0, 255, 255], [0, 255, 0, 255], [255, 0, 0, 255], [255, 255, 255, 255]];
    expected.sort_unstable();
    for (actual, expected) in pixels.into_iter().zip(expected) {
        common::assert_pixel_near(actual, expected);
    }
    Ok(())
}
