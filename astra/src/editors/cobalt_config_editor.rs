use std::sync::Arc;

use astra_core::{Astra, ModConfig};
use egui::CentralPanel;
use egui_notify::Toasts;
use parking_lot::RwLock;
use tracing::error;

use crate::{EditorState, PropertyGrid};

pub struct CobaltConfigEditor {
    astra: Arc<RwLock<Astra>>,
    placeholder: ModConfig,
    is_cobalt_project: bool,
}

impl CobaltConfigEditor {
    pub fn new(state: &EditorState) -> Self {
        let is_cobalt_project = state.astra.read().project().cobalt_dir.is_some();
        Self {
            astra: state.astra.clone(),
            placeholder: ModConfig::default(),
            is_cobalt_project,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, toasts: &mut Toasts) {
        CentralPanel::default().show(ctx, |ui| {
            let mut astra = self.astra.write();
            let can_create = self.is_cobalt_project && astra.get_cobalt_config().is_none();
            ui.add_enabled_ui(can_create, |ui| {
                if ui.button("Create Config").clicked() {
                    if let Err(err) = astra.create_cobalt_config() {
                        error!("Failed to create Cobalt config: '{:?}'", err);
                        toasts.error("Failed to create Cobalt config, see log for details");
                    }
                }
            });

            let config = astra.get_cobalt_config();
            ui.add_enabled_ui(config.is_some(), |ui| {
                PropertyGrid::new("cobalt_config", config.unwrap_or(&mut self.placeholder))
                    .new_section("")
                    .default_field("ID", |config| &mut config.id)
                    .default_field("Name", |config| &mut config.name)
                    .default_field("Description", |config| &mut config.description)
                    .default_field("Author", |config| &mut config.author)
                    .default_field("Dependencies", |config| &mut config.dependencies)
                    .default_field("Repository", |config| &mut config.repository)
                    .show(ui);
            });
        });
    }
}
