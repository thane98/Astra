use astra_types::{Item, ItemBook};
use egui::{Grid, Ui};
use indexmap::IndexMap;

use crate::widgets::{keyed_add_modal_content, id_field};
use crate::{
    bitgrid_i32, editable_list, item_kind_drop_down, item_use_type_drop_down, model_drop_down,
    msbt_key_value_multiline, msbt_key_value_singleline, optional_image, staff_type_drop_down,
    stat_column_headers_no_sight, stats_row_no_sight, weapon_rank_drop_down, CachedView,
    DecorationKind, EditorState, ItemSheet, ItemSheetRetriever, ListEditorContent, PropertyGrid,
    ViewItem,
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

pub struct ItemEditor {
    item: ItemSheet,
    content: ListEditorContent<IndexMap<String, Item>, Item>,
    cache: CachedView<ItemSheetRetriever, ItemBook, Item>,
}

impl ItemEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            item: state.item.clone(),
            cache: CachedView::new(state.item.clone(), state),
            content: ListEditorContent::new("item_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        self.content.side_panel(ctx, &self.item, state);

        self.cache.refresh(state);

        self.item.write(|data| {
            self.content.content(ctx, data, |ui, item| {
                let mut changed = false;
                ui.horizontal_top(|ui| {
                    ui.add_sized([250., 0.], |ui: &mut Ui| {
                        ui.group(|ui| {
                            ui.vertical_centered_justified(|ui| {
                                ui.group(|ui| {
                                    ui.add(optional_image(
                                        item.decoration(state, DecorationKind::Other("portrait")),
                                        [ui.available_width() - 12., 90.],
                                    ));
                                });
                                ui.add(id_field(&mut item.iid));
                            });
                            ui.label("");
                        })
                        .response
                    });
                    ui.group(|ui| {
                        Grid::new("item_grid").show(ui, |ui| {
                            stat_column_headers_no_sight(ui);
                            ui.label("Bonuses");
                            stats_row_no_sight!(ui, item, enhance, changed);
                            ui.label("Growth Ratios");
                            stats_row_no_sight!(ui, item, grow_ratio, changed);
                        });
                    });
                });

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
                    .default_field("High Rank Item", |item| &mut item.high_rank_item)
                    .default_field("Price", |item| &mut item.price)
                    .field("Flags", |ui, item| {
                        ui.add(bitgrid_i32(FLAG_LABELS, 3, &mut item.flag))
                    })
                    .new_section("Weapon Data")
                    .default_field("Equip Condition", |item| &mut item.equip_condition)
                    .field("Weapon Rank", |ui, item| {
                        ui.add(weapon_rank_drop_down(&mut item.weapon_level))
                    })
                    .default_field("Weapon Attr", |item| &mut item.weapon_attr)
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
                            ui.add(editable_list(&mut item.equip_sids, |_, value, ui| {
                                ui.add(model_drop_down(data, state, value))
                            }))
                        })
                    })
                    .field("Passive", |ui, item| {
                        state.skill.read(|data| {
                            ui.add(editable_list(&mut item.passive_sids, |_, value, ui| {
                                ui.add(model_drop_down(data, state, value))
                            }))
                        })
                    })
                    .field("Give", |ui, item| {
                        state.skill.read(|data| {
                            ui.add(editable_list(&mut item.give_sids, |_, value, ui| {
                                ui.add(model_drop_down(data, state, value))
                            }))
                        })
                    })
                    .field("Add", |ui, item| {
                        ui.add(editable_list(&mut item.add_sids, |_, value, ui| {
                            ui.add(model_drop_down(self.cache.get(), &(), value))
                        }))
                    })
                    .new_section("Other Effects")
                    .default_field("Add Target", |item| &mut item.add_target)
                    .default_field("Add Type", |item| &mut item.add_type)
                    .default_field("Add Power", |item| &mut item.add_power)
                    .default_field("Add Effect", |item| &mut item.add_effect)
                    .field("Add Help", |ui, item| {
                        ui.text_edit_singleline(&mut item.add_help) // TODO: Find the message archive for this key
                    })
                    .new_section("Visuals")
                    .default_field("AID", |item| &mut item.aid)
                    .default_field("Shoot Effect", |item| &mut item.shoot_effect)
                    .default_field("Hit Effect", |item| &mut item.hit_effect)
                    .default_field("Cannon Effect", |item| &mut item.cannon_effect)
                    .default_field("Attack Motion", |item| &mut item.attack_motion)
                    .default_field("Overlap Terrain", |item| &mut item.overlap_terrain)
                    .new_section("Misc.")
                    .field("Out", |ui, item| ui.text_edit_singleline(&mut item.out))
                    .field("Tutorial", |ui, item| {
                        msbt_key_value_multiline!(ui, state, "item", item.tutorial)
                    })
                    .show(ui)
                    .changed();

                changed
            })
        });
    }
}
