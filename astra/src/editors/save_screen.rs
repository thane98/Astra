use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Duration;

use astra_core::error::Result;
use astra_core::Astra;
use egui::TextEdit;
use egui_modal::Modal;
use egui_notify::Toasts;
use parking_lot::RwLock;

use crate::Screens;

pub struct SaveScreen {
    astra: Arc<RwLock<Astra>>,
    rx: Option<Receiver<Result<()>>>,
    return_screen: Screens,
    error: Option<String>,
}

impl SaveScreen {
    pub fn new(astra: Arc<RwLock<Astra>>) -> Self {
        Self {
            astra,
            return_screen: Screens::Chapter,
            rx: None,
            error: None,
        }
    }

    pub fn set_return_screen(&mut self, screen: Screens) {
        if !matches!(screen, Screens::Save) {
            self.return_screen = screen;
        }
    }

    pub fn ui(&mut self, screen: &mut Screens, ctx: &egui::Context, toasts: &mut Toasts) {
        if let (Some(rx), None) = (&mut self.rx, &self.error) {
            match rx.try_recv() {
                Ok(result) => match result {
                    Ok(_) => {
                        self.rx = None;
                        self.error = None;
                        *screen = self.return_screen;
                        toasts
                            .success("Save complete")
                            .set_duration(Some(Duration::from_secs(2)));
                    }
                    Err(err) => {
                        self.error = Some(format!("{:?}", err));
                    }
                },
                _ => {}
            }
        } else if self.error.is_none() {
            let astra = self.astra.clone();
            let (sx, rx) = std::sync::mpsc::channel();
            self.rx = Some(rx);
            std::thread::spawn(move || {
                sx.send(astra.read().save()).unwrap();
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let modal = Modal::new(ctx, "save_project_error_modal");
            modal.show(|ui| {
                modal.title(ui, "Project Failed to Save");
                let mut buffer = self.error.clone().unwrap_or_default();
                ui.add_enabled_ui(false, |ui| {
                    ui.add(
                        TextEdit::multiline(&mut buffer)
                            .desired_width(f32::INFINITY)
                            .desired_rows(8),
                    );
                });
                modal.buttons(ui, |ui| {
                    if modal.button(ui, "Close").clicked() {
                        self.rx = None;
                        self.error = None;
                        *screen = self.return_screen;
                    }
                    if modal.button(ui, "Copy Error").clicked() {
                        ui.output_mut(|out| {
                            out.copied_text = buffer;
                        });
                    }
                });
            });
            match self.error.as_deref() {
                Some(_) => {
                    modal.open();
                }
                None => {
                    ui.centered_and_justified(|ui| {
                        ui.add(egui::Spinner::new().size(96.0));
                    });
                }
            }
        });
    }
}
