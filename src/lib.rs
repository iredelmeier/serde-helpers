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
