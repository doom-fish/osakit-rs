use core::ffi::c_void;
use std::path::{Path, PathBuf};
use std::ptr;

use bitflags::bitflags;
use libc::free;

use crate::descriptor::AppleEventDescriptor;
use crate::ffi;
use crate::language::Language;
use crate::language_instance::LanguageInstance;
use crate::private::{optional_path_to_cstring, path_to_cstring, to_cstring};
use crate::script_error::{from_swift, OsaKitError};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StorageOptions: u64 {
        const NONE = 0x0000_0000;
        const PREVENT_GET_SOURCE = 0x0000_0001;
        const COMPILE_INTO_CONTEXT = 0x0000_0002;
        const DONT_SET_SCRIPT_LOCATION = 0x0100_0000;
        const STAY_OPEN_APPLET = 0x1000_0000;
        const SHOW_STARTUP_SCREEN = 0x2000_0000;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptStorageType {
    Script,
    ScriptBundle,
    Application,
    ApplicationBundle,
    Text,
}

impl ScriptStorageType {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Script => "script",
            Self::ScriptBundle => "script bundle",
            Self::Application => "application",
            Self::ApplicationBundle => "application bundle",
            Self::Text => "text",
        }
    }
}

#[derive(Debug)]
pub struct ScriptDisplayValue {
    pub descriptor: AppleEventDescriptor,
    pub display_value: Option<String>,
}

#[derive(Debug)]
pub struct Script {
    pub(crate) raw: *mut c_void,
}

