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
