use astra_types::GameParam;

use crate::{f32_drag, EditorState, GameParamSheet, ListEditorContent, PropertyGrid};

pub struct GameParamEditor {
    param: GameParamSheet,
    content: ListEditorContent<Vec<GameParam>, GameParam>,
}

impl GameParamEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            param: state.param.clone(),
            content: ListEditorContent::new("param_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        self.content.side_panel(ctx, &self.param, &());

        self.param.write(|data| {
            self.content.content(ctx, data, |ui, param| {
                PropertyGrid::new("param", param)
                    .new_section("Data")
                    .field("Name", |ui, param| ui.text_edit_singleline(&mut param.name))
                    .field("English", |ui, param| {
                        ui.text_edit_singleline(&mut param.english)
                    })
                    .field("En", |ui, param| ui.text_edit_singleline(&mut param.en))
                    .field("Value", |ui, param| ui.add(f32_drag(&mut param.value)))
                    .field("Min", |ui, param| ui.add(f32_drag(&mut param.min)))
                    .field("Max", |ui, param| ui.add(f32_drag(&mut param.max)))
                    .field("Step", |ui, param| ui.add(f32_drag(&mut param.step)))
                    .field("Out", |ui, param| ui.text_edit_singleline(&mut param.out))
                    .show(ui)
                    .changed()
            })
        });
    }
}
