use core::ffi::c_void;
use std::ptr;

use crate::ffi;
use crate::language::Language;
use crate::script::Script;
use crate::script_error::{from_swift, OsaKitError};
use crate::script_view::ScriptView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptState {
    Stopped,
    Running,
    Recording,
}

impl ScriptState {
    fn from_raw(raw: i32) -> Result<Self, OsaKitError> {
        match raw {
            0 => Ok(Self::Stopped),
            1 => Ok(Self::Running),
            2 => Ok(Self::Recording),
            _ => Err(OsaKitError::FrameworkError(format!(
                "OSAKit returned an unknown script state: {raw}"
            ))),
        }
    }
}

#[derive(Debug)]
pub struct ScriptController {
    raw: *mut c_void,
}

impl ScriptController {
    pub fn new() -> Result<Self, OsaKitError> {
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_controller_new(&mut raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(Self { raw })
    }

    pub fn script_view(&self) -> Result<Option<ScriptView>, OsaKitError> {
        let raw = unsafe { ffi::osa_script_controller_script_view(self.raw) };
        Ok((!raw.is_null()).then_some(ScriptView { raw }))
    }

    pub fn set_script_view(&self, script_view: Option<&ScriptView>) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_controller_set_script_view(
                self.raw,
                script_view.map_or(ptr::null_mut(), |script_view| script_view.raw),
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    pub fn script(&self) -> Result<Option<Script>, OsaKitError> {
        let raw = unsafe { ffi::osa_script_controller_script(self.raw) };
        Ok((!raw.is_null()).then_some(Script { raw }))
    }

    pub fn set_script(&self, script: Option<&Script>) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_controller_set_script(
                self.raw,
                script.map_or(ptr::null_mut(), Script::as_ptr),
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    pub fn language(&self) -> Result<Option<Language>, OsaKitError> {
        let raw = unsafe { ffi::osa_script_controller_language(self.raw) };
        Ok((!raw.is_null()).then_some(Language { raw }))
    }

    pub fn set_language(&self, language: Option<&Language>) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_controller_set_language(
                self.raw,
                language.map_or(ptr::null_mut(), |language| language.raw),
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    pub fn result_text(&self) -> Result<Option<String>, OsaKitError> {
        let ptr = unsafe { ffi::osa_script_controller_result_text(self.raw) };
        Ok((!ptr.is_null()).then(|| crate::script_error::take_owned_c_string(ptr)))
    }

    pub fn script_state(&self) -> Result<ScriptState, OsaKitError> {
        ScriptState::from_raw(unsafe { ffi::osa_script_controller_script_state(self.raw) })
    }

    #[must_use]
    pub fn is_compiling(&self) -> bool {
        unsafe { ffi::osa_script_controller_is_compiling(self.raw) }
    }

    pub fn compile_script(&self) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_controller_compile_script(self.raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    pub fn record_script(&self) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_controller_record_script(self.raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    pub fn run_script(&self) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_controller_run_script(self.raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    pub fn stop_script(&self) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_controller_stop_script(self.raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }
}

impl Drop for ScriptController {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::osa_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
