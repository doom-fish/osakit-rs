use core::ffi::c_void;
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::{from_swift, OsaKitError};
use crate::ffi;
use crate::private::{decode_json, to_cstring};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageSummary {
    pub name: Option<String>,
    pub info: Option<String>,
    pub version: Option<String>,
    pub type_code: u32,
    pub sub_type: u32,
    pub manufacturer: u32,
    pub features: u64,
    pub thread_safe: bool,
}

pub struct Language {
    pub(crate) raw: *mut c_void,
}

pub struct LanguageInstance {
    pub(crate) raw: *mut c_void,
}

impl Language {
    pub fn available_languages() -> Result<Vec<LanguageSummary>, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let json = unsafe { ffi::osa_language_available_languages_json(&mut error_ptr) };
        if json.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(json)
    }

    pub fn for_name(name: &str) -> Result<Option<Self>, OsaKitError> {
        let name = to_cstring(name)?;
        let raw = unsafe { ffi::osa_language_for_name(name.as_ptr()) };
        Ok((!raw.is_null()).then_some(Self { raw }))
    }

    #[must_use]
    pub fn default_language() -> Option<Self> {
        let raw = unsafe { ffi::osa_language_default() };
        (!raw.is_null()).then_some(Self { raw })
    }

    pub fn name(&self) -> Result<Option<String>, OsaKitError> {
        let ptr = unsafe { ffi::osa_language_name(self.raw) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::error::take_owned_c_string(ptr)))
    }

    pub fn summary(&self) -> Result<LanguageSummary, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let json = unsafe { ffi::osa_language_info_json(self.raw, &mut error_ptr) };
        if json.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(json)
    }

    pub fn shared_instance(&self) -> Result<LanguageInstance, OsaKitError> {
        let raw = unsafe { ffi::osa_language_shared_instance(self.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null language instance".into(),
            ));
        }
        Ok(LanguageInstance { raw })
    }
}

impl LanguageInstance {
    pub fn language(&self) -> Result<Language, OsaKitError> {
        let raw = unsafe { ffi::osa_language_instance_language(self.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null language for the language instance".into(),
            ));
        }
        Ok(Language { raw })
    }

    pub fn summary(&self) -> Result<LanguageSummary, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let json = unsafe { ffi::osa_language_instance_info_json(self.raw, &mut error_ptr) };
        if json.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(json)
    }
}

impl Drop for Language {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::osa_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl Drop for LanguageInstance {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::osa_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
