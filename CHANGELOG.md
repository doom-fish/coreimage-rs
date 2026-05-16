# Changelog

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
