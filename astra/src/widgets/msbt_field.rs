use egui::{TextEdit, Ui, Widget};

pub fn msbt_field_singleline(value: Option<&mut String>) -> impl Widget + '_ {
    move |ui: &mut Ui| match value {
        Some(value) => {
            ui.vertical(|ui| {
                let response = ui.text_edit_singleline(value);
                if let Err(err) = astra_formats::parse_astra_script_entry(value) {
                    ui.colored_label(ui.style().visuals.error_fg_color, format!("{}", err));
                }
                response
            })
            .inner
        }
        None => ui.add_enabled(false, TextEdit::singleline(&mut String::new())),
    }
}

#[macro_export]
macro_rules! msbt_key_value_singleline {
    ($ui:ident, $state:ident, $default_archive:expr, $target:expr) => {
        $ui.vertical(|ui| {
            let response = ui.text_edit_singleline(&mut $target);
            $state
                .message_db
                .with_message_mut(&$target, $default_archive, |value| {
                    ui.add($crate::msbt_field_singleline(value)).changed()
                });
            response
        })
        .inner
    };
}

pub fn msbt_field_multiline(value: Option<&mut String>) -> impl Widget + '_ {
    move |ui: &mut Ui| match value {
        Some(value) => {
            ui.vertical(|ui| {
                let response = ui.add(TextEdit::multiline(value));
                if let Err(err) = astra_formats::parse_astra_script_entry(value) {
                    ui.colored_label(ui.style().visuals.error_fg_color, format!("{}", err));
                }
                response
            })
            .inner
        }
        None => ui.add_enabled(false, TextEdit::multiline(&mut String::new())),
    }
}

#[macro_export]
macro_rules! msbt_key_value_multiline {
    ($ui:ident, $state:ident, $default_archive:expr, $target:expr) => {
        $ui.vertical(|ui| {
            let response = ui.text_edit_singleline(&mut $target);
            $state
                .message_db
                .with_message_mut(&$target, $default_archive, |value| {
                    ui.add($crate::msbt_field_multiline(value)).changed()
                });
            response
        })
        .inner
    };
}
