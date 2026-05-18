use std::error::Error;

use coreimage::prelude::*;

fn solid_image() -> CIImage {
    CIImage::from_color(&CIColor::rgba(0.3, 0.2, 0.9, 1.0))
        .cropped_to(CGRect::new(0.0, 0.0, 64.0, 64.0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut filter = CIFilter::new("CIColorMonochrome").ok_or("CIColorMonochrome should exist")?;
    filter.set_input_image(&solid_image());
    filter.set_input_color("inputColor", &CIColor::named(CIColorName::Blue));
    filter.set_input_number("inputIntensity", 0.75);
    let output = filter
        .output_image()
        .ok_or("filter should produce output")?;
    let extent = output.extent();

    println!(
        "filter {} -> {}x{}",
        filter.name(),
        extent.size.width,
        extent.size.height
    );
    Ok(())
}
