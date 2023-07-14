use egui::{TextEdit, Ui, Widget};

pub fn id_field(value: &mut String) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        ui.horizontal(|ui| {
            let response = ui.add_enabled(false, TextEdit::singleline(value));
            if ui.button("🗐 Copy").clicked() {
                ui.output_mut(|out| {
                    out.copied_text = value.clone();
                });
            }
            response
        })
        .inner
    }
}
