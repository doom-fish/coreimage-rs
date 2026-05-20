#![allow(clippy::too_many_lines)]

use std::sync::OnceLock;

use crate::ffi;
use crate::util::take_owned_string;

macro_rules! string_key_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($variant:ident => $index:expr),+ $(,)?
        }
        loader = $loader:path;
        total = $total:expr;
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $(
                #[doc = concat!("Mirrors the `CoreImage` framework case `", stringify!($variant), "`.")]
                $variant,
            )+
        }

        impl $name {
            const TOTAL: usize = $total;

            const fn index(self) -> usize {
                match self {
                    $(Self::$variant => $index),+
                }
            }

            fn values() -> &'static Vec<String> {
                static VALUES: OnceLock<Vec<String>> = OnceLock::new();
                VALUES.get_or_init(|| {
                    (0..Self::TOTAL)
                        .map(|index| {
                            let index = i32::try_from(index).expect("string key index fits in i32");
                            unsafe { take_owned_string($loader(index)).unwrap_or_default() }
                        })
                        .collect()
                })
            }

            /// Calls the `CoreImage` framework counterpart for `as_str`.
            pub fn as_str(self) -> &'static str {
                Self::values()[self.index()].as_str()
            }
        }
    };
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($variant:ident),+ $(,)?
        }
        loader = $loader:path;
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(usize)]
        pub enum $name {
            $(
                #[doc = concat!("Mirrors the `CoreImage` framework case `", stringify!($variant), "`.")]
                $variant,
            )+
        }

        impl $name {
            fn values() -> &'static Vec<String> {
                static VALUES: OnceLock<Vec<String>> = OnceLock::new();
                VALUES.get_or_init(|| {
                    (0..string_key_enum!(@count $($variant),+))
                        .map(|index| {
                            let index = i32::try_from(index).expect("string key index fits in i32");
                            unsafe { take_owned_string($loader(index)).unwrap_or_default() }
                        })
                        .collect()
                })
            }

            /// Calls the `CoreImage` framework counterpart for `as_str`.
            pub fn as_str(self) -> &'static str {
                Self::values()[self as usize].as_str()
            }
        }
    };
    (@count $($variant:ident),+ $(,)?) => {
        <[()]>::len(&[$(string_key_enum!(@replace $variant ())),+])
    };
    (@replace $_variant:ident $sub:expr) => {
        $sub
    };
}

/// Common Core Image color-space selections for APIs that accept a destination or working space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CIColorSpace {
    /// Use the framework's default behavior for the specific API.
    #[default]
    Srgb,
    /// Disable color matching when the API accepts a nullable color space.
    None,
    /// Extended sRGB color space.
    ExtendedSrgb,
    /// Display P3 color space.
    DisplayP3,
    /// Generic gray gamma 2.2 color space.
    GenericGrayGamma2_2,
}

impl CIColorSpace {
    pub(crate) const fn code(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Srgb => 1,
            Self::ExtendedSrgb => 2,
            Self::DisplayP3 => 3,
            Self::GenericGrayGamma2_2 => 4,
        }
    }
}

/// Core Image pixel formats (`CIFormat`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIFormat {
/// Mirrors the `CoreImage` framework case `Argb8`.
    Argb8,
/// Mirrors the `CoreImage` framework case `Bgra8`.
    Bgra8,
/// Mirrors the `CoreImage` framework case `Rgba8`.
    Rgba8,
/// Mirrors the `CoreImage` framework case `Rgbx8`.
    Rgbx8,
/// Mirrors the `CoreImage` framework case `Abgr8`.
    Abgr8,
/// Mirrors the `CoreImage` framework case `RgbaH`.
    RgbaH,
/// Mirrors the `CoreImage` framework case `Rgba16`.
    Rgba16,
/// Mirrors the `CoreImage` framework case `RgbaF`.
    RgbaF,
/// Mirrors the `CoreImage` framework case `Rgbx16`.
    Rgbx16,
