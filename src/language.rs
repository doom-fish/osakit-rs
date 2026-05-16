use core::ffi::c_void;
use std::ptr;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::component::OsaComponent;
use crate::descriptor::AppleEventDescriptor;
use crate::ffi;
use crate::private::{decode_json, to_cstring};
use crate::script_error::{from_swift, OsaKitError};

pub use crate::language_instance::LanguageInstance;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LanguageFeatures: u64 {
        const SUPPORTS_COMPILING = 0x0002;
        const SUPPORTS_GET_SOURCE = 0x0004;
        const SUPPORTS_AE_COERCION = 0x0008;
        const SUPPORTS_AE_SENDING = 0x0010;
        const SUPPORTS_RECORDING = 0x0020;
        const SUPPORTS_CONVENIENCE = 0x0040;
        const SUPPORTS_DIALECTS = 0x0080;
        const SUPPORTS_EVENT_HANDLING = 0x0100;
    }
}

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

impl LanguageSummary {
    #[must_use]
    pub fn feature_flags(&self) -> LanguageFeatures {
        LanguageFeatures::from_bits_truncate(self.features)
    }
}

#[derive(Debug)]
pub struct Language {
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

    pub fn for_script_data_descriptor(
        descriptor: &AppleEventDescriptor,
    ) -> Result<Option<Self>, OsaKitError> {
        let raw = unsafe { ffi::osa_language_for_script_data_descriptor(descriptor.as_ptr()) };
        Ok((!raw.is_null()).then_some(Self { raw }))
    }

    #[must_use]
    pub fn default_language() -> Option<Self> {
        let raw = unsafe { ffi::osa_language_default() };
        (!raw.is_null()).then_some(Self { raw })
    }

    pub fn set_default(language: &Self) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_language_set_default(language.raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    pub fn set_as_default(&self) -> Result<(), OsaKitError> {
        Self::set_default(self)
    }

    pub fn name(&self) -> Result<Option<String>, OsaKitError> {
        let ptr = unsafe { ffi::osa_language_name(self.raw) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::script_error::take_owned_c_string(ptr)))
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

    pub fn component(&self) -> Result<OsaComponent, OsaKitError> {
        OsaComponent::from_language(self)
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
