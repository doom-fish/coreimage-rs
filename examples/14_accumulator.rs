use std::error::Error;

use coreimage::prelude::*;

fn solid_image() -> CIImage {
    CIImage::from_color(&CIColor::rgba(0.9, 0.3, 0.2, 1.0))
        .cropped_to(CGRect::new(0.0, 0.0, 64.0, 64.0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let extent = CGRect::new(0.0, 0.0, 64.0, 64.0);
    let shape = CIFilterShape::new(extent).inset(4, 4);
    let mut accumulator =
        CIImageAccumulator::new(extent, CIFormat::Rgba8).ok_or("failed to create accumulator")?;
    accumulator.set_image(&solid_image());
    let snapshot = accumulator
        .image()
        .ok_or("accumulator should yield an image")?;

    println!(
        "shape {:?} accumulator {}x{}",
        shape.extent(),
        snapshot.extent().width,
        snapshot.extent().height
    );
    Ok(())
}
