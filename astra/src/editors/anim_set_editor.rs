use astra_types::AnimSet;
use indexmap::IndexMap;

use crate::widgets::{id_field, keyed_add_modal_content};
use crate::{AnimSetSheet, EditorState, ListEditorContent, PropertyGrid};

pub struct AnimSetEditor {
    anim_set: AnimSetSheet,
    content: ListEditorContent<IndexMap<String, AnimSet>, AnimSet>,
}

impl AnimSetEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            anim_set: state.anim_set.clone(),
            content: ListEditorContent::new("anim_set_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        self.content.side_panel(ctx, &self.anim_set, &());

        self.anim_set.write(|data| {
            self.content.content(ctx, data, |ui, animset| {
                PropertyGrid::new("anim_set", animset)
                    .new_section("Data")
                    .field("Name", |ui, anim_set| ui.add(id_field(&mut anim_set.name)))
                    .default_field("Attack 1", |anim_set| &mut anim_set.attack_1)
                    .default_field("Attack 2", |anim_set| &mut anim_set.attack_2)
                    .default_field("Attack 3", |anim_set| &mut anim_set.attack_3)
                    .default_field("Attack 4", |anim_set| &mut anim_set.attack_4)
                    .default_field("Attack 5", |anim_set| &mut anim_set.attack_5)
                    .default_field("Attack C", |anim_set| &mut anim_set.attack_c)
                    .default_field("Attack T", |anim_set| &mut anim_set.attack_t)
                    .default_field("Damage High", |anim_set| &mut anim_set.damage_high)
                    .default_field("Damage Mid B", |anim_set| &mut anim_set.damage_mid_b)
                    .default_field("Damage Mid DU", |anim_set| &mut anim_set.damage_mid_du)
                    .default_field("Damage Mid UD", |anim_set| &mut anim_set.damage_mid_ud)
                    .default_field("Die B", |anim_set| &mut anim_set.die_b)
                    .default_field("Die L", |anim_set| &mut anim_set.die_l)
                    .default_field("Die R", |anim_set| &mut anim_set.die_r)
                    .default_field("Dive", |anim_set| &mut anim_set.dive)
                    .default_field("Engage 1", |anim_set| &mut anim_set.engage_1)
                    .default_field("Engage 2", |anim_set| &mut anim_set.engage_2)
                    .default_field("Engage 3", |anim_set| &mut anim_set.engage_3)
                    .default_field("Evasion B", |anim_set| &mut anim_set.evasion_b)
                    .default_field("Evasion L", |anim_set| &mut anim_set.evasion_l)
                    .default_field("Evasion R", |anim_set| &mut anim_set.evasion_r)
                    .default_field("Guard", |anim_set| &mut anim_set.guard)
                    .default_field("Hover Loop", |anim_set| &mut anim_set.hovering_loop)
                    .default_field("Idle (Dying)", |anim_set| &mut anim_set.idle_dying)
                    .default_field("Idle (Normal)", |anim_set| &mut anim_set.idle_normal)
                    .default_field("Parry L", |anim_set| &mut anim_set.parry_l)
                    .default_field("Parry R", |anim_set| &mut anim_set.parry_r)
                    .default_field("Ready", |anim_set| &mut anim_set.ready)
                    .default_field("Relax Loop", |anim_set| &mut anim_set.relax_loop)
                    .default_field("Repelled", |anim_set| &mut anim_set.repelled)
                    .default_field("Run Loop", |anim_set| &mut anim_set.run_loop)
                    .default_field("Run Start", |anim_set| &mut anim_set.run_start)
                    .default_field("Special 1", |anim_set| &mut anim_set.special_1)
                    .default_field("Start", |anim_set| &mut anim_set.start)
                    .default_field("Win", |anim_set| &mut anim_set.win)
                    .default_field("Win Loop", |anim_set| &mut anim_set.win_loop)
                    .show(ui)
                    .changed()
            })
        });
    }
}
