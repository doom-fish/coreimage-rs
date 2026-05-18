use std::error::Error;

use coreimage::prelude::*;

fn solid_image() -> CIImage {
    CIImage::from_color(&CIColor::rgba(0.9, 0.4, 0.1, 1.0))
        .cropped_to(CGRect::new(0.0, 0.0, 64.0, 64.0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let output = CIImageProcessor::apply_passthrough(&solid_image())?;
    let extent = output.extent();
    println!("processor extent: {}x{}", extent.size.width, extent.size.height);
    println!(
        "processor snapshot: {}",
        CIImageProcessor::last_invocation_json()
    );
    Ok(())
}
