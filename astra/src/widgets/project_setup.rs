use std::net::SocketAddrV4;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use egui::{ComboBox, Ui, Widget};

use crate::{folder_picker, ProjectDef, ProjectOutputMode, RomSourceDef};

pub fn rom_source_drop_down(project: &mut ProjectDef) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        ComboBox::from_id_source("project_rom_source")
            .width(300.)
            .selected_text(match &project.rom_source {
                RomSourceDef::Directory { .. } => "Directory",
                RomSourceDef::Network { .. } => "Network (Astra plugin)",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut project.rom_source,
                    RomSourceDef::Directory {
                        romfs_path: Default::default(),
                    },
                    "Directory",
                );
                ui.selectable_value(
                    &mut project.rom_source,
                    RomSourceDef::Network {
                        romfs_ip: Default::default(),
                    },
                    "Network",
                );
            })
            .response
    }
}

pub fn rom_source_config(project: &mut ProjectDef) -> impl Widget + '_ {
    move |ui: &mut Ui| match &mut project.rom_source {
        RomSourceDef::Directory { romfs_path } => {
            ui.label("Extracted ROMFS Data Folder");
            ui.vertical(|ui| {
                let response = ui.add(folder_picker(romfs_path));
                if !romfs_path.is_empty() && !is_valid_data_directory(romfs_path) {
                    ui.colored_label(ui.visuals().error_fg_color, "Not a valid ROMFS folder.");
                }
                response
            })
            .inner
        }
        RomSourceDef::Network { romfs_ip } => {
            ui.label("IP and Port");
            ui.vertical(|ui| {
                let response = ui.text_edit_singleline(romfs_ip);
                if SocketAddrV4::from_str(romfs_ip).is_err() {
                    ui.colored_label(ui.visuals().error_fg_color, "Not a valid IP + Port.");
                }
                response
            })
            .inner
        }
    }
}

fn is_valid_data_directory(path: &str) -> bool {
    let path = Path::new(path);
    path.is_dir() && path.join("StreamingAssets").is_dir()
}

pub fn output_mode_drop_down(project: &mut ProjectDef) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        ComboBox::from_id_source("project_output_mode_combo")
            .width(300.)
            .selected_text(match &project.output_mode {
                ProjectOutputMode::Standard(_) => "LayeredFS Only",
                ProjectOutputMode::Cobalt {
                    data_path: _,
                    patch_path: _,
                    output_msbt: _,
                } => "Cobalt",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut project.output_mode,
                    ProjectOutputMode::Standard(String::new()),
                    "LayeredFS Only",
                );
                ui.selectable_value(
                    &mut project.output_mode,
                    ProjectOutputMode::default(),
                    "Cobalt",
                );
            })
            .response
    }
}

pub fn output_mode_config(project: &mut ProjectDef) -> impl Widget + '_ {
    move |ui: &mut Ui| match &mut project.output_mode {
        ProjectOutputMode::Standard(data_path) => {
            ui.label("Output Path");
            ui.add(folder_picker(data_path))
        }
        ProjectOutputMode::Cobalt {
            data_path,
            patch_path,
            output_msbt,
        } => {
            // This *should* be the only field, but backwards compatibility...
            // Instead, we take a mod path and transform it into data + patch paths.
            let mut mod_path = PathBuf::from(&data_path)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            ui.label("Mod Path");
            let response = ui.add(folder_picker(&mut mod_path));
            if response.changed() {
                *data_path = Path::new(&mod_path)
                    .join("Data")
                    .to_string_lossy()
                    .to_string();
                *patch_path = Path::new(&mod_path)
                    .join("patches")
                    .to_string_lossy()
                    .to_string();
            }
            ui.end_row();

            ui.label("Output MSBT");
            ComboBox::from_id_source("output_msbt_combo")
                .width(300.)
                .selected_text(
                    output_msbt
                        .as_deref()
                        .and_then(|msbt| Path::new(msbt).file_name().map(|p| p.to_string_lossy()))
                        .unwrap_or_default(),
                )
                .show_ui(ui, |ui| {
                    for msbt in get_cobalt_msbt_options(
                        patch_path,
                        &project.active_country_dir_name,
                        &project.active_language_dir_name,
                    ) {
                        if ui
                            .selectable_label(output_msbt.as_deref() == Some(msbt.as_str()), &msbt)
                            .clicked()
                        {
                            *output_msbt = Some(msbt);
                        }
                    }
                })
                .response
        }
    }
}

