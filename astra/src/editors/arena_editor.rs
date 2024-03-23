use std::borrow::Cow;

use astra_types::{ArenaBook, ArenaData};
use indexmap::IndexMap;

use crate::{
    model_drop_down, sheet_retriever, EditorState, GroupEditorContent, GroupViewItem, PropertyGrid, ViewItem
};

sheet_retriever!(Arena, ArenaBook, arena_data, IndexMap<String, Vec<ArenaData>>);

impl GroupViewItem for IndexMap<String, Vec<ArenaData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for ArenaData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        let item = dependencies
            .item
            .read(|data| {
                data.get(&self.iid)
                    .map(|item| Cow::Owned(item.text(dependencies).to_string()))
            })
            .unwrap_or(Cow::Borrowed("{unknown item}"));
        Cow::Owned(format!("{} + {}", &self.pid, item))
    }
}

pub struct ArenaEditor {
    sheet: ArenaSheet,
    content: GroupEditorContent,
}

impl ArenaEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            sheet: state.arena.clone(),
            content: GroupEditorContent::new("arena_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.content.left_panel(ctx, &self.sheet, state);

        self.sheet.write(|data| {
            self.content.content(ctx, data, |ui, selection| {
                PropertyGrid::new("arena", selection)
                    .new_section("")
                    .default_field("Rate", |d| &mut d.rate)
                    .default_field("Pid", |d| &mut d.pid)
                    .field("Item", |ui, d| {
                        state.item.read(|data| {
                            ui.add(model_drop_down(data, state, &mut d.iid))
                        })
                    })
                    .show(ui)
                    .changed()
            })
        });
    }
}
