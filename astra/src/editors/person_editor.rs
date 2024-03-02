use astra_types::{Person, PersonBook};
use egui::{Grid, Ui};
use indexmap::IndexMap;

use crate::widgets::{exist_die_timing_drop_down, id_field, keyed_add_modal_content};
use crate::{
    bitgrid_i32, bitgrid_u8, editable_list, gender_drop_down, i8_drag, model_drop_down,
    msbt_key_value_multiline, msbt_key_value_singleline, nation_drop_down, optional_image,
    standard_stat_column_headers, standard_stats_row, u8_drag, CachedView, DecorationKind,
    EditorState, ListEditorContent, PersonSheet, PersonSheetRetriever, PropertyGrid, ViewItem,
};

const WEAPON_AFFINITY_LABELS: &[&str] = &[
    "None", "Swords", "Lances", "Axes", "Bows", "Daggers", "Magic", "Staves", "Fists",
];

const CHARACTER_FLAG_LABELS: &[&str] = &[
    "Playable",
    "???",
    "???",
    "???",
    "???",
    "Crossdress",
    "Corrupted Emblem",
    "Alear Copy?",
];

const ATTR_LABELS: &[&str] = &[
    "Infantry",
    "Mounted",
    "Armored",
    "Flier",
    "Dragon",
    "Fell Dragon",
    "Corrupted",
    "Medius",
    "Duma",
    "Loptous",
    "Veld",
    "Idun",
    "Nergal",
    "Fomortiis",
    "Ashnard",
    "Ashera",
    "Grima",
    "Anankos",
    "Nemesis",
];

pub struct PersonEditor {
    person: PersonSheet,
    content: ListEditorContent<IndexMap<String, Person>, Person>,
    cache: CachedView<PersonSheetRetriever, PersonBook, Person>,
}

