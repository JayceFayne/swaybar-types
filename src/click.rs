use serde::{Deserialize, Serialize};
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

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "u32")]
#[serde(into = "u32")]
pub enum Button {
    None = 0,
    Left = 1,
    Middle = 2,
    Right = 3,
    ScrollDown = 4,
    ScrollUp = 5,
}

impl From<Button> for u32 {
    fn from(button: Button) -> Self {
        button as u32
    }
}

impl TryFrom<u32> for Button {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Button::None,
            1 => Button::Left,
            2 => Button::Middle,
            3 => Button::Right,
            4 => Button::ScrollDown,
            5 => Button::ScrollUp,
            _ => return Err(format!("click '{}' not implemented", value)),
        })
    }
}
