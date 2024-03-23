use std::borrow::Cow;

use astra_types::{
    DragonRidePresetParamData, DragonRidePresetParamDataBook, DragonRidePrizeData,
    DragonRidePrizeListBook, DragonRideTargetPattern, DragonRideTargetPatternBook,
};
use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, sheet_retriever, EditorState, GroupEditorContent,
    GroupViewItem, KeyedViewItem, ListEditorContent, ModelDropDown, PropertyGrid, ViewItem,
};

sheet_retriever!(
    DragonRidePresetParam,
    DragonRidePresetParamDataBook,
    dragon_ride_preset_param_data,
    IndexMap<String, DragonRidePresetParamData>
);

impl ViewItem for DragonRidePresetParamData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.group)
    }
}

impl KeyedViewItem for DragonRidePresetParamData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.group)
    }

    fn set_key(&mut self, key: String) {
        self.group = key;
    }
}

sheet_retriever!(
    DragonRidePrize,
    DragonRidePrizeListBook,
    dragon_ride_prize_data,
    IndexMap<String, DragonRidePrizeData>
);

impl ViewItem for DragonRidePrizeData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.group)
    }
}

impl KeyedViewItem for DragonRidePrizeData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.group)
    }

    fn set_key(&mut self, key: String) {
        self.group = key;
    }
}

sheet_retriever!(
    DragonRideTargetPattern,
    DragonRideTargetPatternBook,
    dragon_ride_target_patterns,
    IndexMap<String, Vec<DragonRideTargetPattern>>
);

impl GroupViewItem for IndexMap<String, Vec<DragonRideTargetPattern>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for DragonRideTargetPattern {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!(
            "{} {} {} {} {} {} {} {}",
            self.target_1.unwrap_or_default(),
            self.target_2.unwrap_or_default(),
            self.target_3.unwrap_or_default(),
            self.target_4.unwrap_or_default(),
            self.target_5.unwrap_or_default(),
            self.target_6.unwrap_or_default(),
            self.target_7.unwrap_or_default(),
            self.target_8.unwrap_or_default()
        ))
    }
}

// TODO: Move to a util file
fn item_key_transform(key: &str) -> String {
    let mut id = String::from("IID_");
    id.push_str(key);
    id
}

fn item_key_reverse_transform(key: &str) -> String {
    key.trim_start_matches("IID_").to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Presets,
    Prizes,
    TargetPatterns,
}

pub struct DragonRideEditor {
    tab: Tab,
    preset_params: DragonRidePresetParamSheet,
    prizes: DragonRidePrizeSheet,
    target_patterns: DragonRideTargetPatternSheet,
    preset_params_content:
        ListEditorContent<IndexMap<String, DragonRidePresetParamData>, DragonRidePresetParamData>,
    prizes_content: ListEditorContent<IndexMap<String, DragonRidePrizeData>, DragonRidePrizeData>,
    target_patterns_content: GroupEditorContent,
}

