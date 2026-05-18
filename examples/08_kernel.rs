use std::error::Error;

use coreimage::prelude::*;

fn image(color: &CIColor) -> CIImage {
    CIImage::from_color(color).cropped_to(CGRect::new(0.0, 0.0, 32.0, 32.0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let foreground_color = CIColor::rgba(1.0, 0.0, 0.0, 0.5);
    let background_color = CIColor::rgb(0.0, 0.0, 1.0);
    let foreground = image(&foreground_color);
    let background = image(&background_color);
    let kernel = CIBlendKernel::built_in(CIBlendKernelKind::SourceOver);
    let output = kernel.apply(&foreground, &background)?;
    let extent = output.extent();

    println!("kernel output extent: {}x{}", extent.size.width, extent.size.height);
    Ok(())
}