/// Mirrors the `CoreImage` framework case `RgbxH`.
    RgbxH,
/// Mirrors the `CoreImage` framework case `RgbxF`.
    RgbxF,
/// Mirrors the `CoreImage` framework case `Rgb10`.
    Rgb10,
/// Mirrors the `CoreImage` framework case `A8`.
    A8,
/// Mirrors the `CoreImage` framework case `A16`.
    A16,
/// Mirrors the `CoreImage` framework case `AH`.
    AH,
/// Mirrors the `CoreImage` framework case `AF`.
    AF,
/// Mirrors the `CoreImage` framework case `R8`.
    R8,
/// Mirrors the `CoreImage` framework case `R16`.
    R16,
/// Mirrors the `CoreImage` framework case `RH`.
    RH,
/// Mirrors the `CoreImage` framework case `RF`.
    RF,
/// Mirrors the `CoreImage` framework case `Rg8`.
    Rg8,
/// Mirrors the `CoreImage` framework case `Rg16`.
    Rg16,
/// Mirrors the `CoreImage` framework case `RgH`.
    RgH,
/// Mirrors the `CoreImage` framework case `RgF`.
    RgF,
/// Mirrors the `CoreImage` framework case `L8`.
    L8,
/// Mirrors the `CoreImage` framework case `L16`.
    L16,
/// Mirrors the `CoreImage` framework case `LH`.
    LH,
/// Mirrors the `CoreImage` framework case `LF`.
    LF,
/// Mirrors the `CoreImage` framework case `La8`.
    La8,
/// Mirrors the `CoreImage` framework case `La16`.
    La16,
/// Mirrors the `CoreImage` framework case `LaH`.
    LaH,
/// Mirrors the `CoreImage` framework case `LaF`.
    LaF,
}

impl CIFormat {
    const ALL: [Self; 32] = [
        Self::Argb8,
        Self::Bgra8,
        Self::Rgba8,
        Self::Rgbx8,
        Self::Abgr8,
        Self::RgbaH,
        Self::Rgba16,
        Self::RgbaF,
        Self::Rgbx16,
        Self::RgbxH,
        Self::RgbxF,
        Self::Rgb10,
        Self::A8,
        Self::A16,
        Self::AH,
        Self::AF,
        Self::R8,
        Self::R16,
        Self::RH,
        Self::RF,
        Self::Rg8,
        Self::Rg16,
        Self::RgH,
        Self::RgF,
        Self::L8,
        Self::L16,
        Self::LH,
        Self::LF,
        Self::La8,
        Self::La16,
        Self::LaH,
        Self::LaF,
    ];

    const fn index(self) -> i32 {
        match self {
            Self::Argb8 => 0,
            Self::Bgra8 => 1,
            Self::Rgba8 => 2,
            Self::Rgbx8 => 3,
            Self::Abgr8 => 4,
            Self::RgbaH => 5,
            Self::Rgba16 => 6,
            Self::RgbaF => 7,
            Self::Rgbx16 => 8,
            Self::RgbxH => 9,
            Self::RgbxF => 10,
            Self::Rgb10 => 11,
            Self::A8 => 12,
            Self::A16 => 13,
            Self::AH => 14,
            Self::AF => 15,
            Self::R8 => 16,
            Self::R16 => 17,
            Self::RH => 18,
            Self::RF => 19,
            Self::Rg8 => 20,
            Self::Rg16 => 21,
            Self::RgH => 22,
            Self::RgF => 23,
            Self::L8 => 24,
            Self::L16 => 25,
            Self::LH => 26,
            Self::LF => 27,
            Self::La8 => 28,
            Self::La16 => 29,
            Self::LaH => 30,
            Self::LaF => 31,
        }
    }

/// Calls the `CoreImage` framework counterpart for `raw_value`.
    pub fn raw_value(self) -> i32 {
        unsafe { ffi::ci_image_format_value(self.index()) }
    }

/// Calls the `CoreImage` framework counterpart for `from_raw`.
    pub fn from_raw(raw: i32) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|format| format.raw_value() == raw)
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            Self::A8 | Self::R8 | Self::L8 => 1,
            Self::A16
            | Self::AH
            | Self::R16
            | Self::RH
            | Self::L16
            | Self::LH
            | Self::Rg8
            | Self::La8 => 2,
            Self::Argb8
            | Self::Bgra8
            | Self::Rgba8
            | Self::Rgbx8
            | Self::Abgr8
            | Self::Rgb10
            | Self::AF
            | Self::RF
            | Self::LF
            | Self::Rg16
            | Self::RgH
            | Self::La16
            | Self::LaH => 4,
            Self::RgbaH | Self::Rgba16 | Self::Rgbx16 | Self::RgbxH | Self::RgF | Self::LaF => 8,
            Self::RgbaF | Self::RgbxF => 16,
        }
    }
}

