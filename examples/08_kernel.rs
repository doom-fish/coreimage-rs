use std::error::Error;

use coreimage::prelude::*;

const LIBRARY: &[u8] = include_bytes!("../tests/fixtures/kernels.metallib");

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
    println!(
        "Metal kernels in the library: {:?}",
        CIKernel::kernel_names_from_metal_library_data(LIBRARY)
    );
    let warp = CIWarpKernel::from_metal_library_data("shiftRight", LIBRARY, None)?;
    let warped = warp.apply(extent, &output, &[4.0.into()], |_, destination| {
        CGRect::new(
            destination.origin.x - 4.0,
            destination.origin.y,
            destination.size.width,
            destination.size.height,
        )
    })?;
    let tint = CIColorKernel::from_metal_library_data("tintTowards", LIBRARY, None)?;
    let offset = CIVector::new4(0.0, 0.0, 0.0, 0.0);
    let white = CIColor::rgb(1.0, 1.0, 1.0);
    let tinted = tint.apply(
        extent,
        &[(&warped).into(), 0.25.into(), (&offset).into(), (&white).into()],
    )?;
    let roi = warped.region_of_interest_for_image(&output, CGRect::new(8.0, 0.0, 8.0, 8.0));

    println!("kernel output extent: {}x{}", extent.size.width, extent.size.height);
    println!("warp source ROI starts at {},{}", roi.origin.x, roi.origin.y);
    println!("tinted extent: {:?}", tinted.extent());
    Ok(())
}
