use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
