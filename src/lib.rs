use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;

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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Block {
    pub full_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_block_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<Markup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Click {
    pub name: String,
    pub instance: Option<String>,
    pub x: u32,
    pub y: u32,
    pub button: Button,
    pub event: u32,
    pub relative_x: u32,
    pub relative_y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Align {
    Center,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Markup {
    Pango,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "u32")]
#[serde(into = "u32")]
pub enum Button {
    Left,
    Right,
    Middle,
    ScrollDown,
    ScrollUp,
}

impl From<Button> for u32 {
    fn from(value: Button) -> Self {
        match value {
            Button::Left => 1,
            Button::Middle => 2,
            Button::Right => 3,
            Button::ScrollDown => 4,
            Button::ScrollUp => 5,
        }
    }
}

impl TryFrom<u32> for Button {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Button::Left,
            2 => Button::Middle,
            3 => Button::Right,
            4 => Button::ScrollDown,
            5 => Button::ScrollUp,
            _ => return Err(format!("click '{}' not implemented", value)),
        })
    }
}
