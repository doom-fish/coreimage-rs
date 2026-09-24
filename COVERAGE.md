# CoreImage coverage audit

This document tracks the public CoreImage framework headers against the surfaces currently wrapped by this crate.

## Audit summary

- SDK public symbols: 499
- Verified (non-exempt): 466 / 466
- Exempt: 33
- Coverage: 100.00%

Audit-complete here means every non-exempt public symbol from the header audit has a typed Rust surface. It does **not** imply that every Objective-C method overload has a one-for-one ergonomic wrapper; some headers are represented by umbrella handles or typed constant families. The audit tables were generated against MacOSX26.2.sdk and have not been regenerated against the 26.5 or 27.0 SDKs. Before 0.5.0, `CIImageProcessorKernel` was counted through a hard-coded passthrough test kernel that users could not replace; it is now backed by user closures, and the passthrough is removed.

## Semantic correctness notes

- `CIImage::from_bitmap` requires non-zero dimensions, `bytes_per_row >= width × CIFormat::bytes_per_pixel()` and `data.len() >= bytes_per_row × height`, with checked arithmetic, so Core Image never reads rows past the supplied bytes.
- Bitmap-backed `CIRenderDestination` storage is retained by each render task until native completion. Safe byte access is unavailable while a task is pending, and task drop is a completion barrier.
- Synchronous `CVPixelBuffer` and `IOSurface` rendering is documented as a native write that must not overlap unsafe CPU views or retained/foreign aliases accessing the same storage.
- Dynamic filter inputs are checked against `inputKeys` and `kCIAttributeClass`; Objective-C KVC exceptions are converted to `CIError`.
- `CIImageProcessorKernel` closures see bounds-checked byte views derived from each region, format and stride; overlapping input and output memory, fractional or non-finite regions and short strides are refused. An `Err` or a panic zeroes the output tile and fails the render.
- Kernels load from compiled Core Image Metal libraries or, on macOS 12+, from Metal source, and apply to any list of image, scalar, vector and color arguments behind an Objective-C exception boundary. Region-of-interest closures are owned by a reference-counted callback context that Core Image releases with the image; a panicking closure falls back to the input's extent.
- Warp kernels' scalar helpers default to the full source extent, with explicit destination-rect and custom ROI callback variants.
- `CIImage`, `CIContext`, `CIColor`, `CIVector` and the kernel types are `Send + Sync` (CIContext.h documents contexts and images as immutable and thread-safe; the others are `NS_SWIFT_SENDABLE`). Mutable wrappers stay single-threaded.
- Sampler affine transforms use the native six-number array contract.
- Context maximum input/output size methods are unavailable in the macOS SDK and therefore return `CIError::Unsupported`; no successful zero-size value is fabricated.

## Implemented user-requested areas

