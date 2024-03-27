use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, keyed_add_modal_content, model_drop_down, msbt_key_value_multiline,
    msbt_key_value_singleline, sheet_retriever, standard_keyed_display, EditorState,
    GroupEditorContent, GroupViewItem, KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{TipData, TutorialBook, TutorialData};

sheet_retriever!(TutorialData, TutorialBook, tutorials, IndexMap<String, Vec<TutorialData>>);

impl GroupViewItem for IndexMap<String, Vec<TutorialData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for TutorialData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, title, title)
    }
}

sheet_retriever!(TipData, TutorialBook, tips, IndexMap<String, TipData>);

impl ViewItem for TipData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, title, title)
    }
}

impl KeyedViewItem for TipData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    TutorialData,
    TipData,
}

pub struct TutorialEditor {
    tab: Tab,
    tutorials: TutorialDataSheet,
    tips: TipDataSheet,
    tutorials_content: GroupEditorContent,
    tips_content: ListEditorContent<IndexMap<String, TipData>, TipData, EditorState>,
}

impl TutorialEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::TutorialData,
            tutorials: state.tutorials.clone(),
            tips: state.tips.clone(),
            tutorials_content: GroupEditorContent::new("tutorials_editor"),
            tips_content: ListEditorContent::new("tips_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::TutorialData, "Tutorials");
            ui.selectable_value(&mut self.tab, Tab::TipData, "Tips");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::TutorialData => {
                self.tutorials_content
                    .left_panel(ctx, &self.tutorials, state);
                self.tutorials.write(|data| {
                    self.tutorials_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("tutorials", selection)
                            .new_section("")
                            .field("Title", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "tutorial", d.title)
                            })
                            .field("MID", |ui, d| {
                                msbt_key_value_multiline!(ui, state, "tutorial", d.mid)
                            })
                            .default_field("Sprite Atlas", |d| &mut d.sprite_atlas)
                            .default_field("Ty", |d| &mut d.ty)
                            .default_field("Notice", |d| &mut d.notice)
                            .field("Chapter", |ui, d| {
                                state
                                    .chapter
                                    .read(|data| ui.add(model_drop_down(data, state, &mut d.cid)))
                            })
                            .default_field("No", |d| &mut d.no)
                            .default_field("Ss Type", |d| &mut d.ss_type)
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::TipData => {
                self.tips_content.left_panel(ctx, &self.tips, state);
                self.tips.write(|data| {
                    self.tips_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("tips", selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            .field("Title", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "tutorial", d.title)
                            })
                            .field("Tips", |ui, d| {
                                msbt_key_value_multiline!(ui, state, "tutorial", d.tips)
                            })
                            .default_field("Own Id", |d| &mut d.own_id)
                            .default_field("Icon Info Id", |d| &mut d.icon_info_id)
                            .field("Chapter", |ui, d| {
                                state.chapter.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.chapter))
                                })
                            })
                            .default_field("Variable", |d| &mut d.variable)
                            .default_field("Allow", |d| &mut d.allow)
                            .show(ui)
                            .changed()
                    })
                });
            }
        }
    }
}
