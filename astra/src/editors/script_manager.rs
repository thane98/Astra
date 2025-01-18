use std::collections::{BTreeSet, HashSet};
use std::sync::Arc;

use astra_core::Astra;
use egui::{ScrollArea, TextEdit, Ui};
use egui_modal::{Icon, Modal};
use egui_notify::Toasts;
use parking_lot::RwLock;

use crate::AppConfig;

pub struct ScriptManager {
    astra: Arc<RwLock<Astra>>,
    scripts: BTreeSet<String>,
    search: String,
    error: Option<String>,
}

impl ScriptManager {
    pub fn new(astra: Arc<RwLock<Astra>>) -> Self {
        let scripts = astra.read().list_all_scripts();
        Self {
            scripts,
            search: Default::default(),
            error: None,
            astra,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, config: &AppConfig, toasts: &mut Toasts) {
        let error_modal = Modal::new(ctx, "script_manager_error_modal");
        if let Some(error) = self.error.clone() {
            error_modal.show(|ui| {
                error_modal.title(ui, "Failed to open script");
                error_modal.body_and_icon(ui, &error, Icon::Error);
                error_modal.buttons(ui, |ui| {
                    if error_modal.button(ui, "Close").clicked() {
                        self.error = None;
                    }
                    if error_modal.button(ui, "Copy Error").clicked() {
                        ui.output_mut(|out| {
                            out.copied_text = error.to_string();
                        });
                    }
                });
            });
            error_modal.open();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if !config.has_configured_script_editor() {
                ui.centered_and_justified(|ui| {
                    ui.heading(
                        "Please configure a script editor under File -> Preferences",
                    );
                });
            } else {
                self.script_table(config, toasts, ui);
            }
        });
    }

    fn script_table(&mut self, config: &AppConfig, toasts: &mut Toasts, ui: &mut Ui) {
        let mut key_to_forget: Option<String> = None;
        let mut script_to_open = None;

        ui.add(TextEdit::singleline(&mut self.search).desired_width(f32::INFINITY));
        let handle = self.astra.clone();
        let mut astra = handle.write();
        let open_scripts: HashSet<String> = astra.list_open_scripts();
        ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Grid::new("script_manager_grid")
                    .num_columns(3)
                    .show(ui, |ui| {
                        let search = self.search.to_lowercase();
                        for script in &self.scripts {
                            if search.is_empty() || script.contains(&search) {
                                ui.label(script);
                                ui.add_enabled_ui(config.has_configured_script_editor(), |ui| {
                                    if ui.button("Open").clicked() {
                                        script_to_open = Some(script.to_string());
                                    }
                                });
                                ui.add_enabled_ui(open_scripts.contains(script), |ui| {
                                    if ui.button("Forget").clicked() {
                                        key_to_forget = Some(script.to_string());
                                    }
                                });
                                ui.end_row();
                            }
                        }
                    });
            });
        if let Some(key) = key_to_forget {
            astra.forget_script(&key);
        }
        if let Some(script) = script_to_open {
            match astra.open_script(
                &script,
                &config.script_editor_process,
                &config.script_editor_command_args,
            ) {
                Ok(_) => {
                    toasts.success(format!(
                        "Successfully started script editor process for '{}'",
                        script
                    ));
                }
                Err(err) => self.error = Some(format!("{:?}", err)),
            }
        }
    }
}
