use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, model_drop_down, sheet_retriever, EditorState, GroupEditorContent,
    GroupViewItem, PropertyGrid, ViewItem,
};

use astra_types::{Item, KillBonus1, KillBonus2, KillBonusBook};

sheet_retriever!(KillBonus1, KillBonusBook, kill_bonuses_1, IndexMap<String, Vec<KillBonus1>>);

impl GroupViewItem for IndexMap<String, Vec<KillBonus1>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for KillBonus1 {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .item
            .read(|data| {
                data.get(&self.iid)
                    .map(|item| item.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.iid))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Item::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.item.read(|data| {
            data.get(&self.iid)
                .and_then(|item| item.decoration(dependencies, kind))
        })
    }
}

sheet_retriever!(KillBonus2, KillBonusBook, kill_bonuses_2, IndexMap<String, Vec<KillBonus2>>);

impl GroupViewItem for IndexMap<String, Vec<KillBonus2>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for KillBonus2 {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("Kind {}", self.kind))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    KillBonus1,
    KillBonus2,
}

pub struct KillBonusEditor {
    tab: Tab,
    kill_bonuses_1: KillBonus1Sheet,
    kill_bonuses_2: KillBonus2Sheet,
    kill_bonuses_1_content: GroupEditorContent,
    kill_bonuses_2_content: GroupEditorContent,
}

impl KillBonusEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::KillBonus1,
            kill_bonuses_1: state.kill_bonuses_1.clone(),
            kill_bonuses_2: state.kill_bonuses_2.clone(),
            kill_bonuses_1_content: GroupEditorContent::new("kill_bonuses_1_editor"),
            kill_bonuses_2_content: GroupEditorContent::new("kill_bonuses_2_editor"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::KillBonus1, "KillBonus1");
            ui.selectable_value(&mut self.tab, Tab::KillBonus2, "KillBonus2");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::KillBonus1 => {
                self.kill_bonuses_1_content
                    .left_panel(ctx, &self.kill_bonuses_1, state);
                self.kill_bonuses_1.write(|data| {
                    self.kill_bonuses_1_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("kill_bonuses_1", selection)
                                .new_section("")
                                .field("Item", |ui, d| {
                                    state.item.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.iid))
                                    })
                                })
                                .default_field("Rate", |d| &mut d.rate)
                                .field("Chapter", |ui, d| {
                                    state.chapter.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.cid))
                                    })
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::KillBonus2 => {
                self.kill_bonuses_2_content
                    .left_panel(ctx, &self.kill_bonuses_2, state);
                self.kill_bonuses_2.write(|data| {
                    self.kill_bonuses_2_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("kill_bonuses_2", selection)
                                .new_section("")
                                .default_field("Kind", |d| &mut d.kind)
                                .default_field("Value", |d| &mut d.value)
                                .default_field("Rate", |d| &mut d.rate)
                                .default_field("Flag", |d| &mut d.flag)
                                .field("Chapter", |ui, d| {
                                    state.chapter.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.cid))
                                    })
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