string_key_enum! {
    /// Typed names for `CIContextOption` dictionary keys.
    pub enum CIContextOptionKey {
        OutputColorSpace => 0,
        WorkingColorSpace => 1,
        WorkingFormat => 2,
        HighQualityDownsample => 3,
        OutputPremultiplied => 4,
        CacheIntermediates => 5,
        UseSoftwareRenderer => 6,
        PriorityRequestLow => 7,
        AllowLowPower => 8,
        Name => 9,
        CvMetalTextureCache => 10,
        MemoryLimit => 11,
    }
    loader = ffi::ci_context_option_name;
    total = 12;
}

string_key_enum! {
    /// Typed names for `CIImageOption` dictionary keys.
    pub enum CIImageOptionKey {
        ColorSpace => 0,
        ApplyCleanAperture => 1,
        ToneMapHdrToSdr => 2,
        ExpandToHdr => 3,
        ContentHeadroom => 4,
        ContentAverageLightLevel => 5,
        NearestSampling => 6,
        CacheImmediately => 7,
        Properties => 8,
        ApplyOrientationProperty => 9,
        AuxiliaryDepth => 10,
        AuxiliaryDisparity => 11,
        AuxiliaryPortraitEffectsMatte => 12,
        AuxiliarySemanticSegmentationSkinMatte => 13,
        AuxiliarySemanticSegmentationHairMatte => 14,
        AuxiliarySemanticSegmentationTeethMatte => 15,
        AuxiliarySemanticSegmentationGlassesMatte => 16,
        AuxiliarySemanticSegmentationSkyMatte => 17,
        AuxiliaryHdrGainMap => 18,
    }
    loader = ffi::ci_image_option_name;
    total = 19;
}

string_key_enum! {
    /// Typed names for `CIImageAutoAdjustmentOption` dictionary keys.
    pub enum CIImageAutoAdjustmentOptionKey {
        Enhance => 0,
        RedEye => 1,
        Features => 2,
        Crop => 3,
        Level => 4,
    }
    loader = ffi::ci_image_auto_adjust_option_name;
    total = 5;
}

string_key_enum! {
    /// Typed names for `CIImageRepresentationOption` dictionary keys.
    pub enum CIImageRepresentationOptionKey {
        AvDepthData => 0,
        DepthImage => 1,
        DisparityImage => 2,
        AvPortraitEffectsMatte => 3,
        PortraitEffectsMatteImage => 4,
        AvSemanticSegmentationMattes => 5,
        SemanticSegmentationSkinMatteImage => 6,
        SemanticSegmentationHairMatteImage => 7,
        SemanticSegmentationTeethMatteImage => 8,
        SemanticSegmentationGlassesMatteImage => 9,
        SemanticSegmentationSkyMatteImage => 10,
        HdrImage => 11,
        HdrGainMapImage => 12,
        HdrGainMapAsRgb => 13,
    }
    loader = ffi::ci_image_representation_option_name;
    total = 14;
}

string_key_enum! {
    /// Typed names for `CIFilter.apply(..., options:)` dictionary keys.
    pub enum CIApplyOptionKey {
        Extent,
        Definition,
        UserInfo,
        ColorSpace,
    }
    loader = ffi::ci_apply_option_name;
}

