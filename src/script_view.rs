use core::ffi::c_void;
use std::ptr;

use crate::ffi;
use crate::private::to_cstring;
use crate::script_error::{from_swift, OsaKitError};

#[derive(Debug)]
/// Wraps the `OSAScriptView` editing view exposed by `OSAKit`.
pub struct ScriptView {
    pub(crate) raw: *mut c_void,
}

impl ScriptView {
    /// Creates an `OSAScriptView`.
    pub fn new() -> Result<Self, OsaKitError> {
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_view_new(&mut raw, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(Self { raw })
    }

    /// Returns the current source text shown by this `OSAScriptView`.
    pub fn source(&self) -> Result<Option<String>, OsaKitError> {
        let ptr = unsafe { ffi::osa_script_view_source(self.raw) };
        Ok((!ptr.is_null()).then(|| crate::script_error::take_owned_c_string(ptr)))
    }

    /// Sets the source text shown by this `OSAScriptView`.
    pub fn set_source(&self, source: Option<&str>) -> Result<(), OsaKitError> {
        let source = match source {
            Some(source) => Some(to_cstring(source)?),
            None => None,
        };
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_view_set_source(
                self.raw,
                source.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                &mut error_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    #[must_use]
    /// Reports whether `OSAScriptView` shows the script assistant UI.
    pub fn uses_script_assistant(&self) -> bool {
        unsafe { ffi::osa_script_view_uses_script_assistant(self.raw) }
    }

    /// Enables or disables the `OSAScriptView` script assistant UI.
    pub fn set_uses_script_assistant(&self, value: bool) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_view_set_uses_script_assistant(self.raw, value, &mut error_ptr)
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    #[must_use]
    /// Reports whether `OSAScriptView` inserts tab characters.
    pub fn uses_tabs(&self) -> bool {
        unsafe { ffi::osa_script_view_uses_tabs(self.raw) }
    }

    /// Enables or disables tab insertion in `OSAScriptView`.
    pub fn set_uses_tabs(&self, value: bool) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_view_set_uses_tabs(self.raw, value, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    #[must_use]
    /// Returns the tab width configured on `OSAScriptView`.
    pub fn tab_width(&self) -> u64 {
        unsafe { ffi::osa_script_view_tab_width(self.raw) }
    }

    /// Sets the tab width used by `OSAScriptView`.
    pub fn set_tab_width(&self, width: u64) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe { ffi::osa_script_view_set_tab_width(self.raw, width, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    #[must_use]
    /// Reports whether `OSAScriptView` wraps long lines.
    pub fn wraps_lines(&self) -> bool {
        unsafe { ffi::osa_script_view_wraps_lines(self.raw) }
    }

    /// Enables or disables line wrapping in `OSAScriptView`.
    pub fn set_wraps_lines(&self, value: bool) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status =
            unsafe { ffi::osa_script_view_set_wraps_lines(self.raw, value, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    #[must_use]
    /// Reports whether wrapped lines are indented in `OSAScriptView`.
    pub fn indents_wrapped_lines(&self) -> bool {
        unsafe { ffi::osa_script_view_indents_wrapped_lines(self.raw) }
    }

    /// Enables or disables wrapped-line indentation in `OSAScriptView`.
    pub fn set_indents_wrapped_lines(&self, value: bool) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::osa_script_view_set_indents_wrapped_lines(self.raw, value, &mut error_ptr)
        };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }

    #[must_use]
    /// Returns the indentation width configured on `OSAScriptView`.
    pub fn indent_width(&self) -> u64 {
        unsafe { ffi::osa_script_view_indent_width(self.raw) }
    }

    /// Sets the indentation width used by `OSAScriptView`.
    pub fn set_indent_width(&self, width: u64) -> Result<(), OsaKitError> {
        let mut error_ptr = ptr::null_mut();
        let status =
            unsafe { ffi::osa_script_view_set_indent_width(self.raw, width, &mut error_ptr) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error_ptr));
        }
        Ok(())
    }
}

impl Drop for ScriptView {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::osa_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
