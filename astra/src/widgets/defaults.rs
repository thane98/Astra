use egui::{Response, Ui};

use crate::{
    f32_drag, i16_drag, i32_drag, i8_drag, optional_checkbox, u16_drag, u32_drag, u8_drag,
};

/// Trait for types which support a "default" widget.
/// Used to conveniently display fields without thinking about which widget to use.
pub trait DefaultWidget {
    /// Draw the default widget for a type.
    fn default_widget(&mut self, ui: &mut Ui) -> Response;
}

impl DefaultWidget for String {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.text_edit_singleline(self)
    }
}

impl DefaultWidget for Option<bool> {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(optional_checkbox(self))
    }
}

impl DefaultWidget for Option<u8> {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(u8_drag(self))
    }
}

impl DefaultWidget for Option<u16> {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(u16_drag(self))
    }
}

impl DefaultWidget for Option<u32> {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(u32_drag(self))
    }
}

impl DefaultWidget for Option<i8> {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(i8_drag(self))
    }
}

impl DefaultWidget for Option<i16> {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(i16_drag(self))
    }
}

impl DefaultWidget for Option<i32> {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(i32_drag(self))
    }
}

impl DefaultWidget for Option<f32> {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(f32_drag(self))
    }
}
