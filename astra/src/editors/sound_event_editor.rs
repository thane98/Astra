use std::borrow::Cow;

use indexmap::IndexMap;

use crate::{
    id_field, keyed_add_modal_content, sheet_retriever, EditorState, KeyedViewItem,
    ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{SoundEvent, SoundEventBook};

sheet_retriever!(SoundEvent, SoundEventBook, sound_events, IndexMap<String, SoundEvent>);

impl ViewItem for SoundEvent {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.movie_file_name)
    }
}

impl KeyedViewItem for SoundEvent {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.movie_file_name)
    }

    fn set_key(&mut self, key: String) {
        self.movie_file_name = key;
    }
}

pub struct SoundEventEditor {
    sound_events: SoundEventSheet,
    sound_events_content: ListEditorContent<IndexMap<String, SoundEvent>, SoundEvent, EditorState>,
}

impl SoundEventEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            sound_events: state.sound_events.clone(),
            sound_events_content: ListEditorContent::new("sound_events_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.sound_events_content
            .left_panel(ctx, &self.sound_events, state);

        self.sound_events.write(|data| {
            self.sound_events_content
                .content(ctx, data, |ui, selection| {
                    PropertyGrid::new("sound_events", selection)
                        .new_section("")
                        .field("Movie File Name", |ui, d| {
                            ui.add(id_field(&mut d.movie_file_name))
                        })
                        .default_field("Event Name 1", |d| &mut d.event_name_1)
                        .default_field("Event Name 2", |d| &mut d.event_name_2)
                        .default_field("Event Name 3", |d| &mut d.event_name_3)
                        .default_field("Event Name 4", |d| &mut d.event_name_4)
                        .show(ui)
                        .changed()
                })
        });
    }
}
