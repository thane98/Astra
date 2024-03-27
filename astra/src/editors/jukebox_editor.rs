use std::borrow::Cow;

use indexmap::IndexMap;

use crate::{
    id_field, keyed_add_modal_content, sheet_retriever, EditorState, KeyedViewItem,
    ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{JukeboxBook, JukeboxData};

sheet_retriever!(JukeboxData, JukeboxBook, jukebox_data, IndexMap<String, JukeboxData>);

impl ViewItem for JukeboxData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.event_name)
    }
}

impl KeyedViewItem for JukeboxData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.event_name)
    }

    fn set_key(&mut self, key: String) {
        self.event_name = key;
    }
}

pub struct JukeboxEditor {
    jukebox_data: JukeboxDataSheet,
    jukebox_data_content:
        ListEditorContent<IndexMap<String, JukeboxData>, JukeboxData, EditorState>,
}

impl JukeboxEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            jukebox_data: state.jukebox_data.clone(),
            jukebox_data_content: ListEditorContent::new("jukebox_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.jukebox_data_content
            .left_panel(ctx, &self.jukebox_data, state);

        self.jukebox_data.write(|data| {
            self.jukebox_data_content
                .content(ctx, data, |ui, selection| {
                    PropertyGrid::new("jukebox_data", selection)
                        .new_section("")
                        .field("Event Name", |ui, d| ui.add(id_field(&mut d.event_name)))
                        // TOOD: Figure out what MSBT has this
                        .default_field("Name", |d| &mut d.name)
                        .default_field("Condition", |d| &mut d.condition)
                        .show(ui)
                        .changed()
                })
        });
    }
}
