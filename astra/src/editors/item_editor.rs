use std::collections::HashMap;
use std::sync::LazyLock;

use astra_types::{Item, ItemBook, ItemInteractData};
use egui::{CentralPanel, Checkbox, Grid, ScrollArea, Ui};
use egui_extras::{Column, Size, TableBuilder};
use indexmap::IndexMap;

use crate::widgets::{
    editor_tab_strip, id_field, item_add_target_drop_down, item_weapon_attrs_drop_down,
    keyed_add_modal_content,
};
use crate::{
    bitgrid_i32, editable_list, item_kind_drop_down, item_use_type_drop_down, model_drop_down,
    msbt_key_value_multiline, msbt_key_value_singleline, optional_image, sheet_retriever,
    staff_type_drop_down, stat_column_headers_no_sight, stats_row_no_sight, weapon_rank_drop_down,
    CachedView, DecorationKind, EditorState, ItemSheet, ItemSheetRetriever, ListEditorContent,
    PropertyGrid, ViewItem,
};

const FLAG_LABELS: &[&str] = &[
    "Rare",
    "Cannot Trade",
    "Usable",
    "Chapter Only",
    "Enemy Only",
    "Male Only",
    "Female Only",
    "Engage",
    "Ignore Weapon Level",
    "Private",
    "Not Entrust",
    "Invert Weapon Triangle",
    "Download",
    "Door Key",
    "Chest Key",
    "AI Unequiable",
    "Reverse Attribute",
    "Lunch Box",
    "Simple Help",
    "Range Target",
    "Ignore Combat",
    "Force Combat",
    "Bless",
    "Breath",
    "Dragon",
    "Bullet",
];

static KNOWN_WEAPON_TYPES: LazyLock<IndexMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = IndexMap::new();
    map.insert("無し", "None");
    map.insert("剣", "Sword");
    map.insert("槍", "Lance");
    map.insert("斧", "Axe");
    map.insert("弓", "Bow");
    map.insert("短剣", "Dagger");
    map.insert("魔法", "Tome");
    map.insert("杖", "Staff");
    map.insert("拳", "Arts");
    map.insert("特殊", "Special");
    map.insert("反転無し", "None (R)");
    map.insert("反転剣", "Sword (R)");
    map.insert("反転槍", "Lance (R)");
    map.insert("反転斧", "Axe (R)");
    map.insert("反転弓", "Bow (R)");
    map.insert("反転短剣", "Dagger (R)");
    map.insert("反転魔法", "Tome (R)");
    map.insert("反転魔杖", "Staff (R)");
    map.insert("反転拳", "Arts (R)");
    map.insert("反転特殊", "Special (R)");
    map
});

sheet_retriever!(
    ItemInteractData,
    ItemBook,
    item_interact_data,
    Vec<ItemInteractData>
);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Items,
    WeaponInteractions,
}

pub struct ItemEditor {
    item: ItemSheet,
    interact_data: ItemInteractDataSheet,
    content: ListEditorContent<IndexMap<String, Item>, Item, EditorState>,
    cache: CachedView<ItemSheetRetriever, ItemBook, Item>,
    tab: Tab,
}

