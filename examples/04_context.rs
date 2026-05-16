use std::error::Error;
use std::fs;
use std::path::PathBuf;

use coreimage::prelude::*;

fn solid_image() -> CIImage {
    CIImage::from_color(&CIColor::rgba(0.1, 0.7, 0.2, 1.0))
        .cropped_to(CGRect::new(0.0, 0.0, 64.0, 64.0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = CIContext::new_default();
    let image = solid_image();
    let rendered = context.render_to_cg_image(&image)?;
    let output_dir = PathBuf::from("target/coreimage-examples");
    fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("context.png");
    context.write_png(&image, &output_path)?;

    println!(
        "rendered {}x{} -> {}",
        rendered.width(),
        rendered.height(),
        output_path.display()
    );
    Ok(())
}
