use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, model_drop_down, sheet_retriever, EditorState, KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem
};

use astra_types::{TitleBook, TitleCallData, TitlePedestalData};

sheet_retriever!(TitleCallData, TitleBook, call_data, IndexMap<String, TitleCallData>);

impl ViewItem for TitleCallData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.pid_or_gid)
    }
}

impl KeyedViewItem for TitleCallData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.pid_or_gid)
    }

    fn set_key(&mut self, key: String) {
        self.pid_or_gid = key;
    }
}

sheet_retriever!(TitlePedestalData, TitleBook, pedestal_data, IndexMap<String, TitlePedestalData>);

impl ViewItem for TitlePedestalData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.pedestal_name)
    }
}

impl KeyedViewItem for TitlePedestalData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.pedestal_name)
    }

    fn set_key(&mut self, key: String) {
        self.pedestal_name = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    TitleCallData,
    TitlePedestalData,
}

pub struct TitleEditor {
    tab: Tab,
    call_data: TitleCallDataSheet,
    pedestal_data: TitlePedestalDataSheet,
    call_data_content: ListEditorContent<IndexMap<String, TitleCallData>, TitleCallData>,
    pedestal_data_content:
        ListEditorContent<IndexMap<String, TitlePedestalData>, TitlePedestalData>,
}

impl TitleEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::TitleCallData,
            call_data: state.title_call_data.clone(),
            pedestal_data: state.title_pedestal_data.clone(),
            call_data_content: ListEditorContent::new("call_data_editor"),
            pedestal_data_content: ListEditorContent::new("pedestal_data_editor"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::TitleCallData, "Call");
            ui.selectable_value(&mut self.tab, Tab::TitlePedestalData, "Pedestal");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::TitleCallData => {
                self.call_data_content
                    .left_panel(ctx, &self.call_data, state);
                self.call_data.write(|data| {
                    self.call_data_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("call_data", selection)
                            .new_section("")
                            .field("PID or GID", |ui, d| ui.add(id_field(&mut d.pid_or_gid)))
                            .field("Chapter", |ui, d| state.chapter.read(|data| {
                                ui.add(model_drop_down(data, state, &mut d.cid))
                            }))
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::TitlePedestalData => {
                self.pedestal_data_content
                    .left_panel(ctx, &self.pedestal_data, state);
                self.pedestal_data.write(|data| {
                    self.pedestal_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("pedestal_data", selection)
                                .new_section("")
                                .field("Pedestal", |ui, d| ui.add(id_field(&mut d.pedestal_name)))
                                .field("Chapter", |ui, d| state.chapter.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.cid))
                                }))
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
