#![allow(clippy::ref_option)]

#[cfg(feature = "regex")]
pub(crate) mod regex_serde {
  use crate::*;

  use regex::Regex;
  use serde::{Deserialize, Deserializer, Serializer};

  pub fn serialize<S>(regex: &Option<Regex>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match regex {
      Some(re) => serializer.serialize_str(re.as_str()),
      None => serializer.serialize_none(),
    }
  }

  #[track_caller]
  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Regex>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
      Some(pattern) => Ok(Some(Regex::new(&pattern).unwrap())),
      None => Ok(None),
    }
  }
}

#[cfg(feature = "regex")]
pub(crate) mod bytes_regex_serde {
  use crate::*;

  use regex::bytes::Regex;
  use serde::{Deserialize, Deserializer, Serializer};

  pub fn serialize<S>(regex: &Option<Regex>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match regex {
      Some(re) => serializer.serialize_str(re.as_str()),
      None => serializer.serialize_none(),
    }
  }

  #[track_caller]
  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Regex>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
      Some(pattern) => Ok(Some(Regex::new(&pattern).unwrap())),
      None => Ok(None),
    }
  }
}
