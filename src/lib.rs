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

pub mod descriptor;
pub mod error;
pub mod ffi;
pub mod language;
mod private;
pub mod script;

pub use descriptor::AppleEventDescriptor;
pub use error::{OsaKitError, ScriptErrorDetails};
pub use language::{Language, LanguageInstance, LanguageSummary};
pub use script::Script;

/// Common imports.
pub mod prelude {
    pub use crate::descriptor::AppleEventDescriptor;
    pub use crate::error::{OsaKitError, ScriptErrorDetails};
    pub use crate::language::{Language, LanguageInstance, LanguageSummary};
    pub use crate::script::Script;
}
