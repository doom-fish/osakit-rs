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

pub mod component;
pub mod descriptor;
pub mod error;
pub mod ffi;
pub mod language;
pub mod language_instance;
mod private;
pub mod script;
pub mod script_controller;
pub mod script_error;
pub mod script_view;

pub use component::{
    OsaComponent, OsaComponentInstance, OsaComponentInstanceSummary, OsaComponentSummary,
};
pub use descriptor::{AppleEventDescriptor, AppleEventDescriptorInfo};
pub use language::{Language, LanguageFeatures, LanguageSummary};
pub use language_instance::LanguageInstance;
pub use script::{Script, ScriptDisplayValue, ScriptStorageType, StorageOptions};
pub use script_controller::{ScriptController, ScriptState};
pub use script_error::{
    script_error_constants, OsaKitError, ScriptErrorConstants, ScriptErrorDetails, ScriptErrorRange,
};
pub use script_view::ScriptView;

/// Common imports.
pub mod prelude {
    pub use crate::component::{OsaComponent, OsaComponentInstance};
    pub use crate::descriptor::AppleEventDescriptor;
    pub use crate::language::{Language, LanguageFeatures, LanguageSummary};
    pub use crate::language_instance::LanguageInstance;
    pub use crate::script::{Script, ScriptStorageType, StorageOptions};
    pub use crate::script_controller::{ScriptController, ScriptState};
    pub use crate::script_error::{OsaKitError, ScriptErrorDetails};
    pub use crate::script_view::ScriptView;
}
