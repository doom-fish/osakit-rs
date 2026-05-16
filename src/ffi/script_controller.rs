use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn osa_script_controller_new(
        out_controller: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_controller_script_view(controller: *mut c_void) -> *mut c_void;
    pub fn osa_script_controller_set_script_view(
        controller: *mut c_void,
        script_view: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_controller_script(controller: *mut c_void) -> *mut c_void;
    pub fn osa_script_controller_set_script(
        controller: *mut c_void,
        script: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_controller_language(controller: *mut c_void) -> *mut c_void;
    pub fn osa_script_controller_set_language(
        controller: *mut c_void,
        language: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_controller_result_text(controller: *mut c_void) -> *mut c_char;
    pub fn osa_script_controller_script_state(controller: *mut c_void) -> i32;
    pub fn osa_script_controller_is_compiling(controller: *mut c_void) -> bool;
    pub fn osa_script_controller_compile_script(
        controller: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_controller_record_script(
        controller: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_controller_run_script(
        controller: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_controller_stop_script(
        controller: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
}
