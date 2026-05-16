use core::ffi::c_char;
use core::fmt;

use libc::free;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ffi;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptErrorDetails {
    pub message: Option<String>,
    pub brief_message: Option<String>,
    pub number: Option<i64>,
    pub app_name: Option<String>,
    pub range: Option<String>,
    pub raw: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OsaKitError {
    InvalidArgument(String),
    ScriptError(Box<ScriptErrorDetails>),
    FrameworkError(String),
    Unknown { code: i32, message: String },
}

impl OsaKitError {
    #[must_use]
    pub const fn code(&self) -> i32 {
        match self {
            Self::InvalidArgument(_) => ffi::status::INVALID_ARGUMENT,
            Self::ScriptError(_) => ffi::status::SCRIPT_ERROR,
            Self::FrameworkError(_) => ffi::status::FRAMEWORK_ERROR,
            Self::Unknown { code, .. } => *code,
        }
    }

    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::ScriptError(details) => details
                .message
                .clone()
                .or_else(|| details.brief_message.clone())
                .unwrap_or_else(|| "OSAKit script execution failed".into()),
            Self::InvalidArgument(message)
            | Self::FrameworkError(message)
            | Self::Unknown { message, .. } => message.clone(),
        }
    }
}

impl fmt::Display for OsaKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message(), self.code())
    }
}

impl std::error::Error for OsaKitError {}

pub(crate) fn take_owned_c_string(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let string = unsafe { core::ffi::CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { free(ptr.cast()) };
    string
}

pub(crate) fn from_swift(status: i32, error_str: *mut c_char) -> OsaKitError {
    let message = take_owned_c_string(error_str);
    if status == ffi::status::SCRIPT_ERROR {
        if let Ok(details) = serde_json::from_str::<ScriptErrorDetails>(&message) {
            return OsaKitError::ScriptError(Box::new(details));
        }
    }
    from_status_message(status, message)
}

pub(crate) fn from_status_message(status: i32, message: String) -> OsaKitError {
    match status {
        ffi::status::INVALID_ARGUMENT => OsaKitError::InvalidArgument(message),
        ffi::status::FRAMEWORK_ERROR => OsaKitError::FrameworkError(message),
        code => OsaKitError::Unknown { code, message },
    }
}
