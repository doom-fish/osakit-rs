use core::ffi::c_char;
use core::fmt;

use libc::free;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::descriptor::AppleEventDescriptorInfo;
use crate::ffi;
use crate::private::decode_json;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptErrorRange {
    pub location: usize,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptErrorDetails {
    pub message: Option<String>,
    pub brief_message: Option<String>,
    pub number: Option<i64>,
    pub partial_result: Option<AppleEventDescriptorInfo>,
    pub offending_object: Option<AppleEventDescriptorInfo>,
    pub expected_type: Option<AppleEventDescriptorInfo>,
    pub app_address: Option<AppleEventDescriptorInfo>,
    pub app_name: Option<String>,
    pub range: Option<ScriptErrorRange>,
    pub raw: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptErrorConstants {
    pub message_key: String,
    pub brief_message_key: String,
    pub number_key: String,
    pub partial_result_key: String,
    pub offending_object_key: String,
    pub expected_type_key: String,
    pub app_address_key: String,
    pub app_name_key: String,
    pub range_key: String,
    pub message: String,
    pub number: String,
    pub app_name: String,
    pub brief_message: String,
    pub range: String,
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

pub fn script_error_constants() -> Result<ScriptErrorConstants, OsaKitError> {
    let json = unsafe { ffi::osa_script_error_constants_json() };
    if json.is_null() {
        return Err(OsaKitError::FrameworkError(
            "OSAKit returned a null script-error constants payload".into(),
        ));
    }
    decode_json(json)
}

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
