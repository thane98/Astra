use std::borrow::Cow;

use astra_types::{AchieveData, AchievementBook, BelongData};
use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, keyed_add_modal_content, model_drop_down,
    msbt_key_value_singleline, sheet_retriever, standard_keyed_display, EditorState, KeyedViewItem,
    ListEditorContent, PropertyGrid, ViewItem,
};

sheet_retriever!(Achievement, AchievementBook, achievements, IndexMap<String, AchieveData>);

impl ViewItem for AchieveData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, aid, name)
    }

    fn decorated(_: crate::DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        _: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        let icon = match self.category {
            0 => "Companion",
            1 => "Battle",
            2 => "Solanel",
            3 => "Shop",
            4 => "Reward",
            _ => "System",
        };
        dependencies
            .texture_cache
            .borrow_mut()
            .get_achievement(icon)
            .map(|texture| (texture, 0.5))
    }
}

impl KeyedViewItem for AchieveData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.aid)
    }

    fn set_key(&mut self, key: String) {
        self.aid = key;
    }
}

sheet_retriever!(Belong, AchievementBook, belong, IndexMap<String, BelongData>);

impl ViewItem for BelongData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, bid, name)
    }
}

impl KeyedViewItem for BelongData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.bid)
    }

    fn set_key(&mut self, key: String) {
        self.bid = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Achievement,
    Belong,
}

pub struct AchieveEditor {
    tab: Tab,
    achieve: AchievementSheet,
    belong: BelongSheet,
    achieve_content: ListEditorContent<IndexMap<String, AchieveData>, AchieveData, EditorState>,
    belong_content: ListEditorContent<IndexMap<String, BelongData>, BelongData, EditorState>,
}

impl AchieveEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Achievement,
            achieve: state.achieve.clone(),
            belong: state.belong.clone(),
            achieve_content: ListEditorContent::new("achieve_editor")
                .with_add_modal_content(keyed_add_modal_content),
            belong_content: ListEditorContent::new("belong_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Achievement, "Achievement");
            ui.selectable_value(&mut self.tab, Tab::Belong, "Belong");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::Achievement => {
                self.achieve_content.left_panel(ctx, &self.achieve, state);
                self.achieve.write(|data| {
                    self.achieve_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("achievement", selection)
                            .new_section("")
                            .field("AID", |ui, d| ui.add(id_field(&mut d.aid)))
                            .field("Name", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "animal", d.name)
                            })
                            .default_field("Category", |d| &mut d.category)
                            .default_field("Kind", |d| &mut d.kind)
                            .default_field("Count", |d| &mut d.count)
                            .default_field("Arg", |d| &mut d.arg)
                            .default_field("Count Unit", |d| &mut d.count_unit)
                            .default_field("Kizuna Reward", |d| &mut d.kizuna_reward)
                            .field("Chapter", |ui, d| {
                                state.chapter.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.chapter))
                                })
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::Belong => {
                self.belong_content.left_panel(ctx, &self.belong, state);
                self.belong.write(|data| {
                    self.belong_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("achievement", selection)
                            .new_section("")
                            .field("BID", |ui, d| ui.add(id_field(&mut d.bid)))
                            .default_field("Name", |d| &mut d.name)
                            .default_field("Defeat Achieve", |d| &mut d.defeat_achieve)
                            .show(ui)
                            .changed()
                    })
                });
            }
        }
    }
}
