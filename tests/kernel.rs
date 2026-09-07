use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use coreimage::prelude::*;

fn image(color: &CIColor) -> CIImage {
    CIImage::from_color(color).cropped_to(CGRect::new(0.0, 0.0, 64.0, 64.0))
}

#[test]
fn blend_kernel_applies_to_images() -> Result<(), Box<dyn Error>> {
    let foreground_color = CIColor::rgba(1.0, 0.0, 0.0, 0.5);
    let background_color = CIColor::rgb(0.0, 0.0, 1.0);
    let foreground = image(&foreground_color);
    let background = image(&background_color);
    let kernel = CIBlendKernel::built_in(CIBlendKernelKind::SourceOver);
    let output = kernel.apply(&foreground, &background)?;
    let generic = kernel.as_kernel();

    assert!((output.extent().size.width - 64.0).abs() < f64::EPSILON);
    assert!(!kernel.name().is_empty());
    assert_eq!(generic.name(), kernel.name());
    Ok(())
}

fn assert_rect_eq(actual: CGRect, expected: CGRect) {
    assert!((actual.origin.x - expected.origin.x).abs() < f64::EPSILON);
    assert!((actual.origin.y - expected.origin.y).abs() < f64::EPSILON);
    assert!((actual.size.width - expected.size.width).abs() < f64::EPSILON);
    assert!((actual.size.height - expected.size.height).abs() < f64::EPSILON);
}

#[test]
fn warp_kernels_use_conservative_or_caller_supplied_regions() -> Result<(), Box<dyn Error>> {
    let kernel = CIWarpKernel::from_source(
        "kernel vec2 shift(float amount) { return destCoord() + vec2(amount, 0.0); }",
    )?;
    let input = image(&CIColor::rgb(0.2, 0.4, 0.8));
    let extent = CGRect::new(0.0, 0.0, 32.0, 32.0);
    let destination = CGRect::new(3.0, 5.0, 7.0, 11.0);

    let conservative = kernel.apply_image_scalar(&input, 8.0, extent)?;
    assert_rect_eq(
        conservative.region_of_interest_for_image(&input, destination),
        input.extent(),
    );

    let local = kernel.apply_image_scalar_with_destination_roi(&input, 0.0, extent)?;
    assert_rect_eq(
        local.region_of_interest_for_image(&input, destination),
        destination,
    );

    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);
    let custom = kernel.apply_image_scalar_with_roi(&input, 8.0, extent, move |_, rect| {
        callback_calls.fetch_add(1, Ordering::Relaxed);
        CGRect::new(
            rect.origin.x + 8.0,
            rect.origin.y,
            rect.size.width,
            rect.size.height,
        )
    })?;
    assert_rect_eq(
        custom.region_of_interest_for_image(&input, destination),
        CGRect::new(11.0, 5.0, 7.0, 11.0),
    );
    assert!(calls.load(Ordering::Relaxed) > 0);
    Ok(())
}

struct PanicOnDrop;

impl Drop for PanicOnDrop {
    fn drop(&mut self) {
        panic!("warp panic payload destructor");
    }
}

#[test]
fn warp_roi_panics_fall_back_to_the_full_input_extent() -> Result<(), Box<dyn Error>> {
    let kernel = CIWarpKernel::from_source(
        "kernel vec2 identityWarp(float amount) { return destCoord() + vec2(amount, 0.0); }",
    )?;
    let input = image(&CIColor::rgb(0.2, 0.4, 0.8));
    let extent = CGRect::new(0.0, 0.0, 32.0, 32.0);
    let output = kernel.apply_image_scalar_with_roi(&input, 0.0, extent, |_, _| {
        std::panic::panic_any(PanicOnDrop);
    })?;

    assert_rect_eq(
        output.region_of_interest_for_image(&input, CGRect::new(1.0, 2.0, 3.0, 4.0)),
        input.extent(),
    );
    Ok(())
}
