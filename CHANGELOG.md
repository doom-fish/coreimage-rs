# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - Unreleased

### Security

- `CIImage::from_bitmap` accepted a `bytes_per_row` shorter than `width × bytes_per_pixel`, so Core Image could read past the copied bytes when rendering (for example 4 bytes declared as a 1000-pixel RGBA8 row). It now rejects short strides using `CIFormat::bytes_per_pixel`, rejects zero width or height, keeps every size computation checked, and the Swift bridge repeats the length check.

### Fixed

- `CIContext::with_options` returns `CIError::NullResult` instead of panicking when Core Image returns no context.
- The callback trampolines use the `doom-fish-utils` panic helpers instead of a private copy; panic diagnostics now come from `doom-fish-utils`.
- Framework errors carry Core Image's own description (`CINonLocalizedDescriptionKey` and its underlying errors) instead of "The operation couldn't be completed".
- README, COVERAGE and the audit tables no longer count `CIImageProcessorKernel` as covered by the hard-coded passthrough test kernel, state that the audit counts symbols from the macOS 26.2 SDK rather than methods, and list what `CIKernel.h` and `CIImageProcessor.h` still leave unwrapped. The README now states the macOS 11 minimum and how later APIs behave on older systems.
- The Swift bridge checks the class of every handle it borrows. A handle of the wrong class gets the call's empty or error result instead of messages sent to an object of another class, which aborted the process with an uncaught Objective-C exception.
- Rendering to a `CVPixelBuffer` or `IOSurface` and writing image files return `CIError::NullResult` if the sRGB color space can't be created, instead of trapping on a force unwrap.
- The Swift bridge no longer contains `ci_block_on_async`, an unused helper whose status was written from its `Task` without synchronization and whose timeout returned while the `Task` kept running, nor the error case that only it produced.

### Changed

- **Breaking:** `CIPlugIn::load_all_plugins` and `CIPlugIn::load_plugin` are `unsafe`, because they can load executable Image Units, which runs the plug-in's native code inside the process. `load_non_executable_plugins` and `load_non_executable_plugin` stay safe.
- **Breaking:** requires `apple-cf >=0.11, <0.12` and, for the `metal` feature, `apple-metal >=0.10, <0.11`; both appear in the public API (`CGImage`, `CGRect`, `CVPixelBuffer`, `IOSurface`, `MetalDevice`, `CommandQueue`).
- **Breaking:** `rust-version` is 1.82 (was 1.76); `doom-fish-utils >=0.4.1, <0.5` is a new dependency.
- The warp region-of-interest callback of `CIWarpKernel::apply_image_scalar_with_roi` is now owned by a reference-counted callback context shared with the new kernel and processor APIs; behaviour is unchanged.

### Deprecated

- `CIColorKernel::from_source` and `CIWarpKernel::from_source`: the Core Image Kernel Language has been deprecated since macOS 10.14. Use `from_metal_library_data`.
- `CIPlugIn::load_all_plugins` and `CIPlugIn::load_plugin`: loading executable plug-ins has been deprecated since macOS 10.15. Use `load_non_executable_plugins` and `load_non_executable_plugin`.

### Added

- Metal kernels: `from_metal_library_data(function_name, data, output_format)` on `CIKernel`, `CIColorKernel`, `CIWarpKernel` and `CIBlendKernel` (`kernelWithFunctionName:fromMetalLibraryData:` and its `outputPixelFormat:` variant), `CIKernel::kernel_names_from_metal_library_data`, and `CIKernel::kernels_from_metal_source` (macOS 12+, `CIError::Unsupported` before).
- `CIKernel::as_color_kernel`, `as_warp_kernel` and `as_blend_kernel`, because loading through `CIKernel` can return a subclass.
- General kernel application with `CIKernelArgument` lists of images, scalars, vectors and colors: `CIKernel::apply(extent, arguments, region_of_interest)`, `CIColorKernel::apply(extent, arguments)` and `CIWarpKernel::apply(extent, image, arguments, region_of_interest)`. They run behind the Objective-C exception boundary; mismatched arguments return `CIError::NullResult`. Region-of-interest closures are `Send + Sync + 'static`, panic-contained (a panic falls back to that input's extent) and released with the image.
- `CIImageProcessorKernel`: user processing in a `Send + Sync` closure over `CIImageProcessorInputBuffer` and `CIImageProcessorOutputBuffer` views, with `with_format` (Bgra8, RgbaH, RgbaF, R8, RH, RF) and `with_region_of_interest`. The views are bounds-checked against each region, format and stride, input memory overlapping the output is refused, and an `Err` or panic zeroes the output tile and fails the render with the closure's message.
- `CIImage`, `CIContext`, `CIColor`, `CIVector`, `CIKernel`, `CIColorKernel`, `CIWarpKernel` and `CIBlendKernel` implement `Send` and `Sync`, as Core Image documents them as immutable and thread-safe (`NS_SWIFT_SENDABLE`). Mutable types stay single-threaded.
- A compiled Core Image Metal library fixture (`tests/fixtures`) with its source and build script, used by the tests and `examples/08_kernel.rs`.

## [0.4.0] - 2026-09-07

### Changed (breaking)

- `CIRenderTask` now retains bitmap storage and its native destination through completion, waits when dropped, and keeps forgotten tasks conservatively in flight. Bitmap access and destination mutation return errors while native writes may still occur.
- Documented that synchronous `CVPixelBuffer` and `IOSurface` rendering mutates aliased native storage and must not overlap unsafe CPU byte views or other native access.
- Dynamic `CIFilter` input setters now return `Result`, validate supported keys and declared value classes, and use an Objective-C exception boundary for KVC. Named setters cover common image, numeric, vector, color, message, and correction-level inputs.
- `CIImage::applying_orientation` and `CIImage::oriented` now return `Result` and reject EXIF orientation values outside 1 through 8.
- `CIFilterConstructor`, `CIPlugInRegistration`, and custom warp ROI callbacks now require `Send + Sync` captures.
- `CIWarpKernel::apply_image_scalar` now uses the full input extent as a conservative ROI. Added destination-rect and caller-callback variants.
- Context maximum input/output size queries now return `Result`; unsupported macOS calls report `CIError::Unsupported`.
- Raised in-family requirements to `apple-cf >=0.10, <0.11` and
  `apple-metal >=0.9, <0.10`.

### Fixed

- Processor invocation data is copied under a lock and read through one retained snapshot handle, preventing fields from different concurrent invocations from being mixed.
- Callback-body and caught panic-payload destruction are contained at the C ABI boundary, while library-owned release work runs in separately guarded cleanup phases. Callback captures retain Rust's normal non-panicking `Drop` requirement.
- Sampler affine transforms now use the six-number array required by `kCISamplerAffineMatrix`.

## [0.3.4] - 2026-05-20

- Added in-`src/` unit tests across color, constants, context, error, filter_shape, and raw_filter (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.3.3] - 2026-05-18

- Add one-line docs across the public safe and FFI surfaces, raising public-item rustdoc coverage to 99.7%.

## [0.3.2] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

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
