use astra_formats::ParseError;
use egui::{FontId, Id, ScrollArea, Stroke, TextEdit, Ui};
use egui_extras::{Size, StripBuilder};
use itertools::Itertools;

const MIN_LINES: usize = 10;
const FONT_SIZE: f32 = 14.;

pub struct MsbtScriptEditor<'a> {
    id: Id,
    font_size: Option<f32>,
    on_focus_lost: Option<Box<dyn FnOnce(&str) + 'a>>,
}

impl<'a> MsbtScriptEditor<'a> {
    pub fn new(id_source: impl Into<Id>) -> Self {
        Self {
            id: id_source.into(),
            font_size: None,
            on_focus_lost: None,
        }
    }

    pub fn on_focus_lost<F>(self, listener: F) -> Self
    where
        F: FnOnce(&str) + 'a,
    {
        Self {
            on_focus_lost: Some(Box::new(listener)),
            ..self
        }
    }

    pub fn show(&mut self, ui: &mut Ui, script: &mut String) -> bool {
        let mut changed = false;
        StripBuilder::new(ui)
            .size(Size::relative(0.8))
            .size(Size::relative(0.2))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    changed = self.msbt_editor(ui, script);
                });
                strip.cell(|ui| {
                    self.error_pane(ui);
                });
            });
        changed
    }

    fn line_numbers(&self, ui: &mut Ui, text: &str) {
        let font_size = self.font_size.unwrap_or(FONT_SIZE);
        let total = if text.ends_with('\n') || text.is_empty() {
            text.lines().count() + 1
        } else {
            text.lines().count()
        }
        .max(MIN_LINES);
        let max_indent = total.to_string().len();
        let mut counter = (1..=total)
            .map(|i| {
                let label = i.to_string();
                format!(
                    "{}{label}",
                    " ".repeat(max_indent.saturating_sub(label.len()))
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        #[allow(clippy::cast_precision_loss)]
        let width = max_indent as f32 * font_size * 0.5;

        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let layout_job = egui::text::LayoutJob::single_section(
                string.to_string(),
                egui::TextFormat::simple(
                    egui::FontId::monospace(font_size),
                    ui.visuals().weak_text_color(),
                ),
            );
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(&mut counter)
                .id_source(self.id.with("num_lines"))
                .font(egui::TextStyle::Monospace)
                .interactive(false)
                .frame(false)
                .desired_rows(MIN_LINES)
                .desired_width(width)
                .layouter(&mut layouter),
        );
    }

    fn msbt_editor(&mut self, ui: &mut Ui, script: &mut String) -> bool {
        ui.horizontal_top(|ui| {
            ScrollArea::both()
                .id_source(ui.auto_id_with("script_pane"))
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.line_numbers(ui, script);
                    ui.visuals_mut().selection.stroke = Stroke::NONE;
                    ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::NONE;
                    ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::NONE;
                    ui.visuals_mut().widgets.hovered.expansion = 0.;
                    ui.visuals_mut().widgets.active.bg_stroke = Stroke::NONE;
                    let response = ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(script)
                            .code_editor()
                            .font(FontId::monospace(FONT_SIZE))
                            .desired_width(f32::INFINITY),
                    );
                    if response.lost_focus() {
                        if let Some(listener) = std::mem::take(&mut self.on_focus_lost) {
                            listener(script);
                        }
                    }
                    if response.changed() {
                        if let Err(err) = astra_formats::parse_astra_script(script) {
                            let errors: Vec<ParseError> = err.into();
                            ui.memory_mut(|mem| {
                                mem.data.insert_persisted(self.id, errors.iter().join("\n"));
                            });
                        } else {
                            ui.memory_mut(|mem| {
                                mem.data.insert_persisted(self.id, String::new());
                            });
                        }
                    }
                    response
                })
                .inner
        })
        .inner
        .changed()
    }

    fn error_pane(&self, ui: &mut Ui) {
        ui.allocate_space([ui.available_width(), 6.].into());
        let mut error_message = ui
            .memory_mut(|mem| mem.data.get_persisted::<String>(self.id))
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
                        .font(FontId::monospace(FONT_SIZE))
                        .desired_width(f32::MAX),
                );
            });
    }
}
