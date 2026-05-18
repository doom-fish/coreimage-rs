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

mod barcode_descriptor;
mod color;
mod constants;
mod context;
mod detector;
mod error;
mod feature;
mod ffi;
mod filter;
mod filter_constructor;
mod filter_generator;
mod filter_shape;
/// Groups `CoreImage` framework constants for `filters`.
#[cfg(feature = "filters")]
#[cfg_attr(docsrs, doc(cfg(feature = "filters")))]
pub mod filters;
mod image;
mod image_accumulator;
mod image_processor;
mod kernel;
mod plugin;
mod raw_filter;
mod render_destination;
mod sampler;
mod util;
mod vector;

/// Re-exports the `CoreImage` framework surface for this item.
pub use apple_cf::cg::{CGAffineTransform, CGImage, CGPoint, CGRect, CGSize};
/// Re-exports the `CoreImage` framework surface for this item.
pub use apple_cf::cv::CVPixelBuffer;
/// Re-exports the `CoreImage` framework surface for this item.
pub use apple_cf::iosurface::IOSurface;
/// Re-exports the `CoreImage` framework surface for this item.
pub use barcode_descriptor::{
    CIBarcodeDescriptor, CIBarcodeDescriptorKind, CIDataMatrixCodeECCVersion,
    CIQRCodeErrorCorrectionLevel,
};
/// Re-exports the `CoreImage` framework surface for this item.
pub use color::{CIColor, CIColorName};
/// Re-exports the `CoreImage` framework surface for this item.
pub use constants::{
    CIApplyOptionKey, CIAttributeKey, CIAttributeType, CIColorSpace, CIContextOptionKey,
    CIDynamicRange, CIFilterCategory, CIFilterGeneratorExportedKey, CIFormat,
    CIImageAutoAdjustmentOptionKey, CIImageOptionKey, CIImageProviderOptionKey,
    CIImageRepresentationOptionKey, CIInputKey, CIOutputKey, CISamplerOptionKey, CIUIParameterSet,
    CIUIParameterSetKey,
};
/// Re-exports the `CoreImage` framework surface for this item.
pub use context::{CIContext, CIContextOptions};
/// Re-exports the `CoreImage` framework surface for this item.
pub use detector::{
    CIDetectionOptions, CIDetector, CIDetectorAccuracy, CIDetectorOptions, CIDetectorType,
};
/// Re-exports the `CoreImage` framework surface for this item.
pub use error::CIError;
/// Re-exports the `CoreImage` framework surface for this item.
pub use feature::{CIFeature, CIFeatureKind};
/// Re-exports the `CoreImage` framework surface for this item.
pub use filter::CIFilter;
/// Re-exports the `CoreImage` framework surface for this item.
pub use filter_constructor::CIFilterConstructor;
/// Re-exports the `CoreImage` framework surface for this item.
pub use filter_generator::CIFilterGenerator;
/// Re-exports the `CoreImage` framework surface for this item.
pub use filter_shape::CIFilterShape;
/// Re-exports the `CoreImage` framework surface for this item.
pub use image::CIImage;
/// Re-exports the `CoreImage` framework surface for this item.
pub use image_accumulator::CIImageAccumulator;
/// Re-exports the `CoreImage` framework surface for this item.
pub use image_processor::{
    CIImageProcessor, CIImageProcessorInput, CIImageProcessorInvocation, CIImageProcessorOutput,
};
/// Re-exports the `CoreImage` framework surface for this item.
pub use kernel::{CIBlendKernel, CIBlendKernelKind, CIColorKernel, CIKernel, CIWarpKernel};
/// Re-exports the `CoreImage` framework surface for this item.
pub use plugin::{CIPlugIn, CIPlugInRegistration};
/// Re-exports the `CoreImage` framework surface for this item.
pub use raw_filter::{CIRAWDecoderVersion, CIRAWFilter};
/// Re-exports the `CoreImage` framework surface for this item.
pub use render_destination::{
    CIRenderDestination, CIRenderDestinationAlphaMode, CIRenderInfo, CIRenderTask,
};
/// Re-exports the `CoreImage` framework surface for this item.
pub use sampler::{CISampler, CISamplerFilterMode, CISamplerOptions, CISamplerWrapMode};
/// Re-exports the `CoreImage` framework surface for this item.
pub use vector::CIVector;

/// Re-exports the `CoreImage` framework surface for this item.
#[cfg(feature = "metal")]
#[cfg_attr(docsrs, doc(cfg(feature = "metal")))]
pub use apple_metal::{CommandQueue, MetalDevice};

/// Common imports for users of this crate.
pub mod prelude {
/// Re-exports the `CoreImage` framework surface for this item.
    pub use apple_cf::cg::{CGAffineTransform, CGImage, CGPoint, CGRect, CGSize};
/// Re-exports the `CoreImage` framework surface for this item.
    pub use apple_cf::cv::CVPixelBuffer;
/// Re-exports the `CoreImage` framework surface for this item.
    pub use apple_cf::iosurface::IOSurface;

/// Re-exports the `CoreImage` framework surface for this item.
    #[cfg(feature = "metal")]
    pub use apple_metal::{CommandQueue, MetalDevice};

/// Re-exports the `CoreImage` framework surface for this item.
    pub use crate::{
        CIApplyOptionKey, CIAttributeKey, CIAttributeType, CIBarcodeDescriptor,
        CIBarcodeDescriptorKind, CIBlendKernel, CIBlendKernelKind, CIColor, CIColorKernel,
        CIColorName, CIColorSpace, CIContext, CIContextOptionKey, CIContextOptions,
        CIDataMatrixCodeECCVersion, CIDetectionOptions, CIDetector, CIDetectorAccuracy,
        CIDetectorOptions, CIDetectorType, CIDynamicRange, CIError, CIFeature, CIFeatureKind,
        CIFilter, CIFilterCategory, CIFilterConstructor, CIFilterGenerator,
        CIFilterGeneratorExportedKey, CIFilterShape, CIFormat, CIImage, CIImageAccumulator,
        CIImageAutoAdjustmentOptionKey, CIImageOptionKey, CIImageProcessor, CIImageProcessorInput,
        CIImageProcessorInvocation, CIImageProcessorOutput, CIImageProviderOptionKey,
        CIImageRepresentationOptionKey, CIInputKey, CIKernel, CIOutputKey, CIPlugIn,
        CIPlugInRegistration, CIQRCodeErrorCorrectionLevel, CIRAWDecoderVersion, CIRAWFilter,
        CIRenderDestination, CIRenderDestinationAlphaMode, CIRenderInfo, CIRenderTask, CISampler,
        CISamplerFilterMode, CISamplerOptionKey, CISamplerOptions, CISamplerWrapMode,
        CIUIParameterSet, CIUIParameterSetKey, CIVector, CIWarpKernel,
    };

/// Re-exports the `CoreImage` framework surface for this item.
    #[cfg(feature = "filters")]
    pub use crate::filters;
}
