use core::ffi::c_char;

unsafe extern "C" {
    pub fn ci_image_format_value(code: i32) -> i32;
    pub fn ci_context_option_name(code: i32) -> *mut c_char;
    pub fn ci_image_option_name(code: i32) -> *mut c_char;
    pub fn ci_image_auto_adjust_option_name(code: i32) -> *mut c_char;
    pub fn ci_image_representation_option_name(code: i32) -> *mut c_char;
    pub fn ci_apply_option_name(code: i32) -> *mut c_char;
    pub fn ci_attribute_key_name(code: i32) -> *mut c_char;
    pub fn ci_attribute_type_name(code: i32) -> *mut c_char;
    pub fn ci_filter_category_name(code: i32) -> *mut c_char;
    pub fn ci_filter_category_value(code: i32) -> *mut c_char;
    pub fn ci_dynamic_range_name(code: i32) -> *mut c_char;
    pub fn ci_input_key_name(code: i32) -> *mut c_char;
    pub fn ci_input_key_value(code: i32) -> *mut c_char;
    pub fn ci_output_key_name(code: i32) -> *mut c_char;
    pub fn ci_output_key_value(code: i32) -> *mut c_char;
    pub fn ci_ui_parameter_set_key_name(code: i32) -> *mut c_char;
    pub fn ci_ui_parameter_set_name(code: i32) -> *mut c_char;
    pub fn ci_filter_generator_exported_key_name(code: i32) -> *mut c_char;
    pub fn ci_image_provider_option_name(code: i32) -> *mut c_char;
    pub fn ci_sampler_option_name(code: i32) -> *mut c_char;
    pub fn ci_raw_decoder_version_name(code: i32) -> *mut c_char;
}
