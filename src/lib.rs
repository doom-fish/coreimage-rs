#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API Documentation
//!
//! Safe Rust bindings for Apple's [CoreImage](https://developer.apple.com/documentation/coreimage)
//! framework — GPU-accelerated image processing, filtering, and rendering on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cast_precision_loss,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::return_self_not_must_use
)]

mod context;
mod error;
mod ffi;
mod filter;
#[cfg(feature = "filters")]
#[cfg_attr(docsrs, doc(cfg(feature = "filters")))]
pub mod filters;
mod image;
mod util;
mod value;

pub use apple_cf::cg::{CGAffineTransform, CGImage, CGPoint, CGRect, CGSize};
pub use apple_cf::cv::CVPixelBuffer;
pub use apple_cf::iosurface::IOSurface;
pub use context::CIContext;
pub use error::CIError;
pub use filter::CIFilter;
pub use image::CIImage;
pub use value::{CIColor, CIVector};

#[cfg(feature = "metal")]
#[cfg_attr(docsrs, doc(cfg(feature = "metal")))]
pub use apple_metal::{CommandQueue, MetalDevice};

/// Common imports for users of this crate.
pub mod prelude {
    pub use apple_cf::cg::{CGAffineTransform, CGImage, CGPoint, CGRect, CGSize};
    pub use apple_cf::cv::CVPixelBuffer;
    pub use apple_cf::iosurface::IOSurface;

    #[cfg(feature = "metal")]
    pub use apple_metal::{CommandQueue, MetalDevice};

    pub use crate::{CIColor, CIContext, CIError, CIFilter, CIImage, CIVector};

    #[cfg(feature = "filters")]
    pub use crate::filters;
}
