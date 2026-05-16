# CoreImage coverage audit for `coreimage` 0.2.1

This document tracks the public CoreImage framework headers against the surfaces currently wrapped by this crate.

## Implemented user-requested areas

| Area | Status | Rust surface | Example | Test |
| --- | --- | --- | --- | --- |
| `CIImage` | Implemented | `CIImage`, `CIFormat`, `CIColorSpace`, image option key enums | `02_image` | `tests/image.rs`, `tests/constants.rs` |
| `CIFilter` | Partial | `CIFilter`, optional `filters` module, builtin constructor coverage | `03_filter` | `tests/filter.rs`, `tests/filter_builtins.rs` |
| `CIContext` | Partial | `CIContext`, `CIContextOptions`, context option key enums | `04_context` | `tests/context.rs`, `tests/constants.rs` |
| `CIDetector` | Implemented | `CIDetector`, `CIDetectorOptions`, `CIDetectionOptions` | `05_detector` | `tests/detector.rs` |
| `CIColor` | Implemented | `CIColor`, `CIColorName` | `06_color` | `tests/color.rs` |
| `CIVector` | Implemented | `CIVector` | `07_vector` | `tests/vector.rs` |
| `CIKernel` | Partial | `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel` | `08_kernel` | `tests/kernel.rs` |
| `CIBarcodeDescriptor` | Implemented | `CIBarcodeDescriptor`, `CIQRCodeErrorCorrectionLevel`, `CIDataMatrixCodeECCVersion` | `09_barcode_descriptor` | `tests/barcode_descriptor.rs` |
| `CIImageProcessor` | Partial | `CIImageProcessor` passthrough bridge | `10_image_processor` | `tests/image_processor.rs` |
| `CIFeature` | Implemented | `CIFeature`, `CIFeatureKind` | `11_feature` | `tests/feature.rs` |
| `CIFilterGenerator` | Partial | `CIFilterGenerator` | `12_filter_generator` | `tests/filter_generator.rs` |
| `CISampler` | Implemented | `CISampler`, `CISamplerOptions` | `13_sampler` | `tests/sampler.rs` |
| `CIImageAccumulator` | Implemented | `CIImageAccumulator` | `14_accumulator` | `tests/image_accumulator.rs` |
| `CIRenderDestination` | Partial | `CIRenderDestination`, `CIRenderTask`, `CIRenderInfo`, `CIRenderDestinationAlphaMode` | `15_render_destination` | `tests/render_destination.rs` |
| `CIRAWFilter` | Partial | `CIRAWFilter`, `CIRAWDecoderVersion` | `16_raw_filter` | `tests/raw_filter.rs` |
| `CIFilterShape` | Implemented | `CIFilterShape` | — | `tests/filter_shape.rs` |

## Public-header audit

| Header | Status | Notes |
| --- | --- | --- |
| `CIBarcodeDescriptor.h` | Implemented | Constructors + inspection for QR/Aztec/PDF417/Data Matrix descriptors. |
| `CIColor.h` | Implemented | RGBA/named colors, parsing, component access, string conversion. |
| `CIContext.h` | Partial | Core context creation/render/export helpers are wrapped, plus typed option keys, working-format inspection, and bitmap-backed render-task helpers. The image-maximum-size accessors are still conservative/stubbed on macOS due Swift import limitations. |
| `CIDetector.h` | Implemented | Detector creation + feature extraction for face/rectangle/QR/text detectors. |
| `CIFeature.h` | Implemented | Feature kind, bounds, JSON details, QR message/symbol descriptor, text sub-features. |
| `CIFilter.h` | Partial | Filter construction, registry, localization, metadata, typed setters, and output image are wrapped, but many header constant families remain to be modeled explicitly. |
| `CIFilterBuiltins.h` | Partial | `filters` now exposes typed constructors for 149 instantiable built-in protocols; the remaining gap is mostly abstract/non-instantiable protocol coverage. |
| `CIFilterConstructor.h` | Deferred | Not yet wrapped. |
| `CIFilterGenerator.h` | Partial | Core graph creation/serialization/extraction helpers are wrapped, but exported-key constant families remain open in the symbol audit. |
| `CIFilterShape.h` | Implemented | Extent, transform, inset, union, and intersection helpers are wrapped. |
| `CIImage.h` | Implemented | File/data/color/bitmap creation, typed format/color-space constants, transforms, compositing, ROI helpers, and gain-map/headroom entry points with runtime availability checks. |
| `CIImageAccumulator.h` | Implemented | Creation, extent/format/image access, mutation, dirty-rect updates, and clear are wrapped. |
| `CIImageProcessor.h` | Partial | The crate ships a passthrough processor bridge for exercising `CIImageProcessorKernel`, but not the full image-processor protocol surface. |
| `CIImageProvider.h` | Deferred | Not yet wrapped. |
| `CIKernel.h` | Partial | Color/warp/blend kernel wrappers are included; the full general-purpose argument/ROI/render-task surface is not. |
| `CIKernelMetalLib.h` | Deferred | Not yet wrapped. |
| `CIPlugIn.h` | Deferred | Not yet wrapped. |
| `CIPlugInInterface.h` | Deferred | Not yet wrapped. |
| `CIRAWFilter_Deprecated.h` | Deferred | Deprecated RAW filter constants remain intentionally unwrapped. |
| `CIRAWFilter.h` | Partial | High-value construction, decoder-version discovery, preview/output image access, and common adjustment properties are wrapped; the full property matrix is still open. |
| `CIRenderDestination.h` | Partial | Bitmap-backed destinations, alpha mode, render tasks, and render info are wrapped; pixel-buffer/IOSurface/Metal/OpenGL initializers and advanced destination properties remain open. |
| `CISampler.h` | Implemented | Sampler creation, wrap/filter modes, affine transform, extent queries. |
| `CIVector.h` | Implemented | Scalar/point/rect/transform creation, indexed access, geometry round-trips. |
| `CoreImage.h` / `CoreImageDefines.h` | N/A | Umbrella / macro headers, not standalone wrapper targets. |

## Notes on partial areas

- `CIContext`: the high-value rendering/export path is covered, along with working-format inspection and render-task helpers, but `input_image_maximum_size` and `output_image_maximum_size` still need a better macOS-specific bridge.
- `CIFilter` / `CIFilterGenerator`: remaining symbol gaps are concentrated in constant families rather than core object wrappers.
- `CIFilterBuiltins`: the crate now covers the instantiable protocols directly; the remaining gaps are largely abstract protocols that do not map cleanly to standalone filter construction.
- `CIRAWFilter`: the wrapper focuses on the practical live surface and does not yet expose every adjustment key from the header.
- `CIRenderDestination`: only bitmap-backed destinations are wrapped today.
- `CIKernel` and `CIImageProcessor`: the crate still focuses on pragmatic entry points that are easy to drive safely from Rust.

## Verification

The 0.2.1 surface was verified with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test --quiet
```
