use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ci_vector_new1(x: f64) -> *mut c_void;
    pub fn ci_vector_new2(x: f64, y: f64) -> *mut c_void;
    pub fn ci_vector_new3(x: f64, y: f64, z: f64) -> *mut c_void;
    pub fn ci_vector_new4(x: f64, y: f64, z: f64, w: f64) -> *mut c_void;
    pub fn ci_vector_from_rect(x: f64, y: f64, width: f64, height: f64) -> *mut c_void;
    pub fn ci_vector_from_transform(
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        tx: f64,
        ty: f64,
    ) -> *mut c_void;
    pub fn ci_vector_from_string(representation: *const c_char) -> *mut c_void;
    pub fn ci_vector_count(handle: *mut c_void) -> usize;
    pub fn ci_vector_value_at(handle: *mut c_void, index: usize) -> f64;
    pub fn ci_vector_x(handle: *mut c_void) -> f64;
    pub fn ci_vector_y(handle: *mut c_void) -> f64;
    pub fn ci_vector_z(handle: *mut c_void) -> f64;
    pub fn ci_vector_w(handle: *mut c_void) -> f64;
    pub fn ci_vector_point(handle: *mut c_void, out_x: *mut f64, out_y: *mut f64);
    pub fn ci_vector_rect(
        handle: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn ci_vector_transform(
        handle: *mut c_void,
        out_a: *mut f64,
        out_b: *mut f64,
        out_c: *mut f64,
        out_d: *mut f64,
        out_tx: *mut f64,
        out_ty: *mut f64,
    );
    pub fn ci_vector_string_representation(handle: *mut c_void) -> *mut c_char;
}
