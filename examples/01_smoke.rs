use std::error::Error;
use std::path::PathBuf;

use coreimage::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let red = CIColor::rgba(1.0, 0.0, 0.0, 1.0);
    let source =
        filters::constant_color(100, 100, &red).ok_or("failed to create constant-color image")?;
    let blurred = filters::gaussian_blur(&source, 10.0)
        .ok_or("failed to blur image")?
        .cropped_to(CGRect::new(0.0, 0.0, 100.0, 100.0));

    let context = CIContext::new_default();
    let rendered = context.render_to_cg_image(&blurred)?;

    if rendered.width() != 100 || rendered.height() != 100 {
        return Err(format!(
            "unexpected rendered size: {}x{}",
            rendered.width(),
            rendered.height()
        )
        .into());
    }

    let output = PathBuf::from("target/coreimage_smoke.png");
    context.write_png(&blurred, &output)?;

    println!("✅ coreimage smoke OK");
    Ok(())
}
