# coreimage

Safe Rust bindings for Apple's [CoreImage](https://developer.apple.com/documentation/coreimage) framework — GPU-accelerated image processing, filtering, rendering, detection, and kernel work on macOS.

> **Status:** Every one of the 466 non-exempt symbols in the Core Image header audit (33 umbrella/deprecated/helper symbols exempt) has a typed Rust surface. The audit counts symbols, not methods: some classes are covered by their main entry points rather than every overload (see [COVERAGE.md](COVERAGE.md)). The safe surface also enforces asynchronous bitmap ownership, bitmap stride checks, validated dynamic filter inputs, bounds-checked image-processor buffers, and explicit region-of-interest contracts.

## Installation

```toml
[dependencies]
coreimage = "0.5"
```

Features: `filters` (default) adds the typed built-in filter constructors in `coreimage::filters`; `metal` adds Metal-backed contexts through [`apple-metal`](https://github.com/doom-fish/apple-metal-rs).

Requires macOS 11 or later, the Swift bridge's deployment target. APIs from later releases check the OS at run time: `CIKernel::kernels_from_metal_source` returns `CIError::Unsupported` before macOS 12, and the gain-map and headroom helpers (macOS 15 and 26) return the input image unchanged, or `0.0` for the headroom getters, on older systems.

## Highlights

- `CIImage` constructors from file paths, encoded bytes, `CGImage`, `CVPixelBuffer`, `IOSurface`, colors, and typed bitmap buffers via `CIFormat` / `CIColorSpace`
- `CIFilter` registry/localization helpers plus validated fallible setters, typed common inputs, typed apply/attribute/category/input/output/UI constants, and 157 typed `CIFilterBuiltins` constructors in `filters`
- `CIContext` creation for default, CPU, and optional Metal backends, plus `CGImage` rendering, task-owned bitmap-backed `CIRenderDestination` storage, and PNG/JPEG/HEIF/HEIF10/TIFF/OpenEXR export helpers
- `CIFilterShape`, `CIImageAccumulator`, and `CIRAWFilter` wrappers for shape math, incremental rendering, and RAW decoding workflows
- Detector + feature inspection coverage for faces, rectangles, QR codes, and text via `CIDetector` / `CIFeature`
- `CIColor`, `CIVector`, `CIBarcodeDescriptor`, `CISampler`, `CIFilterGenerator`, `CIFilterConstructor`, and `CIPlugIn`
- Metal kernels: `CIKernel`, `CIColorKernel`, `CIWarpKernel`, and `CIBlendKernel` load from compiled Core Image Metal libraries (`from_metal_library_data`) or, on macOS 12+, from Metal source (`kernels_from_metal_source`), and apply to any list of image, scalar, vector, and color arguments with caller-supplied region-of-interest closures
- `CIImageProcessorKernel`: custom CPU image processing in a Rust closure over bounds-checked input and output buffers
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

## Safety contracts

- `CIRenderTask` retains the native destination and bitmap allocation until completion. Dropping a task waits for completion; forgetting one leaks its ownership and keeps bitmap access closed rather than exposing storage still writable by native code.
- `CIRenderDestination::bitmap_data` and `bitmap_data_mut` return `Result` and reject access while a render or clear task is pending.
- `render_to_cv_pixel_buffer` and `render_to_iosurface` synchronously mutate native destination storage. `apple-cf` byte views are unsafe, and callers must exclude retained aliases and other native or GPU access while such a view exists.
- Dynamic `CIFilter` setters validate the filter's supported input keys and declared value classes, contain Objective-C KVC exceptions, and return `Result`. Common inputs such as image, background image, mask, radius, angle, intensity, center, color, message, and correction level have named setters.
- `CIImage::from_bitmap` rejects empty dimensions, rows shorter than `width × CIFormat::bytes_per_pixel()`, and buffers shorter than `bytes_per_row × height`; all size arithmetic is checked.
- `CIImage`, `CIContext`, `CIColor`, `CIVector`, and the kernel types are `Send + Sync`: Core Image documents contexts and images as immutable and shareable between threads, and marks the value and kernel classes `NS_SWIFT_SENDABLE`. Mutable types such as `CIFilter`, `CIImageAccumulator`, `CIRenderDestination`, `CIRAWFilter`, and `CIFilterGenerator` stay on one thread.
- `CIFilterConstructor`, `CIPlugInRegistration`, kernel region-of-interest closures, and `CIImageProcessorKernel` closures require `Send + Sync + 'static` captures, because Core Image may call them on its render threads for as long as an image built from them exists. Kernel and processor closures are owned by a reference-counted callback context that Core Image releases together with the image. Callback-body and caught panic-payload destruction are contained; a panicking region-of-interest closure falls back to that input's full extent. User closure state must still implement non-panicking `Drop`; safe Rust cannot recover from multiple destructor panics in one opaque aggregate.
- Kernel `apply` methods run behind the Objective-C exception boundary. Arguments that don't match the kernel's parameters return `CIError::NullResult`. Kernels compiled from Metal source only run on Metal-backed contexts whose device supports dynamic libraries. The Core Image Kernel Language constructors (`CIColorKernel::from_source`, `CIWarpKernel::from_source`) are deprecated, as they are in the SDK since macOS 10.14.
- `CIImageProcessorKernel` closures get byte views built from the region, pixel format, and stride Core Image reports: each view covers exactly `bytes_per_row × (height − 1) + width × bytes_per_pixel` bytes, `row(y)` returns `None` past the region, regions must be whole pixels, and input memory overlapping the output is refused. Returning `Err` or panicking zeroes the output tile and fails the render; render tasks report the closure's message, while `CIContext::render_to_cg_image` can still return the (transparent) image. Processors run on CPU memory; Metal textures and command buffers aren't exposed.
- `CIPlugIn::load_all_plugins` and `CIPlugIn::load_plugin` can load executable Image Units, which runs their native code inside the process, so they are `unsafe`. They are also deprecated, as they are in the SDK since macOS 10.15. `load_non_executable_plugins` and `load_non_executable_plugin` stay safe.
- `CIWarpKernel::apply_image_scalar` uses the full input extent as a conservative ROI. Use `apply_image_scalar_with_destination_roi` for known local kernels, `apply_image_scalar_with_roi` for an explicit callback, or `CIWarpKernel::apply` for other argument lists.
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
- `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, `CIKernel`: Metal library and Metal source kernels, general `apply` with `CIKernelArgument` lists and region-of-interest closures, class checks (`as_color_kernel`, `as_warp_kernel`, `as_blend_kernel`), built-in blend kernels, and the deprecated Core Image Kernel Language constructors
- `CIImageProcessorKernel`: closure-backed processors with `CIImageProcessorInputBuffer` / `CIImageProcessorOutputBuffer` views, optional region-of-interest closures and processor pixel formats
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

See [COVERAGE.md](COVERAGE.md) for the framework-header audit: 466/466 non-exempt symbols (33 exempt) have a typed Rust surface. That figure measures symbol presence in the headers of the macOS 26.2 SDK the audit was generated from, not per-method coverage; the file lists what is only partly wrapped, such as direct image-provider callbacks and GPU access inside image processors.

## Roadmap

- [x] `CIImage`, `CIFilter`, `CIContext`, `CIVector`, `CIColor`
- [x] `CIDetector`, `CIFeature`, QR feature/message inspection
- [x] `CIBarcodeDescriptor`, `CISampler`, `CIFilterGenerator`
- [x] Core kernel coverage (`CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, `CIKernel`), including Metal kernels and general argument lists
- [x] Closure-backed `CIImageProcessorKernel`
- [x] Builtin filter constructors + typed constant families (`CIFormat`, `CIColorSpace`, filter/input/output/apply/UI/exported/image-provider/sampler keys)
- [x] `CIFilterShape`, `CIImageAccumulator`, `CIRAWFilter`, `CIRenderDestination`, `CIRenderTask`
- [x] `CIPlugIn`, `CIPlugInRegistration`, `CIFilterConstructor`
- [x] Complete non-exempt public-symbol header audit (466/466)
- [ ] Optional future ergonomic expansions (`CIImageProvider` callback bridge, Metal texture access and multiple outputs in image processors, additional convenience APIs)

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
