# Changelog

## [0.3.1] - 2026-05-18

- Widen apple-metal version bound so the 0.x bump dep resolves. No source changes.

## [0.3.0] - 2026-05-18

### Breaking

- Raised `apple-cf` support to `>=0.7, <0.9` and migrated crate-wide `CGRect` usage to the nested `origin`/`size` representation introduced by `apple-cf 0.8.0`.

## [0.2.3] - 2026-05-18

### Fixed

- Added comprehensive SAFETY comments to all unsafe blocks explaining pointer validity,
  lifetime semantics, and memory management guarantees from the FFI layer. This improves
  auditability of the 332 unsafe FFI calls throughout the crate.

## [0.2.2] - 2026-05-17

### Added

- Added typed `CIFilter` constant families for apply options, attribute keys/types, filter categories, dynamic-range values, input/output keys, and UI parameter sets.
- Added typed constant coverage for `CIFilterGeneratorExportedKey`, `CIImageProviderOptionKey`, and `CISamplerOptionKey::ColorSpace`.
- Added long-tail builtin coverage with 157 typed `CIFilterBuiltins` constructors plus family helpers for abstract protocols such as `CICompositeOperation`, `CIFourCoordinateGeometryFilter`, and `CITransitionFilter`.
- Added safe wrappers for `CIFilterConstructor`, `CIPlugIn`, `CIPlugInRegistration`, and shared `CIKernel` handles.
- Expanded `CIImageProcessor` with typed `CIImageProcessorInput` / `CIImageProcessorOutput` invocation snapshots.

### Changed

- `CIImageProcessor::apply_passthrough` now forces a render so invocation snapshots are populated deterministically for tests and examples.
- Updated `README.md`, `COVERAGE.md`, and `COVERAGE_AUDIT.md` to reflect 466 audited non-exempt public symbols (100% coverage, 33 exempt).

## [0.2.1] - 2026-05-16

### Added

- Added typed `CIFilterBuiltins` constructors for 149 instantiable built-in protocols, with regression coverage in `tests/filter_builtins.rs`.
- Added typed Core Image constant families: `CIColorSpace`, `CIFormat`, `CIContextOptionKey`, `CIImageOptionKey`, `CIImageAutoAdjustmentOptionKey`, and `CIImageRepresentationOptionKey`.
- Added safe wrappers for `CIFilterShape`, `CIImageAccumulator`, `CIRAWFilter`, `CIRAWDecoderVersion`, `CIRenderDestination`, `CIRenderTask`, `CIRenderInfo`, and `CIRenderDestinationAlphaMode`.
- Added examples `14_accumulator`, `15_render_destination`, and `16_raw_filter`, plus integration tests for the new wrappers and constants.

### Changed

- Generalized `CIImage::from_bitmap(...)` to accept explicit `CIFormat`, `bytes_per_row`, and optional `CIColorSpace`, while keeping `from_bitmap_rgba8(...)` as a convenience helper.
- Expanded `CIContextOptions` and `CIContext` with additional typed option coverage, working-format inspection, and bitmap-backed render-task helpers.
- Updated `COVERAGE.md`, `README.md`, and `COVERAGE_AUDIT.md` to reflect the broader 0.2.1 surface and 339 verified public symbols (72.75%).

## [0.2.0] - 2026-05-16

### Added

- Split the Swift bridge into logical area files for image, filter, context, detector, feature, color, vector, kernel, barcode descriptor, image processor, filter generator, and sampler coverage.
- Reorganized Rust FFI declarations into `src/ffi/` per-area modules and added safe wrappers for `CIDetector`, `CIFeature`, `CIBarcodeDescriptor`, `CISampler`, `CIFilterGenerator`, `CIColorKernel`, `CIWarpKernel`, `CIBlendKernel`, and `CIImageProcessor`.
- Expanded `CIImage`, `CIFilter`, `CIContext`, `CIColor`, and `CIVector` with broader constructors, transforms, metadata, export helpers, and value accessors.
- Added per-area examples `02_image` through `13_sampler` and integration tests covering every shipped area.
- Added `COVERAGE.md` with a CoreImage framework header audit and deferred-surface notes.

### Changed

- Updated crate documentation for the broader v0.2.0 surface.
- `CIContext::with_options` now borrows `CIContextOptions` instead of consuming it.
- Detector option structs are `Copy`, making detector configuration cheaper to reuse.

### Deferred

- `CIPlugIn` / `CIPlugInInterface`
- `CIFilterConstructor`, `CIFilterShape`, `CIImageAccumulator`, `CIImageProvider`
- `CIKernelMetalLib`, `CIRAWFilter`, `CIRenderDestination`, and related render-task APIs

## [0.1.0] - 2026-05-16

### Added

- Initial `coreimage` release for macOS CoreImage pipelines.
- `CIImage` creation from file paths, encoded image data, `CGImage`, `CVPixelBuffer`, `IOSurface`, and RGBA bitmap buffers.
- `CIImage` transforms and variants: extent, properties JSON, crop, translate, scale, affine transform, orientation, compositing, and filter application.
- `CIFilter` registry helpers, input/output-key inspection, JSON attributes, typed value setters, and output-image access.
- `CIContext` creation for default, CPU, Metal device, and Metal command-queue backends.
- `CIContext` rendering to `CGImage`, `CVPixelBuffer`, `IOSurface`, plus PNG/JPEG/HEIF/TIFF file output.
- `CIVector` and `CIColor` safe wrappers for common filter input types.
- `filters` module with common built-in helpers for blur, sharpen, color adjustment, edge detection, perspective correction, compositing, gradients, and generators.
- Smoke example `examples/01_smoke.rs` covering image generation, Gaussian blur, render-to-`CGImage`, and PNG export.
