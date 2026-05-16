# coreimage

Safe Rust bindings for Apple's [CoreImage](https://developer.apple.com/documentation/coreimage) framework — GPU-accelerated image processing, filtering, and rendering on macOS.

> **Status:** v0.1.0 ships the essential practical image-pipeline surface: `CIImage`, `CIFilter`, `CIContext`, `CIVector`, `CIColor`, plus ~30 common built-in filter helpers. Legacy `CIDetector` and `CIPlugIn` bindings are planned for v0.2.

## Highlights

- `CIImage` creation from file paths, encoded bytes, `CGImage`, `CVPixelBuffer`, `IOSurface`, and RGBA bitmap data
- `CIFilter` registry + input/output key inspection + typed setters for numbers, strings, images, vectors, and colors
- `filters` module with common built-ins: blur, sharpen, color adjust, edge detection, perspective correction, compositing, gradients, generators, and more
- `CIContext` rendering to `CGImage`, `CVPixelBuffer`, `IOSurface`, PNG, JPEG, HEIF, and TIFF
- Optional Metal-backed contexts via [`apple-metal`](https://github.com/doom-fish/apple-metal-rs)
- Shared CoreFoundation/CoreGraphics/CoreVideo/IOSurface interop via [`apple-cf`](https://github.com/doom-fish/apple-cf-rs)

## Quick start

```rust,no_run
use coreimage::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let red = CIColor::rgba(1.0, 0.0, 0.0, 1.0);
    let source = filters::constant_color(100, 100, &red)
        .ok_or("failed to create constant-color image")?;
    let blurred = filters::gaussian_blur(&source, 10.0)
        .ok_or("failed to blur image")?
        .cropped_to(CGRect::new(0.0, 0.0, 100.0, 100.0));

    let context = CIContext::new_default();
    let rendered = context.render_to_cg_image(&blurred)?;
    assert_eq!(rendered.width(), 100);
    assert_eq!(rendered.height(), 100);

    context.write_png(&blurred, "target/coreimage_demo.png")?;
    Ok(())
}
```

## Surface overview

### `CIImage`

- `from_path`, `from_encoded_data`, `from_cg_image`, `from_cv_pixel_buffer`, `from_iosurface`, `from_bitmap_rgba8`
- `extent`, `properties_json`
- `cropped_to`, `translated`, `scaled`, `transformed`, `applying_orientation`, `composited_over`
- `apply_filter(&mut CIFilter)`

### `CIFilter`

- `CIFilter::new(name)`
- `CIFilter::all_names()` / `CIFilter::names_in_category(category)`
- `input_keys`, `output_keys`, `attributes_json`
- `set_input_image`, `set_input_image_for_key`, `set_input_number`, `set_input_string`, `set_input_vector`, `set_input_color`
- `output_image`

### `CIContext`

- `CIContext::new_default()`
- `CIContext::new_cpu()`
- `CIContext::new_metal(&MetalDevice)` / `CIContext::new_command_queue(&CommandQueue)` (`metal` feature)
- `render_to_cg_image`, `render_to_cv_pixel_buffer`, `render_to_iosurface`
- `write_png`, `write_jpeg`, `write_heif`, `write_tiff`

## Smoke example

Run the end-to-end smoke test with:

```bash
cargo run --all-features --example 01_smoke
```

It generates a red 100×100 CoreImage image, applies Gaussian blur, renders the result to `CGImage`, writes `target/coreimage_smoke.png`, and prints `✅ coreimage smoke OK`.

## Roadmap

- [x] `CIImage`, `CIFilter`, `CIContext`, `CIVector`, `CIColor`
- [x] Metal-backed `CIContext`
- [x] Common built-in filter helpers (`filters` feature)
- [ ] `CIDetector`, `CIFeature`, `CIFaceFeature`, `CIQRCodeFeature`
- [ ] `CIPlugIn`

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
