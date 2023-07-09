use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use egui::{Grid, TextEdit, Ui};
use egui_modal::Modal;

use crate::{AppConfig, Theme};

pub fn config_editor_modal(ctx: &egui::Context, config: &mut AppConfig) -> Modal {
    let modal = Modal::new(ctx, "config_editor_modal");
    modal.show(|ui| {
        modal.title(ui, "Preferences");
        config_editor(ui, config);
        modal.buttons(ui, |ui| {
            modal.button(ui, "Close");
        });
    });
    modal
}

pub fn config_editor(ui: &mut Ui, config: &mut AppConfig) {
    Grid::new("config_editor").num_columns(2).show(ui, |ui| {
        ui.vertical(|ui| {
            ui.label("Theme");
        });
        ui.vertical(|ui| {
            if ui
                .radio_value(&mut config.theme, Theme::Latte, "Latte")
                .clicked()
            {
                catppuccin_egui::set_theme(ui.ctx(), LATTE);
            }
            if ui
                .radio_value(&mut config.theme, Theme::Frappe, "Frappé")
                .clicked()
            {
                catppuccin_egui::set_theme(ui.ctx(), FRAPPE);
            }
            if ui
                .radio_value(&mut config.theme, Theme::Macchiato, "Macchiato")
                .clicked()
            {
                catppuccin_egui::set_theme(ui.ctx(), MACCHIATO);
            }
            if ui
                .radio_value(&mut config.theme, Theme::Mocha, "Mocha")
                .clicked()
            {
                catppuccin_egui::set_theme(ui.ctx(), MOCHA);
            }
        });
        ui.end_row();

        ui.label("Script Editor Program");
        ui.add_sized(
            [300., 0.],
            TextEdit::singleline(&mut config.script_editor_process),
        );
        ui.end_row();

        ui.label("Script Editor Command Args");
        ui.add_sized(
            [300., 0.],
            TextEdit::singleline(&mut config.script_editor_command_args),
        );
        ui.end_row();
    });
}
