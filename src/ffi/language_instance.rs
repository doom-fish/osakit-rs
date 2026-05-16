use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn osa_language_instance_new(
        language: *mut c_void,
        out_instance: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_language_instance_language(instance: *mut c_void) -> *mut c_void;
    pub fn osa_language_instance_info_json(
        instance: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn osa_language_instance_default_target(instance: *mut c_void) -> *mut c_void;
    pub fn osa_language_instance_set_default_target(
        instance: *mut c_void,
        target: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_language_instance_rich_text_from_descriptor(
        instance: *mut c_void,
        descriptor: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}
