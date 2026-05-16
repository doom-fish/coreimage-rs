use core::ffi::c_void;

unsafe extern "C" {
    pub fn ci_filter_shape_new(x: f64, y: f64, width: f64, height: f64) -> *mut c_void;
    pub fn ci_filter_shape_transform(
        handle: *mut c_void,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        tx: f64,
        ty: f64,
        interior: bool,
    ) -> *mut c_void;
    pub fn ci_filter_shape_inset(handle: *mut c_void, dx: i32, dy: i32) -> *mut c_void;
    pub fn ci_filter_shape_union(handle: *mut c_void, other: *mut c_void) -> *mut c_void;
    pub fn ci_filter_shape_union_rect(
        handle: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    pub fn ci_filter_shape_intersect(handle: *mut c_void, other: *mut c_void) -> *mut c_void;
    pub fn ci_filter_shape_intersect_rect(
        handle: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    pub fn ci_filter_shape_extent(
        handle: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
}
