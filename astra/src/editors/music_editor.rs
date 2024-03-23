use std::borrow::Cow;


use indexmap::IndexMap;

use crate::{
    model_drop_down, msbt_key_value_multiline, msbt_key_value_singleline, sheet_retriever, standard_keyed_display, EditorState, KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem
};

use astra_types::{MusicBook, MusicData};

sheet_retriever!(MusicData, MusicBook, music, IndexMap<String, MusicData>);

impl ViewItem for MusicData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, event_name, name)
    }
}

impl KeyedViewItem for MusicData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.event_name)
    }

    fn set_key(&mut self, key: String) {
        self.event_name = key;
    }
}

pub struct MusicEditor {
    music: MusicDataSheet,
    music_content: ListEditorContent<IndexMap<String, MusicData>, MusicData>,
}

impl MusicEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            music: state.music.clone(),
            music_content: ListEditorContent::new("music_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.music_content.left_panel(ctx, &self.music, state);

        self.music.write(|data| {
            self.music_content.content(ctx, data, |ui, selection| {
                PropertyGrid::new("music", selection)
                    .new_section("")
                    .default_field("Event Name", |d| &mut d.event_name)
                    .field("Name", |ui, d| msbt_key_value_singleline!(ui, state, "musicname", d.name))
                    .field("Help", |ui, d| msbt_key_value_multiline!(ui, state, "gamedata", d.help))
                    .default_field("Help", |d| &mut d.help)
                    .default_field("Condition", |d| &mut d.condition)
                    .default_field("Amiibo", |d| &mut d.amiibo)
                    .default_field("Change Event Name", |d| &mut d.change_event_name)
                    .default_field("Is Change", |d| &mut d.is_change)
                    .field("Emblem", |ui, d| state.god.read(|data| {
                        ui.add(model_drop_down(data, state, &mut d.gid))
                    }))
                    .show(ui)
                    .changed()
            })
        });
    }
}
