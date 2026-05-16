use core::ffi::{c_char, c_void};

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const SCRIPT_ERROR: i32 = -2;
    pub const FRAMEWORK_ERROR: i32 = -3;
}

unsafe extern "C" {
    pub fn osa_object_release(ptr: *mut c_void);

    pub fn osa_language_available_languages_json(error_out: *mut *mut c_char) -> *mut c_char;
    pub fn osa_language_for_name(name: *const c_char) -> *mut c_void;
    pub fn osa_language_default() -> *mut c_void;
    pub fn osa_language_name(language: *mut c_void) -> *mut c_char;
    pub fn osa_language_info_json(
        language: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn osa_language_shared_instance(language: *mut c_void) -> *mut c_void;

    pub fn osa_language_instance_language(instance: *mut c_void) -> *mut c_void;
    pub fn osa_language_instance_info_json(
        instance: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;

    pub fn osa_script_new(
        source: *const c_char,
        language: *mut c_void,
        out_script: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_from_file(
        path: *const c_char,
        language: *mut c_void,
        out_script: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_source(script: *mut c_void) -> *mut c_char;
    pub fn osa_script_is_compiled(script: *mut c_void) -> bool;
    pub fn osa_script_language(script: *mut c_void) -> *mut c_void;
    pub fn osa_script_language_instance(script: *mut c_void) -> *mut c_void;
    pub fn osa_script_compile(script: *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn osa_script_execute(
        script: *mut c_void,
        out_descriptor: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_execute_apple_event(
        script: *mut c_void,
        event: *mut c_void,
        out_descriptor: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn osa_descriptor_int32(value: i32) -> *mut c_void;
    pub fn osa_descriptor_string(value: *const c_char) -> *mut c_void;
    pub fn osa_descriptor_null() -> *mut c_void;
    pub fn osa_descriptor_descriptor_type(descriptor: *mut c_void) -> u32;
    pub fn osa_descriptor_int32_value(descriptor: *mut c_void) -> i32;
    pub fn osa_descriptor_boolean_value(descriptor: *mut c_void) -> bool;
    pub fn osa_descriptor_string_value(descriptor: *mut c_void) -> *mut c_char;
}
