use astra_types::{Skill, SkillBook};
use indexmap::IndexMap;

use crate::widgets::{
    bitgrid_i32, bitgrid_u64, id_field, keyed_add_modal_content, skill_around_centers_drop_down,
    skill_around_targets_drop_down, skill_cycle_drop_down, skill_frequencies_drop_down,
    skill_give_targets_drop_down, skill_stance_drop_down, skill_targets_drop_down,
    skill_timing_drop_down, u8_drag, weapon_rank_numbered_drop_down,
};
use crate::{
    editable_list, model_drop_down, msbt_key_value_multiline, msbt_key_value_singleline,
    CachedView, DefaultWidget, EditorState, ListEditorContent, PropertyGrid, SkillSheet,
    SkillSheetRetriever,
};

const WEAPON_LABELS: &[&str] = &[
    "None", "Swords", "Lances", "Axes", "Bows", "Daggers", "Magic", "Staves", "Fists",
];

const STATE_FLAG_LABELS: &[&str] = &[
    "Poison",
    "Deadly Poison",
    "Severe Poison",
    "Heal",
    "Sleep",
    "Silence",
    "Charm",
    "Confusion",
    "Freeze",
    "Weakness",
    "Stun",
    "Interact",
    "Decoy",
    "Not Enhance",
    "Enhance",
    "Immovable",
    "Not Move",
    "Not Weapon Weight",
    "Not Chain Attack",
];