string_key_enum! {
    /// Typed names for `CIFilter.attributes` dictionary entries.
    pub enum CIAttributeKey {
        FilterName,
        FilterDisplayName,
        Description,
        FilterAvailableMac,
        FilterAvailableIos,
        ReferenceDocumentation,
        FilterCategories,
        Class,
        Type,
        Min,
        Max,
        SliderMin,
        SliderMax,
        Default,
        Identity,
        Name,
        DisplayName,
    }
    loader = ffi::ci_attribute_key_name;
}

string_key_enum! {
    /// Typed `kCIAttributeType*` values found in filter metadata.
    pub enum CIAttributeType {
        Time,
        Scalar,
        Distance,
        Angle,
        Boolean,
        Integer,
        Count,
        Position,
        Offset,
        Position3,
        Rectangle,
        OpaqueColor,
        Color,
        Gradient,
        Image,
        Transform,
    }
    loader = ffi::ci_attribute_type_name;
}

string_key_enum! {
    /// Typed Core Image filter categories for discovery APIs.
    pub enum CIFilterCategory {
        DistortionEffect,
        GeometryAdjustment,
        CompositeOperation,
        HalftoneEffect,
        ColorAdjustment,
        ColorEffect,
        Transition,
        TileEffect,
        Generator,
        Reduction,
        Gradient,
        Stylize,
        Sharpen,
        Blur,
        Video,
        StillImage,
        Interlaced,
        NonSquarePixels,
        HighDynamicRange,
        BuiltIn,
        FilterGenerator,
    }
    loader = ffi::ci_filter_category_name;
}

impl CIFilterCategory {
/// Calls the `CoreImage` framework counterpart for `value`.
    pub fn value(self) -> &'static str {
        static VALUES: OnceLock<Vec<String>> = OnceLock::new();
        VALUES.get_or_init(|| {
            (0_i32..21)
                .map(|index| unsafe {
                    take_owned_string(ffi::ci_filter_category_value(index)).unwrap_or_default()
                })
                .collect()
        })[self as usize]
            .as_str()
    }
}

string_key_enum! {
    /// Typed dynamic-range metadata values used by Core Image filters.
    pub enum CIDynamicRange {
        Standard,
        High,
        ConstrainedHigh,
    }
    loader = ffi::ci_dynamic_range_name;
}

string_key_enum! {
    /// Common typed filter input keys.
    pub enum CIInputKey {
        BackgroundImage,
        Image,
        DepthImage,
        DisparityImage,
        Amount,
        Count,
        Threshold,
        Time,
        Transform,
        Scale,
        AspectRatio,
        Center,
        Radius,
        Radius0,
        Radius1,
        Angle,
        Refraction,
        Width,
        Sharpness,
        Intensity,
        Ev,
        Saturation,
        Color,
        Color0,
        Color1,
        ColorSpace,
        Brightness,
        Contrast,
        Extrapolate,
        Perceptual,
        Bias,
        BiasVector,
        GradientImage,
        MaskImage,
        MatteImage,
        BacksideImage,
        ShadingImage,
        TargetImage,
        PaletteImage,
        Extent,
        Version,
        Point0,
        Point1,
        Weights,
    }
    loader = ffi::ci_input_key_name;
}

impl CIInputKey {
/// Calls the `CoreImage` framework counterpart for `value`.
    pub fn value(self) -> &'static str {
        static VALUES: OnceLock<Vec<String>> = OnceLock::new();
        VALUES.get_or_init(|| {
            (0_i32..44)
                .map(|index| unsafe {
                    take_owned_string(ffi::ci_input_key_value(index)).unwrap_or_default()
                })
                .collect()
        })[self as usize]
            .as_str()
    }
}

string_key_enum! {
    /// Common typed filter output keys.
    pub enum CIOutputKey {
        Image,
    }
    loader = ffi::ci_output_key_name;
}