fn get_cobalt_msbt_options(cobalt_path: &str, region_dir: &str, language_dir: &str) -> Vec<String> {
    if region_dir.is_empty() || language_dir.is_empty() {
        return vec![];
    }
    let path = Path::new(cobalt_path)
        .join("msbt")
        .join("message")
        .join(region_dir)
        .join(language_dir);
    std::fs::read_dir(path)
        .ok()
        .map(|entries| {
            let mut dirs = vec![];
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    dirs.push(entry.file_name().to_string_lossy().to_string());
                }
            }
            dirs
        })
        .unwrap_or_default()
}

pub fn region_dir_config(project: &mut ProjectDef) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        ComboBox::from_id_source("region_dir_combo")
            .width(300.)
            .selected_text(match project.active_country_dir_name.as_str() {
                "ch" => "China",
                "eu" => "Europse",
                "jp" => "Japan",
                "kr" => "Korea",
                "us" => "North America",
                "tw" => "Taiwan",
                _ => "",
            })
            .show_ui(ui, |ui| {
                let mut response = ui.selectable_value(
                    &mut project.active_country_dir_name,
                    "ch".to_string(),
                    "China",
                );
                response |= ui.selectable_value(
                    &mut project.active_country_dir_name,
                    "eu".to_string(),
                    "Europe",
                );
                response |= ui.selectable_value(
                    &mut project.active_country_dir_name,
                    "jp".to_string(),
                    "Japan",
                );
                response |= ui.selectable_value(
                    &mut project.active_country_dir_name,
                    "kr".to_string(),
                    "Korea",
                );
                response |= ui.selectable_value(
                    &mut project.active_country_dir_name,
                    "us".to_string(),
                    "North America",
                );
                response |= ui.selectable_value(
                    &mut project.active_country_dir_name,
                    "tw".to_string(),
                    "Taiwan",
                );
                if response.changed() {
                    project.active_language_dir_name = String::new();
                }
            })
            .response
    }
}

pub fn language_dir_config(project: &mut ProjectDef) -> impl Widget + '_ {
    move |ui: &mut Ui| {
        ComboBox::from_id_source("language_dir_combo")
            .width(300.)
            .selected_text(match project.active_language_dir_name.as_str() {
                "cnch" | "twch" => "Chinese",
                "euen" | "usen" => "English",
                "eufr" | "usfr" => "French",
                "eues" | "uses" => "Spanish",
                "eude" => "German",
                "euit" => "Italian",
                "jpja" => "Japanese",
                "krko" => "Korean",
                _ => "",
            })
            .show_ui(ui, |ui| match project.active_country_dir_name.as_str() {
                "ch" => {
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "cnch".to_string(),
                        "Chinese",
                    );
                }
                "eu" => {
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "euen".to_string(),
                        "English",
                    );
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "eufr".to_string(),
                        "French",
                    );
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "eude".to_string(),
                        "German",
                    );
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "euit".to_string(),
                        "Italian",
                    );
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "eues".to_string(),
                        "Spanish",
                    );
                }
                "jp" => {
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "jpja".to_string(),
                        "Japanese",
                    );
                }
                "kr" => {
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "krko".to_string(),
                        "Korean",
                    );
                }
                "us" => {
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "usen".to_string(),
                        "English",
                    );
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "usfr".to_string(),
                        "French",
                    );
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "uses".to_string(),
                        "Spanish",
                    );
                }
                "tw" => {
                    ui.selectable_value(
                        &mut project.active_language_dir_name,
                        "twch".to_string(),
                        "Chinese",
                    );
                }
                _ => {}
            })
            .response
    }
}
