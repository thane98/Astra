use astra_types::{Skill, SkillBook};
use egui::{TextEdit, Ui};
use indexmap::IndexMap;

use crate::widgets::keyed_add_modal_content;
use crate::{
    editable_list, i8_drag, model_drop_down, msbt_key_value_multiline, msbt_key_value_singleline,
    optional_image, CachedView, DecorationKind, DefaultWidget, EditorState, ListEditorContent,
    PropertyGrid, SkillSheet, SkillSheetRetriever, ViewItem,
};

pub struct SkillEditor {
    skill: SkillSheet,
    content: ListEditorContent<IndexMap<String, Skill>, Skill>,
    cache: CachedView<SkillSheetRetriever, SkillBook, Skill>,
}

impl SkillEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            skill: state.skill.clone(),
            cache: CachedView::new(state.skill.clone(), state),
            content: ListEditorContent::new("skill_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        self.content.side_panel(ctx, &self.skill, state);

        self.cache.refresh(state);

        self.skill.write(|data| {
            self.content.content(ctx, data, |ui, skill| {
                let mut changed = false;
                ui.add_sized([200., 0.], |ui: &mut Ui| {
                    ui.group(|ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.group(|ui| {
                                ui.add(optional_image(
                                    skill.decoration(state, DecorationKind::Other("portrait")),
                                    [ui.available_width() - 12., 40.],
                                ));
                            });
                            ui.add_enabled(false, TextEdit::singleline(&mut skill.sid));
                        });
                    })
                    .response
                });

                changed |= PropertyGrid::new("skill", skill)
                    .new_section("Core")
                    .field("Name", |ui, skill| {
                        msbt_key_value_singleline!(ui, state, "skill", skill.name)
                    })
                    .field("Help", |ui, skill| {
                        msbt_key_value_multiline!(ui, state, "skill", skill.help)
                    })
                    .field("Condition", |ui, skill| {
                        ui.vertical_centered_justified(|ui| {
                            (&mut skill.condition).default_widget(ui)
                        })
                        .inner
                    })
                    .default_field("Cost", |skill| &mut skill.cost)
                    .default_field("Priority", |skill| &mut skill.priority)
                    .default_field("Layer", |skill| &mut skill.layer)
                    .default_field("Order", |skill| &mut skill.order)
                    .default_field("Cycle", |skill| &mut skill.cycle)
                    .default_field("Life", |skill| &mut skill.life)
                    .default_field("Timing", |skill| &mut skill.timing)
                    .default_field("Target", |skill| &mut skill.target)
                    .default_field("Frequency", |skill| &mut skill.frequency)
                    .default_field("Stand", |skill| &mut skill.stand)
                    .default_field("Action", |skill| &mut skill.action)
                    // TODO: FLAG
                    .default_field("Prohibit Weapons", |skill| &mut skill.weapon_prohibit)
                    .default_field("Attack Range", |skill| &mut skill.attack_range)
                    .default_field("Power", |skill| &mut skill.power)
                    .default_field("Rewarp", |skill| &mut skill.rewarp)
                    .default_field("Removable", |skill| &mut skill.removable)
                    .default_field("Vision Count", |skill| &mut skill.vision_count)
                    .default_field("Effect", |skill| &mut skill.effect)
                    .field("Equip Items", |ui, skill| {
                        state.item.read(|data| {
                            ui.add(editable_list(&mut skill.equip_iids, |_, value, ui| {
                                ui.add(model_drop_down(data, state, value))
                            }))
                        })
                    })
                    .new_section("Associated Skills")
                    .field("Infantry", |ui, skill| {
                        ui.add(model_drop_down(
                            self.cache.get(),
                            &(),
                            &mut skill.cooperation_skill,
                        ))
                    })
                    .field("Mounted", |ui, skill| {
                        ui.add(model_drop_down(
                            self.cache.get(),
                            &(),
                            &mut skill.horse_skill,
                        ))
                    })
                    .field("Covert", |ui, skill| {
                        ui.add(model_drop_down(
                            self.cache.get(),
                            &(),
                            &mut skill.covert_skill,
                        ))
                    })
                    .field("Armored", |ui, skill| {
                        ui.add(model_drop_down(
                            self.cache.get(),
                            &(),
                            &mut skill.heavy_skill,
                        ))
                    })
                    .field("Flier", |ui, skill| {
                        ui.add(model_drop_down(self.cache.get(), &(), &mut skill.fly_skill))
                    })
                    .field("Magic", |ui, skill| {
                        ui.add(model_drop_down(
                            self.cache.get(),
                            &(),
                            &mut skill.magic_skill,
                        ))
                    })
                    .field("Monk", |ui, skill| {
                        ui.add(model_drop_down(
                            self.cache.get(),
                            &(),
                            &mut skill.prana_skill,
                        ))
                    })
                    .field("Dragon", |ui, skill| {
                        ui.add(model_drop_down(
                            self.cache.get(),
                            &(),
                            &mut skill.dragon_skill,
                        ))
                    })
                    .new_section("Icon")
                    .default_field("Kind", |skill| &mut skill.icon_kind)
                    .default_field("Label", |skill| &mut skill.icon_label)
                    .default_field("BMap", |skill| &mut skill.icon_bmap)
                    .new_section("Stat Bonuses")
                    .default_field("Level", |skill| &mut skill.enhance_level)
                    .default_field("HP", |skill| &mut skill.enhance_value_hp)
                    .default_field("Str", |skill| &mut skill.enhance_value_str)
                    .default_field("Def", |skill| &mut skill.enhance_value_def)
                    .default_field("Skl", |skill| &mut skill.enhance_value_tech)
                    .default_field("Spd", |skill| &mut skill.enhance_value_quick)
                    .default_field("Mag", |skill| &mut skill.enhance_value_magic)
                    .default_field("Res", |skill| &mut skill.enhance_value_mdef)
                    .default_field("Con", |skill| &mut skill.enhance_value_phys)
                    .default_field("Mov", |skill| &mut skill.enhance_value_move)
                    .new_section("Weapon Levels")
                    .field("None", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_none))
                    })
                    .field("Sword", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_sword))
                    })
                    .field("Axe", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_axe))
                    })
                    .field("Lance", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_lance))
                    })
                    .field("Bow", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_bow))
                    })
                    .field("Dagger", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_dagger))
                    })
                    .field("Magic", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_magic))
                    })
                    .field("Staff", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_rod))
                    })
                    .field("Fist", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_fist))
                    })
                    .field("Special", |ui, skill| {
                        ui.add(i8_drag(&mut skill.weapon_level_special))
                    })
                    .new_section("Command")
                    // TODO
                    .new_section("Act")
                    .field("Names", |ui, skill| {
                        ui.add(editable_list(&mut skill.act_names, |_, value, ui| {
                            ui.text_edit_singleline(value)
                        }))
                    })
                    .field("Operations", |ui, skill| {
                        ui.add(editable_list(&mut skill.act_operations, |_, value, ui| {
                            ui.text_edit_singleline(value)
                        }))
                    })
                    .field("Values", |ui, skill| {
                        ui.add(editable_list(&mut skill.act_values, |_, value, ui| {
                            ui.text_edit_singleline(value)
                        }))
                    })
                    .new_section("Around")
                    .default_field("Condition", |skill| &mut skill.around_condition)
                    .default_field("Name", |skill| &mut skill.around_name)
                    .default_field("Operation", |skill| &mut skill.around_operation)
                    .default_field("Value", |skill| &mut skill.around_value)
                    .default_field("Center", |skill| &mut skill.around_center)
                    .default_field("Target", |skill| &mut skill.around_target)
                    .new_section("Give")
                    .default_field("Target", |skill| &mut skill.give_target)
                    .default_field("Condition", |skill| &mut skill.give_condition)
                    .field("Skills", |ui, skill| {
                        ui.add(editable_list(&mut skill.act_values, |_, value, ui| {
                            ui.add(model_drop_down(self.cache.get(), &(), value))
                        }))
                    })
                    .new_section("Overlap")
                    .default_field("Range", |skill| &mut skill.overlap_range)
                    .default_field("Terrain", |skill| &mut skill.overlap_terrain)
                    .new_section("ZOC")
                    .default_field("Range", |skill| &mut skill.zoc_range)
                    .default_field("Type", |skill| &mut skill.zoc_type)
                    .new_section("Work")
                    .default_field("Type", |skill| &mut skill.work)
                    .default_field("Operation", |skill| &mut skill.work_operation)
                    .default_field("Value", |skill| &mut skill.work_value)
                    .new_section("Move")
                    .default_field("Self", |skill| &mut skill.move_self)
                    .default_field("Target", |skill| &mut skill.move_target)
                    .new_section("Range")
                    .default_field("Target", |skill| &mut skill.range_target)
                    .default_field("Inner", |skill| &mut skill.range_i)
                    .default_field("Outer", |skill| &mut skill.range_o)
                    .default_field("Add", |skill| &mut skill.range_add)
                    .default_field("Extend", |skill| &mut skill.range_extend)
                    .new_section("Efficacy")
                    .default_field("Efficacy", |skill| &mut skill.efficacy)
                    .default_field("Value", |skill| &mut skill.efficacy_value)
                    .default_field("Ignore", |skill| &mut skill.efficacy_ignore)
                    .new_section("Bad")
                    .default_field("State", |skill| &mut skill.bad_state)
                    .default_field("Ignore", |skill| &mut skill.bad_ignore)
                    .new_section("Inheritance")
                    .default_field("Cost", |skill| &mut skill.inheritance_cost)
                    .default_field("Sort", |skill| &mut skill.inheritance_sort)
                    .show(ui)
                    .changed();

                changed
            })
        });
    }
}
