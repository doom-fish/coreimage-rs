use coreimage::prelude::*;

fn solid_image() -> CIImage {
    CIImage::from_color(&CIColor::rgba(0.5, 0.5, 0.5, 1.0))
        .cropped_to(CGRect::new(0.0, 0.0, 64.0, 64.0))
}

fn main() {
    let image = solid_image();
    let sampler = CISampler::new(&image, CISamplerOptions::default());
    let extent = sampler.extent();
    println!("sampler extent: {}x{}", extent.size.width, extent.size.height);
}
