# CoreImage coverage audit for `coreimage` 0.2.0

This document tracks the public CoreImage framework headers against the surfaces currently wrapped by this crate.

## Implemented user-requested areas

| Area | Status | Rust surface | Example | Test |
| --- | --- | --- | --- | --- |
| `CIImage` | Implemented | `CIImage` | `02_image` | `tests/image.rs` |
| `CIFilter` | Implemented | `CIFilter`, optional `filters` module | `03_filter` | `tests/filter.rs` |
| `CIContext` | Partial | `CIContext`, `CIContextOptions` | `04_context` | `tests/context.rs` |
| `CIDetector` | Implemented | `CIDetector`, `CIDetectorOptions`, `CIDetectionOptions` | `05_detector` | `tests/detector.rs` |
| `CIColor` | Implemented | `CIColor`, `CIColorName` | `06_color` | `tests/color.rs` |
| `CIVector` | Implemented | `CIVector` | `07_vector` | `tests/vector.rs` |
| `CIKernel` | Partial | `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel` | `08_kernel` | `tests/kernel.rs` |
| `CIBarcodeDescriptor` | Implemented | `CIBarcodeDescriptor`, `CIQRCodeErrorCorrectionLevel`, `CIDataMatrixCodeECCVersion` | `09_barcode_descriptor` | `tests/barcode_descriptor.rs` |
| `CIImageProcessor` | Partial | `CIImageProcessor` passthrough bridge | `10_image_processor` | `tests/image_processor.rs` |
| `CIFeature` | Implemented | `CIFeature`, `CIFeatureKind` | `11_feature` | `tests/feature.rs` |
| `CIFilterGenerator` | Implemented | `CIFilterGenerator` | `12_filter_generator` | `tests/filter_generator.rs` |
| `CISampler` | Implemented | `CISampler`, `CISamplerOptions` | `13_sampler` | `tests/sampler.rs` |

## Public-header audit

| Header | Status | Notes |
| --- | --- | --- |
| `CIBarcodeDescriptor.h` | Implemented | Constructors + inspection for QR/Aztec/PDF417/Data Matrix descriptors. |
| `CIColor.h` | Implemented | RGBA/named colors, parsing, component access, string conversion. |
| `CIContext.h` | Partial | Core context creation/render/export helpers are wrapped. `workingFormat` and image-maximum-size accessors are currently conservative/stubbed on macOS due API/import limitations. |
| `CIDetector.h` | Implemented | Detector creation + feature extraction for face/rectangle/QR/text detectors. |
| `CIFeature.h` | Implemented | Feature kind, bounds, JSON details, QR message/symbol descriptor, text sub-features. |
| `CIFilter.h` | Implemented | Filter construction, registry, localization, metadata, typed setters, output image. |
| `CIFilterBuiltins.h` | Partial | `filters` covers a common practical subset; anything else can still be reached through raw `CIFilter::new(name)`. |
| `CIFilterConstructor.h` | Deferred | Not yet wrapped. |
| `CIFilterGenerator.h` | Implemented | Creation, graph connections, export keys, serialization, filter extraction. |
| `CIFilterShape.h` | Deferred | Not yet wrapped. |
| `CIImage.h` | Implemented | File/data/color/bitmap creation, transforms, compositing, ROI helpers, gain-map/headroom entry points with runtime availability checks. |
| `CIImageAccumulator.h` | Deferred | Not yet wrapped. |
| `CIImageProcessor.h` | Partial | The crate ships a passthrough processor bridge for exercising `CIImageProcessorKernel`, but not the full image-processor protocol surface. |
| `CIImageProvider.h` | Deferred | Not yet wrapped. |
| `CIKernel.h` | Partial | Color/warp/blend kernel wrappers are included; the full general-purpose argument/ROI/render-task surface is not. |
| `CIKernelMetalLib.h` | Deferred | Not yet wrapped. |
| `CIPlugIn.h` | Deferred | Not yet wrapped. |
| `CIPlugInInterface.h` | Deferred | Not yet wrapped. |
| `CIRAWFilter_Deprecated.h` | Deferred | Not wrapped. |
| `CIRAWFilter.h` | Deferred | Not wrapped. |
| `CIRenderDestination.h` | Deferred | Not wrapped. |
| `CISampler.h` | Implemented | Sampler creation, wrap/filter modes, affine transform, extent queries. |
| `CIVector.h` | Implemented | Scalar/point/rect/transform creation, indexed access, geometry round-trips. |
| `CoreImage.h` / `CoreImageDefines.h` | N/A | Umbrella / macro headers, not standalone wrapper targets. |

## Notes on partial areas

- `CIContext`: the high-value rendering/export path is covered, but `working_format`, `input_image_maximum_size`, and `output_image_maximum_size` need a better macOS-specific bridge.
- `CIKernel`: the crate currently focuses on pragmatic kernel entry points that are easy to drive safely from Rust.
- `CIImageProcessor`: only a built-in passthrough processor bridge is exposed today.
- `CIFilterBuiltins`: the convenience `filters` module remains intentionally curated rather than auto-generated.

## Verification

The 0.2.0 surface was verified with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```
