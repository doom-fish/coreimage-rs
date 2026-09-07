# coreimage

Safe Rust bindings for Apple's [CoreImage](https://developer.apple.com/documentation/coreimage) framework — GPU-accelerated image processing, filtering, rendering, detection, and kernel work on macOS.

> **Status:** The crate covers all 466 non-exempt symbols in the Core Image header audit, with 33 umbrella/deprecated/helper symbols exempt. The safe surface also enforces asynchronous bitmap ownership, validated dynamic filter inputs, synchronized processor snapshots, and explicit warp region-of-interest contracts.

## Highlights

- `CIImage` constructors from file paths, encoded bytes, `CGImage`, `CVPixelBuffer`, `IOSurface`, colors, and typed bitmap buffers via `CIFormat` / `CIColorSpace`
- `CIFilter` registry/localization helpers plus validated fallible setters, typed common inputs, typed apply/attribute/category/input/output/UI constants, and 157 typed `CIFilterBuiltins` constructors in `filters`
- `CIContext` creation for default, CPU, and optional Metal backends, plus `CGImage` rendering, task-owned bitmap-backed `CIRenderDestination` storage, and PNG/JPEG/HEIF/HEIF10/TIFF/OpenEXR export helpers
- `CIFilterShape`, `CIImageAccumulator`, and `CIRAWFilter` wrappers for shape math, incremental rendering, and RAW decoding workflows
- Detector + feature inspection coverage for faces, rectangles, QR codes, and text via `CIDetector` / `CIFeature`
- `CIColor`, `CIVector`, `CIBarcodeDescriptor`, `CISampler`, `CIFilterGenerator`, `CIFilterConstructor`, `CIPlugIn`, and atomically transferred `CIImageProcessor` invocation snapshots
- `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, and `CIKernel` support, including conservative and caller-supplied warp ROI contracts, plus shared CoreFoundation/CoreGraphics/CoreVideo/IOSurface interop via [`apple-cf`](https://github.com/doom-fish/apple-cf-rs)

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

## Safety contracts

- `CIRenderTask` retains the native destination and bitmap allocation until completion. Dropping a task waits for completion; forgetting one leaks its ownership and keeps bitmap access closed rather than exposing storage still writable by native code.
- `CIRenderDestination::bitmap_data` and `bitmap_data_mut` return `Result` and reject access while a render or clear task is pending.
- `render_to_cv_pixel_buffer` and `render_to_iosurface` synchronously mutate native destination storage. `apple-cf` byte views are unsafe, and callers must exclude retained aliases and other native or GPU access while such a view exists.
- Dynamic `CIFilter` setters validate the filter's supported input keys and declared value classes, contain Objective-C KVC exceptions, and return `Result`. Common inputs such as image, background image, mask, radius, angle, intensity, center, color, message, and correction level have named setters.
- `CIFilterConstructor`, `CIPlugInRegistration`, and warp ROI callbacks require `Send + Sync` captures. Callback-body and caught panic-payload destruction are contained, and library-owned cleanup runs in separate guarded phases. User closure state must still implement non-panicking `Drop`; safe Rust cannot recover from multiple destructor panics in one opaque aggregate.
- `CIImageProcessor::last_invocation` reads every field from one locked, retained snapshot.
- `CIWarpKernel::apply_image_scalar` uses the full input extent as a conservative ROI. Use `apply_image_scalar_with_destination_roi` for known local kernels or `apply_image_scalar_with_roi` for an explicit callback.
- EXIF orientation methods return `Result` and accept only values 1 through 8. Context maximum-size queries return `Unsupported` on macOS rather than reporting fabricated zero dimensions.

## Surface overview

- `CIImage` + typed constants: loading, color/bitmap creation with `CIFormat` / `CIColorSpace`, validated EXIF orientation, geometric transforms, compositing, blur, gain-map/headroom helpers, and region-of-interest inspection
- `CIFilter` + `filters`: filter discovery, localization, metadata, validated typed inputs, 157 instantiable built-in constructors, and helpers for abstract builtin families
- `CIContext` + `CIRenderDestination` / `CIRenderTask`: default/CPU/Metal contexts, rendering to `CGImage`/`CVPixelBuffer`/`IOSurface`/owned bitmap destinations, cache management, completion-safe render tasks, and file output
- `CIFilterShape` + `CIImageAccumulator`: shape transforms/intersections and incremental image accumulation
- `CIRAWFilter`: supported camera-model/decoder discovery plus preview/output image access and common RAW adjustments
- `CIDetector` + `CIFeature`: detector construction plus QR/face/rectangle/text feature inspection, message strings, symbol descriptors, and sub-features
- `CIColor` + `CIVector`: structured value wrappers for graph inputs, geometry, and transform round-tripping
- `CIBarcodeDescriptor`: QR/Aztec/PDF417/Data Matrix descriptor construction and inspection
- `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, `CIKernel`: custom kernel compilation, built-in blend kernels, explicit warp ROI selection, and shared kernel handles
- `CIImageProcessor`: passthrough processor bridge with coherent typed `CIImageProcessorInput` / `CIImageProcessorOutput` invocation snapshots
- `CIFilterGenerator`, `CIFilterConstructor`, `CIPlugIn`, `CISampler`: graph composition/export helpers, thread-safe custom filter registration, plug-in loading, exported-key constants, and native affine-matrix sampler configuration

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

See [COVERAGE.md](COVERAGE.md) for the framework-header audit, current 466/466 non-exempt symbol coverage (100%, 33 exempt), semantic correctness notes, and follow-up ergonomic work such as direct image-provider callbacks or `CIKernelMetalLib` helpers.

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
