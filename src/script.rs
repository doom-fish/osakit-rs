use core::ffi::c_void;
use std::path::Path;
use std::ptr;

use crate::descriptor::AppleEventDescriptor;
use crate::error::{from_swift, OsaKitError};
use crate::ffi;
use crate::language::{Language, LanguageInstance};
use crate::private::to_cstring;

pub struct Script {
    raw: *mut c_void,
}

impl Script {
    pub fn new(source: &str, language: Option<&Language>) -> Result<Self, OsaKitError> {
        let source = to_cstring(source)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_new(
                source.as_ptr(),
                language.map_or(ptr::null_mut(), |language| language.raw),
                &mut raw,
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(Self { raw })
    }

    pub fn from_file(
        path: impl AsRef<Path>,
        language: Option<&Language>,
    ) -> Result<Self, OsaKitError> {
        let path = path.as_ref().to_string_lossy().into_owned();
        let path = to_cstring(&path)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_from_file(
                path.as_ptr(),
                language.map_or(ptr::null_mut(), |language| language.raw),
                &mut raw,
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(Self { raw })
    }

    pub fn source(&self) -> Result<String, OsaKitError> {
        let ptr = unsafe { ffi::osa_script_source(self.raw) };
        if ptr.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null script source".into(),
            ));
        }
        Ok(crate::error::take_owned_c_string(ptr))
    }

    #[must_use]
    pub fn is_compiled(&self) -> bool {
        unsafe { ffi::osa_script_is_compiled(self.raw) }
    }

    pub fn language(&self) -> Result<Language, OsaKitError> {
        let raw = unsafe { ffi::osa_script_language(self.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null language for the script".into(),
            ));
        }
        Ok(Language { raw })
    }

    pub fn language_instance(&self) -> Result<LanguageInstance, OsaKitError> {
        let raw = unsafe { ffi::osa_script_language_instance(self.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null language instance for the script".into(),
            ));
        }
        Ok(LanguageInstance { raw })
    }

    pub fn compile(&self) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_compile(self.raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    pub fn execute(&self) -> Result<AppleEventDescriptor, OsaKitError> {
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_execute(self.raw, &mut raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit execution returned a null Apple event descriptor".into(),
            ));
        }
        Ok(AppleEventDescriptor { raw })
    }

    pub fn execute_apple_event(
        &self,
        event: &AppleEventDescriptor,
    ) -> Result<AppleEventDescriptor, OsaKitError> {
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_execute_apple_event(self.raw, event.raw, &mut raw, &mut error_ptr)
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit Apple event execution returned a null descriptor".into(),
            ));
        }
        Ok(AppleEventDescriptor { raw })
    }
}

impl Drop for Script {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::osa_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
