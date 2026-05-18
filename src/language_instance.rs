use core::ffi::c_void;
use std::ptr;

use crate::component::OsaComponentInstance;
use crate::descriptor::AppleEventDescriptor;
use crate::ffi;
use crate::language::{Language, LanguageSummary};
use crate::private::decode_json;
use crate::script_error::{from_swift, OsaKitError};

#[derive(Debug)]
/// Wraps the `OSALanguageInstance` execution context exposed by `OSAKit`.
pub struct LanguageInstance {
    pub(crate) raw: *mut c_void,
}

impl LanguageInstance {
    /// Creates an `OSALanguageInstance` for the given `OSALanguage`.
    pub fn new(language: &Language) -> Result<Self, OsaKitError> {
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status =
            unsafe { ffi::osa_language_instance_new(language.raw, &mut raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null language instance".into(),
            ));
        }
        Ok(Self { raw })
    }

    /// Returns the `OSALanguage` associated with this language instance.
    pub fn language(&self) -> Result<Language, OsaKitError> {
        let raw = unsafe { ffi::osa_language_instance_language(self.raw) };
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null language for the language instance".into(),
            ));
        }
        Ok(Language { raw })
    }

    /// Returns summary metadata for this `OSALanguageInstance`.
    pub fn summary(&self) -> Result<LanguageSummary, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let json = unsafe { ffi::osa_language_instance_info_json(self.raw, &mut error_ptr) };
        if json.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(json)
    }

    /// Returns the OSA component instance backing this `OSALanguageInstance`.
    pub fn component_instance(&self) -> Result<OsaComponentInstance, OsaKitError> {
        OsaComponentInstance::from_language_instance(self)
    }

    /// Returns the default target descriptor used by this `OSALanguageInstance`.
    pub fn default_target(&self) -> Result<Option<AppleEventDescriptor>, OsaKitError> {
        let raw = unsafe { ffi::osa_language_instance_default_target(self.raw) };
        Ok((!raw.is_null()).then_some(AppleEventDescriptor { raw }))
    }

    /// Sets the default target descriptor used by this `OSALanguageInstance`.
    pub fn set_default_target(
        &self,
        target: Option<&AppleEventDescriptor>,
    ) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_language_instance_set_default_target(
                self.raw,
                target.map_or(ptr::null_mut(), AppleEventDescriptor::as_ptr),
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    /// Returns `OSAKit`-generated rich text for a descriptor via this `OSALanguageInstance`.
    pub fn rich_text_from_descriptor(
        &self,
        descriptor: &AppleEventDescriptor,
    ) -> Result<Option<String>, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe {
            ffi::osa_language_instance_rich_text_from_descriptor(
                self.raw,
                descriptor.as_ptr(),
                &mut error_ptr,
            )
        };
        if ptr.is_null() {
            return if error_ptr.is_null() {
                Ok(None)
            } else {
                Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr))
            };
        }
        Ok(Some(crate::script_error::take_owned_c_string(ptr)))
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
