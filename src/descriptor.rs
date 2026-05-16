use core::ffi::c_void;
use std::ptr;

use crate::error::OsaKitError;
use crate::ffi;
use crate::private::to_cstring;

pub struct AppleEventDescriptor {
    pub(crate) raw: *mut c_void,
}

impl AppleEventDescriptor {
    #[must_use]
    pub fn int32(value: i32) -> Self {
        Self {
            raw: unsafe { ffi::osa_descriptor_int32(value) },
        }
    }

    pub fn string(value: &str) -> Result<Self, OsaKitError> {
        let value = to_cstring(value)?;
        Ok(Self {
            raw: unsafe { ffi::osa_descriptor_string(value.as_ptr()) },
        })
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
        Some(crate::error::take_owned_c_string(ptr))
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
