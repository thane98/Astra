use std::path::Path;

use anyhow::{bail, Result};
use egui::{Align2, TextEdit, Vec2};
use egui_modal::Modal;

use crate::{
    language_dir_config, output_mode_config, output_mode_drop_down, region_dir_config,
    rom_source_config, rom_source_drop_down, AppConfig, AppState, ProjectDef, ProjectOutputMode,
    RomSourceDef,
};

const COBALT_PLUGIN: &[u8] = include_bytes!("../../assets/libastra_cobalt_plugin.nro");
const PLUGIN_FILE_NAME: &str = "libastra_cobalt_plugin.nro";

#[derive(Debug)]
pub struct CreateProjectState {
    project: ProjectDef,
    target_index: Option<usize>,
    new_cobalt_project: bool,
    install_plugin: bool,
    is_editing: bool,
    error: Option<String>,
}

impl Default for CreateProjectState {
    fn default() -> Self {
        Self {
            project: Default::default(),
            target_index: Default::default(),
            new_cobalt_project: true,
            install_plugin: true,
            is_editing: Default::default(),
            error: Default::default(),
        }
    }
}

impl CreateProjectState {
    pub fn new_edit(project: ProjectDef, target_index: usize) -> Self {
        Self {
            project,
            target_index: Some(target_index),
            new_cobalt_project: false,
            install_plugin: false,
            is_editing: true,
            error: Default::default(),
        }
    }

    fn should_show_output_mode_config(&self) -> bool {
        !(self.new_cobalt_project
            && matches!(self.project.output_mode, ProjectOutputMode::Cobalt { .. }))
    }

    fn should_show_new_cobalt_checkbox(&self, config: &AppConfig) -> bool {
        matches!(self.project.output_mode, ProjectOutputMode::Cobalt { .. })
            && !config.cobalt_path.is_empty()
            && !self.is_editing
    }

    fn should_show_install_plugin_checkbox(&self, config: &AppConfig) -> bool {
        matches!(self.project.output_mode, ProjectOutputMode::Cobalt { .. })
            && matches!(self.project.rom_source, RomSourceDef::Network { .. })
            && !config.cobalt_path.is_empty()
            && !self.is_editing
    }

    fn should_create_new_cobalt_project(&self, config: &AppConfig) -> bool {
        self.should_show_new_cobalt_checkbox(config) && self.new_cobalt_project
    }

    fn should_install_plugin(&self, config: &AppConfig) -> bool {
        self.should_show_install_plugin_checkbox(config) && self.install_plugin
    }
}

