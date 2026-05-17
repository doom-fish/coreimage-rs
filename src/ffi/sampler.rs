use core::ffi::c_void;

unsafe extern "C" {
    pub fn ci_sampler_new(
        image: *mut c_void,
        wrap_mode: i32,
        filter_mode: i32,
        use_transform: bool,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        tx: f64,
        ty: f64,
        use_color_space: bool,
        color_space: i32,
    ) -> *mut c_void;
    pub fn ci_sampler_extent(
        handle: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn ci_sampler_definition_extent(
        handle: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
}
