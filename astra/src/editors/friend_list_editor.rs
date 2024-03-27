use std::borrow::Cow;

use indexmap::IndexMap;

use crate::{
    id_field, keyed_add_modal_content, model_drop_down, msbt_key_value_multiline, sheet_retriever,
    EditorState, KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{FriendListBook, FriendListData};

sheet_retriever!(FriendListData, FriendListBook, friend_list_data, IndexMap<String, FriendListData>);

impl ViewItem for FriendListData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .person
            .read(|data| {
                data.get(&self.pid)
                    .map(|person| person.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.flid))
    }

    fn decorated(_: crate::DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        _: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies
            .texture_cache
            .borrow_mut()
            .get_notebook(&self.stamp_name)
            .map(|texture| (texture, 0.25))
    }
}

impl KeyedViewItem for FriendListData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.flid)
    }

    fn set_key(&mut self, key: String) {
        self.flid = key;
    }
}

pub struct FriendListEditor {
    friend_list_data: FriendListDataSheet,
    friend_list_data_content:
        ListEditorContent<IndexMap<String, FriendListData>, FriendListData, EditorState>,
}

impl FriendListEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            friend_list_data: state.friend_list_data.clone(),
            friend_list_data_content: ListEditorContent::new("friend_list_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.friend_list_data_content
            .left_panel(ctx, &self.friend_list_data, state);

        self.friend_list_data.write(|data| {
            self.friend_list_data_content
                .content(ctx, data, |ui, selection| {
                    PropertyGrid::new("friend_list_data", selection)
                        .new_section("")
                        .field("FLID", |ui, d| ui.add(id_field(&mut d.flid)))
                        .field("Person", |ui, d| {
                            state
                                .person
                                .read(|data| ui.add(model_drop_down(data, state, &mut d.pid)))
                        })
                        .default_field("Level", |d| &mut d.level)
                        .field("Content Text", |ui, d| {
                            msbt_key_value_multiline!(ui, state, "friendlist", d.content_text)
                        })
                        .default_field("Stamp Name", |d| &mut d.stamp_name)
                        .default_field("Image Name", |d| &mut d.image_name)
                        .default_field("Image Name S", |d| &mut d.image_name_s)
                        .default_field("Country", |d| &mut d.country)
                        .show(ui)
                        .changed()
                })
        });
    }
}
