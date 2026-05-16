# Changelog

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

### Planned

- Legacy detector coverage (`CIDetector`, `CIFeature`, `CIFaceFeature`, `CIQRCodeFeature`).
- `CIPlugIn` support for custom filter bundles.
