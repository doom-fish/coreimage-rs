#![allow(missing_docs, dead_code)]

pub mod barcode_descriptor;
pub mod color;
pub mod common;
pub mod context;
pub mod detector;
pub mod feature;
pub mod filter;
pub mod filter_generator;
pub mod image;
pub mod image_processor;
pub mod kernel;
pub mod sampler;
pub mod vector;

pub use barcode_descriptor::*;
pub use color::*;
pub use common::*;
pub use context::*;
pub use detector::*;
pub use feature::*;
pub use filter::*;
pub use filter_generator::*;
pub use image::*;
pub use image_processor::*;
pub use kernel::*;
pub use sampler::*;
pub use vector::*;

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const NULL_RESULT: i32 = -2;
    pub const FRAMEWORK: i32 = -3;
    pub const IO: i32 = -4;
    pub const UNSUPPORTED: i32 = -5;
}
