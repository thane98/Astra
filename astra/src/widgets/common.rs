use egui::{Color32, Frame, Image, Label, Response, Stroke, TextureHandle, Ui, Widget};
use rfd::FileDialog;

use crate::{DefaultWidget, EditorState};

pub fn editor_tab_strip(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.separator();
    ui.horizontal_wrapped(|ui| {
        add_contents(ui);
    });
    ui.allocate_space([0., ui.spacing().item_spacing.y].into());
}

pub fn blank_slate(ui: &mut Ui) {
    ui.centered_and_justified(|ui| {
        ui.heading("Select an entry.");
    });
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
            let mut response = ui.text_edit_singleline(value);
            if ui.button("Open").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    *value = path.to_string_lossy().to_string();
                    response.mark_changed();
                }
            }
            response
        })
        .inner
    }
}

pub fn iron_field_i8(ui: &mut Ui, state: &EditorState, value: &mut impl DefaultWidget) -> Response {
    system_icon_field(ui, state, value, "Iron")
}

pub fn steel_field(ui: &mut Ui, state: &EditorState, value: &mut impl DefaultWidget) -> Response {
    system_icon_field(ui, state, value, "Steel")
}

pub fn silver_field(ui: &mut Ui, state: &EditorState, value: &mut impl DefaultWidget) -> Response {
    system_icon_field(ui, state, value, "Silver")
}

pub fn bond_fragment_field(
    ui: &mut Ui,
    state: &EditorState,
    value: &mut impl DefaultWidget,
) -> Response {
    system_icon_field(ui, state, value, "Bonds")
}

pub fn gold_field(ui: &mut Ui, state: &EditorState, value: &mut impl DefaultWidget) -> Response {
    system_icon_field(ui, state, value, "Coin")
}

pub fn system_icon_field(
    ui: &mut Ui,
    state: &EditorState,
    value: &mut impl DefaultWidget,
    icon: &str,
) -> Response {
    ui.horizontal(|ui| {
        let response = value.default_widget(ui);
        if let Some(texture) = state.texture_cache.borrow_mut().get_system(icon) {
            ui.add(Image::from_texture(&texture).max_size(texture.size_vec2() * 0.5));
        }
        response
    })
    .inner
}

macro_rules! optional_numeric_get_set {
    ($get_set_name:ident, $widget_name:ident, $target:ty) => {
        pub fn $get_set_name(value: &mut Option<$target>) -> impl FnMut(Option<f64>) -> f64 + '_ {
            move |new_value: Option<f64>| {
                if let Some(new_value) = new_value {
                    *value = Some(new_value as $target);
                }
                value.unwrap_or_default() as f64
            }
        }

        pub fn $widget_name(value: &mut Option<$target>) -> egui::DragValue<'_> {
            egui::DragValue::from_get_set($get_set_name(value))
        }
    };
}

optional_numeric_get_set!(u8_numeric_get_set, optional_u8_drag, u8);
optional_numeric_get_set!(u16_numeric_get_set, optional_u16_drag, u16);
optional_numeric_get_set!(u32_numeric_get_set, optional_u32_drag, u32);
optional_numeric_get_set!(i8_numeric_get_set, optional_i8_drag, i8);
optional_numeric_get_set!(i16_numeric_get_set, optional_i16_drag, i16);
optional_numeric_get_set!(i32_numeric_get_set, optional_i32_drag, i32);
optional_numeric_get_set!(f32_numeric_get_set, optional_f32_drag, f32);
