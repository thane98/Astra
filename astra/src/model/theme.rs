use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Theme {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl From<Theme> for catppuccin_egui::Theme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Latte => LATTE,
            Theme::Frappe => FRAPPE,
            Theme::Macchiato => MACCHIATO,
            Theme::Mocha => MOCHA,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::Mocha
    }
}
