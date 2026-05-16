use core::ffi::c_char;

unsafe extern "C" {
    pub fn osa_script_error_constants_json() -> *mut c_char;
}
