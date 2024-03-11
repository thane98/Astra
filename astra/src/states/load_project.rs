use std::sync::mpsc::Receiver;
use std::sync::Arc;

use astra_core::error::Result;
use astra_core::{Astra, AstraProject};
use egui::TextEdit;
use egui_modal::Modal;
use parking_lot::RwLock;

use crate::texture_cache::TextureCache;
use crate::{AppConfig, AppState, MainState, MessageDb};

struct LoadedData {
    astra: Arc<RwLock<Astra>>,
    message_db: MessageDb,
    texture_cache: TextureCache,
}

#[derive(Default)]
pub struct LoadProjectState {
    receiver: Option<Receiver<Result<LoadedData>>>,
    error: Option<String>,
}

pub fn project_loader(
    state: &mut LoadProjectState,
    config: &AppConfig,
    next_state: &mut Option<AppState>,
    ctx: &egui::Context,
) {
    if let (Some(receiver), None) = (&mut state.receiver, &state.error) {
        if let Ok(load_result) = receiver.try_recv() {
            match load_result {
                Ok(data) => {
                    *next_state = Some(AppState::Main(Box::new(MainState::new(
                        data.astra,
                        data.message_db,
                        data.texture_cache,
                    ))));
                }
                Err(err) => {
                    state.error = Some(format!("{:?}", err));
                }
            }
        }
    } else if state.error.is_none() {
        let project = config.get_active_project().unwrap(); // TODO
        let project: AstraProject = project.clone().into();
        let (sender, receiver) = std::sync::mpsc::channel();
        let ctx = ctx.clone();
        state.receiver = Some(receiver);
        std::thread::spawn(move || {
            let load_result = Astra::load(project).map(|mut astra| {
                let texture_cache = TextureCache::new(ctx, &mut astra);
                let astra = Arc::new(RwLock::new(astra));
                let message_db = MessageDb::new(astra.clone());
                LoadedData {
                    astra,
                    message_db,
                    texture_cache,
                }
            });
            sender.send(load_result).unwrap();
        });
    }
    egui::CentralPanel::default().show(ctx, |ui| {
        let modal = Modal::new(ctx, "load_project_error_modal");
        modal.show(|ui| {
            modal.title(ui, "Project Failed to Load");
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
                    *next_state = Some(AppState::SelectProject);
                    state.error = None;
                }
                if modal.button(ui, "Copy Error").clicked() {
                    ui.output_mut(|out| {
                        out.copied_text = buffer;
                    });
                }
            });
        });
        match state.error.as_deref() {
            Some(_) => {
                modal.open();
            }
            None => {
                if ui.button("Cancel").clicked() {
                    *next_state = Some(AppState::SelectProject);
                }
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Spinner::new().size(96.0));
                });
            }
        }
    });
}
