mod common;

use std::error::Error;
use std::mem;

use coreimage::prelude::*;

#[test]
fn render_destination_runs_tasks_and_exposes_bitmap_bytes() -> Result<(), Box<dyn Error>> {
    let context = CIContext::new_default();
    let image = common::solid_image();
    let mut destination = CIRenderDestination::bitmap_rgba8(64, 64)?;
    destination.set_alpha_mode(CIRenderDestinationAlphaMode::Premultiplied)?;
    destination.set_flipped(true)?;
    destination.set_clamped(true)?;

    context.prepare_render(&image, &destination)?;
    let info = context
        .start_render_task(&image, &mut destination)?
        .wait_until_completed()?;
    assert!(info.pass_count() >= 1);
    assert!(info.pixels_processed() >= 64 * 64);
    assert!(destination.bitmap_data()?.iter().any(|byte| *byte != 0));
    assert_eq!(destination.bytes_per_row(), 64 * 4);
    assert_eq!(destination.format(), CIFormat::Rgba8);

    let clear_info = context
        .start_clear_task(&mut destination)?
        .wait_until_completed()?;
    assert!(clear_info.pass_count() >= 1);
    Ok(())
}

#[test]
fn render_and_clear_tasks_keep_dropped_destinations_alive() -> Result<(), Box<dyn Error>> {
    let context = CIContext::new_default();
    let image = common::solid_image();

    let render_task = {
        let mut destination = CIRenderDestination::bitmap_rgba8(64, 64)?;
        context.start_render_task(&image, &mut destination)?
    };
    assert!(render_task.wait_until_completed()?.pixels_processed() >= 64 * 64);

    let clear_task = {
        let mut destination = CIRenderDestination::bitmap_rgba8(64, 64)?;
        context.start_clear_task(&mut destination)?
    };
    assert!(clear_task.wait_until_completed()?.pass_count() >= 1);
    Ok(())
}

#[test]
fn pending_tasks_exclude_bitmap_access_and_destination_mutation() -> Result<(), Box<dyn Error>> {
    let context = CIContext::new_default();
    let image = common::solid_image();
    let mut destination = CIRenderDestination::bitmap_rgba8(64, 64)?;
    let task = context.start_render_task(&image, &mut destination)?;

    assert!(matches!(
        destination.bitmap_data(),
        Err(CIError::InvalidArgument(_))
    ));
    assert!(matches!(
        destination.bitmap_data_mut(),
        Err(CIError::InvalidArgument(_))
    ));
    assert!(matches!(
        destination.set_flipped(true),
        Err(CIError::InvalidArgument(_))
    ));
    assert!(matches!(
        context.start_clear_task(&mut destination),
        Err(CIError::InvalidArgument(_))
    ));

    drop(task);
    assert!(destination.bitmap_data()?.iter().any(|byte| *byte != 0));
    destination.bitmap_data_mut()?.fill(0);
    assert!(destination.bitmap_data()?.iter().all(|byte| *byte == 0));
    Ok(())
}

#[test]
fn forgotten_task_keeps_bitmap_access_closed() -> Result<(), Box<dyn Error>> {
    let context = CIContext::new_default();
    let image = common::solid_image();
    let mut destination = CIRenderDestination::bitmap_rgba8(64, 64)?;
    let task = context.start_render_task(&image, &mut destination)?;

    mem::forget(task);
    assert!(matches!(
        destination.bitmap_data(),
        Err(CIError::InvalidArgument(_))
    ));
    mem::forget(destination);
    Ok(())
}
