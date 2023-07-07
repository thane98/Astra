use astra_core::OpenMessageScript;
use astra_formats::ParseError;
use egui::{Id, ScrollArea, TextEdit, Ui};
use egui_extras::{Size, StripBuilder};
use itertools::Itertools;

pub fn msbt_script_editor(ui: &mut Ui, script: &OpenMessageScript) {
    let id = Id::new(script.path());
    StripBuilder::new(ui)
        .size(Size::relative(0.8))
        .size(Size::relative(0.2))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                ScrollArea::both()
                    .id_source(ui.auto_id_with("script_pane"))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        script.script(|script| {
                            let response = ui.add_sized(
                                ui.available_size(),
                                TextEdit::multiline(script)
                                    .code_editor()
                                    .desired_width(f32::INFINITY),
                            );
                            if response.changed() {
                                if let Err(err) = astra_formats::parse_astra_script(script) {
                                    let errors: Vec<ParseError> = err.into();
                                    ui.memory_mut(|mem| {
                                        mem.data.insert_persisted(id, errors.iter().join("\n"));
                                    });
                                } else {
                                    ui.memory_mut(|mem| {
                                        mem.data.insert_persisted(id, String::new());
                                    });
                                }
                            }
                            response.changed()
                        });
                    });
            });
            strip.cell(|ui| {
                ui.allocate_space([ui.available_width(), 6.].into());
                let mut error_message = ui
                    .memory_mut(|mem| mem.data.get_persisted::<String>(id))
                    .unwrap_or_default();
                ScrollArea::both()
                    .id_source(ui.auto_id_with("error_pane"))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_sized(
                            ui.available_size(),
                            TextEdit::multiline(&mut error_message)
                                .interactive(false)
                                .text_color(ui.visuals().error_fg_color)
                                .code_editor()
                                .desired_width(f32::INFINITY),
                        );
                    });
            });
        });
}
