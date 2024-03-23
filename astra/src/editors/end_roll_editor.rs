use std::borrow::Cow;

use crate::{sheet_retriever, EditorState, ListEditorContent, PropertyGrid, ViewItem};

use astra_types::{EndRollBook, EndRollData};

sheet_retriever!(EndRollData, EndRollBook, end_roll_data, Vec<EndRollData>);

impl ViewItem for EndRollData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        let mut text = self.text_1.clone();
        text.push_str(&self.text_2);
        text.push_str(&self.text_3);
        if text.is_empty() {
            Cow::Borrowed("{empty}")
        } else {
            Cow::Owned(text)
        }
    }
}

pub struct EndRollEditor {
    end_roll_data: EndRollDataSheet,
    end_roll_data_content: ListEditorContent<Vec<EndRollData>, EndRollData>,
}

impl EndRollEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            end_roll_data: state.end_roll_data.clone(),
            end_roll_data_content: ListEditorContent::new("end_roll_data_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.end_roll_data_content
            .left_panel(ctx, &self.end_roll_data, state);

        self.end_roll_data.write(|data| {
            self.end_roll_data_content
                .content(ctx, data, |ui, selection| {
                    PropertyGrid::new("end_roll_data", selection)
                        .new_section("")
                        .default_field("Ty", |d| &mut d.ty)
                        .default_field("Text 1", |d| &mut d.text_1)
                        .default_field("Text 2", |d| &mut d.text_2)
                        .default_field("Text 3", |d| &mut d.text_3)
                        .show(ui)
                        .changed()
                })
        });
    }
}
