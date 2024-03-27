use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, keyed_add_modal_content, model_drop_down, msbt_key_value_multiline,
    msbt_key_value_singleline, sheet_retriever, standard_keyed_display, EditorState, KeyedViewItem,
    ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{
    FishSizeData, FishSpawn, FishingAssistData, FishingFishBook, FishingFishData,
    FishingRadicalParamData, FishingTargetListData,
};

sheet_retriever!(FishingFishData, FishingFishBook, fish, IndexMap<String, FishingFishData>);

impl ViewItem for FishingFishData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, id, name_label)
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
            .get_fish(&self.texture_id)
            .map(|texture| (texture, 0.2))
    }
}

impl KeyedViewItem for FishingFishData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(FishSizeData, FishingFishBook, size_data, IndexMap<String, FishSizeData>);

impl ViewItem for FishSizeData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for FishSizeData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(FishSpawn, FishingFishBook, spawns, IndexMap<String, FishSpawn>);

impl ViewItem for FishSpawn {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for FishSpawn {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(FishingTargetListData, FishingFishBook, target_list, IndexMap<String, FishingTargetListData>);

impl ViewItem for FishingTargetListData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for FishingTargetListData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(FishingAssistData, FishingFishBook, assist_data, IndexMap<String, FishingAssistData>);

impl ViewItem for FishingAssistData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for FishingAssistData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(FishingRadicalParamData, FishingFishBook, radical_param_data, IndexMap<String, FishingRadicalParamData>);

impl ViewItem for FishingRadicalParamData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for FishingRadicalParamData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    FishingFishData,
    FishSizeData,
    FishSpawn,
    FishingTargetListData,
    FishingAssistData,
    FishingRadicalParamData,
}

pub struct FishingFishEditor {
    tab: Tab,
    fish: FishingFishDataSheet,
    size_data: FishSizeDataSheet,
    spawns: FishSpawnSheet,
    target_list: FishingTargetListDataSheet,
    assist_data: FishingAssistDataSheet,
    radical_param_data: FishingRadicalParamDataSheet,
    fish_content:
        ListEditorContent<IndexMap<String, FishingFishData>, FishingFishData, EditorState>,
    size_data_content: ListEditorContent<IndexMap<String, FishSizeData>, FishSizeData, EditorState>,
    spawns_content: ListEditorContent<IndexMap<String, FishSpawn>, FishSpawn, EditorState>,
    target_list_content: ListEditorContent<
        IndexMap<String, FishingTargetListData>,
        FishingTargetListData,
        EditorState,
    >,
    assist_data_content:
        ListEditorContent<IndexMap<String, FishingAssistData>, FishingAssistData, EditorState>,
    radical_param_data_content: ListEditorContent<
        IndexMap<String, FishingRadicalParamData>,
        FishingRadicalParamData,
        EditorState,
    >,
}

impl FishingFishEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::FishingFishData,
            fish: state.fishing_fish_data.clone(),
            size_data: state.fishing_size_data.clone(),
            spawns: state.fish_spawns.clone(),
            target_list: state.fishing_target_list.clone(),
            assist_data: state.fishing_assist_data.clone(),
            radical_param_data: state.fishing_radical_param_data.clone(),
            fish_content: ListEditorContent::new("fish_editor")
                .with_add_modal_content(keyed_add_modal_content),
            size_data_content: ListEditorContent::new("size_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            spawns_content: ListEditorContent::new("spawns_editor")
                .with_add_modal_content(keyed_add_modal_content),
            target_list_content: ListEditorContent::new("target_list_editor")
                .with_add_modal_content(keyed_add_modal_content),
            assist_data_content: ListEditorContent::new("assist_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            radical_param_data_content: ListEditorContent::new("radical_param_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::FishingFishData, "Fish");
            ui.selectable_value(&mut self.tab, Tab::FishSizeData, "Size");
            ui.selectable_value(&mut self.tab, Tab::FishSpawn, "Spawn");
            ui.selectable_value(&mut self.tab, Tab::FishingTargetListData, "Target List");
            ui.selectable_value(&mut self.tab, Tab::FishingAssistData, "Assist");
            ui.selectable_value(&mut self.tab, Tab::FishingRadicalParamData, "Radical Param");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::FishingFishData => {
                self.fish_content.left_panel(ctx, &self.fish, state);
                self.fish.write(|data| {
                    self.fish_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("fish", selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            // TODO: Set both fields when adding new fish
                            .field("Fish Name", |ui, d| ui.add(id_field(&mut d.fish_name)))
                            .default_field("Large Type", |d| &mut d.large_type)
                            .default_field("Shadow Size", |d| &mut d.shadow_size)
                            .default_field("Radar Size Mult", |d| &mut d.radar_size_mult)
                            .field("Item", |ui, d| {
                                state.item.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.food_type))
                                })
                            })
                            .default_field("Piece Count", |d| &mut d.piece_count)
                            .default_field("Counter Time", |d| &mut d.counter_time)
                            .default_field("Turn Counter Time", |d| &mut d.turn_counter_time)
                            .default_field("Turn Angle Min", |d| &mut d.turn_angle_min)
                            .default_field("Turn Angle Max", |d| &mut d.turn_angle_max)
                            .default_field("Escape Speed", |d| &mut d.escape_speed)
                            .default_field("Counter Speed H", |d| &mut d.counter_speed_h)
                            .default_field("Counter Speed M", |d| &mut d.counter_speed_m)
                            .default_field("Counter Speed L", |d| &mut d.counter_speed_l)
                            .default_field("Catch Time", |d| &mut d.catch_time)
                            .default_field("Catch Time Random Add", |d| {
                                &mut d.catch_time_random_add
                            })
                            .default_field("Escape Time", |d| &mut d.escape_time)
                            .default_field("Hp", |d| &mut d.hp)
                            .default_field("Lethal Hp", |d| &mut d.lethal_hp)
                            .default_field("Regenarate Per Sec", |d| &mut d.regenarate_per_sec)
                            .default_field("Base Size", |d| &mut d.base_size)
                            .field("Name", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "hub", d.name_label)
                            })
                            .field("Help", |ui, d| {
                                msbt_key_value_multiline!(ui, state, "hub", d.message_label)
                            })
                            .default_field("Time Flag Morning", |d| &mut d.time_flag_morning)
                            .default_field("Time Flag Day", |d| &mut d.time_flag_day)
                            .default_field("Time Flag Night", |d| &mut d.time_flag_night)
                            .default_field("Best Rod Type", |d| &mut d.best_rod_type)
                            .default_field("Texture Id", |d| &mut d.texture_id)
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::FishSizeData => {
                self.size_data_content
                    .left_panel(ctx, &self.size_data, state);
                self.size_data.write(|data| {
                    self.size_data_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("size_data", selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            .default_field("Size Name", |d| &mut d.size_name)
                            .default_field("Size Minimum", |d| &mut d.size_minimum)
                            .default_field("Size Maximum", |d| &mut d.size_maximum)
                            .default_field("Bonus Minimum", |d| &mut d.bonus_minimum)
                            .default_field("Bonus Maximum", |d| &mut d.bonus_maximum)
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::FishSpawn => {
                self.spawns_content.left_panel(ctx, &self.spawns, state);
                self.spawns.write(|data| {
                    self.spawns_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("spawns", selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            .default_field("Stick Type", |d| &mut d.stick_type)
                            .default_field("Time", |d| &mut d.time)
                            .default_field("Position Num", |d| &mut d.position_num)
                            .default_field("Lottery Param", |d| &mut d.lottery_param)
                            .default_field("Fish Id", |d| &mut d.fish_id)
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::FishingTargetListData => {
                self.target_list_content
                    .left_panel(ctx, &self.target_list, state);
                self.target_list.write(|data| {
                    self.target_list_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("target_list", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Fish Id", |d| &mut d.fish_id)
                                .default_field("Priority", |d| &mut d.priority)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::FishingAssistData => {
                self.assist_data_content
                    .left_panel(ctx, &self.assist_data, state);
                self.assist_data.write(|data| {
                    self.assist_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("assist_data", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Level 00", |d| &mut d.level_00)
                                .default_field("Level 01", |d| &mut d.level_01)
                                .default_field("Level 02", |d| &mut d.level_02)
                                .default_field("Level 03", |d| &mut d.level_03)
                                .default_field("Level 04", |d| &mut d.level_04)
                                .default_field("Level 05", |d| &mut d.level_05)
                                .default_field("Level 06", |d| &mut d.level_06)
                                .default_field("Level 07", |d| &mut d.level_07)
                                .default_field("Level 08", |d| &mut d.level_08)
                                .default_field("Level 09", |d| &mut d.level_09)
                                .default_field("Level 10", |d| &mut d.level_10)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::FishingRadicalParamData => {
                self.radical_param_data_content
                    .left_panel(ctx, &self.radical_param_data, state);
                self.radical_param_data.write(|data| {
                    self.radical_param_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("radical_param_data", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Sec 01", |d| &mut d.sec_01)
                                .default_field("Power 01", |d| &mut d.power_01)
                                .default_field("Regene 01", |d| &mut d.regene_01)
                                .default_field("Sec 02", |d| &mut d.sec_02)
                                .default_field("Power 02", |d| &mut d.power_02)
                                .default_field("Regene 02", |d| &mut d.regene_02)
                                .default_field("Sec 03", |d| &mut d.sec_03)
                                .default_field("Power 03", |d| &mut d.power_03)
                                .default_field("Regene 03", |d| &mut d.regene_03)
                                .default_field("Sec 04", |d| &mut d.sec_04)
                                .default_field("Power 04", |d| &mut d.power_04)
                                .default_field("Regene 04", |d| &mut d.regene_04)
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
