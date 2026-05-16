use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn osa_object_release(ptr: *mut c_void);

    pub fn osa_descriptor_int32(value: i32) -> *mut c_void;
    pub fn osa_descriptor_string(value: *const c_char) -> *mut c_void;
    pub fn osa_descriptor_null() -> *mut c_void;
    pub fn osa_descriptor_descriptor_type(descriptor: *mut c_void) -> u32;
    pub fn osa_descriptor_int32_value(descriptor: *mut c_void) -> i32;
    pub fn osa_descriptor_boolean_value(descriptor: *mut c_void) -> bool;
    pub fn osa_descriptor_string_value(descriptor: *mut c_void) -> *mut c_char;
}
