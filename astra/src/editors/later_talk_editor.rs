use std::borrow::Cow;


use indexmap::IndexMap;

use crate::{
    model_drop_down, sheet_retriever, EditorState, GroupEditorContent, GroupViewItem, PropertyGrid, ViewItem
};

use astra_types::{LaterTalkBook, Person, PostBattleConversation};

sheet_retriever!(PostBattleConversation, LaterTalkBook, post_battle_conversations, IndexMap<String, Vec<PostBattleConversation>>);

impl GroupViewItem for IndexMap<String, Vec<PostBattleConversation>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for PostBattleConversation {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .person
            .read(|data| {
                data.get(&self.person).map(|person| person.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.person))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Person::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.person.read(|data| {
            data.get(&self.person)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

pub struct LaterTalkEditor {
    post_battle_conversations: PostBattleConversationSheet,
    post_battle_conversations_content: GroupEditorContent,
}

impl LaterTalkEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            post_battle_conversations: state.post_battle_conversations.clone(),
            post_battle_conversations_content: GroupEditorContent::new(
                "post_battle_conversations_editor",
            ),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.post_battle_conversations_content.left_panel(
            ctx,
            &self.post_battle_conversations,
            state,
        );

        self.post_battle_conversations.write(|data| {
            self.post_battle_conversations_content
                .content(ctx, data, |ui, selection| {
                    PropertyGrid::new("post_battle_conversations", selection)
                        .new_section("")
                        .field("Person", |ui, d| state.person.read(|data| {
                            ui.add(model_drop_down(data, state, &mut d.person))
                        }))
                        .default_field("Field", |d| &mut d.field)
                        .default_field("Back Degree", |d| &mut d.back_degree)
                        .default_field("Light Degree", |d| &mut d.light_degree)
                        .show(ui)
                        .changed()
                })
        });
    }
}
