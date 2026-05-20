use core::ffi::c_void;
use core::fmt;
use core::ptr;

use crate::ffi;
use crate::util::{string_to_cstring, take_owned_string};

/// Named Core Image colors in the sRGB color space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIColorName {
/// Mirrors the `CoreImage` framework case `Black`.
    Black,
/// Mirrors the `CoreImage` framework case `White`.
    White,
/// Mirrors the `CoreImage` framework case `Gray`.
    Gray,
/// Mirrors the `CoreImage` framework case `Red`.
    Red,
/// Mirrors the `CoreImage` framework case `Green`.
    Green,
/// Mirrors the `CoreImage` framework case `Blue`.
    Blue,
/// Mirrors the `CoreImage` framework case `Cyan`.
    Cyan,
/// Mirrors the `CoreImage` framework case `Magenta`.
    Magenta,
/// Mirrors the `CoreImage` framework case `Yellow`.
    Yellow,
/// Mirrors the `CoreImage` framework case `Clear`.
    Clear,
}

impl CIColorName {
    const fn code(self) -> i32 {
        match self {
            Self::Black => 0,
            Self::White => 1,
            Self::Gray => 2,
            Self::Red => 3,
            Self::Green => 4,
            Self::Blue => 5,
            Self::Cyan => 6,
            Self::Magenta => 7,
            Self::Yellow => 8,
            Self::Clear => 9,
        }
    }
}

/// A Core Image RGBA color value used for filter inputs and generators.
pub struct CIColor {
    ptr: *mut c_void,
}

impl Drop for CIColor {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIColor {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIColor")
            .field("ptr", &self.ptr)
            .field("components", &self.components())
            .finish_non_exhaustive()
    }
}

impl CIColor {
    pub(crate) const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    fn from_non_null(ptr: *mut c_void, kind: &str) -> Self {
        assert!(!ptr.is_null(), "{kind} returned nil");
        unsafe { Self::from_raw(ptr) }
    }

/// Mirrors the `CoreImage` framework constant `fn`.
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

/// Calls the `CoreImage` framework counterpart for `rgba`.
    pub fn rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self::from_non_null(
            unsafe { ffi::ci_color_new_rgba(red, green, blue, alpha) },
            "CIColor",
        )
    }

/// Calls the `CoreImage` framework counterpart for `rgb`.
    pub fn rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::rgba(red, green, blue, 1.0)
    }

/// Calls the `CoreImage` framework counterpart for `white`.
    pub fn white(intensity: f64, alpha: f64) -> Self {
        Self::rgba(intensity, intensity, intensity, alpha)
    }

/// Calls the `CoreImage` framework counterpart for `from_string`.
    pub fn from_string(representation: &str) -> Option<Self> {
        let representation = string_to_cstring(representation, "representation").ok()?;
        let handle = unsafe { ffi::ci_color_from_string(representation.as_ptr()) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(handle) })
        }
    }

/// Calls the `CoreImage` framework counterpart for `named`.
    pub fn named(name: CIColorName) -> Self {
        Self::from_non_null(unsafe { ffi::ci_color_named(name.code()) }, "CIColor")
    }

/// Calls the `CoreImage` framework counterpart for `black`.
    pub fn black() -> Self {
        Self::named(CIColorName::Black)
    }

/// Calls the `CoreImage` framework counterpart for `clear`.
    pub fn clear() -> Self {
        Self::named(CIColorName::Clear)
    }

/// Calls the `CoreImage` framework counterpart for `number_of_components`.
    pub fn number_of_components(&self) -> usize {
        unsafe { ffi::ci_color_number_of_components(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `components`.
    pub fn components(&self) -> Vec<f64> {
        (0..self.number_of_components())
            .map(|index| unsafe { ffi::ci_color_component_at(self.ptr, index) })
            .collect()
    }

/// Calls the `CoreImage` framework counterpart for `alpha`.
    pub fn alpha(&self) -> f64 {
        unsafe { ffi::ci_color_alpha(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `red`.
    pub fn red(&self) -> f64 {
        unsafe { ffi::ci_color_red(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `green`.
    pub fn green(&self) -> f64 {
        unsafe { ffi::ci_color_green(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `blue`.
    pub fn blue(&self) -> f64 {
        unsafe { ffi::ci_color_blue(self.ptr) }
    }

/// Calls the `CoreImage` framework counterpart for `string_representation`.
    pub fn string_representation(&self) -> String {
        unsafe { take_owned_string(ffi::ci_color_string_representation(self.ptr)) }
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(left: f64, right: f64) {
        assert!(
            (left - right).abs() < 1e-6,
            "expected {left} to be within 1e-6 of {right}"
        );
    }

    #[test]
    fn color_name_codes_are_stable() {
        let cases = [
            (CIColorName::Black, 0),
            (CIColorName::White, 1),
            (CIColorName::Gray, 2),
            (CIColorName::Red, 3),
            (CIColorName::Green, 4),
            (CIColorName::Blue, 5),
            (CIColorName::Cyan, 6),
            (CIColorName::Magenta, 7),
            (CIColorName::Yellow, 8),
            (CIColorName::Clear, 9),
        ];

        for (name, expected_code) in cases {
            assert_eq!(name.code(), expected_code);
        }
    }

    #[test]
    fn rgba_colors_expose_expected_channels() {
        let color = CIColor::rgba(0.1, 0.2, 0.3, 0.4);

        assert_eq!(color.number_of_components(), 4);
        assert_close(color.red(), 0.1);
        assert_close(color.green(), 0.2);
        assert_close(color.blue(), 0.3);
        assert_close(color.alpha(), 0.4);
        assert_eq!(color.components().len(), 4);
    }

    #[test]
    fn helper_constructors_match_named_color_expectations() {
        let rgb = CIColor::rgb(0.25, 0.5, 0.75);
        let black = CIColor::black();
        let clear = CIColor::clear();
        let red = CIColor::named(CIColorName::Red);

        assert_close(rgb.alpha(), 1.0);
        assert_close(black.red(), 0.0);
        assert_close(black.green(), 0.0);
        assert_close(black.blue(), 0.0);
        assert_close(black.alpha(), 1.0);
        assert_close(clear.alpha(), 0.0);
        assert!(red.red() > 0.9);
    }

    #[test]
    fn string_round_trip_preserves_components() {
        let original = CIColor::rgba(0.9, 0.4, 0.2, 0.7);
        let representation = original.string_representation();
        let round_trip = CIColor::from_string(&representation)
            .expect("Core Image should parse its own string representation");

        let original_components = original.components();
        let round_trip_components = round_trip.components();
        assert_eq!(original_components.len(), round_trip_components.len());
        for (expected, actual) in original_components.iter().zip(&round_trip_components) {
            assert_close(*expected, *actual);
        }
    }
}
