use coreimage::prelude::*;

fn main() {
    let camera_models = CIRAWFilter::supported_camera_models();
    println!(
        "RAW support: {} models, newest decoder {}",
        camera_models.len(),
        CIRAWDecoderVersion::Version9.as_str()
    );
}