impl DragonRideEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Presets,
            preset_params: state.dragon_ride_presets.clone(),
            prizes: state.dragon_ride_prizes.clone(),
            target_patterns: state.dragon_ride_target_patterns.clone(),
            preset_params_content: ListEditorContent::new("dragon_ride_preset_params"),
            prizes_content: ListEditorContent::new("dragon_ride_prizes"),
            target_patterns_content: GroupEditorContent::new("dragon_ride_target_patterns"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Presets, "Presets");
            ui.selectable_value(&mut self.tab, Tab::Prizes, "Prizes");
            ui.selectable_value(&mut self.tab, Tab::TargetPatterns, "Target Patterns");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::Presets => {
                self.preset_params_content
                    .left_panel(ctx, &self.preset_params, state);
                self.preset_params.write(|data| {
                    self.preset_params_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("dragon_ridepreset_params", selection)
                                .new_section("")
                                .field("Group", |ui, d| ui.add(id_field(&mut d.group)))
                                .default_field("Is Time Test", |d| &mut d.is_time_test)
                                .default_field("Is Walk Through On", |d| &mut d.is_walk_through_on)
                                .default_field("Course 1", |d| &mut d.course_1)
                                .default_field("Stime 1", |d| &mut d.stime_1)
                                .default_field("Srandom 1", |d| &mut d.srandom_1)
                                .default_field("Course 2", |d| &mut d.course_2)
                                .default_field("Stime 2", |d| &mut d.stime_2)
                                .default_field("Srandom 2", |d| &mut d.srandom_2)
                                .default_field("Course 3", |d| &mut d.course_3)
                                .default_field("Stime 3", |d| &mut d.stime_3)
                                .default_field("Srandom 3", |d| &mut d.srandom_3)
                                .default_field("Course 4", |d| &mut d.course_4)
                                .default_field("Stime 4", |d| &mut d.stime_4)
                                .default_field("Srandom 4", |d| &mut d.srandom_4)
                                .default_field("Course 5", |d| &mut d.course_5)
                                .default_field("Stime 5", |d| &mut d.stime_5)
                                .default_field("Srandom 5", |d| &mut d.srandom_5)
                                .default_field("Course 6", |d| &mut d.course_6)
                                .default_field("Stime 6", |d| &mut d.stime_6)
                                .default_field("Srandom 6", |d| &mut d.srandom_6)
                                .default_field("Course 7", |d| &mut d.course_7)
                                .default_field("Stime 7", |d| &mut d.stime_7)
                                .default_field("Srandom 7", |d| &mut d.srandom_7)
                                .default_field("Course 8", |d| &mut d.course_8)
                                .default_field("Stime 8", |d| &mut d.stime_8)
                                .default_field("Srandom 8", |d| &mut d.srandom_8)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Prizes => {
                self.prizes_content.left_panel(ctx, &self.prizes, state);
                self.prizes.write(|data| {
                    self.prizes_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("dragon_ride_prizes", selection)
                            .new_section("")
                            .field("Group", |ui, d| ui.add(id_field(&mut d.group)))
                            .default_field("Bond Fragments", |d| &mut d.piece_of_bond)
                            .default_field("Item Count", |d| &mut d.item_count)
                            .field("Item 1", |ui, d| {
                                state.item.read(|data| {
                                    ModelDropDown::default()
                                        .transform(&item_key_transform, &item_key_reverse_transform)
                                        .show(ui, data, state, &mut d.item_1)
                                })
                            })
                            .field("Item 2", |ui, d| {
                                state.item.read(|data| {
                                    ModelDropDown::default()
                                        .transform(&item_key_transform, &item_key_reverse_transform)
                                        .show(ui, data, state, &mut d.item_2)
                                })
                            })
                            .field("Item 3", |ui, d| {
                                state.item.read(|data| {
                                    ModelDropDown::default()
                                        .transform(&item_key_transform, &item_key_reverse_transform)
                                        .show(ui, data, state, &mut d.item_3)
                                })
                            })
                            .field("Item 4", |ui, d| {
                                state.item.read(|data| {
                                    ModelDropDown::default()
                                        .transform(&item_key_transform, &item_key_reverse_transform)
                                        .show(ui, data, state, &mut d.item_4)
                                })
                            })
                            .field("Item 5", |ui, d| {
                                state.item.read(|data| {
                                    ModelDropDown::default()
                                        .transform(&item_key_transform, &item_key_reverse_transform)
                                        .show(ui, data, state, &mut d.item_5)
                                })
                            })
                            .field("Item 6", |ui, d| {
                                state.item.read(|data| {
                                    ModelDropDown::default()
                                        .transform(&item_key_transform, &item_key_reverse_transform)
                                        .show(ui, data, state, &mut d.item_6)
                                })
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::TargetPatterns => {
                self.target_patterns_content
                    .left_panel(ctx, &self.target_patterns, state);
                self.target_patterns.write(|data| {
                    self.target_patterns_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("dragon_ride_target_patterns", selection)
                                .new_section("")
                                .default_field("Target 1", |d| &mut d.target_1)
                                .default_field("Target 2", |d| &mut d.target_2)
                                .default_field("Target 3", |d| &mut d.target_3)
                                .default_field("Target 4", |d| &mut d.target_4)
                                .default_field("Target 5", |d| &mut d.target_5)
                                .default_field("Target 6", |d| &mut d.target_6)
                                .default_field("Target 7", |d| &mut d.target_7)
                                .default_field("Target 8", |d| &mut d.target_8)
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
