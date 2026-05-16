use std::ffi::CString;
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::script_error::OsaKitError;

pub fn to_cstring(value: &str) -> Result<CString, OsaKitError> {
    CString::new(value).map_err(|_| {
        OsaKitError::InvalidArgument("strings must not contain interior NUL bytes".into())
    })
}

pub fn path_to_cstring(path: &Path) -> Result<CString, OsaKitError> {
    let path = path.to_string_lossy();
    to_cstring(&path)
}

pub fn optional_path_to_cstring(path: Option<&Path>) -> Result<Option<CString>, OsaKitError> {
    path.map(path_to_cstring).transpose()
}

pub fn decode_json<T: DeserializeOwned>(ptr: *mut core::ffi::c_char) -> Result<T, OsaKitError> {
    let json = crate::script_error::take_owned_c_string(ptr);
    serde_json::from_str(&json).map_err(|error| {
        OsaKitError::FrameworkError(format!("failed to decode bridge JSON payload: {error}"))
    })
}
