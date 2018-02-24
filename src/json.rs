use serde::{Deserialize, Serialize};
use serde_json;

use Error;

pub trait DeserializeJson<'a>: Deserialize<'a> {
    fn from_json_str(s: &'a str) -> Result<Self, Error> {
        serde_json::from_str(s).map_err(Error::from)
    }

    fn from_json_slice(v: &'a [u8]) -> Result<Self, Error> {
        serde_json::from_slice(v).map_err(Error::from)
    }
}

impl<'a, T> DeserializeJson<'a> for T
where
    T: Deserialize<'a>,
{
}

pub trait SerializeJson: Serialize {
    fn to_json_string(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(Error::from)
    }

    fn to_json_string_pretty(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(self).map_err(Error::from)
    }

    fn to_json_vec(&self) -> Result<Vec<u8>, Error> {
        serde_json::to_vec(self).map_err(Error::from)
    }

    fn to_json_vec_pretty(&self) -> Result<Vec<u8>, Error> {
        serde_json::to_vec_pretty(self).map_err(Error::from)
    }
}

impl<T> SerializeJson for T
where
    T: Serialize,
{
}
