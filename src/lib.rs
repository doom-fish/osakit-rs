#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [OSAKit](https://developer.apple.com/documentation/osakit)
//! framework.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default
)]

/// Wraps the Carbon OSA component objects resolved through `OSAKit`.
pub mod component;
/// Wraps the Apple event descriptors used by `OSAKit` script APIs.
pub mod descriptor;
/// Re-exports structured `OSAKit` error types and helpers.
pub mod error;
/// Exposes the raw `OSAKit` FFI layer used by the safe wrappers.
pub mod ffi;
/// Wraps `OSALanguage` language catalog and lookup APIs.
pub mod language;
/// Wraps `OSALanguageInstance` execution-context APIs.
pub mod language_instance;
mod private;
/// Wraps `OSAScript` creation, compilation, and execution APIs.
pub mod script;
/// Wraps `OSAScriptController` editor-style control APIs.
pub mod script_controller;
/// Normalizes `OSAKit` script-error dictionaries and status codes.
pub mod script_error;
/// Wraps `OSAScriptView` editing configuration APIs.
pub mod script_view;

/// Re-exports OSA component wrappers resolved through `OSAKit`.
pub use component::{
    OsaComponent, OsaComponentInstance, OsaComponentInstanceSummary, OsaComponentSummary,
};
/// Re-exports the Apple event descriptor helpers used by `OSAKit`.
pub use descriptor::{AppleEventDescriptor, AppleEventDescriptorInfo};
/// Re-exports the `OSALanguage` wrappers and summary types.
pub use language::{Language, LanguageFeatures, LanguageSummary};
/// Re-exports the `OSALanguageInstance` wrapper.
pub use language_instance::LanguageInstance;
/// Re-exports the `OSAScript` wrapper and storage helpers.
pub use script::{Script, ScriptDisplayValue, ScriptStorageType, StorageOptions};
/// Re-exports the `OSAScriptController` wrapper and controller state.
pub use script_controller::{ScriptController, ScriptState};
/// Re-exports structured `OSAKit` error values and constants.
pub use script_error::{
    script_error_constants, OsaKitError, ScriptErrorConstants, ScriptErrorDetails, ScriptErrorRange,
};
/// Re-exports the `OSAScriptView` wrapper.
pub use script_view::ScriptView;

/// Common imports.
pub mod prelude {
    /// Re-exports the component wrappers for OSA component access.
    pub use crate::component::{OsaComponent, OsaComponentInstance};
    /// Re-exports the Apple event descriptor helper type.
    pub use crate::descriptor::AppleEventDescriptor;
    /// Re-exports the `OSALanguage` wrappers and feature flags.
    pub use crate::language::{Language, LanguageFeatures, LanguageSummary};
    /// Re-exports the `OSALanguageInstance` wrapper.
    pub use crate::language_instance::LanguageInstance;
    /// Re-exports the `OSAScript` wrapper and storage helpers.
    pub use crate::script::{Script, ScriptStorageType, StorageOptions};
    /// Re-exports the `OSAScriptController` wrapper and run states.
    pub use crate::script_controller::{ScriptController, ScriptState};
    /// Re-exports the primary `OSAKit` error types.
    pub use crate::script_error::{OsaKitError, ScriptErrorDetails};
    /// Re-exports the `OSAScriptView` wrapper.
    pub use crate::script_view::ScriptView;
}
