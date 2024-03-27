use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editable_list, editor_tab_strip, id_field, keyed_add_modal_content, model_drop_down,
    msbt_key_value_multiline, msbt_key_value_singleline, sheet_retriever, standard_keyed_display,
    DropDownModal, EditorState, GroupEditorContent, GroupViewItem, KeyedViewItem,
    ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{
    Item, RelayAwardData, RelayBook, RelayClearAwardData, RelayData, RelayStampData,
};

sheet_retriever!(RelayData, RelayBook, relay_data, IndexMap<String, RelayData>);

impl ViewItem for RelayData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .chapter
            .read(|data| {
                data.get(&self.cid)
                    .map(|chapter| chapter.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(if self.cid.is_empty() {
                "{unknown chapter}"
            } else {
                &self.cid
            }))
    }
}

impl KeyedViewItem for RelayData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.cid)
    }

    fn set_key(&mut self, key: String) {
        self.cid = key;
    }
}

sheet_retriever!(RelayStampData, RelayBook, relay_stamp_data, IndexMap<String, RelayStampData>);

impl ViewItem for RelayStampData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
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
            .get_relay_stamp(&self.name)
            .map(|texture| (texture, 0.5))
    }
}

impl KeyedViewItem for RelayStampData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    fn set_key(&mut self, key: String) {
        self.name = key;
    }
}

sheet_retriever!(RelayClearAwardData, RelayBook, relay_clear_award_data, IndexMap<String, Vec<RelayClearAwardData>>);

impl GroupViewItem for IndexMap<String, Vec<RelayClearAwardData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for RelayClearAwardData {
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

sheet_retriever!(RelayAwardData, RelayBook, relay_award_data, IndexMap<String, RelayAwardData>);

impl ViewItem for RelayAwardData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, raid, name)
    }
}

impl KeyedViewItem for RelayAwardData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.raid)
    }

    fn set_key(&mut self, key: String) {
        self.raid = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Relay,
    Stamp,
    ClearAward,
    Award,
}

pub struct RelayEditor {
    tab: Tab,
    relay_data: RelayDataSheet,
    relay_stamp_data: RelayStampDataSheet,
    relay_clear_award_data: RelayClearAwardDataSheet,
    relay_award_data: RelayAwardDataSheet,
    relay_data_content: ListEditorContent<IndexMap<String, RelayData>, RelayData, EditorState>,
    relay_stamp_data_content:
        ListEditorContent<IndexMap<String, RelayStampData>, RelayStampData, EditorState>,
    relay_clear_award_data_content: GroupEditorContent,
    relay_award_data_content:
        ListEditorContent<IndexMap<String, RelayAwardData>, RelayAwardData, EditorState>,
}

impl RelayEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Relay,
            relay_data: state.relay_data.clone(),
            relay_stamp_data: state.relay_stamp_data.clone(),
            relay_clear_award_data: state.relay_clear_award_data.clone(),
            relay_award_data: state.relay_award_data.clone(),
            relay_data_content: ListEditorContent::new("relay_data_editor")
                .with_add_modal_content(DropDownModal::new(state.chapter.clone())),
            relay_stamp_data_content: ListEditorContent::new("relay_stamp_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            relay_clear_award_data_content: GroupEditorContent::new(
                "relay_clear_award_data_editor",
            ),
            relay_award_data_content: ListEditorContent::new("relay_award_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Relay, "Relay");
            ui.selectable_value(&mut self.tab, Tab::Stamp, "Stamp");
            ui.selectable_value(&mut self.tab, Tab::ClearAward, "Clear Award");
            ui.selectable_value(&mut self.tab, Tab::Award, "Award");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::Relay => {
                self.relay_data_content
                    .left_panel(ctx, &self.relay_data, state);
                self.relay_data.write(|data| {
                    self.relay_data_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("relay_data", selection)
                            .new_section("")
                            .field("CID", |ui, d| ui.add(id_field(&mut d.cid)))
                            .default_field("Difficulty", |d| &mut d.difficulty)
                            .default_field("Max Turn", |d| &mut d.max_turn)
                            .default_field("Max Unit", |d| &mut d.max_unit)
                            .default_field("New Turn", |d| &mut d.new_turn)
                            .default_field("Take Over Turn", |d| &mut d.take_over_turn)
                            .default_field("Take Over Unit", |d| &mut d.take_over_unit)
                            .default_field("Completion Award Main", |d| {
                                &mut d.completion_award_main
                            })
                            .default_field("Completion Award Sub", |d| &mut d.completion_award_sub)
                            .field("Game Over Award", |ui, d| {
                                state.item.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.game_over_award))
                                })
                            })
                            .field("Unlock Chapter", |ui, d| {
                                state.chapter.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.unlock_cid))
                                })
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::Stamp => {
                self.relay_stamp_data_content
                    .left_panel(ctx, &self.relay_stamp_data, state);
                self.relay_stamp_data.write(|data| {
                    self.relay_stamp_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("relay_stamp_data", selection)
                                .new_section("")
                                .field("Name", |ui, d| ui.add(id_field(&mut d.name)))
                                .default_field("Serial No", |d| &mut d.serial_no)
                                .field("Person", |ui, d| {
                                    state.person.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.pid))
                                    })
                                })
                                .field("Emblem", |ui, d| {
                                    state.god.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.gid))
                                    })
                                })
                                .default_field("Sort", |d| &mut d.sort)
                                .default_field("Flag", |d| &mut d.flag)
                                .default_field("Voice", |d| &mut d.voice)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::ClearAward => {
                self.relay_clear_award_data_content.left_panel(
                    ctx,
                    &self.relay_clear_award_data,
                    state,
                );
                self.relay_clear_award_data.write(|data| {
                    self.relay_clear_award_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("relay_clear_award_data", selection)
                                .new_section("")
                                .field("Item", |ui, d| {
                                    state.item.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.iid))
                                    })
                                })
                                .default_field("Rate", |d| &mut d.rate)
                                .default_field("Min Count", |d| &mut d.min_count)
                                .default_field("Max Count", |d| &mut d.max_count)
                                .default_field("Flag", |d| &mut d.flag)
                                .default_field("Condition", |d| &mut d.condition)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::Award => {
                self.relay_award_data_content
                    .left_panel(ctx, &self.relay_award_data, state);
                self.relay_award_data.write(|data| {
                    self.relay_award_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("relay_award_data", selection)
                                .new_section("")
                                .field("RAID", |ui, d| ui.add(id_field(&mut d.raid)))
                                .field("Name", |ui, d| {
                                    msbt_key_value_singleline!(ui, state, "network", d.name)
                                })
                                .field("Result Text", |ui, d| {
                                    msbt_key_value_multiline!(ui, state, "network", d.result_text)
                                })
                                .field("Awards", |ui, d| {
                                    state.item.read(|data| {
                                        ui.add(editable_list(&mut d.awards, |_, d, ui| {
                                            ui.add(model_drop_down(data, state, d))
                                        }))
                                    })
                                })
                                .default_field("Flag", |d| &mut d.flag)
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
