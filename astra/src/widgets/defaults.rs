use egui::{DragValue, Response, Ui};

use crate::editable_list;

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

impl DefaultWidget for bool {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.checkbox(self, "")
    }
}

impl DefaultWidget for u8 {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self))
    }
}

impl DefaultWidget for u16 {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self))
    }
}

impl DefaultWidget for u32 {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self))
    }
}

impl DefaultWidget for i8 {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self))
    }
}

impl DefaultWidget for i16 {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self))
    }
}

impl DefaultWidget for i32 {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self))
    }
}

impl DefaultWidget for f32 {
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self))
    }
}

impl<T> DefaultWidget for Vec<T>
where
    T: DefaultWidget + Default,
{
    fn default_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(editable_list(self, |_, d, ui| {
            DefaultWidget::default_widget(d, ui)
        }))
    }
}
