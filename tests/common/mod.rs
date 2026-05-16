#![allow(dead_code)]

use coreimage::prelude::*;
use std::fs;
use std::path::PathBuf;

pub fn solid_image() -> CIImage {
    CIImage::from_color(&CIColor::rgba(0.2, 0.4, 0.8, 1.0))
        .cropped_to(CGRect::new(0.0, 0.0, 64.0, 64.0))
}

pub fn qr_image() -> CIImage {
    let mut filter = CIFilter::new("CIQRCodeGenerator").expect("CIQRCodeGenerator should exist");
    filter.set_input_bytes("inputMessage", b"coreimage-rs");
    filter.set_input_string("inputCorrectionLevel", "M");
    filter
        .output_image()
        .expect("CIQRCodeGenerator should produce an image")
        .scaled(8.0, 8.0)
}

pub fn target_path(name: &str) -> PathBuf {
    let dir = PathBuf::from("target/coreimage-tests");
    fs::create_dir_all(&dir).expect("failed to create target/coreimage-tests");
    dir.join(name)
}
