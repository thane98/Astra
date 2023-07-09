use std::path::Path;

use egui::{Align2, ComboBox, Vec2};

use crate::model::ProjectOutputMode;
use crate::{folder_picker, AppConfig, AppState, ProjectDef};

const MESSAGE_DIR: &str = "StreamingAssets\\aa\\Switch\\fe_assets_message";

#[derive(Debug, Default)]
pub struct CreateProjectState {
    project: ProjectDef,
    target_index: Option<usize>,
}

impl CreateProjectState {
    pub fn new_edit(project: ProjectDef, target_index: usize) -> Self {
        Self {
            project,
            target_index: Some(target_index),
        }
    }
}

pub fn project_creator(
    state: &mut CreateProjectState,
    config: &mut AppConfig,
    next_state: &mut Option<AppState>,
    ctx: &egui::Context,
) {
    egui::CentralPanel::default().show(ctx, |_| {
        egui::Window::new("project_creator")
            .open(&mut true)
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .show(ctx, |ui| {
                egui::Grid::new("project_creator_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut state.project.name);
                        ui.end_row();

                        ui.label("RomFS Path");
                        ui.add(folder_picker(&mut state.project.romfs_path));
                        ui.end_row();

                        ui.label("Output Mode");
                        ComboBox::from_id_source("project_output_mode_combo")
                            .width(300.)
                            .selected_text(match &state.project.output_mode {
                                ProjectOutputMode::Standard(_) => "LayeredFS Only",
                                ProjectOutputMode::Cobalt {
                                    data_path: _,
                                    patch_path: _,
                                    output_msbt: _,
                                } => "Cobalt",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut state.project.output_mode,
                                    ProjectOutputMode::Standard(String::new()),
                                    "LayeredFS Only",
                                );
                                ui.selectable_value(
                                    &mut state.project.output_mode,
                                    ProjectOutputMode::default(),
                                    "Cobalt",
                                );
                            });
                        ui.end_row();

                        match &mut state.project.output_mode {
                            ProjectOutputMode::Standard(data_path) => {
                                ui.label("Output Path");
                                ui.add(folder_picker(data_path));
                                ui.end_row();
                            }
                            ProjectOutputMode::Cobalt {
                                data_path,
                                patch_path,
                                output_msbt,
                            } => {
                                ui.label("Output Path");
                                ui.add(folder_picker(data_path));
                                ui.end_row();

                                ui.label("Mod's Patch Path");
                                ui.add(folder_picker(patch_path));
                                ui.end_row();

                                ui.label("Output MSBT");
                                ComboBox::from_id_source("region_dir_combbo")
                                    .width(300.)
                                    .selected_text(output_msbt.as_deref().unwrap_or_default())
                                    .show_ui(ui, |ui| {
                                        for msbt in get_cobalt_msbt_options(
                                            &patch_path,
                                            &state.project.active_country_dir_name,
                                            &state.project.active_language_dir_name,
                                        ) {
                                            if ui
                                                .selectable_label(
                                                    output_msbt.as_deref() == Some(msbt.as_str()),
                                                    &msbt,
                                                )
                                                .clicked()
                                            {
                                                *output_msbt = Some(msbt);
                                            }
                                        }
                                    });
                                ui.end_row();
                            }
                        }
                    });
                ui.horizontal(|ui| {
                    ComboBox::from_id_source("region_dir_combbo")
                        .selected_text(&state.project.active_country_dir_name)
                        .show_ui(ui, |ui| {
                            for dir in get_region_dirs(&state.project.romfs_path) {
                                if ui
                                    .selectable_label(
                                        state.project.active_country_dir_name == dir,
                                        &dir,
                                    )
                                    .clicked()
                                {
                                    state.project.active_country_dir_name = dir;
                                }
                            }
                        });
                    ComboBox::from_id_source("language_dir_combbo")
                        .selected_text(&state.project.active_language_dir_name)
                        .show_ui(ui, |ui| {
                            for dir in get_language_dirs(
                                &state.project.romfs_path,
                                &state.project.active_country_dir_name,
                            ) {
                                if ui
                                    .selectable_label(
                                        state.project.active_language_dir_name == dir,
                                        &dir,
                                    )
                                    .clicked()
                                {
                                    state.project.active_language_dir_name = dir;
                                }
                            }
                        });
                });
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(state.project.is_valid(), |ui| {
                        if ui.button("Save").clicked() {
                            if let Some(index) = state.target_index {
                                config.projects[index] = std::mem::take(&mut state.project);
                            } else {
                                config.projects.push(std::mem::take(&mut state.project));
                            }
                            config.active_project = Some(config.projects.len() - 1);
                            *next_state = Some(AppState::SelectProject);
                        }
                    });
                    if ui.button("Cancel").clicked() {
                        *next_state = Some(AppState::SelectProject);
                    }
                });
            });
    });
}

fn get_region_dirs(romfs_path: &str) -> Vec<String> {
    let path = Path::new(romfs_path).join(MESSAGE_DIR);
    std::fs::read_dir(path)
        .ok()
        .map(|entries| {
            let mut dirs = vec![];
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        dirs.push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
            dirs
        })
        .unwrap_or_default()
}

fn get_language_dirs(romfs_path: &str, region_dir: &str) -> Vec<String> {
    let path = Path::new(romfs_path).join(MESSAGE_DIR).join(region_dir);
    std::fs::read_dir(path)
        .ok()
        .map(|entries| {
            let mut dirs = vec![];
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        dirs.push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
            dirs
        })
        .unwrap_or_default()
}

fn get_cobalt_msbt_options(cobalt_path: &str, region_dir: &str, language_dir: &str) -> Vec<String> {
    let path = Path::new(cobalt_path)
        .join("msbt")
        .join("message")
        .join(region_dir)
        .join(language_dir);
    std::fs::read_dir(path)
        .ok()
        .map(|entries| {
            let mut dirs = vec![];
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_file() {
                        dirs.push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
            dirs
        })
        .unwrap_or_default()
}
