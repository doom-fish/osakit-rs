use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn osa_script_view_new(out_view: *mut *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn osa_script_view_source(view: *mut c_void) -> *mut c_char;
    pub fn osa_script_view_set_source(
        view: *mut c_void,
        source: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_view_uses_script_assistant(view: *mut c_void) -> bool;
    pub fn osa_script_view_set_uses_script_assistant(
        view: *mut c_void,
        value: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_view_uses_tabs(view: *mut c_void) -> bool;
    pub fn osa_script_view_set_uses_tabs(
        view: *mut c_void,
        value: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_view_tab_width(view: *mut c_void) -> u64;
    pub fn osa_script_view_set_tab_width(
        view: *mut c_void,
        width: u64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_view_wraps_lines(view: *mut c_void) -> bool;
    pub fn osa_script_view_set_wraps_lines(
        view: *mut c_void,
        value: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_view_indents_wrapped_lines(view: *mut c_void) -> bool;
    pub fn osa_script_view_set_indents_wrapped_lines(
        view: *mut c_void,
        value: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn osa_script_view_indent_width(view: *mut c_void) -> u64;
    pub fn osa_script_view_set_indent_width(
        view: *mut c_void,
        width: u64,
        error_out: *mut *mut c_char,
    ) -> i32;
}