impl PersonEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            person: state.person.clone(),
            cache: CachedView::new(state.person.clone(), state),
            content: ListEditorContent::new("person_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.content.select(index);
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        self.content.side_panel(ctx, &self.person, state);

        self.cache.refresh(state);

        self.person.write(|data| {
            self.content.content(ctx, data, |ui, person| {
                let mut changed = false;
                ui.horizontal_top(|ui| {
                    ui.add_sized([300., 0.], |ui: &mut Ui| {
                        ui.group(|ui| {
                            ui.vertical_centered_justified(|ui| {
                                ui.group(|ui| {
                                    ui.add(optional_image(
                                        person.decoration(state, DecorationKind::Other("portrait")),
                                        [ui.available_width() - 12., 90.],
                                    ));
                                });
                                ui.add(id_field(&mut person.pid));
                            });
                            ui.label("");
                        })
                        .response
                    });
                    ui.group(|ui| {
                        Grid::new("person_grid").show(ui, |ui| {
                            standard_stat_column_headers(ui);
                            ui.label("Offsets (N)");
                            standard_stats_row!(ui, person, offset_n, changed);
                            ui.label("Offsets (H)");
                            standard_stats_row!(ui, person, offset_h, changed);
                            ui.label("Offsets (L)");
                            standard_stats_row!(ui, person, offset_l, changed);
                            ui.label("Limits");
                            standard_stats_row!(ui, person, limit, changed);
                            ui.label("Growths");
                            standard_stats_row!(ui, person, grow, changed);
                        });
                    });
                });

                ui.separator();

                changed |= PropertyGrid::new("person", person)
                    .new_section("Core")
                    .field("Name", |ui, p| {
                        msbt_key_value_singleline!(ui, state, "person", p.name)
                    })
                    .field("Help", |ui, p| {
                        msbt_key_value_multiline!(ui, state, "person", p.help)
                    })
                    .default_field("FID", |p| &mut p.fid)
                    .default_field("AID", |p| &mut p.aid)
                    .default_field("Icon ID", |p| &mut p.unit_icon_id)
                    .default_field("Army", |p| &mut p.belong)
                    .field("Class", |ui, p| {
                        state
                            .job
                            .read(|data| ui.add(model_drop_down(data, state, &mut p.jid)))
                    })
                    .field("Gender", |ui, p| ui.add(gender_drop_down(&mut p.gender)))
                    .field("Nation", |ui, p| ui.add(nation_drop_down(&mut p.hometown)))
                    .default_field("Age", |p| &mut p.age)
                    .field("Birthday", |ui, p| {
                        ui.horizontal(|ui| {
                            ui.add(u8_drag(&mut p.birth_month))
                                .union(ui.add(u8_drag(&mut p.birth_day)))
                        })
                        .inner
                    })
                    .field("Flags", |ui, p| {
                        ui.add(bitgrid_u8(CHARACTER_FLAG_LABELS, 3, &mut p.flag))
                    })
                    .new_section("Stats")
                    .default_field("Level", |p| &mut p.level)
                    .default_field("Internal Level", |p| &mut p.internal_level)
                    .default_field("SP", |p| &mut p.skill_point)
                    .field("Primary Weapons", |ui, p| {
                        ui.add(bitgrid_i32(WEAPON_AFFINITY_LABELS, 3, &mut p.aptitude))
                    })
                    .field("Secondary Weapons", |ui, p| {
                        ui.add(bitgrid_i32(WEAPON_AFFINITY_LABELS, 3, &mut p.sub_aptitude))
                    })
                    .field("Auto Grow Offsets", |ui, p| {
                        ui.horizontal_top(|ui| {
                            ui.add(i8_drag(&mut p.auto_grow_offset_n))
                                .union(ui.add(i8_drag(&mut p.auto_grow_offset_h)))
                                .union(ui.add(i8_drag(&mut p.auto_grow_offset_l)))
                        })
                        .inner
                    })
                    .field("Attrs", |ui, p| {
                        ui.add(bitgrid_i32(ATTR_LABELS, 3, &mut p.attrs))
                    })
                    .new_section("Skills")
                    .field("Common", |ui, p| {
                        ui.add(editable_list(&mut p.common_sids, |_, value, ui| {
                            state
                                .skill
                                .read(|data| ui.add(model_drop_down(data, state, value)))
                        }))
                    })
                    .field("Normal", |ui, p| {
                        ui.add(editable_list(&mut p.normal_sids, |_, value, ui| {
                            state
                                .skill
                                .read(|data| ui.add(model_drop_down(data, state, value)))
                        }))
                    })
                    .field("Hard", |ui, p| {
                        ui.add(editable_list(&mut p.hard_sids, |_, value, ui| {
                            state
                                .skill
                                .read(|data| ui.add(model_drop_down(data, state, value)))
                        }))
                    })
                    .field("Lunatic", |ui, p| {
                        ui.add(editable_list(&mut p.lunatic_sids, |_, value, ui| {
                            state
                                .skill
                                .read(|data| ui.add(model_drop_down(data, state, value)))
                        }))
                    })
                    .new_section("Emblem")
                    .field("Engage Skill", |ui, p| {
                        state
                            .skill
                            .read(|data| ui.add(model_drop_down(data, state, &mut p.engage_sid)))
                    })
                    .default_field("Summon Emblem", |p| &mut p.summon_god)
                    .default_field("Summon Rank", |p| &mut p.summon_rank)
                    .default_field("Summon Rate", |p| &mut p.summon_rate)
                    .default_field("Summon Color", |p| &mut p.summon_color)
                    .new_section("Items")
                    .field("Drop Item", |ui, p| {
                        state
                            .item
                            .read(|data| ui.add(model_drop_down(data, state, &mut p.drop_item)))
                    })
                    .default_field("Drop Ratio", |p| &mut p.drop_ratio)
                    .field("Inventory", |ui, p| {
                        ui.add(editable_list(&mut p.items, |_, value, ui| {
                            state
                                .item
                                .read(|data| ui.add(model_drop_down(data, state, value)))
                        }))
                    })
                    .new_section("Misc.")
                    .default_field("Die", |p| &mut p.die)
                    .default_field("Support Category", |p| &mut p.support_category)
                    .default_field("Combat Music", |p| &mut p.combat_bgm)
                    .field("Exist Die Chapter", |ui, p| {
                        state
                            .chapter
                            .read(|data| ui.add(model_drop_down(data, state, &mut p.exist_die_cid)))
                    })
                    .field("Exist Die Timing", |ui, p| {
                        ui.add(exist_die_timing_drop_down(&mut p.exist_die_timing))
                    })
                    .default_field("Talk Pause Speed", |p| &mut p.talk_pause_speed)
                    .default_field("Talk Pause Min Delay", |p| &mut p.talk_pause_delay_min)
                    .default_field("Talk Pause Max Delay", |p| &mut p.talk_pause_delay_max)
                    .default_field("BMap Size", |p| &mut p.bmap_size)
                    .default_field("Asset Force", |p| &mut p.asset_force)
                    .field("No Level Up Dialogue", |ui, p| {
                        ui.add(editable_list(&mut p.not_lv_up_talk_pids, |_, value, ui| {
                            ui.add(model_drop_down(self.cache.get(), &(), value))
                        }))
                    })
                    .show(ui)
                    .changed();

                changed
            })
        });
    }
}
