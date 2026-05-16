use std::error::Error;

use coreimage::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let pixels = [
        255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
    ];
    let image = CIImage::from_bitmap_rgba8(&pixels, 2, 2)?;
    let extent = image.translated(10.0, 5.0).extent();

    println!(
        "image extent: {}x{} at {},{}",
        extent.width, extent.height, extent.x, extent.y
    );
    Ok(())
}
