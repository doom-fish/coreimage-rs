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
            $($variant),+
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

            pub fn as_str(self) -> &'static str {
                Self::values()[self.index()].as_str()
            }
        }
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
    Argb8,
    Bgra8,
    Rgba8,
    Rgbx8,
    Abgr8,
    RgbaH,
    Rgba16,
    RgbaF,
    Rgbx16,
    RgbxH,
    RgbxF,
    Rgb10,
    A8,
    A16,
    AH,
    AF,
    R8,
    R16,
    RH,
    RF,
    Rg8,
    Rg16,
    RgH,
    RgF,
    L8,
    L16,
    LH,
    LF,
    La8,
    La16,
    LaH,
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

    pub fn raw_value(self) -> i32 {
        unsafe { ffi::ci_image_format_value(self.index()) }
    }

    pub fn from_raw(raw: i32) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|format| format.raw_value() == raw)
    }

    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            Self::A8 | Self::R8 | Self::L8 => 1,
            Self::A16 | Self::AH | Self::R16 | Self::RH | Self::L16 | Self::LH | Self::Rg8 | Self::La8 => 2,
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
