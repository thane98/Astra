use std::path::Path;

use egui::{Align2, Frame, Vec2};

use crate::{folder_picker, AppConfig, AppState};

fn is_valid_cobalt_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn first_run(config: &mut AppConfig, next_state: &mut Option<AppState>, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |_| {
        egui::Window::new("first_run")
            .open(&mut true)
            .frame(Frame::none())
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .max_size([300., 200.])
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .show(ctx, |ui| {
                ui.heading("Welcome to Astra");
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(
                        "Please install ",
                    );
                    ui.hyperlink_to("Cobalt", "https://github.com/Raytwo/Cobalt");
                    ui.label(" first.");
                });
                ui.label("Once that's done, enter the path to Cobalt's \"engage\" folder on your SD card (or emulator equivalent) below.");
                ui.add(folder_picker(&mut config.cobalt_path));
                let is_valid_cobalt_dir = is_valid_cobalt_dir(&config.cobalt_path);
                if !is_valid_cobalt_dir && !config.cobalt_path.is_empty() {
                    ui.colored_label(
                        ui.visuals().error_fg_color,
                        "Path does not go to Cobalt's \"engage\" folder.",
                    );
                }
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(is_valid_cobalt_dir, |ui| {
                        if ui.button("Next").clicked() {
                            *next_state =
                                Some(AppState::CreateProject(Box::default()));
                        }
                    });
                    if ui.button("Cancel").clicked() {
                        *next_state = Some(AppState::SelectProject);
                    }
                });
            });
    });
}
