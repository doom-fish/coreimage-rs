# coreimage

Safe Rust bindings for Apple's [CoreImage](https://developer.apple.com/documentation/coreimage) framework — GPU-accelerated image processing, filtering, rendering, detection, and kernel work on macOS.

> **Status:** v0.2.2 closes every non-exempt gap in the Core Image header audit: 466/466 verified public symbols (100%), with 33 exempt umbrella/deprecated/helper symbols. This release adds the long-tail `CIFilterBuiltins` families, typed `CIFilter`/`CIFilterGenerator`/`CIImageProvider`/`CISampler` constant families, `CIFilterConstructor`, `CIPlugIn` / `CIPlugInRegistration`, a shared `CIKernel` handle, and typed `CIImageProcessor` invocation snapshots.

## Highlights

- `CIImage` constructors from file paths, encoded bytes, `CGImage`, `CVPixelBuffer`, `IOSurface`, colors, and typed bitmap buffers via `CIFormat` / `CIColorSpace`
- `CIFilter` registry/localization helpers plus typed setters, typed apply/attribute/category/input/output/UI constants, and 157 typed `CIFilterBuiltins` constructors in `filters`, plus family helpers for abstract builtin protocols
- `CIContext` creation for default, CPU, and optional Metal backends, plus `CGImage` rendering, bitmap-backed `CIRenderDestination` / `CIRenderTask`, and PNG/JPEG/HEIF/HEIF10/TIFF/OpenEXR export helpers
- `CIFilterShape`, `CIImageAccumulator`, and `CIRAWFilter` wrappers for shape math, incremental rendering, and RAW decoding workflows
- Detector + feature inspection coverage for faces, rectangles, QR codes, and text via `CIDetector` / `CIFeature`
- `CIColor`, `CIVector`, `CIBarcodeDescriptor`, `CISampler`, `CIFilterGenerator`, `CIFilterConstructor`, `CIPlugIn`, and `CIImageProcessor` wrappers for graph-building, registration, and plug-in workflows
- `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, and `CIKernel` support, plus shared CoreFoundation/CoreGraphics/CoreVideo/IOSurface interop via [`apple-cf`](https://github.com/doom-fish/apple-cf-rs)

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

- `CIImage` + typed constants: loading, color/bitmap creation with `CIFormat` / `CIColorSpace`, geometric transforms, compositing, blur, gain-map/headroom helpers, and region-of-interest inspection
- `CIFilter` + `filters`: filter discovery, localization, metadata, typed apply/attribute/category/input/output/UI constants, barcode-descriptor inputs, 157 instantiable built-in constructors, and helpers for abstract builtin families
- `CIContext` + `CIRenderDestination` / `CIRenderTask`: default/CPU/Metal contexts, rendering to `CGImage`/`CVPixelBuffer`/`IOSurface`/bitmap destinations, cache management, async render-task helpers, and file output
- `CIFilterShape` + `CIImageAccumulator`: shape transforms/intersections and incremental image accumulation
- `CIRAWFilter`: supported camera-model/decoder discovery plus preview/output image access and common RAW adjustments
- `CIDetector` + `CIFeature`: detector construction plus QR/face/rectangle/text feature inspection, message strings, symbol descriptors, and sub-features
- `CIColor` + `CIVector`: structured value wrappers for graph inputs, geometry, and transform round-tripping
- `CIBarcodeDescriptor`: QR/Aztec/PDF417/Data Matrix descriptor construction and inspection
- `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, `CIKernel`: custom kernel compilation, built-in blend kernels, and shared kernel handles
- `CIImageProcessor`: passthrough processor bridge with typed `CIImageProcessorInput` / `CIImageProcessorOutput` invocation snapshots
- `CIFilterGenerator`, `CIFilterConstructor`, `CIPlugIn`, `CISampler`: graph composition/export helpers, custom filter registration, plug-in loading, exported-key constants, and sampler configuration

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
- `14_accumulator`
- `15_render_destination`
- `16_raw_filter`

Run them all with:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Coverage audit

See [COVERAGE.md](COVERAGE.md) for the framework-header audit, current 466/466 non-exempt symbol coverage (100%, 33 exempt), and notes on follow-up ergonomic work such as direct image-provider callbacks or `CIKernelMetalLib` helpers.

## Roadmap

- [x] `CIImage`, `CIFilter`, `CIContext`, `CIVector`, `CIColor`
- [x] `CIDetector`, `CIFeature`, QR feature/message inspection
- [x] `CIBarcodeDescriptor`, `CISampler`, `CIFilterGenerator`, `CIImageProcessor`
- [x] Core kernel coverage (`CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, `CIKernel`)
- [x] Builtin filter constructors + typed constant families (`CIFormat`, `CIColorSpace`, filter/input/output/apply/UI/exported/image-provider/sampler keys)
- [x] `CIFilterShape`, `CIImageAccumulator`, `CIRAWFilter`, `CIRenderDestination`, `CIRenderTask`
- [x] `CIPlugIn`, `CIPlugInRegistration`, `CIFilterConstructor`
- [x] Complete non-exempt public-symbol header audit (466/466)
- [ ] Optional future ergonomic expansions (`CIImageProvider` callback bridge, `CIKernelMetalLib` helpers, additional convenience APIs)

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
