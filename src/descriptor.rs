use core::ffi::c_void;
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::ffi;
use crate::private::to_cstring;
use crate::script_error::OsaKitError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Summarizes the bridged `NSAppleEventDescriptor` value returned by `OSAKit`.
pub struct AppleEventDescriptorInfo {
    /// Mirrors the four-char descriptor type reported by `NSAppleEventDescriptor`.
    pub descriptor_type: u32,
    /// Mirrors the 32-bit integer payload when the descriptor stores one.
    pub int32_value: i32,
    /// Mirrors the boolean payload when the descriptor stores one.
    pub boolean_value: bool,
    /// Mirrors the string payload when the descriptor stores one.
    pub string_value: Option<String>,
}

#[derive(Debug)]
/// Wraps the `NSAppleEventDescriptor` values consumed and produced by `OSAKit`.
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
    /// Creates an `NSAppleEventDescriptor` integer value.
    pub fn int32(value: i32) -> Self {
        Self {
            raw: unsafe { ffi::osa_descriptor_int32(value) },
        }
    }

    /// Creates an `NSAppleEventDescriptor` string value.
    pub fn string(value: &str) -> Result<Self, OsaKitError> {
        let value = to_cstring(value)?;
        Self::from_raw(unsafe { ffi::osa_descriptor_string(value.as_ptr()) })
    }

    #[must_use]
    /// Creates the null `NSAppleEventDescriptor` value used by `OSAKit`.
    pub fn null() -> Self {
        Self {
            raw: unsafe { ffi::osa_descriptor_null() },
        }
    }

    #[must_use]
    /// Returns the raw descriptor type of this `NSAppleEventDescriptor`.
    pub fn descriptor_type(&self) -> u32 {
        unsafe { ffi::osa_descriptor_descriptor_type(self.raw) }
    }

    #[must_use]
    /// Returns the 32-bit integer payload of this descriptor when available.
    pub fn int32_value(&self) -> Option<i32> {
        let value = unsafe { ffi::osa_descriptor_int32_value(self.raw) };
        (self.descriptor_type() != 0).then_some(value)
    }

    #[must_use]
    /// Returns the boolean payload of this descriptor.
    pub fn boolean_value(&self) -> bool {
        unsafe { ffi::osa_descriptor_boolean_value(self.raw) }
    }

    #[must_use]
    /// Returns the string payload of this descriptor when available.
    pub fn string_value(&self) -> Option<String> {
        let ptr = unsafe { ffi::osa_descriptor_string_value(self.raw) };
        if ptr.is_null() {
            return None;
        }
        Some(crate::script_error::take_owned_c_string(ptr))
    }

    #[must_use]
    /// Returns a summary of this `NSAppleEventDescriptor` for debugging or errors.
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
