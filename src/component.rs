use core::ffi::c_void;
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::ffi;
use crate::language::{Language, LanguageSummary};
use crate::language_instance::LanguageInstance;
use crate::private::decode_json;
use crate::script_error::{from_swift, OsaKitError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Summarizes the Carbon OSA component backing an `OSALanguage`.
pub struct OsaComponentSummary {
    /// Holds the raw Carbon `ComponentInstance` pointer value reported by `OSAKit`.
    pub component_instance_pointer: u64,
    /// Captures the `OSALanguage` summary reported for the component.
    pub language: LanguageSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Summarizes the Carbon OSA component instance backing an `OSALanguageInstance`.
pub struct OsaComponentInstanceSummary {
    /// Holds the raw Carbon `ComponentInstance` pointer value reported by `OSAKit`.
    pub component_instance_pointer: u64,
    /// Captures the `OSALanguage` summary reported for the component.
    pub language: LanguageSummary,
}

#[derive(Debug)]
/// Wraps the Carbon OSA component resolved for an `OSALanguage`.
pub struct OsaComponent {
    raw: *mut c_void,
}

#[derive(Debug)]
/// Wraps the Carbon OSA component instance resolved for an `OSALanguageInstance`.
pub struct OsaComponentInstance {
    raw: *mut c_void,
}

impl OsaComponent {
    /// Resolves this wrapper from an `OSALanguage`.
    pub fn from_language(language: &Language) -> Result<Self, OsaKitError> {
        let raw = unsafe { ffi::osa_component_from_language(language.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit could not resolve a Carbon component for the language".into(),
            ));
        }
        Ok(Self { raw })
    }

    /// Resolves this wrapper from an `OSALanguageInstance`.
    pub fn from_language_instance(instance: &LanguageInstance) -> Result<Self, OsaKitError> {
        let raw = unsafe { ffi::osa_component_from_language_instance(instance.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit could not resolve a Carbon component for the language instance".into(),
            ));
        }
        Ok(Self { raw })
    }

    /// Returns the `OSALanguage` associated with this OSA component.
    pub fn language(&self) -> Result<Language, OsaKitError> {
        let raw = unsafe { ffi::osa_component_language(self.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null language for the component".into(),
            ));
        }
        Ok(Language { raw })
    }

    /// Returns summary metadata for this OSA component.
    pub fn summary(&self) -> Result<OsaComponentSummary, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let json = unsafe { ffi::osa_component_summary_json(self.raw, &mut error_ptr) };
        if json.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(json)
    }
}

impl Drop for OsaComponent {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::osa_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl OsaComponentInstance {
    /// Resolves this wrapper from an `OSALanguage`.
    pub fn from_language(language: &Language) -> Result<Self, OsaKitError> {
        let raw = unsafe { ffi::osa_component_instance_from_language(language.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null component instance for the language".into(),
            ));
        }
        Ok(Self { raw })
    }

    /// Resolves this wrapper from an `OSALanguageInstance`.
    pub fn from_language_instance(instance: &LanguageInstance) -> Result<Self, OsaKitError> {
        let raw = unsafe { ffi::osa_component_instance_from_language_instance(instance.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null component instance for the language instance".into(),
            ));
        }
        Ok(Self { raw })
    }

    /// Returns the backing OSA component for this component instance.
    pub fn component(&self) -> Result<OsaComponent, OsaKitError> {
        let raw = unsafe { ffi::osa_component_instance_component(self.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null component for the component instance".into(),
            ));
        }
        Ok(OsaComponent { raw })
    }

    /// Returns summary metadata for this OSA component instance.
    pub fn summary(&self) -> Result<OsaComponentInstanceSummary, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let json = unsafe { ffi::osa_component_instance_summary_json(self.raw, &mut error_ptr) };
        if json.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(json)
    }
}

impl Drop for OsaComponentInstance {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::osa_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
