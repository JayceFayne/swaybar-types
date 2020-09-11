use serde::{Deserialize, Serialize};

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
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "u32")]
#[serde(into = "u32")]
pub enum Button {
    None,
    Left,
    Middle,
    Right,
    WheelUp,
    WheelDown,
    WheelLeft,
    WheelRight,
    Other(u32),
}

impl From<Button> for u32 {
    fn from(button: Button) -> Self {
        match button {
            Button::None => 0,
            Button::Left => 1,
            Button::Middle => 2,
            Button::Right => 3,
            Button::WheelUp => 4,
            Button::WheelDown => 5,
            Button::WheelLeft => 6,
            Button::WheelRight => 7,
            Button::Other(u) => u,
        }
    }
}

impl From<u32> for Button {
    fn from(value: u32) -> Self {
        match value {
            0 => Button::None,
            1 => Button::Left,
            2 => Button::Middle,
            3 => Button::Right,
            4 => Button::WheelUp,
            5 => Button::WheelDown,
            6 => Button::WheelLeft,
            7 => Button::WheelRight,
            v => Button::Other(v),
        }
    }
}
