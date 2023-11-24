use egui::{Color32, Frame, Image, Label, Stroke, TextureHandle, Ui, Widget};
use rfd::FileDialog;

pub fn editor_tab_strip(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.separator();
    ui.horizontal_top(|ui| {
        add_contents(ui);
    });
    ui.allocate_space([0., ui.spacing().item_spacing.y].into());
}

pub fn blank_slate(ui: &mut Ui) {
    ui.centered_and_justified(|ui| {
        ui.heading("Select an entry.");
    });
}

pub fn optional_checkbox(value: &mut Option<bool>) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        let mut temp = value.unwrap_or_default();
        let response = ui.checkbox(&mut temp, "");
        if response.changed() {
            *value = Some(temp);
        }
        response
    }
}

pub fn optional_image(
    content: Option<(TextureHandle, f32)>,
    placeholder_size: [f32; 2],
) -> impl Widget {
    move |ui: &mut Ui| {
        if let Some((image, scale)) = content {
            ui.add_sized(
                placeholder_size,
                Image::from_texture(&image).max_size(image.size_vec2() * scale),
            )
        } else {
            ui.add_sized(placeholder_size, Label::new(""))
        }
    }
}

pub fn raised_heading(text: &str) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        let fill = ui.visuals().code_bg_color;
        Frame::group(ui.style())
            .fill(fill)
            .stroke(Stroke::new(0., Color32::default()))
            .show(ui, |ui| ui.heading(text))
            .response
    }
}

pub fn folder_picker(value: &mut String) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(value);
            if ui.button("Open").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    *value = path.to_string_lossy().to_string();
                }
            }
        })
        .response
    }
}
