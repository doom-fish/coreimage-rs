use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ci_detector_new(
        kind: i32,
        context: *mut c_void,
        accuracy: i32,
        tracking: bool,
        min_feature_size: f64,
        max_feature_count: i32,
        number_of_angles: i32,
        out_detector: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn ci_detector_features(
        handle: *mut c_void,
        image: *mut c_void,
        orientation: i32,
        eye_blink: bool,
        smile: bool,
        focal_length: f64,
        aspect_ratio: f64,
        return_sub_features: bool,
        out_features: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
