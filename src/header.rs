use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cont_signal: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_events: Option<bool>,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
pub enum Version {
    One = 1,
}

impl From<Version> for u8 {
    fn from(version: Version) -> Self {
        version as u8
    }
}

impl TryFrom<u8> for Version {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Version::One,
            _ => return Err(format!("invalid version number '{}'", value)),
        })
    }
}