const SKILL_FLAG_LABELS: &[&str] = &[
    "Invisible",
    "Engage Attack",
    "Engage Charge",
    "Engage Link",
    "Engage Wait",
    "Engage Summon",
    "Ignore Engage Attack",
    "Ignore No Engage Attack",
    "Enable Chaining",
    "Enable Destroy",
    "Enable Cannon",
    "Enable Rod",
    "Ignore Alone",
    "Ignore Multi Attacking",
    "Ignore Training",
    "Ignore Trial",
    "Ignore Sim",
    "Exclusive Dance",
    "Revenge Auto Equip",
    "Swap Order",
    "Interrupt Order",
    "Continue Battle",
    "Force Late Order",
    "Each Support",
    "Reactable",
    "Remagicable",
    "Before Move",
    "Allow Chain Attack",
    "Allow Chain Guard",
    "Allow Engage Guard",
    "Force Chain Attack",
    "Join Chain Attack",
    "Range Reliance",
    "Pickup Reliance",
    "Move Cost Free",
    "Move Enemy Pass",
    "Reset Disorder",
    "Item Heal Around",
    "Item Heal Give",
    "Self Heal Rod",
    "Only Recover Rod",
    "Decay Enhance",
    "Sub Engage Count Limit",
    "Reverse Count",
    "Re Cooking",
    "Basis Skill",
    "Unstoppable",
    "Hide Change God",
    "Over Exp Change",
    "Move Fly",
    "View Restriction",
    "Has Icon BMap",
    "Has Contract",
    "Haunt Chain Attack",
    "Has Root Command",
    "Has ZOC",
    "Has Work",
    "Has Vision",
    "Not Condition",
    "Has Condition",
    "Has Enhance",
    "Has Range Target",
];

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

    pub fn select(&mut self, index: Option<usize>) {
        self.content.select(index);
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        self.content.side_panel(ctx, &self.skill, state);

        self.cache.refresh(state);

        self.skill.write(|data| {
            self.content.content(ctx, data, |ui, skill| {
                PropertyGrid::new("skill", skill)
                    .new_section("Core")
                    .field("SID", |ui, skill| ui.add(id_field(&mut skill.sid)))
                    .field("Name", |ui, skill| {
                        msbt_key_value_singleline!(ui, state, "skill", skill.name)
                    })
                    .field("Help", |ui, skill| {
                        msbt_key_value_multiline!(ui, state, "skill", skill.help)
                    })
                    .field("Condition", |ui, skill| {
                        ui.vertical_centered_justified(|ui| {
                            skill.condition.default_widget(ui)
                        })
                        .inner
                    })
                    .default_field("Cost", |skill| &mut skill.cost)
                    .default_field("Priority", |skill| &mut skill.priority)
                    .default_field("Layer", |skill| &mut skill.layer)
                    .default_field("Order", |skill| &mut skill.order)
                    .field("Duration", |ui, skill| {
                        ui.horizontal(|ui| {
                            let mut response = ui.add(skill_cycle_drop_down(&mut skill.cycle));
                            ui.label("x");
                            response = response.union(ui.add(u8_drag(&mut skill.life)));
                            response
                        })
                        .inner
                    })
                    .field("Timing", |ui, skill| {
                        ui.add(skill_timing_drop_down(&mut skill.timing))
                    })
                    .field("Target", |ui, skill| {
                        ui.add(skill_targets_drop_down(&mut skill.target))
                    })
                    .field("Frequency", |ui, skill| {
                        ui.add(skill_frequencies_drop_down(&mut skill.frequency))
                    })
                    .field("Stance", |ui, skill| {
                        ui.add(skill_stance_drop_down(&mut skill.stand))
                    })
                    .default_field("Action", |skill| &mut skill.action)
                    .field("Flags", |ui, skill| {
                        ui.add(bitgrid_u64(SKILL_FLAG_LABELS, 3, &mut skill.flag))
                    })
                    .field("Prohibit Weapons", |ui, skill| {
                        ui.add(bitgrid_i32(WEAPON_LABELS, 3, &mut skill.weapon_prohibit))
                    })
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
                        ui.add(weapon_rank_numbered_drop_down(&mut skill.weapon_level_none))
                    })
                    .field("Sword", |ui, skill| {
                        ui.add(weapon_rank_numbered_drop_down(
                            &mut skill.weapon_level_sword,
                        ))
                    })
                    .field("Axe", |ui, skill| {
                        ui.add(weapon_rank_numbered_drop_down(&mut skill.weapon_level_axe))
                    })
                    .field("Lance", |ui, skill| {
                        ui.add(weapon_rank_numbered_drop_down(
                            &mut skill.weapon_level_lance,
                        ))
                    })
                    .field("Bow", |ui, skill| {
                        ui.add(weapon_rank_numbered_drop_down(&mut skill.weapon_level_bow))
                    })
                    .field("Dagger", |ui, skill| {
                        ui.add(weapon_rank_numbered_drop_down(
                            &mut skill.weapon_level_dagger,
                        ))
                    })
                    .field("Magic", |ui, skill| {
                        ui.add(weapon_rank_numbered_drop_down(
                            &mut skill.weapon_level_magic,
                        ))
                    })
                    .field("Staff", |ui, skill| {
                        ui.add(weapon_rank_numbered_drop_down(&mut skill.weapon_level_rod))
                    })
                    .field("Fist", |ui, skill| {
                        ui.add(weapon_rank_numbered_drop_down(&mut skill.weapon_level_fist))
                    })
                    .field("Special", |ui, skill| {
                        ui.add(weapon_rank_numbered_drop_down(
                            &mut skill.weapon_level_special,
                        ))
                    })
                    .new_section("Command")
                    .field("Root SID", |ui, skill| {
                        ui.add(model_drop_down(
                            self.cache.get(),
                            &(),
                            &mut skill.root_command_sid,
                        ))
                    })
                    .field("Name", |ui, skill| {
                        msbt_key_value_singleline!(ui, state, "skill", skill.command_name)
                    })
                    .field("Help", |ui, skill| {
                        msbt_key_value_multiline!(ui, state, "skill", skill.command_help)
                    })
                    .field("Warning", |ui, skill| {
                        msbt_key_value_multiline!(ui, state, "skill", skill.command_warning)
                    })
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
                    .field("Center", |ui, skill| {
                        ui.add(skill_around_centers_drop_down(&mut skill.around_center))
                    })
                    .field("Target", |ui, skill| {
                        ui.add(skill_around_targets_drop_down(&mut skill.around_target))
                    })
                    .new_section("Give")
                    .field("Target", |ui, skill| {
                        ui.add(skill_give_targets_drop_down(&mut skill.give_target))
                    })
                    .default_field("Condition", |skill| &mut skill.give_condition)
                    .field("Skills", |ui, skill| {
                        ui.add(editable_list(&mut skill.give_sids, |_, value, ui| {
                            ui.add(model_drop_down(self.cache.get(), &(), value))
                        }))
                    })
                    .new_section("Remove")
                    .field("Skills", |ui, skill| {
                        ui.add(editable_list(&mut skill.remove_sids, |_, value, ui| {
                            ui.add(model_drop_down(self.cache.get(), &(), value))
                        }))
                    })
                    .new_section("Sync")
                    .field("Conditions", |ui, skill| {
                        ui.add(editable_list(&mut skill.sync_conditions, |_, value, ui| {
                            ui.text_edit_singleline(value)
                        }))
                    })
                    .field("Skills", |ui, skill| {
                        ui.add(editable_list(&mut skill.sync_sids, |_, value, ui| {
                            ui.add(model_drop_down(self.cache.get(), &(), value))
                        }))
                    })
                    .new_section("Other Related Skills")
                    .field("Rebirth Skill", |ui, skill| {
                        ui.add(model_drop_down(self.cache.get(), &(), &mut skill.rebirth_sid))
                    })
                    .field("Engage Skill", |ui, skill| {
                        ui.add(model_drop_down(self.cache.get(), &(), &mut skill.engage_sid))
                    })
                    .new_section("Overlap")
                    .default_field("Range", |skill| &mut skill.overlap_range)
                    .field("Terrain", |ui, skill| {
                        state.terrain.read(|data| {
                            ui.add(model_drop_down(data, state, &mut skill.overlap_terrain))
                        })
                    })
                    .field("Change Skills", |ui, skill| {
                        ui.add(editable_list(&mut skill.change_sids, |_, value, ui| {
                            ui.add(model_drop_down(self.cache.get(), &(), value))
                        }))
                    })
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
                    .field("State", |ui, skill| {
                        ui.add(bitgrid_i32(STATE_FLAG_LABELS, 3, &mut skill.bad_state))
                    })
                    .field("Ignore", |ui, skill| {
                        ui.add(bitgrid_i32(STATE_FLAG_LABELS, 3, &mut skill.bad_ignore))
                    })
                    .new_section("Inheritance")
                    .default_field("Cost", |skill| &mut skill.inheritance_cost)
                    .default_field("Sort", |skill| &mut skill.inheritance_sort)
                    .show(ui)
                    .changed()
            })
        });
    }
}
