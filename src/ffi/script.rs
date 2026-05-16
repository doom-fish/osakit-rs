use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn osa_script_new(
        source: *const c_char,
        language: *mut c_void,
        out_script: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_new_with_options(
        source: *const c_char,
        url_path: *const c_char,
        language_instance: *mut c_void,
        storage_options: u64,
        out_script: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_from_file(
        path: *const c_char,
        language: *mut c_void,
        out_script: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_from_file_with_options(
        path: *const c_char,
        language_instance: *mut c_void,
        storage_options: u64,
        out_script: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_from_compiled_data(
        bytes: *const c_void,
        length: usize,
        url_path: *const c_char,
        storage_options: u64,
        out_script: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_from_script_data_descriptor(
        descriptor: *mut c_void,
        url_path: *const c_char,
        language_instance: *mut c_void,
        storage_options: u64,
        out_script: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_data_descriptor_from_file(
        path: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn osa_script_source(script: *mut c_void) -> *mut c_char;
    pub fn osa_script_rich_text_source(script: *mut c_void) -> *mut c_char;
    pub fn osa_script_url(script: *mut c_void) -> *mut c_char;
    pub fn osa_script_is_compiled(script: *mut c_void) -> bool;
    pub fn osa_script_language(script: *mut c_void) -> *mut c_void;
    pub fn osa_script_set_language(
        script: *mut c_void,
        language: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_language_instance(script: *mut c_void) -> *mut c_void;
    pub fn osa_script_set_language_instance(
        script: *mut c_void,
        instance: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
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
    pub fn osa_script_execute_and_return_display_value(
        script: *mut c_void,
        out_descriptor: *mut *mut c_void,
        out_display_value: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_execute_handler(
        script: *mut c_void,
        name: *const c_char,
        arguments: *const *mut c_void,
        argument_count: usize,
        out_descriptor: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_rich_text_from_descriptor(
        script: *mut c_void,
        descriptor: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn osa_script_write_to_url(
        script: *mut c_void,
        path: *const c_char,
        storage_type: *const c_char,
        storage_options: u64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_compiled_data(
        script: *mut c_void,
        storage_type: *const c_char,
        storage_options: u64,
        out_length: *mut usize,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
}
