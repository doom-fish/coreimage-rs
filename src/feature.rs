use core::ffi::c_void;
use core::fmt;
use core::ptr;

use apple_cf::cg::CGRect;

use crate::barcode_descriptor::CIBarcodeDescriptor;
use crate::ffi;
use crate::util::{take_array_objects, take_owned_string};

/// Detected feature kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIFeatureKind {
    Face,
    Rectangle,
    QrCode,
    Text,
    Unknown,
}

/// A feature found by `CIDetector`.
pub struct CIFeature {
    ptr: *mut c_void,
}

impl Drop for CIFeature {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ci_object_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CIFeature {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ffi::ci_object_retain(self.ptr) },
        }
    }
}

impl fmt::Debug for CIFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CIFeature")
            .field("ptr", &self.ptr)
            .field("kind", &self.kind())
            .field("bounds", &self.bounds())
            .finish_non_exhaustive()
    }
}

impl CIFeature {
    pub(crate) const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn kind(&self) -> CIFeatureKind {
        match unsafe { ffi::ci_feature_type_code(self.ptr) } {
            0 => CIFeatureKind::Face,
            1 => CIFeatureKind::Rectangle,
            2 => CIFeatureKind::QrCode,
            3 => CIFeatureKind::Text,
            _ => CIFeatureKind::Unknown,
        }
    }

    pub fn bounds(&self) -> CGRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { ffi::ci_feature_bounds(self.ptr, &mut x, &mut y, &mut width, &mut height) };
        CGRect::new(x, y, width, height)
    }

    pub fn details_json(&self) -> String {
        unsafe { take_owned_string(ffi::ci_feature_details_json(self.ptr)) }
            .unwrap_or_else(|| "{}".to_string())
    }

    pub fn message_string(&self) -> Option<String> {
        unsafe { take_owned_string(ffi::ci_feature_message_string(self.ptr)) }
            .filter(|value| !value.is_empty())
    }

    pub fn symbol_descriptor(&self) -> Option<CIBarcodeDescriptor> {
        let handle = unsafe { ffi::ci_feature_symbol_descriptor(self.ptr) };
        if handle.is_null() {
            None
        } else {
            Some(unsafe { CIBarcodeDescriptor::from_raw(handle) })
        }
    }

    pub fn sub_features(&self) -> Vec<Self> {
        let handle = unsafe { ffi::ci_feature_subfeatures(self.ptr) };
        let objects = unsafe { take_array_objects(handle) };
        objects
            .into_iter()
            .map(|object| unsafe { Self::from_raw(object) })
            .collect()
    }
}
