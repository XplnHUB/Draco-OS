pub mod parser;

use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Intent {
    OpenApp(String),
    CloseActiveApp,
    GetSystemInfo(String),
    Shutdown,
    Reboot,
    Sleep,
    SwitchApp(String),
    VolumeUp,
    VolumeDown,
    ToggleMute,
    LockScreen,
    Unknown(String),
}

impl fmt::Display for Intent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Intent::OpenApp(a) => write!(f, "OpenApp({})", a),
            Intent::CloseActiveApp => write!(f, "CloseActiveApp"),
            Intent::GetSystemInfo(i) => write!(f, "GetSystemInfo({})", i),
            Intent::Shutdown => write!(f, "Shutdown"),
            Intent::Reboot => write!(f, "Reboot"),
            Intent::Sleep => write!(f, "Sleep"),
            Intent::SwitchApp(a) => write!(f, "SwitchApp({})", a),
            Intent::VolumeUp => write!(f, "VolumeUp"),
            Intent::VolumeDown => write!(f, "VolumeDown"),
            Intent::ToggleMute => write!(f, "ToggleMute"),
            Intent::LockScreen => write!(f, "LockScreen"),
            Intent::Unknown(r) => write!(f, "Unknown({})", r),
        }
    }
}

pub use parser::IntentParser;
