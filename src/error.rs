use std::borrow::Borrow;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[cfg(feature = "serde_json")]
use serde_json::error::Error as JsonError;
#[cfg(feature = "serde_yaml")]
use serde_yaml::Error as YamlError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorKind {
    #[cfg(feature = "serde_json")]
    Json,
    #[cfg(feature = "serde_yaml")]
    Yaml,
    #[doc(hidden)]
    __Unknown,
}

#[derive(Debug)]
pub struct Error {
    description: String,
    kind: ErrorKind,
    cause: Option<Box<StdError>>,
}

impl Error {
    #[cfg(feature = "serde_json")]
    pub fn json(err: JsonError) -> Self {
        Error {
            description: err.description().to_string(),
            kind: ErrorKind::Json,
            cause: Some(Box::new(err)),
        }
    }

    #[cfg(feature = "serde_yaml")]
    pub fn yaml(err: YamlError) -> Self {
        Error {
            description: err.description().to_string(),
            kind: ErrorKind::Yaml,
            cause: Some(Box::new(err)),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn cause(&self) -> Option<&StdError> {
        match self.cause {
            Some(ref err) => Some(err.borrow()),
            None => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.description.fmt(f)
    }
}

#[cfg(feature = "serde_json")]
impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Self::json(err)
    }
}

#[cfg(feature = "serde_yaml")]
impl From<YamlError> for Error {
    fn from(err: YamlError) -> Self {
        Self::yaml(err)
    }
}
