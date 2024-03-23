use std::borrow::Cow;

use indexmap::IndexMap;

use crate::{
    sheet_retriever, EditorState, GroupEditorContent, GroupViewItem, PropertyGrid, ViewItem,
};

use astra_types::{RangeBook, RangeData};

sheet_retriever!(RangeData, RangeBook, ranges, IndexMap<String, Vec<RangeData>>);

impl GroupViewItem for IndexMap<String, Vec<RangeData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for RangeData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!(
            "{} {} {} {} {} {} {} {}",
            self.value_1.unwrap_or_default(),
            self.value_2.unwrap_or_default(),
            self.value_3.unwrap_or_default(),
            self.value_4.unwrap_or_default(),
            self.value_5.unwrap_or_default(),
            self.value_6.unwrap_or_default(),
            self.value_7.unwrap_or_default(),
            self.value_8.unwrap_or_default(),
        ))
    }
}

pub struct RangeEditor {
    ranges: RangeDataSheet,
    ranges_content: GroupEditorContent,
}

impl RangeEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            ranges: state.ranges.clone(),
            ranges_content: GroupEditorContent::new("ranges_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.ranges_content.left_panel(ctx, &self.ranges, state);

        self.ranges.write(|data| {
            self.ranges_content.content(ctx, data, |ui, selection| {
                PropertyGrid::new("ranges", selection)
                    .new_section("")
                    .default_field("Value 1", |d| &mut d.value_1)
                    .default_field("Value 2", |d| &mut d.value_2)
                    .default_field("Value 3", |d| &mut d.value_3)
                    .default_field("Value 4", |d| &mut d.value_4)
                    .default_field("Value 5", |d| &mut d.value_5)
                    .default_field("Value 6", |d| &mut d.value_6)
                    .default_field("Value 7", |d| &mut d.value_7)
                    .default_field("Value 8", |d| &mut d.value_8)
                    .show(ui)
                    .changed()
            })
        });
    }
}
