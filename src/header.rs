use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct Header {
    pub version: u8, //currently this must be 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cont_signal: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_events: Option<bool>,
}
