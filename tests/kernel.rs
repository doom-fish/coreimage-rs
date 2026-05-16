use std::error::Error;

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

    assert!((output.extent().width - 64.0).abs() < f64::EPSILON);
    assert!(!kernel.name().is_empty());
    Ok(())
}
