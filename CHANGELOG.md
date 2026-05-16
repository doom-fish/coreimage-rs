# Changelog

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
