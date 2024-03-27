use std::borrow::Cow;

use astra_types::{ChartBook, ChartData, ChartGodData, ChartParam};
use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, keyed_add_modal_content, model_drop_down, sheet_retriever,
    EditorState, GroupEditorContent, GroupViewItem, KeyedViewItem, ListEditorContent, PropertyGrid,
    ViewItem,
};

sheet_retriever!(Chart, ChartBook, chart_data, IndexMap<String, Vec<ChartData>>);

impl GroupViewItem for IndexMap<String, Vec<ChartData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for ChartData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies.person.read(|data| {
            data.get(&self.pid)
                .map(|p| Cow::Owned(p.text(dependencies).to_string()))
                .unwrap_or(Cow::Borrowed("{unknown person}"))
        })
    }

    // TODO: Decoration
}

sheet_retriever!(ChartGodData, ChartBook, chart_god_data, IndexMap<String, ChartGodData>);

impl ViewItem for ChartGodData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies.chapter.read(|data| {
            let chapter = "CID_".to_owned() + &self.chapter;
            data.get(&chapter)
                .map(|p| Cow::Owned(p.text(dependencies).to_string()))
                .unwrap_or(Cow::Borrowed("{unknown chapter}"))
        })
    }
}

impl KeyedViewItem for ChartGodData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.chapter)
    }

    fn set_key(&mut self, key: String) {
        self.chapter = key;
    }
}

sheet_retriever!(ChartParam, ChartBook, chart_params, IndexMap<String, Vec<ChartParam>>);

impl GroupViewItem for IndexMap<String, Vec<ChartParam>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, dependencies: &'a Self::Dependencies) -> Cow<'a, str> {
        dependencies.chapter.read(|data| {
            let chapter = "CID_".to_owned() + key;
            data.get(&chapter)
                .map(|p| Cow::Owned(p.text(dependencies).to_string()))
                .unwrap_or(Cow::Borrowed("{unknown chapter}"))
        })
    }
}

impl ViewItem for ChartParam {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Main,
    GodData,
    Params,
}

pub struct ChartEditor {
    tab: Tab,
    chart: ChartSheet,
    chart_god: ChartGodDataSheet,
    chart_param: ChartParamSheet,
    chart_content: GroupEditorContent,
    chart_god_content: ListEditorContent<IndexMap<String, ChartGodData>, ChartGodData, EditorState>,
    chart_param_content: GroupEditorContent,
}

impl ChartEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Main,
            chart: state.chart.clone(),
            chart_god: state.chart_god.clone(),
            chart_param: state.chart_param.clone(),
            chart_content: GroupEditorContent::new("chart_editor"),
            chart_god_content: ListEditorContent::new("chart_god_editor")
                .with_add_modal_content(keyed_add_modal_content),
            chart_param_content: GroupEditorContent::new("chart_param_editor"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Main, "Main");
            ui.selectable_value(&mut self.tab, Tab::GodData, "God Data");
            ui.selectable_value(&mut self.tab, Tab::Params, "Params");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::Main => {
                self.chart_content.left_panel(ctx, &self.chart, state);
                self.chart.write(|data| {
                    self.chart_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("chart", selection)
                            .new_section("")
                            .field("PID", |ui, d| {
                                state
                                    .person
                                    .read(|data| ui.add(model_drop_down(data, state, &mut d.pid)))
                            })
                            .default_field("Level N", |d| &mut d.level_n)
                            .default_field("Level H", |d| &mut d.level_h)
                            .default_field("Level L", |d| &mut d.level_l)
                            .field("Class", |ui, d| {
                                state
                                    .job
                                    .read(|data| ui.add(model_drop_down(data, state, &mut d.jid)))
                            })
                            .field("Item 1", |ui, d| {
                                state.item.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.item_1_iid))
                                })
                            })
                            .field("Item 2", |ui, d| {
                                state.item.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.item_2_iid))
                                })
                            })
                            .field("Item 3", |ui, d| {
                                state.item.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.item_3_iid))
                                })
                            })
                            .field("Item 4", |ui, d| {
                                state.item.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.item_4_iid))
                                })
                            })
                            .field("Item 5", |ui, d| {
                                state.item.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.item_5_iid))
                                })
                            })
                            .field("God", |ui, d| {
                                state.god.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.god_id))
                                })
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::GodData => {
                self.chart_god_content
                    .left_panel(ctx, &self.chart_god, state);
                self.chart_god.write(|data| {
                    self.chart_god_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("chart_god", selection)
                            .new_section("")
                            .field("Chapter", |ui, d| ui.add(id_field(&mut d.chapter)))
                            .default_field("Marth Level", |d| &mut d.marth_level)
                            .default_field("Siglud Level", |d| &mut d.siglud_level)
                            .default_field("Celica Level", |d| &mut d.celica_level)
                            .default_field("Micaiah Level", |d| &mut d.micaiah_level)
                            .default_field("Roy Level", |d| &mut d.roy_level)
                            .default_field("Leaf Level", |d| &mut d.leaf_level)
                            .default_field("Lucina Level", |d| &mut d.lucina_level)
                            .default_field("Lin Level", |d| &mut d.lin_level)
                            .default_field("Ike Level", |d| &mut d.ike_level)
                            .default_field("Byleth Level", |d| &mut d.byleth_level)
                            .default_field("Kamui Level", |d| &mut d.kamui_level)
                            .default_field("Eirik Level", |d| &mut d.eirik_level)
                            .default_field("Flag", |d| &mut d.flag)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::Params => {
                self.chart_param_content
                    .left_panel(ctx, &self.chart_param, state);
                self.chart_param.write(|data| {
                    self.chart_param_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("chart_param", selection)
                                .new_section("")
                                .default_field("Name", |d| &mut d.name)
                                .default_field("Value", |d| &mut d.value)
                                .show(ui)
                                .changed()
                        })
                })
            }
        }
    }
}
