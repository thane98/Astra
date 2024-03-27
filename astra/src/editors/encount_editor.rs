use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editable_list, editor_tab_strip, keyed_add_modal_content, model_drop_down, sheet_retriever,
    EditorState, GroupEditorContent, GroupViewItem, KeyedViewItem, ListEditorContent, PropertyGrid,
    ViewItem,
};

use astra_types::{
    EncountBook, EncountEnemyType, EncountEquipment, EncountRarityConfig, EncountWeaponCategory,
    Item,
};

sheet_retriever!(EncountEquipment, EncountBook, encount_equipment, IndexMap<String, Vec<EncountEquipment>>);

impl GroupViewItem for IndexMap<String, Vec<EncountEquipment>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for EncountEquipment {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.category)
    }
}

sheet_retriever!(EncountWeaponCategory, EncountBook, encount_weapon_categories, IndexMap<String, Vec<EncountWeaponCategory>>);

impl GroupViewItem for IndexMap<String, Vec<EncountWeaponCategory>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for EncountWeaponCategory {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .item
            .read(|data| {
                data.get(&self.iid)
                    .map(|item| item.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed("{unknown item}"))
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

sheet_retriever!(EncountEnemyType, EncountBook, encount_enemy_types, IndexMap<String, EncountEnemyType>);

impl ViewItem for EncountEnemyType {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        self.key()
    }
}

impl KeyedViewItem for EncountEnemyType {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.e_jid)
    }

    fn set_key(&mut self, key: String) {
        self.e_jid = key;
    }
}

sheet_retriever!(EncountRarityConfig, EncountBook, encount_rarity_configs, IndexMap<String, Vec<EncountRarityConfig>>);

impl GroupViewItem for IndexMap<String, Vec<EncountRarityConfig>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for EncountRarityConfig {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("Level {}", self.nation_level.unwrap_or_default()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Equipment,
    WeaponCategory,
    EnemyType,
    RarityConfig,
}

pub struct EncountEditor {
    tab: Tab,
    encount_equipment: EncountEquipmentSheet,
    encount_weapon_categories: EncountWeaponCategorySheet,
    encount_enemy_types: EncountEnemyTypeSheet,
    encount_rarity_configs: EncountRarityConfigSheet,
    encount_equipment_content: GroupEditorContent,
    encount_weapon_categories_content: GroupEditorContent,
    encount_enemy_types_content:
        ListEditorContent<IndexMap<String, EncountEnemyType>, EncountEnemyType, EditorState>,
    encount_rarity_configs_content: GroupEditorContent,
}

impl EncountEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Equipment,
            encount_equipment: state.encount_equipment.clone(),
            encount_weapon_categories: state.encount_weapon_categories.clone(),
            encount_enemy_types: state.encount_enemy_types.clone(),
            encount_rarity_configs: state.encount_rarity_configs.clone(),
            encount_equipment_content: GroupEditorContent::new("encount_equipment_editor"),
            encount_weapon_categories_content: GroupEditorContent::new(
                "encount_weapon_categories_editor",
            ),
            encount_enemy_types_content: ListEditorContent::new("encount_enemy_types_editor")
                .with_add_modal_content(keyed_add_modal_content),
            encount_rarity_configs_content: GroupEditorContent::new(
                "encount_rarity_configs_editor",
            ),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Equipment, "Equipment");
            ui.selectable_value(&mut self.tab, Tab::WeaponCategory, "Weapon");
            ui.selectable_value(&mut self.tab, Tab::EnemyType, "EnemyType");
            ui.selectable_value(&mut self.tab, Tab::RarityConfig, "Rarity");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::Equipment => {
                self.encount_equipment_content
                    .left_panel(ctx, &self.encount_equipment, state);
                self.encount_equipment.write(|data| {
                    self.encount_equipment_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("encount_equipment", selection)
                                .new_section("")
                                .default_field("Category", |d| &mut d.category)
                                .default_field("Percentage", |d| &mut d.percentage)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::WeaponCategory => {
                self.encount_weapon_categories_content.left_panel(
                    ctx,
                    &self.encount_weapon_categories,
                    state,
                );
                self.encount_weapon_categories.write(|data| {
                    self.encount_weapon_categories_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("encount_weapon_categories", selection)
                                .new_section("")
                                .field("Item", |ui, d| {
                                    state.item.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.iid))
                                    })
                                })
                                .default_field("Rank Condition More", |d| {
                                    &mut d.rank_condition_more
                                })
                                .default_field("Rank Condition Less", |d| {
                                    &mut d.rank_condition_less
                                })
                                .default_field("Percentage", |d| &mut d.percentage)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::EnemyType => {
                self.encount_enemy_types_content
                    .left_panel(ctx, &self.encount_enemy_types, state);
                self.encount_enemy_types.write(|data| {
                    self.encount_enemy_types_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("encount_enemy_types", selection)
                                .new_section("")
                                .default_field("E Jid", |d| &mut d.e_jid)
                                .field("Classes", |ui, d| {
                                    state.job.read(|data| {
                                        ui.add(editable_list(&mut d.jobs, |_, d, ui| {
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

            Tab::RarityConfig => {
                self.encount_rarity_configs_content.left_panel(
                    ctx,
                    &self.encount_rarity_configs,
                    state,
                );
                self.encount_rarity_configs.write(|data| {
                    self.encount_rarity_configs_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("encount_rarity_configs", selection)
                                .new_section("")
                                .default_field("Nation Level", |d| &mut d.nation_level)
                                .field("Item", |ui, d| {
                                    state.item.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.iid))
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
