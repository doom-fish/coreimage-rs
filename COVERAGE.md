# CoreImage coverage audit for `coreimage` 0.2.2

This document tracks the public CoreImage framework headers against the surfaces currently wrapped by this crate.

## Audit summary

- SDK public symbols: 499
- Verified (non-exempt): 466 / 466
- Exempt: 33
- Coverage: 100.00%

Audit-complete here means every non-exempt public symbol from the header audit has a typed Rust surface. It does **not** imply that every Objective-C method overload has a one-for-one ergonomic wrapper; some headers are represented by umbrella handles, typed constant families, or invocation snapshots.

## Implemented user-requested areas

| Area | Status | Rust surface | Example | Test |
| --- | --- | --- | --- | --- |
| `CIImage` | Implemented | `CIImage`, `CIFormat`, `CIColorSpace`, image option key enums | `02_image` | `tests/image.rs`, `tests/constants.rs` |
| `CIFilter` | Implemented | `CIFilter`, optional `filters` module, typed filter constant families, builtin constructor coverage | `03_filter` | `tests/filter.rs`, `tests/filter_builtins.rs` |
| `CIContext` | Implemented | `CIContext`, `CIContextOptions`, context option key enums | `04_context` | `tests/context.rs`, `tests/constants.rs` |
| `CIDetector` | Implemented | `CIDetector`, `CIDetectorOptions`, `CIDetectionOptions` | `05_detector` | `tests/detector.rs` |
| `CIColor` | Implemented | `CIColor`, `CIColorName` | `06_color` | `tests/color.rs` |
| `CIVector` | Implemented | `CIVector` | `07_vector` | `tests/vector.rs` |
| `CIKernel` | Implemented | `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, `CIKernel` | `08_kernel` | `tests/kernel.rs` |
| `CIBarcodeDescriptor` | Implemented | `CIBarcodeDescriptor`, `CIQRCodeErrorCorrectionLevel`, `CIDataMatrixCodeECCVersion` | `09_barcode_descriptor` | `tests/barcode_descriptor.rs` |
| `CIImageProcessor` | Implemented | `CIImageProcessor`, `CIImageProcessorInput`, `CIImageProcessorOutput`, `CIImageProcessorInvocation` | `10_image_processor` | `tests/image_processor.rs` |
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
| `CIContext.h` | Implemented | Core context creation/render/export helpers, typed option keys, working-format inspection, and bitmap-backed render-task helpers. Audit-complete; image-maximum-size accessors remain conservative macOS bridges. |
| `CIDetector.h` | Implemented | Detector creation + feature extraction for face/rectangle/QR/text detectors. |
| `CIFeature.h` | Implemented | Feature kind, bounds, JSON details, QR message/symbol descriptor, text sub-features. |
| `CIFilter.h` | Implemented | Filter construction, registry, localization, metadata, typed apply/attribute/category/input/output/UI constants, typed setters, and output image coverage. |
| `CIFilterBuiltins.h` | Implemented | `filters` exposes 157 typed constructors for instantiable built-ins plus family helpers for abstract protocols. |
| `CIFilterConstructor.h` | Implemented | Callback-backed `CIFilterConstructor` bridge plus `CIFilter::register_filter_name`. |
| `CIFilterGenerator.h` | Implemented | Graph creation/serialization/extraction helpers plus typed exported-key constants. |
| `CIFilterShape.h` | Implemented | Extent, transform, inset, union, and intersection helpers are wrapped. |
| `CIImage.h` | Implemented | File/data/color/bitmap creation, typed format/color-space constants, transforms, compositing, ROI helpers, and gain-map/headroom entry points with runtime availability checks. |
| `CIImageAccumulator.h` | Implemented | Creation, extent/format/image access, mutation, dirty-rect updates, and clear are wrapped. |
| `CIImageProcessor.h` | Implemented | Passthrough processor bridge for `CIImageProcessorKernel` plus typed input/output invocation snapshots. |
| `CIImageProvider.h` | Implemented | Typed `CIImageProviderOptionKey` coverage for the audited symbols in this header; a direct provider callback bridge would be future ergonomic work. |
| `CIKernel.h` | Implemented | `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, and shared `CIKernel` handle coverage. |
| `CIKernelMetalLib.h` | N/A | Metal shader helper header; not part of the audited Objective-C/Core Image symbol surface. |
| `CIPlugIn.h` | Implemented | `CIPlugIn` loading helpers are wrapped. |
| `CIPlugInInterface.h` | Implemented | `CIPlugInRegistration` callback bridge is wrapped. |
| `CIRAWFilter_Deprecated.h` | Exempt | Deprecated RAW filter constants remain intentionally excluded from the audit. |
| `CIRAWFilter.h` | Implemented | Practical RAW construction, decoder-version discovery, preview/output access, and common adjustment helpers cover every non-exempt audited symbol. |
| `CIRenderDestination.h` | Implemented | Bitmap-backed destinations, alpha mode, render tasks, and render info cover every non-exempt audited symbol. |
| `CISampler.h` | Implemented | Sampler creation, wrap/filter/color-space modes, affine transform, extent queries. |
| `CIVector.h` | Implemented | Scalar/point/rect/transform creation, indexed access, geometry round-trips. |
| `CoreImage.h` / `CoreImageDefines.h` | N/A | Umbrella / macro headers, not standalone wrapper targets. |

## Notes

- The audit counts dedicated typed Rust surfaces for SDK symbols. Generic stringly-typed escape hatches such as `CIFilter::new(name)` do not count unless a matching typed wrapper exists.
- `CIImageProcessorInput` and `CIImageProcessorOutput` are surfaced as typed invocation snapshots captured by the passthrough processor bridge, rather than as long-lived opaque protocol objects.
- `CIImageProvider.h` is audit-complete via typed provider-option constants; a direct callback bridge and `CIKernelMetalLib` helpers remain optional ergonomic follow-up work outside the non-exempt symbol audit.

## Verification

The 0.2.2 surface was verified with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test --quiet
```
