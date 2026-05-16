# coreimage

Safe Rust bindings for Apple's [CoreImage](https://developer.apple.com/documentation/coreimage) framework — GPU-accelerated image processing, filtering, rendering, detection, and kernel work on macOS.

> **Status:** v0.2.0 expands the crate across the major practical Core Image surfaces: `CIImage`, `CIFilter`, `CIContext`, `CIDetector`, `CIFeature`, `CIColor`, `CIVector`, `CIKernel`, `CIBarcodeDescriptor`, `CIImageProcessorKernel`, `CIFilterGenerator`, and `CISampler`, with examples and tests for each area.

## Highlights

- `CIImage` constructors from file paths, encoded bytes, `CGImage`, `CVPixelBuffer`, `IOSurface`, colors, and RGBA bitmap buffers
- `CIFilter` registry/localization helpers plus typed setters for images, numbers, strings, bytes, vectors, colors, and barcode descriptors
- `CIContext` creation for default, CPU, and optional Metal backends, plus `CGImage` rendering and PNG/JPEG/HEIF/HEIF10/TIFF/OpenEXR export helpers
- Detector + feature inspection coverage for faces, rectangles, QR codes, and text via `CIDetector` / `CIFeature`
- `CIColor`, `CIVector`, `CIBarcodeDescriptor`, `CISampler`, `CIFilterGenerator`, and `CIImageProcessor` wrappers for common graph-building workflows
- `CIColorKernel`, `CIWarpKernel`, and `CIBlendKernel` support, plus the existing `filters` convenience module for common built-ins
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

- `CIImage`: loading, color/bitmap creation, geometric transforms, compositing, blur, gain-map/headroom helpers, and region-of-interest inspection
- `CIFilter`: filter discovery, localization, metadata, typed input setters, output image extraction, and barcode-descriptor inputs
- `CIContext`: default/CPU/Metal contexts, rendering to `CGImage`/`CVPixelBuffer`/`IOSurface`, cache management, and file output helpers
- `CIDetector` + `CIFeature`: detector construction plus QR/face/rectangle/text feature inspection, message strings, symbol descriptors, and sub-features
- `CIColor` + `CIVector`: structured value wrappers for graph inputs, geometry, and transform round-tripping
- `CIBarcodeDescriptor`: QR/Aztec/PDF417/Data Matrix descriptor construction and inspection
- `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`: custom kernel compilation and built-in blend kernels
- `CIImageProcessor`: passthrough processor bridge for exercising `CIImageProcessorKernel`
- `CIFilterGenerator` + `CISampler`: graph composition/export helpers and sampler configuration

## Examples

The crate ships runnable examples for each major area:

- `01_smoke`
- `02_image`
- `03_filter`
- `04_context`
- `05_detector`
- `06_color`
- `07_vector`
- `08_kernel`
- `09_barcode_descriptor`
- `10_image_processor`
- `11_feature`
- `12_filter_generator`
- `13_sampler`

Run them all with:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Coverage audit

See [COVERAGE.md](COVERAGE.md) for the framework-header audit, implemented surfaces, and deferred areas such as plug-ins, RAW filters, render destinations/tasks, filter constructors, and image accumulators/providers.

## Roadmap

- [x] `CIImage`, `CIFilter`, `CIContext`, `CIVector`, `CIColor`
- [x] `CIDetector`, `CIFeature`, QR feature/message inspection
- [x] `CIBarcodeDescriptor`, `CISampler`, `CIFilterGenerator`, `CIImageProcessor`
- [x] Core kernel coverage (`CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`)
- [x] Common built-in filter helpers (`filters` feature)
- [ ] `CIPlugIn` / `CIPlugInInterface`
- [ ] `CIFilterConstructor`, `CIFilterShape`, `CIImageAccumulator`, `CIImageProvider`
- [ ] `CIRAWFilter`, `CIRenderDestination`, and related render-task APIs

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
