use std::error::Error;

use coreimage::prelude::*;

fn solid_image() -> CIImage {
    CIImage::from_color(&CIColor::rgba(0.2, 0.6, 0.9, 1.0))
        .cropped_to(CGRect::new(0.0, 0.0, 64.0, 64.0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = CIContext::new_default();
    let image = solid_image();
    let destination = CIRenderDestination::bitmap_rgba8(64, 64)?;
    let info = context.start_render_task(&image, &destination)?.wait_until_completed()?;

    println!(
        "rendered {} pixels in {} passes ({} bytes)",
        info.pixels_processed(),
        info.pass_count(),
        destination.bitmap_data().len()
    );
    Ok(())
}
