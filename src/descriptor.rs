use core::ffi::c_void;
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::ffi;
use crate::private::to_cstring;
use crate::script_error::OsaKitError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppleEventDescriptorInfo {
    pub descriptor_type: u32,
    pub int32_value: i32,
    pub boolean_value: bool,
    pub string_value: Option<String>,
}

#[derive(Debug)]
pub struct AppleEventDescriptor {
    pub(crate) raw: *mut c_void,
}

impl AppleEventDescriptor {
    pub(crate) fn from_raw(raw: *mut c_void) -> Result<Self, OsaKitError> {
        if raw.is_null() {
            return Err(OsaKitError::FrameworkError(
                "OSAKit returned a null Apple event descriptor".into(),
            ));
        }
        Ok(Self { raw })
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw
    }

    #[must_use]
    pub fn int32(value: i32) -> Self {
        Self {
            raw: unsafe { ffi::osa_descriptor_int32(value) },
        }
    }

    pub fn string(value: &str) -> Result<Self, OsaKitError> {
        let value = to_cstring(value)?;
        Self::from_raw(unsafe { ffi::osa_descriptor_string(value.as_ptr()) })
    }

    #[must_use]
    pub fn null() -> Self {
        Self {
            raw: unsafe { ffi::osa_descriptor_null() },
        }
    }

    #[must_use]
    pub fn descriptor_type(&self) -> u32 {
        unsafe { ffi::osa_descriptor_descriptor_type(self.raw) }
    }

    #[must_use]
    pub fn int32_value(&self) -> Option<i32> {
        let value = unsafe { ffi::osa_descriptor_int32_value(self.raw) };
        (self.descriptor_type() != 0).then_some(value)
    }

    #[must_use]
    pub fn boolean_value(&self) -> bool {
        unsafe { ffi::osa_descriptor_boolean_value(self.raw) }
    }

    #[must_use]
    pub fn string_value(&self) -> Option<String> {
        let ptr = unsafe { ffi::osa_descriptor_string_value(self.raw) };
        if ptr.is_null() {
            return None;
        }
        Some(crate::script_error::take_owned_c_string(ptr))
    }

    #[must_use]
    pub fn info(&self) -> AppleEventDescriptorInfo {
        AppleEventDescriptorInfo {
            descriptor_type: self.descriptor_type(),
            int32_value: self.int32_value().unwrap_or_default(),
            boolean_value: self.boolean_value(),
            string_value: self.string_value(),
        }
    }
}

impl Drop for AppleEventDescriptor {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::osa_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
