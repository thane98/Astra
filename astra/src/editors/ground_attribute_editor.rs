use std::borrow::Cow;

use indexmap::IndexMap;

use crate::{
    id_field, sheet_retriever, EditorState, KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem
};

use astra_types::{GroundAttribute, GroundAttributeBook};

sheet_retriever!(GroundAttribute, GroundAttributeBook, ground_attributes, IndexMap<String, GroundAttribute>);

impl ViewItem for GroundAttribute {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.label)
    }
}

impl KeyedViewItem for GroundAttribute {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.label)
    }

    fn set_key(&mut self, key: String) {
        self.label = key;
    }
}

pub struct GroundAttributeEditor {
    ground_attributes: GroundAttributeSheet,
    ground_attributes_content:
        ListEditorContent<IndexMap<String, GroundAttribute>, GroundAttribute>,
}

impl GroundAttributeEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            ground_attributes: state.ground_attributes.clone(),
            ground_attributes_content: ListEditorContent::new("ground_attributes_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.ground_attributes_content
            .left_panel(ctx, &self.ground_attributes, state);

        self.ground_attributes.write(|data| {
            self.ground_attributes_content
                .content(ctx, data, |ui, selection| {
                    PropertyGrid::new("ground_attributes", selection)
                        .new_section("")
                        .field("Label", |ui, d| ui.add(id_field(&mut d.label)))
                        .default_field("Name", |d| &mut d.name)
                        .default_field("Sound", |d| &mut d.sound)
                        .default_field("Particle", |d| &mut d.particle)
                        .show(ui)
                        .changed()
                })
        });
    }
}
