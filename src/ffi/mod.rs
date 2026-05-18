#![allow(missing_docs, dead_code)]

/// Groups `CoreImage` framework constants for `barcode_descriptor`.
pub mod barcode_descriptor;
/// Groups `CoreImage` framework constants for `color`.
pub mod color;
/// Groups `CoreImage` framework constants for `common`.
pub mod common;
/// Groups `CoreImage` framework constants for `constants`.
pub mod constants;
/// Groups `CoreImage` framework constants for `context`.
pub mod context;
/// Groups `CoreImage` framework constants for `detector`.
pub mod detector;
/// Groups `CoreImage` framework constants for `feature`.
pub mod feature;
/// Groups `CoreImage` framework constants for `filter`.
pub mod filter;
/// Groups `CoreImage` framework constants for `filter_constructor`.
pub mod filter_constructor;
/// Groups `CoreImage` framework constants for `filter_generator`.
pub mod filter_generator;
/// Groups `CoreImage` framework constants for `filter_shape`.
pub mod filter_shape;
/// Groups `CoreImage` framework constants for `image`.
pub mod image;
/// Groups `CoreImage` framework constants for `image_accumulator`.
pub mod image_accumulator;
/// Groups `CoreImage` framework constants for `image_processor`.
pub mod image_processor;
/// Groups `CoreImage` framework constants for `kernel`.
pub mod kernel;
/// Groups `CoreImage` framework constants for `plugin`.
pub mod plugin;
/// Groups `CoreImage` framework constants for `raw_filter`.
pub mod raw_filter;
/// Groups `CoreImage` framework constants for `render_destination`.
pub mod render_destination;
/// Groups `CoreImage` framework constants for `sampler`.
pub mod sampler;
/// Groups `CoreImage` framework constants for `vector`.
pub mod vector;

/// Re-exports the `CoreImage` framework surface for this item.
pub use barcode_descriptor::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use color::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use common::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use constants::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use context::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use detector::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use feature::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use filter::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use filter_constructor::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use filter_generator::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use filter_shape::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use image::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use image_accumulator::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use image_processor::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use kernel::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use plugin::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use raw_filter::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use render_destination::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use sampler::*;
/// Re-exports the `CoreImage` framework surface for this item.
pub use vector::*;

/// Groups `CoreImage` framework constants for `status`.
pub mod status {
/// Mirrors the `CoreImage` framework constant `OK`.
    pub const OK: i32 = 0;
/// Mirrors the `CoreImage` framework constant `INVALID_ARGUMENT`.
    pub const INVALID_ARGUMENT: i32 = -1;
/// Mirrors the `CoreImage` framework constant `NULL_RESULT`.
    pub const NULL_RESULT: i32 = -2;
/// Mirrors the `CoreImage` framework constant `FRAMEWORK`.
    pub const FRAMEWORK: i32 = -3;
/// Mirrors the `CoreImage` framework constant `IO`.
    pub const IO: i32 = -4;
/// Mirrors the `CoreImage` framework constant `UNSUPPORTED`.
    pub const UNSUPPORTED: i32 = -5;
}
