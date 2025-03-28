use astra_types::GameParam;
use egui::DragValue;

use crate::{EditorState, GameParamSheet, ListEditorContent, PropertyGrid};

pub struct GameParamEditor {
    param: GameParamSheet,
    content: ListEditorContent<Vec<GameParam>, GameParam, ()>,
}

impl GameParamEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            param: state.param.clone(),
            content: ListEditorContent::new("param_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        self.content.left_panel(ctx, &self.param, &());

        self.param.write(|data| {
            self.content.content(ctx, data, |ui, param| {
                PropertyGrid::new("param", param)
                    .new_section("")
                    .field("Name", |ui, param| ui.text_edit_singleline(&mut param.name))
                    .field("English", |ui, param| {
                        ui.text_edit_singleline(&mut param.english)
                    })
                    .field("En", |ui, param| ui.text_edit_singleline(&mut param.en))
                    .field("Value", |ui, param| {
                        ui.add(DragValue::new(&mut param.value))
                    })
                    .field("Min", |ui, param| ui.add(DragValue::new(&mut param.min)))
                    .field("Max", |ui, param| ui.add(DragValue::new(&mut param.max)))
                    .field("Step", |ui, param| ui.add(DragValue::new(&mut param.step)))
                    .field("Out", |ui, param| ui.text_edit_singleline(&mut param.out))
                    .show(ui)
                    .changed()
            })
        });
    }
}
