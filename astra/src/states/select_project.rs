use egui::{Button, Ui};
use egui_modal::Modal;

use crate::model::ProjectOutputMode;
use crate::{config_editor, AppConfig, AppState, CreateProjectState, LoadProjectState};

pub fn project_selector(
    config: &mut AppConfig,
    next_state: &mut Option<AppState>,
    ctx: &egui::Context,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let settings_modal = Modal::new(ui.ctx(), "select_project_settings_modal");
        settings_modal.show(|ui| {
            settings_modal.title(ui, "Preferences");
            config_editor(ui, config);
            settings_modal.button(ui, "Close");
        });

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Select Project");
                ui.separator();
                if ui.button("New").clicked() {
                    *next_state = Some(AppState::CreateProject(Box::default()));
                }
                if ui.button("Settings").clicked() {
                    settings_modal.open();
                }
            });
            ui.separator();
            ui.centered_and_justified(|ui| project_table(config, next_state, ui))
        });
    });
}

fn project_table(config: &mut AppConfig, next_state: &mut Option<AppState>, ui: &mut Ui) {
    egui::Grid::new("project_creator_grid")
        .spacing([10., 10.])
        .num_columns(3)
        .show(ui, |ui| {
            ui.heading("Name");
            ui.heading("Output Mode");
            ui.heading("Output Path");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.end_row();
            let mut removed = None;
            for (i, project) in config.projects.iter().enumerate() {
                ui.label(&project.name);
                match &project.output_mode {
                    ProjectOutputMode::Standard(data_path) => {
                        ui.label("LayeredFS Only");
                        ui.label(data_path);
                    }
                    ProjectOutputMode::Cobalt {
                        data_path,
                        patch_path: _,
                        output_msbt: _,
                    } => {
                        ui.label("Cobalt");
                        ui.label(data_path);
                    }
                }

                if ui
                    .add(Button::new("Open").min_size([40., 0.].into()))
                    .clicked()
                {
                    config.active_project = Some(i);
                    *next_state = Some(AppState::LoadProject(LoadProjectState::default()));
                }
                if ui
                    .add(Button::new("Edit").min_size([40., 0.].into()))
                    .clicked()
                {
                    *next_state = Some(AppState::CreateProject(Box::new(
                        CreateProjectState::new_edit(project.clone(), i),
                    )));
                }
                if ui
                    .add(Button::new("Delete").min_size([40., 0.].into()))
                    .clicked()
                {
                    removed = Some(i);
                }
                ui.end_row();
            }
            if let Some(index) = removed {
                config.projects.remove(index);
            }
        });
}
