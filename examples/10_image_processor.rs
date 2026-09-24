use std::error::Error;

use coreimage::prelude::*;

fn solid_image() -> CIImage {
    CIImage::from_color(&CIColor::rgba(0.9, 0.4, 0.1, 1.0))
        .cropped_to(CGRect::new(0.0, 0.0, 64.0, 64.0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let invert = CIImageProcessorKernel::new(|inputs, output| {
        let input = inputs.first().ok_or("missing input")?;
        for y in 0..output.height() {
            let source = input.row(y).ok_or("input row missing")?;
            let target = output.row_mut(y).ok_or("output row missing")?;
            for (pixel, from) in target.chunks_exact_mut(4).zip(source.chunks_exact(4)) {
                pixel.copy_from_slice(&[255 - from[0], 255 - from[1], 255 - from[2], from[3]]);
            }
        }
        Ok(())
    })
    .with_format(CIFormat::Bgra8)?;
    let source = solid_image();
    let inverted = invert.apply(source.extent(), &[&source])?;
    let rendered = CIContext::new_default().render_to_cg_image(&inverted)?;
    println!("inverted image: {}x{}", rendered.width(), rendered.height());
    Ok(())
}