impl CIOutputKey {
/// Calls the `CoreImage` framework counterpart for `value`.
    pub fn value(self) -> &'static str {
        static VALUES: OnceLock<Vec<String>> = OnceLock::new();
        VALUES.get_or_init(|| {
            (0_i32..1)
                .map(|index| unsafe {
                    take_owned_string(ffi::ci_output_key_value(index)).unwrap_or_default()
                })
                .collect()
        })[self as usize]
            .as_str()
    }
}

string_key_enum! {
    /// The metadata key used to select which UI parameter set a filter should expose.
    pub enum CIUIParameterSetKey {
        ParameterSet,
    }
    loader = ffi::ci_ui_parameter_set_key_name;
}

string_key_enum! {
    /// Predefined UI parameter-set values for filter metadata.
    pub enum CIUIParameterSet {
        Basic,
        Intermediate,
        Advanced,
        Development,
    }
    loader = ffi::ci_ui_parameter_set_name;
}

string_key_enum! {
    /// Typed names for exported-key dictionaries in `CIFilterGenerator` graphs.
    pub enum CIFilterGeneratorExportedKey {
        Key,
        Name,
        TargetObject,
    }
    loader = ffi::ci_filter_generator_exported_key_name;
}

string_key_enum! {
    /// Typed `CIImageOption` keys used with `CIImage` image-provider creation.
    pub enum CIImageProviderOptionKey {
        TileSize,
        UserInfo,
    }
    loader = ffi::ci_image_provider_option_name;
}

string_key_enum! {
    /// Typed keys accepted by `CISampler` option dictionaries.
    pub enum CISamplerOptionKey {
        AffineMatrix,
        FilterMode,
        WrapMode,
        ColorSpace,
    }
    loader = ffi::ci_sampler_option_name;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_space_defaults_and_codes_are_stable() {
        assert_eq!(CIColorSpace::default(), CIColorSpace::Srgb);

        let cases = [
            (CIColorSpace::None, 0),
            (CIColorSpace::Srgb, 1),
            (CIColorSpace::ExtendedSrgb, 2),
            (CIColorSpace::DisplayP3, 3),
            (CIColorSpace::GenericGrayGamma2_2, 4),
        ];

        for (color_space, expected_code) in cases {
            assert_eq!(color_space.code(), expected_code);
        }
    }

    #[test]
    fn pixel_formats_round_trip_through_raw_values() {
        for format in CIFormat::ALL {
            assert_eq!(CIFormat::from_raw(format.raw_value()), Some(format));
        }

        assert_eq!(CIFormat::from_raw(i32::MIN), None);
    }

    #[test]
    fn bytes_per_pixel_match_expected_groups() {
        let cases = [
            (CIFormat::A8, 1),
            (CIFormat::Rg8, 2),
            (CIFormat::Rgba8, 4),
            (CIFormat::Rgba16, 8),
            (CIFormat::RgbaF, 16),
        ];

        for (format, expected_bytes) in cases {
            assert_eq!(format.bytes_per_pixel(), expected_bytes);
        }
    }

    #[test]
    fn typed_string_keys_match_framework_constants() {
        assert_eq!(CIContextOptionKey::MemoryLimit.as_str(), "kCIContextMemoryLimit");
        assert_eq!(CIImageOptionKey::AuxiliaryHdrGainMap.as_str(), "kCIImageAuxiliaryHDRGainMap");
        assert_eq!(CIApplyOptionKey::ColorSpace.as_str(), "kCIApplyOptionColorSpace");
        assert_eq!(CIInputKey::Intensity.as_str(), "kCIInputIntensityKey");
        assert_eq!(CIOutputKey::Image.as_str(), "kCIOutputImageKey");
        assert_eq!(CIUIParameterSet::Advanced.as_str(), "kCIUISetAdvanced");
        assert_eq!(CISamplerOptionKey::ColorSpace.as_str(), "kCISamplerColorSpace");
    }
}