| Area | Status | Rust surface | Example | Test |
| --- | --- | --- | --- | --- |
| `CIImage` | Implemented | `CIImage`, `CIFormat`, `CIColorSpace`, image option key enums | `02_image` | `tests/image.rs`, `tests/constants.rs` |
| `CIFilter` | Implemented | `CIFilter`, optional `filters` module, typed filter constant families, builtin constructor coverage | `03_filter` | `tests/filter.rs`, `tests/filter_builtins.rs` |
| `CIContext` | Implemented | `CIContext`, `CIContextOptions`, context option key enums | `04_context` | `tests/context.rs`, `tests/constants.rs` |
| `CIDetector` | Implemented | `CIDetector`, `CIDetectorOptions`, `CIDetectionOptions` | `05_detector` | `tests/detector.rs` |
| `CIColor` | Implemented | `CIColor`, `CIColorName` | `06_color` | `tests/color.rs` |
| `CIVector` | Implemented | `CIVector` | `07_vector` | `tests/vector.rs` |
| `CIKernel` | Implemented | `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, `CIKernel`, `CIKernelArgument` | `08_kernel` | `tests/kernel.rs`, `tests/metal_kernel.rs` |
| `CIBarcodeDescriptor` | Implemented | `CIBarcodeDescriptor`, `CIQRCodeErrorCorrectionLevel`, `CIDataMatrixCodeECCVersion` | `09_barcode_descriptor` | `tests/barcode_descriptor.rs` |
| `CIImageProcessor` | Implemented | `CIImageProcessorKernel`, `CIImageProcessorInputBuffer`, `CIImageProcessorOutputBuffer` | `10_image_processor` | `tests/image_processor.rs` |
| `CIFeature` | Implemented | `CIFeature`, `CIFeatureKind` | `11_feature` | `tests/feature.rs` |
| `CIFilterGenerator` | Implemented | `CIFilterGenerator`, `CIFilterGeneratorExportedKey` | `12_filter_generator` | `tests/filter_generator.rs` |
| `CISampler` | Implemented | `CISampler`, `CISamplerOptions`, `CISamplerOptionKey` | `13_sampler` | `tests/sampler.rs` |
| `CIImageAccumulator` | Implemented | `CIImageAccumulator` | `14_accumulator` | `tests/image_accumulator.rs` |
| `CIRenderDestination` | Implemented | `CIRenderDestination`, `CIRenderTask`, `CIRenderInfo`, `CIRenderDestinationAlphaMode` | `15_render_destination` | `tests/render_destination.rs` |
| `CIRAWFilter` | Implemented | `CIRAWFilter`, `CIRAWDecoderVersion` | `16_raw_filter` | `tests/raw_filter.rs` |
| `CIFilterShape` | Implemented | `CIFilterShape` | — | `tests/filter_shape.rs` |
| `CIFilterConstructor` | Implemented | `CIFilterConstructor` | — | `tests/filter_constructor.rs` |
| `CIPlugIn` | Implemented | `CIPlugIn`, `CIPlugInRegistration` | — | `tests/plugin.rs` |

## Public-header audit

| Header | Audit status | Notes |
| --- | --- | --- |
| `CIBarcodeDescriptor.h` | Implemented | Constructors + inspection for QR/Aztec/PDF417/Data Matrix descriptors. |
| `CIColor.h` | Implemented | RGBA/named colors, parsing, component access, string conversion. |
| `CIContext.h` | Implemented | Core context creation/render/export helpers, typed option keys, working-format inspection, completion-safe bitmap render tasks, and explicit unavailability for maximum-size methods on macOS. |
| `CIDetector.h` | Implemented | Detector creation + feature extraction for face/rectangle/QR/text detectors. |
| `CIFeature.h` | Implemented | Feature kind, bounds, JSON details, QR message/symbol descriptor, text sub-features. |
| `CIFilter.h` | Implemented | Filter construction, registry, localization, metadata, typed apply/attribute/category/input/output/UI constants, validated fallible setters, and output image coverage. |
| `CIFilterBuiltins.h` | Implemented | `filters` exposes 157 typed constructors for instantiable built-ins plus family helpers for abstract protocols. |
| `CIFilterConstructor.h` | Implemented | Callback-backed `CIFilterConstructor` bridge plus `CIFilter::register_filter_name`. |
| `CIFilterGenerator.h` | Implemented | Graph creation/serialization/extraction helpers plus typed exported-key constants. |
| `CIFilterShape.h` | Implemented | Extent, transform, inset, union, and intersection helpers are wrapped. |
| `CIImage.h` | Implemented | File/data/color/bitmap creation, typed format/color-space constants, transforms, compositing, ROI helpers, and gain-map/headroom entry points with runtime availability checks. |
| `CIImageAccumulator.h` | Implemented | Creation, extent/format/image access, mutation, dirty-rect updates, and clear are wrapped. |
| `CIImageProcessor.h` | Implemented | Closure-backed `CIImageProcessorKernel` with CPU input/output buffers, region-of-interest closures and the six processor formats. Not wrapped: Metal texture and command-buffer access, `roiTileArrayForInput:`, and the multiple-output methods. |
| `CIImageProvider.h` | Implemented | Typed `CIImageProviderOptionKey` coverage for the audited symbols in this header; a direct provider callback bridge would be future ergonomic work. |
| `CIKernel.h` | Implemented | Metal library kernels (`kernelWithFunctionName:fromMetalLibraryData:` with optional output format, `kernelNamesFromMetalLibraryData:`), Metal source kernels (`kernelsWithMetalString:`, macOS 12+), general `apply` for kernel, color and warp kernels, class checks, built-in blend kernels, and the deprecated Core Image Kernel Language constructors. Not wrapped: `setROISelector:` and the blend `colorSpace:` variant. |
| `CIKernelMetalLib.h` | N/A | Metal-side header that kernel sources include (see `tests/fixtures/kernels.metal`); it declares nothing a Rust binding can call. |
| `CIPlugIn.h` | Implemented | `CIPlugIn` loading helpers are wrapped. The loaders that can run executable Image Units (`loadAllPlugIns`, `loadPlugIn:allowExecutableCode:`) are `unsafe` and deprecated, as in the SDK since macOS 10.15. |
| `CIPlugInInterface.h` | Implemented | `CIPlugInRegistration` callback bridge is wrapped. |
| `CIRAWFilter_Deprecated.h` | Exempt | Deprecated RAW filter constants remain intentionally excluded from the audit. |
| `CIRAWFilter.h` | Implemented | Practical RAW construction, decoder-version discovery, preview/output access, and common adjustment helpers cover every non-exempt audited symbol. |
| `CIRenderDestination.h` | Implemented | Bitmap-backed destinations with task-owned storage, guarded byte access, alpha mode, render tasks, and render info. |
| `CISampler.h` | Implemented | Sampler creation, wrap/filter/color-space modes, native six-number affine matrices, and extent queries. |
| `CIVector.h` | Implemented | Scalar/point/rect/transform creation, indexed access, geometry round-trips. |
| `CoreImage.h` / `CoreImageDefines.h` | N/A | Umbrella / macro headers, not standalone wrapper targets. |

## Notes

- The audit counts dedicated typed Rust surfaces for SDK symbols. Generic stringly-typed escape hatches such as `CIFilter::new(name)` do not count unless a matching typed wrapper exists.
- The `CIImageProcessorInput` and `CIImageProcessorOutput` protocols are surfaced as the borrowed `CIImageProcessorInputBuffer` / `CIImageProcessorOutputBuffer` views passed to processor closures rather than as long-lived opaque protocol objects.
- `CIImageProvider.h` is audit-complete via typed provider-option constants; a direct callback bridge remains optional ergonomic follow-up work outside the non-exempt symbol audit.

## Verification

The 0.5.0 surface was verified with:

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo +1.82.0 check --lib --all-features
swift test --package-path swift-bridge
```
