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
    filter
        .set_message(b"coreimage-rs")
        .expect("inputMessage should accept bytes");
    filter
        .set_correction_level("M")
        .expect("inputCorrectionLevel should accept a string");
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

pub fn render_rgba8(context: &CIContext, image: &CIImage, width: usize, height: usize) -> Vec<[u8; 4]> {
    let mut destination =
        CIRenderDestination::bitmap_rgba8(width, height).expect("RGBA8 destination should exist");
    context
        .start_render_task(image, &mut destination)
        .expect("render task should start")
        .wait_until_completed()
        .expect("render task should complete");
    let row_bytes = destination.bytes_per_row();
    let bytes = destination
        .bitmap_data()
        .expect("completed destination should expose its bytes");
    (0..height)
        .flat_map(|row| {
            (0..width).map(move |column| {
                let start = row * row_bytes + column * 4;
                [bytes[start], bytes[start + 1], bytes[start + 2], bytes[start + 3]]
            })
        })
        .collect()
}

pub fn assert_pixel_near(actual: [u8; 4], expected: [u8; 4]) {
    for (channel, (actual_value, expected_value)) in actual.iter().zip(expected).enumerate() {
        assert!(
            actual_value.abs_diff(expected_value) <= 2,
            "channel {channel}: expected {expected:?}, got {actual:?}"
        );
    }
}
