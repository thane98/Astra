use std::borrow::Cow;

use astra_types::{AiBook, AiData};
use indexmap::IndexMap;

use crate::{
    sheet_retriever, EditorState, GroupEditorContent, GroupViewItem, PropertyGrid, ViewItem,
};

sheet_retriever!(Ai, AiBook, ai_data, IndexMap<String, Vec<AiData>>);

impl GroupViewItem for IndexMap<String, Vec<AiData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for AiData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        format!(
            "Active {} Code {} Mind {}",
            self.active,
            self.code,
            self.mind
        )
        .into()
    }
}

pub struct AiEditor {
    sheet: AiSheet,
    content: GroupEditorContent,
}

impl AiEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            sheet: state.ai.clone(),
            content: GroupEditorContent::new("ai_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.content.left_panel(ctx, &self.sheet, state);

        self.sheet.write(|data| {
            self.content.content(ctx, data, |ui, selection| {
                PropertyGrid::new("ai", selection)
                    .new_section("")
                    .default_field("Active", |d| &mut d.active)
                    .default_field("Code", |d| &mut d.code)
                    .default_field("Mind", |d| &mut d.mind)
                    .default_field("Str Value 0", |d| &mut d.str_value_0)
                    .default_field("Str Value 1", |d| &mut d.str_value_1)
                    .default_field("Trans", |d| &mut d.trans)
                    .show(ui)
                    .changed()
            })
        });
    }
}
