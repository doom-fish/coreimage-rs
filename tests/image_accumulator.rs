mod common;

use coreimage::prelude::*;

#[test]
fn image_accumulator_tracks_image_contents() {
    let extent = CGRect::new(0.0, 0.0, 64.0, 64.0);
    let mut accumulator =
        CIImageAccumulator::new(extent, CIFormat::Rgba8).expect("accumulator should be created");
    let image = common::solid_image();

    accumulator.set_image(&image);
    accumulator.set_image_dirty_rect(&image, extent);

    let snapshot = accumulator
        .image()
        .expect("accumulator should produce an image");
    let snapshot_extent = snapshot.extent();
    assert!((snapshot_extent.size.width - 64.0).abs() < f64::EPSILON);
    assert!((snapshot_extent.size.height - 64.0).abs() < f64::EPSILON);
    assert!(accumulator.format().is_some());

    accumulator.clear();
}
