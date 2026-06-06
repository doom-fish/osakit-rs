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
/// Normalizes the `NSRange` payload attached to `OSAKit` script errors.
pub struct ScriptErrorRange {
    /// Holds the starting character location reported by `OSAKit`.
    pub location: usize,
    /// Holds the character length reported by `OSAKit`.
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Normalizes the user-info dictionary returned for `OSAKit` script errors.
pub struct ScriptErrorDetails {
    /// Mirrors the `OSAScriptErrorMessage` entry from an `OSAKit` error dictionary.
    pub message: Option<String>,
    /// Mirrors the `OSAScriptErrorBriefMessage` entry from an `OSAKit` error dictionary.
    pub brief_message: Option<String>,
    /// Mirrors the `OSAScriptErrorNumber` entry from an `OSAKit` error dictionary.
    pub number: Option<i64>,
    /// Mirrors the `OSAScriptErrorPartialResult` descriptor entry from an `OSAKit` error dictionary.
    pub partial_result: Option<AppleEventDescriptorInfo>,
    /// Mirrors the `OSAScriptErrorOffendingObject` descriptor entry from an `OSAKit` error dictionary.
    pub offending_object: Option<AppleEventDescriptorInfo>,
    /// Mirrors the `OSAScriptErrorExpectedType` descriptor entry from an `OSAKit` error dictionary.
    pub expected_type: Option<AppleEventDescriptorInfo>,
    /// Mirrors the `OSAScriptErrorAppAddress` descriptor entry from an `OSAKit` error dictionary.
    pub app_address: Option<AppleEventDescriptorInfo>,
    /// Mirrors the `OSAScriptErrorAppName` entry from an `OSAKit` error dictionary.
    pub app_name: Option<String>,
    /// Mirrors the `OSAScriptErrorRange` entry from an `OSAKit` error dictionary.
    pub range: Option<ScriptErrorRange>,
    /// Retains the full `OSAKit` error dictionary payload for unmodeled keys.
    pub raw: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Exposes the script-error key strings exported by `OSAKit`.
pub struct ScriptErrorConstants {
    /// Contains the key string `OSAKit` uses for the message entry.
    pub message_key: String,
    /// Contains the key string `OSAKit` uses for the brief-message entry.
    pub brief_message_key: String,
    /// Contains the key string `OSAKit` uses for the number entry.
    pub number_key: String,
    /// Contains the key string `OSAKit` uses for the partial-result entry.
    pub partial_result_key: String,
    /// Contains the key string `OSAKit` uses for the offending-object entry.
    pub offending_object_key: String,
    /// Contains the key string `OSAKit` uses for the expected-type entry.
    pub expected_type_key: String,
    /// Contains the key string `OSAKit` uses for the app-address entry.
    pub app_address_key: String,
    /// Contains the key string `OSAKit` uses for the app-name entry.
    pub app_name_key: String,
    /// Contains the key string `OSAKit` uses for the range entry.
    pub range_key: String,
    /// Contains the exported constant value for the message entry.
    pub message: String,
    /// Contains the exported constant value for the number entry.
    pub number: String,
    /// Contains the exported constant value for the app-name entry.
    pub app_name: String,
    /// Contains the exported constant value for the brief-message entry.
    pub brief_message: String,
    /// Contains the exported constant value for the range entry.
    pub range: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Represents a failure returned by `OSAKit`.
pub enum OsaKitError {
    /// Indicates that invalid arguments were passed into `OSAKit`.
    InvalidArgument(String),
    /// Carries the structured script-error dictionary returned by `OSAKit`.
    ScriptError(Box<ScriptErrorDetails>),
    /// Carries a non-script framework error reported by `OSAKit`.
    FrameworkError(String),
    /// Carries an unclassified `OSAKit` status code and message.
    Unknown {
        /// Stores the raw `OSAKit` status code.
        code: i32,
        /// Stores the `OSAKit` message paired with the raw status code.
        message: String,
    },
}

impl OsaKitError {
    #[must_use]
    /// Returns the underlying `OSAKit` status code for this error.
    pub const fn code(&self) -> i32 {
        match self {
            Self::InvalidArgument(_) => ffi::status::INVALID_ARGUMENT,
            Self::ScriptError(_) => ffi::status::SCRIPT_ERROR,
            Self::FrameworkError(_) => ffi::status::FRAMEWORK_ERROR,
            Self::Unknown { code, .. } => *code,
        }
    }

    #[must_use]
    /// Returns the best human-readable message for this `OSAKit` error.
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

/// Returns the script-error key strings exported by `OSAKit`.
pub fn script_error_constants() -> Result<ScriptErrorConstants, OsaKitError> {
    // SAFETY: ffi::osa_script_error_constants_json() returns a freshly malloc'd,
    // null-terminated C string that transfers ownership to the caller. decode_json()
    // takes ownership and frees it exactly once via take_owned_c_string().
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

    // SAFETY: ptr came from Swift's OSAKit framework and is guaranteed to be a valid,
    // null-terminated C string allocated with malloc. We convert it to a Rust String
    // immediately and then free the original allocation, making this a safe transfer of ownership.
    let string = unsafe { core::ffi::CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    // SAFETY: ptr is guaranteed to have been allocated by OSAKit with malloc,
    // and we are freeing it exactly once here.
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
