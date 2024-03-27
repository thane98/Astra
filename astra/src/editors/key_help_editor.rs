use std::borrow::Cow;

use indexmap::IndexMap;

use crate::{
    msbt_key_value_multiline, sheet_retriever, standard_keyed_display, EditorState,
    GroupEditorContent, GroupViewItem, PropertyGrid, ViewItem,
};

use astra_types::{KeyHelpData, KeyHelpDataBook};

sheet_retriever!(KeyHelpData, KeyHelpDataBook, key_help_data, IndexMap<String, Vec<KeyHelpData>>);

impl GroupViewItem for IndexMap<String, Vec<KeyHelpData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for KeyHelpData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, mid, mid)
    }
}

pub struct KeyHelpDataEditor {
    key_help_data: KeyHelpDataSheet,
    key_help_data_content: GroupEditorContent,
}

impl KeyHelpDataEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            key_help_data: state.key_help_data.clone(),
            key_help_data_content: GroupEditorContent::new("key_help_data_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.key_help_data_content
            .left_panel(ctx, &self.key_help_data, state);

        self.key_help_data.write(|data| {
            self.key_help_data_content
                .content(ctx, data, |ui, selection| {
                    PropertyGrid::new("key_help_data", selection)
                        .new_section("")
                        .default_field("Button Index", |d| &mut d.button_index)
                        .field("MID", |ui, d| {
                            msbt_key_value_multiline!(ui, state, "system", d.mid)
                        })
                        .show(ui)
                        .changed()
                })
        });
    }
}
