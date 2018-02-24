use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_yaml;

use Error;

pub trait DeserializeYaml: DeserializeOwned {
    fn from_yaml_str(s: &str) -> Result<Self, Error> {
        serde_yaml::from_str(s).map_err(Error::from)
    }

    fn from_yaml_slice(v: &[u8]) -> Result<Self, Error> {
        serde_yaml::from_slice(v).map_err(Error::from)
    }
}

impl<T> DeserializeYaml for T
where
    T: DeserializeOwned,
{
}

pub trait SerializeYaml: Serialize {
    fn to_yaml_string(&self) -> Result<String, Error> {
        serde_yaml::to_string(self).map_err(Error::from)
    }

    fn to_yaml_vec(&self) -> Result<Vec<u8>, Error> {
        serde_yaml::to_vec(self).map_err(Error::from)
    }
}

impl<T> SerializeYaml for T
where
    T: Serialize,
{
}
