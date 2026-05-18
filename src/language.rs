use core::ffi::c_void;
use std::ptr;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::component::OsaComponent;
use crate::descriptor::AppleEventDescriptor;
use crate::ffi;
use crate::private::{decode_json, to_cstring};
use crate::script_error::{from_swift, OsaKitError};

/// Re-exports the `OSALanguageInstance` wrapper used by `OSALanguage`.
pub use crate::language_instance::LanguageInstance;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// Mirrors the feature flags exposed by `OSALanguage`.
    pub struct LanguageFeatures: u64 {
        /// Matches `OSALanguage` languages that can compile source.
        const SUPPORTS_COMPILING = 0x0002;
        /// Matches `OSALanguage` languages that can return source text.
        const SUPPORTS_GET_SOURCE = 0x0004;
        /// Matches `OSALanguage` languages that coerce Apple event descriptors.
        const SUPPORTS_AE_COERCION = 0x0008;
        /// Matches `OSALanguage` languages that send Apple events.
        const SUPPORTS_AE_SENDING = 0x0010;
        /// Matches `OSALanguage` languages that support recording.
        const SUPPORTS_RECORDING = 0x0020;
        /// Matches `OSALanguage` languages that expose convenience routines.
        const SUPPORTS_CONVENIENCE = 0x0040;
        /// Matches `OSALanguage` languages that support dialects.
        const SUPPORTS_DIALECTS = 0x0080;
        /// Matches `OSALanguage` languages that expose Apple event handlers.
        const SUPPORTS_EVENT_HANDLING = 0x0100;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Summarizes an `OSALanguage` entry reported by `OSAKit`.
pub struct LanguageSummary {
    /// Carries the human-readable name reported by `OSALanguage`.
    pub name: Option<String>,
    /// Carries the descriptive info string reported by `OSALanguage`.
    pub info: Option<String>,
    /// Carries the version string reported by `OSALanguage`.
    pub version: Option<String>,
    /// Holds the `OSType` language type code reported by `OSAKit`.
    pub type_code: u32,
    /// Holds the `OSType` language subtype code reported by `OSAKit`.
    pub sub_type: u32,
    /// Holds the `OSType` manufacturer code reported by `OSAKit`.
    pub manufacturer: u32,
    /// Holds the raw `OSALanguage` feature bitmask.
    pub features: u64,
    /// Reports whether `OSALanguage` declares thread-safe execution.
    pub thread_safe: bool,
}

impl LanguageSummary {
    #[must_use]
    /// Decodes the raw `OSALanguage` feature bitmask into flags.
    pub fn feature_flags(&self) -> LanguageFeatures {
        LanguageFeatures::from_bits_truncate(self.features)
    }
}

#[derive(Debug)]
/// Wraps the `OSALanguage` objects exposed by `OSAKit`.
pub struct Language {
    pub(crate) raw: *mut c_void,
}

impl Language {
    /// Lists the languages exposed by `OSALanguage.availableLanguages`.
    pub fn available_languages() -> Result<Vec<LanguageSummary>, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let json = unsafe { ffi::osa_language_available_languages_json(&mut error_ptr) };
        if json.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(json)
    }

    /// Looks up an `OSALanguage` by its user-visible name.
    pub fn for_name(name: &str) -> Result<Option<Self>, OsaKitError> {
        let name = to_cstring(name)?;
        let raw = unsafe { ffi::osa_language_for_name(name.as_ptr()) };
        Ok((!raw.is_null()).then_some(Self { raw }))
    }

    /// Resolves the `OSALanguage` for a compiled script data descriptor.
    pub fn for_script_data_descriptor(
        descriptor: &AppleEventDescriptor,
    ) -> Result<Option<Self>, OsaKitError> {
        let raw = unsafe { ffi::osa_language_for_script_data_descriptor(descriptor.as_ptr()) };
        Ok((!raw.is_null()).then_some(Self { raw }))
    }

    #[must_use]
    /// Returns the default `OSALanguage` selected by `OSAKit`.
    pub fn default_language() -> Option<Self> {
        let raw = unsafe { ffi::osa_language_default() };
        (!raw.is_null()).then_some(Self { raw })
    }

    /// Sets the default `OSALanguage` used by `OSAKit`.
    pub fn set_default(language: &Self) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_language_set_default(language.raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    /// Sets this `OSALanguage` as `OSAKit`'s default language.
    pub fn set_as_default(&self) -> Result<(), OsaKitError> {
        Self::set_default(self)
    }

    /// Returns the localized name of this `OSALanguage`.
    pub fn name(&self) -> Result<Option<String>, OsaKitError> {
        let ptr = unsafe { ffi::osa_language_name(self.raw) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::script_error::take_owned_c_string(ptr)))
    }

    /// Returns summary metadata for this `OSALanguage`.
    pub fn summary(&self) -> Result<LanguageSummary, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let json = unsafe { ffi::osa_language_info_json(self.raw, &mut error_ptr) };
        if json.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(json)
    }

    /// Returns the shared `OSALanguageInstance` for this language.
    pub fn shared_instance(&self) -> Result<LanguageInstance, OsaKitError> {
        let raw = unsafe { ffi::osa_language_shared_instance(self.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null language instance".into(),
            ));
        }
        Ok(LanguageInstance { raw })
    }

    /// Returns the backing OSA component for this `OSALanguage`.
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