pub fn project_creator(
    state: &mut CreateProjectState,
    config: &mut AppConfig,
    next_state: &mut Option<AppState>,
    ctx: &egui::Context,
) {
    egui::CentralPanel::default().show(ctx, |_| {
        let modal = Modal::new(ctx, "create_project_error_modal");
        modal.show(|ui| {
            modal.title(ui, "Failed to Create Project");
            let mut buffer = state.error.clone().unwrap_or_default();
            ui.add_enabled_ui(false, |ui| {
                ui.add(
                    TextEdit::multiline(&mut buffer)
                        .desired_width(f32::INFINITY)
                        .desired_rows(8),
                );
            });
            modal.buttons(ui, |ui| {
                if modal.button(ui, "Close").clicked() {
                    state.error = None;
                }
                if modal.button(ui, "Copy Error").clicked() {
                    ui.output_mut(|out| {
                        out.copied_text = buffer;
                    });
                }
            });
        });

        egui::Window::new("project_creator")
            .open(&mut true)
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .show(ctx, |ui| {
                ui.heading("Create Project");
                egui::Grid::new("project_creator_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut state.project.name);
                        ui.end_row();

                        ui.label("ROM Source");
                        ui.add(rom_source_drop_down(&mut state.project));
                        ui.end_row();

                        ui.add(rom_source_config(&mut state.project));
                        ui.end_row();

                        ui.label("Output Mode");
                        ui.add(output_mode_drop_down(&mut state.project));
                        ui.end_row();

                        if state.should_show_new_cobalt_checkbox(config) {
                            ui.label("New Cobalt Project");
                            ui.checkbox(&mut state.new_cobalt_project, "");
                            ui.end_row();
                        }

                        if state.should_show_install_plugin_checkbox(config) {
                            ui.label("Install Astra Plugin");
                            ui.checkbox(&mut state.install_plugin, "");
                            ui.end_row();
                        }

                        if state.should_show_output_mode_config() {
                            ui.add(output_mode_config(&mut state.project));
                            ui.end_row();
                        }

                        ui.label("Region");
                        ui.add(region_dir_config(&mut state.project));
                        ui.end_row();

                        ui.label("Language");
                        ui.add(language_dir_config(&mut state.project));
                        ui.end_row();
                    });

                if let Some(error) = state.error.as_deref() {
                    ui.colored_label(ui.visuals().error_fg_color, error);
                }
                ui.horizontal(|ui| {
                    let can_create = if state.new_cobalt_project {
                        state.project.is_valid_for_new_cobalt_project()
                    } else {
                        state.project.is_valid()
                    };
                    ui.add_enabled_ui(can_create, |ui| {
                        if ui.button("Save").clicked() {
                            if state.should_install_plugin(config) {
                                if let Err(err) = install_plugin(config) {
                                    state.error = Some(format!("{:?}", err));
                                    return;
                                }
                            }
                            if state.should_create_new_cobalt_project(config) {
                                if let Err(err) = create_cobalt_project(&mut state.project, config)
                                {
                                    state.error = Some(format!("{:?}", err));
                                    return;
                                }
                            }
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

fn create_cobalt_project(project: &mut ProjectDef, config: &AppConfig) -> Result<()> {
    if let ProjectOutputMode::Cobalt {
        data_path,
        patch_path,
        output_msbt,
    } = &mut project.output_mode
    {
        // Sanity check: did the user set Cobalt's path directly?
        let mods_path = Path::new(&config.cobalt_path).join("mods");
        if !mods_path.is_dir() {
            bail!("Invalid Cobalt path '{}'", config.cobalt_path);
        }

        // Create the mod directories
        let mod_path = mods_path.join(&project.name);
        if !mod_path.is_dir() {
            std::fs::create_dir(&mod_path)?;
        }
        let new_data_path = mod_path.join("Data");
        if !new_data_path.is_dir() {
            std::fs::create_dir(&new_data_path)?;
        }
        let patches_patch = mod_path.join("patches");
        if !patches_patch.is_dir() {
            std::fs::create_dir(&patches_patch)?;
        }

        // Create the output MSBT file.
        let msbt_folder_path = patches_patch
            .join("msbt")
            .join("message")
            .join(&project.active_country_dir_name)
            .join(&project.active_language_dir_name);
        if !msbt_folder_path.is_dir() {
            std::fs::create_dir_all(&msbt_folder_path)?;
        }
        let msbt_path = msbt_folder_path.join("editedtext.txt");
        if !msbt_path.is_file() {
            std::fs::write(&msbt_path, "")?;
        }

        *data_path = new_data_path.to_string_lossy().to_string();
        *patch_path = patches_patch.to_string_lossy().to_string();
        *output_msbt = Some(msbt_path.to_string_lossy().to_string());
    }
    Ok(())
}

fn install_plugin(config: &AppConfig) -> Result<()> {
    let plugin_dir = Path::new(&config.cobalt_path)
        .join("mods")
        .join("astra-cobalt-plugin");
    if !plugin_dir.is_dir() {
        std::fs::create_dir(&plugin_dir)?;
    }

    let plugin_file_path = plugin_dir.join(PLUGIN_FILE_NAME);
    if !plugin_file_path.is_file() {
        std::fs::write(plugin_dir.join(PLUGIN_FILE_NAME), COBALT_PLUGIN)?;
    }
    Ok(())
}
