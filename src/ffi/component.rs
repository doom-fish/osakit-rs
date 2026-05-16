use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn osa_component_from_language(language: *mut c_void) -> *mut c_void;
    pub fn osa_component_from_language_instance(instance: *mut c_void) -> *mut c_void;
    pub fn osa_component_language(component: *mut c_void) -> *mut c_void;
    pub fn osa_component_summary_json(
        component: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;

    pub fn osa_component_instance_from_language(language: *mut c_void) -> *mut c_void;
    pub fn osa_component_instance_from_language_instance(instance: *mut c_void) -> *mut c_void;
    pub fn osa_component_instance_component(instance: *mut c_void) -> *mut c_void;
    pub fn osa_component_instance_summary_json(
        instance: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}
