use std::borrow::Cow;


use indexmap::IndexMap;

use crate::{
    id_field, msbt_key_value_singleline, sheet_retriever, EditorState, KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem
};

use astra_types::{MapHistory, MapHistoryBook};

sheet_retriever!(MapHistory, MapHistoryBook, history, IndexMap<String, MapHistory>);

impl ViewItem for MapHistory {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.mhid)
    }
}

impl KeyedViewItem for MapHistory {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.mhid)
    }

    fn set_key(&mut self, key: String) {
        self.mhid = key;
    }
}

pub struct MapHistoryEditor {
    history: MapHistorySheet,
    history_content: ListEditorContent<IndexMap<String, MapHistory>, MapHistory>,
}

impl MapHistoryEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            history: state.map_history.clone(),
            history_content: ListEditorContent::new("history_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.history_content.left_panel(ctx, &self.history, state);

        self.history.write(|data| {
            self.history_content.content(ctx, data, |ui, selection| {
                PropertyGrid::new("history", selection)
                    .new_section("")
                    .field("MHID", |ui, d| ui.add(id_field(&mut d.mhid)))
                    .field("Action", |ui, d| msbt_key_value_singleline!(ui, state, "maphistory", d.action))
                    .default_field("Priority", |d| &mut d.priority)
                    .show(ui)
                    .changed()
            })
        });
    }
}