impl ItemEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            item: state.item.clone(),
            interact_data: state.item_interact_data.clone(),
            cache: CachedView::new(state.item.clone(), state),
            content: ListEditorContent::new("item_editor")
                .with_add_modal_content(keyed_add_modal_content),
            tab: Tab::Items,
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Items, "Items");
            ui.selectable_value(
                &mut self.tab,
                Tab::WeaponInteractions,
                "Weapon Interactions",
            );
        });
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.content.select(index);
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        match self.tab {
            Tab::Items => {
                self.content.left_panel(ctx, &self.item, state);
                self.cache.refresh(state);
                self.item.write(|data| {
                    self.content.content(ctx, data, |ui, item| {
                        let mut changed = false;
                        egui_grid::GridBuilder::new()
                            .new_row(Size::exact(150.))
                            .cell(Size::exact(375.))
                            .cell(Size::remainder())
                            .new_row(Size::remainder())
                            .cell(Size::remainder())
                            .show(ui, |mut grid| {
                                grid.cell(|ui| {
                                    ui.group(|ui| {
                                        ui.vertical_centered_justified(|ui| {
                                            ui.group(|ui| {
                                                ui.add(optional_image(
                                                    item.decoration(
                                                        state,
                                                        DecorationKind::Other("portrait"),
                                                    ),
                                                    [ui.available_width() - 12., 90.],
                                                ));
                                            });
                                            ui.add(id_field(&mut item.iid));
                                        });
                                        ui.allocate_space(ui.available_size());
                                    });
                                });
                                grid.cell(|ui| {
                                    ui.group(|ui| {
                                        Grid::new("item_grid").show(ui, |ui| {
                                            stat_column_headers_no_sight(ui);
                                            ui.label("Bonuses");
                                            stats_row_no_sight!(ui, item, enhance, changed);
                                            ui.label("Growth Ratios");
                                            stats_row_no_sight!(ui, item, grow_ratio, changed);
                                        });
                                        ui.allocate_space(ui.available_size());
                                    });
                                });
                                grid.cell(|ui| {
                                    ui.separator();
                                    changed |= PropertyGrid::new("item", item)
                                        .new_section("Core")
                                        .field("Name", |ui, item| {
                                            msbt_key_value_singleline!(ui, state, "item", item.name)
                                        })
                                        .field("Help", |ui, item| {
                                            msbt_key_value_multiline!(ui, state, "item", item.help)
                                        })
                                        .default_field("Icon", |item| &mut item.icon)
                                        .field("Kind", |ui, item| {
                                            ui.add(item_kind_drop_down(&mut item.kind))
                                        })
                                        .field("Use Type", |ui, item| {
                                            ui.add(item_use_type_drop_down(&mut item.use_type))
                                        })
                                        .default_field("High Rank Item", |item| {
                                            &mut item.high_rank_item
                                        })
                                        .default_field("Price", |item| &mut item.price)
                                        .field("Flags", |ui, item| {
                                            ui.add(bitgrid_i32(FLAG_LABELS, 3, &mut item.flag))
                                        })
                                        .new_section("Weapon Data")
                                        .default_field("Equip Condition", |item| {
                                            &mut item.equip_condition
                                        })
                                        .field("Weapon Rank", |ui, item| {
                                            ui.add(weapon_rank_drop_down(&mut item.weapon_level))
                                        })
                                        .field("Weapon Attr", |ui, item| {
                                            ui.add(item_weapon_attrs_drop_down(
                                                &mut item.weapon_attr,
                                            ))
                                        })
                                        .default_field("Uses", |item| &mut item.endurance)
                                        .default_field("Power", |item| &mut item.power)
                                        .default_field("Weight", |item| &mut item.weight)
                                        .default_field("Range (I)", |item| &mut item.range_i)
                                        .default_field("Range (O)", |item| &mut item.range_o)
                                        .default_field("Distance", |item| &mut item.distance)
                                        .default_field("Hit", |item| &mut item.hit)
                                        .default_field("Crit", |item| &mut item.critical)
                                        .default_field("Avoid", |item| &mut item.avoid)
                                        .default_field("Dodge", |item| &mut item.secure)
                                        .default_field("Arena Rate", |item| &mut item.rate_arena)
                                        .new_section("Staff Data")
                                        .field("Type", |ui, item| {
                                            ui.add(staff_type_drop_down(&mut item.rod_type))
                                        })
                                        .default_field("Exp", |item| &mut item.rod_exp)
                                        .new_section("Skills")
                                        .field("Equip", |ui, item| {
                                            state.skill.read(|data| {
                                                ui.add(editable_list(
                                                    &mut item.equip_sids,
                                                    |_, value, ui| {
                                                        ui.add(model_drop_down(data, state, value))
                                                    },
                                                ))
                                            })
                                        })
                                        .field("Passive", |ui, item| {
                                            state.skill.read(|data| {
                                                ui.add(editable_list(
                                                    &mut item.passive_sids,
                                                    |_, value, ui| {
                                                        ui.add(model_drop_down(data, state, value))
                                                    },
                                                ))
                                            })
                                        })
                                        .field("Give", |ui, item| {
                                            state.skill.read(|data| {
                                                ui.add(editable_list(
                                                    &mut item.give_sids,
                                                    |_, value, ui| {
                                                        ui.add(model_drop_down(data, state, value))
                                                    },
                                                ))
                                            })
                                        })
                                        .field("Add", |ui, item| {
                                            state.skill.read(|data| {
                                                ui.add(editable_list(
                                                    &mut item.add_sids,
                                                    |_, value, ui| {
                                                        ui.add(model_drop_down(data, state, value))
                                                    },
                                                ))
                                            })
                                        })
                                        .new_section("Other Effects")
                                        .field("Add Target", |ui, item| {
                                            ui.add(item_add_target_drop_down(&mut item.add_target))
                                        })
                                        .field("Add Type", |ui, item| {
                                            ui.add(item_use_type_drop_down(&mut item.add_type))
                                        })
                                        .default_field("Add Power", |item| &mut item.add_power)
                                        .default_field("Add Range", |item| &mut item.add_range)
                                        .default_field("Add Effect", |item| &mut item.add_effect)
                                        .field("Add Help", |ui, item| {
                                            ui.text_edit_singleline(&mut item.add_help)
                                            // TODO: Find the message archive for this key
                                        })
                                        .new_section("Visuals")
                                        .default_field("AID", |item| &mut item.aid)
                                        .default_field("Shoot Effect", |item| {
                                            &mut item.shoot_effect
                                        })
                                        .default_field("Hit Effect", |item| &mut item.hit_effect)
                                        .default_field("Cannon Effect", |item| {
                                            &mut item.cannon_effect
                                        })
                                        .default_field("Attack Motion", |item| {
                                            &mut item.attack_motion
                                        })
                                        .default_field("Overlap Terrain", |item| {
                                            &mut item.overlap_terrain
                                        })
                                        .new_section("Misc.")
                                        .field("Out", |ui, item| {
                                            ui.text_edit_singleline(&mut item.out)
                                        })
                                        .field("Tutorial", |ui, item| {
                                            msbt_key_value_multiline!(
                                                ui,
                                                state,
                                                "item",
                                                item.tutorial
                                            )
                                        })
                                        .show(ui)
                                        .changed();
                                });
                            });
                        changed
                    })
                });
            }
            Tab::WeaponInteractions => {
                CentralPanel::default().show(ctx, |ui| {
                    ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                        TableBuilder::new(ui)
                            .columns(Column::exact(60.), KNOWN_WEAPON_TYPES.len() + 1)
                            .header(20.0, |mut header| {
                                header.col(|_| {});
                                for column_name in KNOWN_WEAPON_TYPES.values() {
                                    header.col(|ui| {
                                        ui.label(*column_name);
                                    });
                                }
                            })
                            .body(|mut body| {
                                self.interact_data.write(|data| {
                                    let mut changed = false;
                                    for entry in data {
                                        let original_value = entry.flag;
                                        let mut new_value = entry.flag;
                                        let name = KNOWN_WEAPON_TYPES.get(entry.kind.as_str());
                                        body.row(20.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label(if let Some(name) = name {
                                                    *name
                                                } else {
                                                    entry.kind.as_str()
                                                });
                                            });
                                            for i in 0..KNOWN_WEAPON_TYPES.len() {
                                                row.col(|ui| {
                                                    let original_bit =
                                                        original_value & (1 << i) != 0;
                                                    let mut new_bit = original_bit;
                                                    ui.add(Checkbox::without_text(&mut new_bit));
                                                    if original_bit != new_bit {
                                                        new_value ^= 1 << i;
                                                    }
                                                });
                                            }
                                            if new_value != original_value {
                                                entry.flag = new_value;
                                                changed = true;
                                            }
                                        });
                                    }
                                    changed
                                });
                            });
                    })
                });
            }
        }
    }
}
