use std::sync::Arc;

use astra_core::Astra;
use egui::{ScrollArea, Ui};
use parking_lot::RwLock;

pub struct ScriptManager {
    astra: Arc<RwLock<Astra>>,
}

impl ScriptManager {
    pub fn new(astra: Arc<RwLock<Astra>>) -> Self {
        Self { astra }
    }

    pub fn ui(&self, ctx: &egui::Context) {
        let mut key_to_forget = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            let astra = self.astra.read();
            let scripts: Vec<&String> = astra.list_scripts().collect();
            if scripts.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.heading(
                        "No scripts have been opened. You can do this through the Chapter Editor.",
                    );
                });
            }
            ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("script_manager_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        key_to_forget = self.script_manager_rows(ui, &scripts);
                    });
            });
        });
        if let Some(key) = key_to_forget {
            let mut astra = self.astra.write();
            astra.forget_script(&key);
        }
    }

    fn script_manager_rows(&self, ui: &mut Ui, scripts: &[&String]) -> Option<String> {
        let mut key_to_forget = None;
        for script in scripts {
            ui.label(*script);
            if ui.button("Forget").clicked() {
                key_to_forget = Some(script.to_string());
            }
            ui.end_row();
        }
        key_to_forget
    }
}