impl Script {
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw
    }

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

    pub fn from_source_with_options(
        source: &str,
        source_url: Option<&Path>,
        language_instance: Option<&LanguageInstance>,
        storage_options: StorageOptions,
    ) -> Result<Self, OsaKitError> {
        let source = to_cstring(source)?;
        let source_url = optional_path_to_cstring(source_url)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_new_with_options(
                source.as_ptr(),
                source_url
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                language_instance.map_or(ptr::null_mut(), |instance| instance.raw),
                storage_options.bits(),
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
        let path = path_to_cstring(path.as_ref())?;
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

    pub fn from_file_with_options(
        path: impl AsRef<Path>,
        language_instance: Option<&LanguageInstance>,
        storage_options: StorageOptions,
    ) -> Result<Self, OsaKitError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_from_file_with_options(
                path.as_ptr(),
                language_instance.map_or(ptr::null_mut(), |instance| instance.raw),
                storage_options.bits(),
                &mut raw,
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(Self { raw })
    }

    pub fn from_compiled_data(
        data: &[u8],
        source_url: Option<&Path>,
        storage_options: StorageOptions,
    ) -> Result<Self, OsaKitError> {
        let source_url = optional_path_to_cstring(source_url)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_from_compiled_data(
                data.as_ptr().cast(),
                data.len(),
                source_url
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                storage_options.bits(),
                &mut raw,
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(Self { raw })
    }

    pub fn from_script_data_descriptor(
        descriptor: &AppleEventDescriptor,
        source_url: Option<&Path>,
        language_instance: Option<&LanguageInstance>,
        storage_options: StorageOptions,
    ) -> Result<Self, OsaKitError> {
        let source_url = optional_path_to_cstring(source_url)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_from_script_data_descriptor(
                descriptor.as_ptr(),
                source_url
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                language_instance.map_or(ptr::null_mut(), |instance| instance.raw),
                storage_options.bits(),
                &mut raw,
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(Self { raw })
    }

    pub fn script_data_descriptor_from_file(
        path: impl AsRef<Path>,
    ) -> Result<AppleEventDescriptor, OsaKitError> {
        let path = path_to_cstring(path.as_ref())?;
        let mut error_ptr = ptr::null_mut();
        let raw =
            unsafe { ffi::osa_script_data_descriptor_from_file(path.as_ptr(), &mut error_ptr) };
        if raw.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        Ok(AppleEventDescriptor { raw })
    }

    pub fn source(&self) -> Result<String, OsaKitError> {
        let ptr = unsafe { ffi::osa_script_source(self.raw) };
        if ptr.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null script source".into(),
            ));
        }
        Ok(crate::script_error::take_owned_c_string(ptr))
    }

    pub fn rich_text_source(&self) -> Result<Option<String>, OsaKitError> {
        let ptr = unsafe { ffi::osa_script_rich_text_source(self.raw) };
        Ok((!ptr.is_null()).then(|| crate::script_error::take_owned_c_string(ptr)))
    }

    pub fn url(&self) -> Result<Option<PathBuf>, OsaKitError> {
        let ptr = unsafe { ffi::osa_script_url(self.raw) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(PathBuf::from(
            crate::script_error::take_owned_c_string(ptr),
        )))
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

    pub fn set_language(&self, language: &Language) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status =
            unsafe { ffi::osa_script_set_language(self.raw, language.raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
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

    pub fn set_language_instance(&self, instance: &LanguageInstance) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_set_language_instance(self.raw, instance.raw, &mut error_ptr)
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
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
        AppleEventDescriptor::from_raw(raw)
    }

    pub fn execute_apple_event(
        &self,
        event: &AppleEventDescriptor,
    ) -> Result<AppleEventDescriptor, OsaKitError> {
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_execute_apple_event(self.raw, event.as_ptr(), &mut raw, &mut error_ptr)
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        AppleEventDescriptor::from_raw(raw)
    }

    pub fn execute_and_return_display_value(&self) -> Result<ScriptDisplayValue, OsaKitError> {
        let mut raw = ptr::null_mut();
        let mut display_ptr = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_execute_and_return_display_value(
                self.raw,
                &mut raw,
                &mut display_ptr,
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(ScriptDisplayValue {
            descriptor: AppleEventDescriptor::from_raw(raw)?,
            display_value: (!display_ptr.is_null())
                .then(|| crate::script_error::take_owned_c_string(display_ptr)),
        })
    }

    pub fn execute_handler(
        &self,
        name: &str,
        arguments: &[AppleEventDescriptor],
    ) -> Result<AppleEventDescriptor, OsaKitError> {
        let name = to_cstring(name)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let raw_arguments = arguments
            .iter()
            .map(AppleEventDescriptor::as_ptr)
            .collect::<Vec<_>>();
        let status = unsafe {
            ffi::osa_script_execute_handler(
                self.raw,
                name.as_ptr(),
                if raw_arguments.is_empty() {
                    ptr::null()
                } else {
                    raw_arguments.as_ptr()
                },
                raw_arguments.len(),
                &mut raw,
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        AppleEventDescriptor::from_raw(raw)
    }

    pub fn rich_text_from_descriptor(
        &self,
        descriptor: &AppleEventDescriptor,
    ) -> Result<Option<String>, OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe {
            ffi::osa_script_rich_text_from_descriptor(self.raw, descriptor.as_ptr(), &mut error_ptr)
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

    pub fn write_to_file(
        &self,
        path: impl AsRef<Path>,
        storage_type: ScriptStorageType,
        storage_options: StorageOptions,
    ) -> Result<(), OsaKitError> {
        let path = path_to_cstring(path.as_ref())?;
        let storage_type = to_cstring(storage_type.as_str())?;
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_write_to_url(
                self.raw,
                path.as_ptr(),
                storage_type.as_ptr(),
                storage_options.bits(),
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    pub fn compiled_data(
        &self,
        storage_type: ScriptStorageType,
        storage_options: StorageOptions,
    ) -> Result<Vec<u8>, OsaKitError> {
        let storage_type = to_cstring(storage_type.as_str())?;
        let mut out_length = 0_usize;
        let mut error_ptr = ptr::null_mut();
        let raw = unsafe {
            ffi::osa_script_compiled_data(
                self.raw,
                storage_type.as_ptr(),
                storage_options.bits(),
                &mut out_length,
                &mut error_ptr,
            )
        };
        if raw.is_null() {
            return if error_ptr.is_null() && out_length == 0 {
                Ok(Vec::new())
            } else {
                Err(from_swift(ffi::status::SCRIPT_ERROR, error_ptr))
            };
        }
        let bytes = unsafe { std::slice::from_raw_parts(raw.cast::<u8>(), out_length) }.to_vec();
        unsafe { free(raw.cast()) };
        Ok(bytes)
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
