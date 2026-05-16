use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn osa_language_available_languages_json(error_out: *mut *mut c_char) -> *mut c_char;
    pub fn osa_language_for_name(name: *const c_char) -> *mut c_void;
    pub fn osa_language_for_script_data_descriptor(descriptor: *mut c_void) -> *mut c_void;
    pub fn osa_language_default() -> *mut c_void;
    pub fn osa_language_set_default(language: *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn osa_language_name(language: *mut c_void) -> *mut c_char;
    pub fn osa_language_info_json(
        language: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn osa_language_shared_instance(language: *mut c_void) -> *mut c_void;
}
