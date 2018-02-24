//! # serde-helpers
//!
//! `serde-helpers` provides wrappers for serializing and deserializing data structures
//! to and from certain data formats.
//!
//! The library is primarily intended to keep crates that need to serialize or deserialize
//! data from having to be aware of `serde` itself, if they do not otherwise have a need
//! to consume it.
//!
//! Additionally, `serde-helpers` provides some additional consistency across data formats,
//! e.g., by providing a single, consolidated `Error` struct that wraps the format-specific
//! error.
extern crate serde;
#[cfg(feature = "serde_json")]
extern crate serde_json;
#[cfg(feature = "serde_yaml")]
extern crate serde_yaml;

pub use error::{Error, ErrorKind};
#[cfg(feature = "serde_json")]
pub use json::{DeserializeJson, SerializeJson};
#[cfg(feature = "serde_yaml")]
pub use yaml::{DeserializeYaml, SerializeYaml};

mod error;
#[cfg(feature = "serde_json")]
mod json;
#[cfg(feature = "serde_yaml")]
mod yaml;
