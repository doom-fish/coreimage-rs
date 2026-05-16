mod common;

use std::error::Error;

use coreimage::prelude::*;

#[test]
fn render_destination_runs_tasks_and_exposes_bitmap_bytes() -> Result<(), Box<dyn Error>> {
    let context = CIContext::new_default();
    let image = common::solid_image();
    let mut destination = CIRenderDestination::bitmap_rgba8(64, 64)?;
    destination.set_alpha_mode(CIRenderDestinationAlphaMode::Premultiplied);
    destination.set_flipped(true);
    destination.set_clamped(true);

    context.prepare_render(&image, &destination)?;
    let info = context.start_render_task(&image, &destination)?.wait_until_completed()?;
    assert!(info.pass_count() >= 1);
    assert!(info.pixels_processed() >= 64 * 64);
    assert!(destination.bitmap_data().iter().any(|byte| *byte != 0));
    assert_eq!(destination.bytes_per_row(), 64 * 4);
    assert_eq!(destination.format(), CIFormat::Rgba8);

    let clear_info = context.start_clear_task(&destination)?.wait_until_completed()?;
    assert!(clear_info.pass_count() >= 1);
    Ok(())
}
