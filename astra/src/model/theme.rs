use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Theme {
    Egui,
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Mocha
    }
}
