use std::borrow::Cow;

use indexmap::IndexMap;

use crate::{
    id_field, keyed_add_modal_content, sheet_retriever, EditorState, KeyedViewItem,
    ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{VibrationBook, VibrationDefineData};

sheet_retriever!(VibrationDefineData, VibrationBook, vibration_data, IndexMap<String, VibrationDefineData>);

impl ViewItem for VibrationDefineData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.event_name)
    }
}

impl KeyedViewItem for VibrationDefineData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.event_name)
    }

    fn set_key(&mut self, key: String) {
        self.event_name = key;
    }
}

pub struct VibrationEditor {
    vibration_data: VibrationDefineDataSheet,
    vibration_data_content:
        ListEditorContent<IndexMap<String, VibrationDefineData>, VibrationDefineData, EditorState>,
}

impl VibrationEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            vibration_data: state.vibration_data.clone(),
            vibration_data_content: ListEditorContent::new("vibration_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.vibration_data_content
            .left_panel(ctx, &self.vibration_data, state);

        self.vibration_data.write(|data| {
            self.vibration_data_content
                .content(ctx, data, |ui, selection| {
                    PropertyGrid::new("vibration_data", selection)
                        .new_section("")
                        .field("Event Name", |ui, d| ui.add(id_field(&mut d.event_name)))
                        .default_field("Vibration File Name", |d| &mut d.vibration_file_name)
                        .default_field("Amplitude Magnitude", |d| &mut d.amplitude_magnitude)
                        .show(ui)
                        .changed()
                })
        });
    }
}
