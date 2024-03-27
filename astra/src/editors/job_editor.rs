use astra_types::{Job, JobBook};
use egui::Grid;
use egui_extras::Size;
use indexmap::IndexMap;

use crate::widgets::{id_field, keyed_add_modal_content};
use crate::{
    bitgrid_i32, bitgrid_u8, editable_list, job_rank_drop_down, model_drop_down,
    msbt_key_value_multiline, msbt_key_value_singleline, optional_image,
    standard_stat_column_headers, standard_stats_row, weapon_rank_drop_down,
    weapon_rank_numbered_drop_down, CachedView, DecorationKind, EditorState, JobSheet,
    JobSheetRetriever, ListEditorContent, ModelDropDown, PropertyGrid, ViewItem,
};

const FLAG_LABELS: &[&str] = &[
    "Can Class Change",
    "Common Class",
    "Female Only",
    "Can Appear in Encounters",
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

fn item_key_transform(key: &str) -> String {
    let mut id = String::from("IID_");
    id.push_str(key);
    id
}

fn item_key_reverse_transform(key: &str) -> String {
    key.trim_start_matches("IID_").to_owned()
}

pub struct JobEditor {
    job: JobSheet,
    content: ListEditorContent<IndexMap<String, Job>, Job, EditorState>,
    cache: CachedView<JobSheetRetriever, JobBook, Job>,
}

impl JobEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            job: state.job.clone(),
            cache: CachedView::new(state.job.clone(), state),
            content: ListEditorContent::new("job_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.content.select(index);
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        self.content.left_panel(ctx, &self.job, state);

        self.cache.refresh(state);

        self.job.write(|data| {
            self.content.content(ctx, data, |ui, job| {
                let mut changed = false;
                egui_grid::GridBuilder::new()
                    .new_row(Size::exact(200.))
                    .cell(Size::exact(350.))
                    .cell(Size::remainder())
                    .new_row(Size::remainder())
                    .cell(Size::remainder())
                    .show(ui, |mut grid| {
                        grid.cell(|ui| {
                            ui.group(|ui| {
                                ui.vertical_centered_justified(|ui| {
                                    ui.group(|ui| {
                                        ui.add(optional_image(
                                            job.decoration(
                                                state,
                                                DecorationKind::Other("portrait"),
                                            ),
                                            [ui.available_width() - 12., 90.],
                                        ));
                                    });
                                    ui.add(id_field(&mut job.jid));
                                });
                                ui.allocate_space(ui.available_size());
                            });
                        });
                        grid.cell(|ui| {
                            ui.group(|ui| {
                                Grid::new("item_grid").show(ui, |ui| {
                                    standard_stat_column_headers(ui);
                                    ui.label("Limits");
                                    standard_stats_row!(ui, job, limit, changed);
                                    ui.label("Bases");
                                    standard_stats_row!(ui, job, base, changed);
                                    ui.label("Base Growths");
                                    standard_stats_row!(ui, job, base_grow, changed);
                                    ui.label("Diff Growths");
                                    standard_stats_row!(ui, job, diff_grow, changed);
                                    ui.label("Diff Growths (N)");
                                    standard_stats_row!(ui, job, diff_grow_normal, changed);
                                    ui.label("Diff Growths (H)");
                                    standard_stats_row!(ui, job, diff_grow_hard, changed);
                                    ui.label("Diff Growths (L)");
                                    standard_stats_row!(ui, job, diff_grow_lunatic, changed);
                                });
                                ui.allocate_space(ui.available_size());
                            });
                        });
                        grid.cell(|ui| {
                            ui.separator();
                            changed |= PropertyGrid::new("item", job)
                                .new_section("Core")
                                .field("Name", |ui, job| {
                                    msbt_key_value_singleline!(ui, state, "job", job.name)
                                })
                                .field("Help", |ui, job| {
                                    msbt_key_value_multiline!(ui, state, "job", job.help)
                                })
                                .field("Rank", |ui, job| ui.add(job_rank_drop_down(&mut job.rank)))
                                .default_field("Style", |job| &mut job.style_name)
                                .default_field("Short Name", |job| &mut job.short_name)
                                .default_field("Sort", |job| &mut job.sort)
                                .field("Flags", |ui, job| {
                                    ui.add(bitgrid_u8(FLAG_LABELS, 4, &mut job.flag))
                                })
                                .field("Attrs", |ui, job| {
                                    ui.add(bitgrid_i32(ATTR_LABELS, 3, &mut job.attrs))
                                })
                                .new_section("Stats")
                                .default_field("Move Type", |job| &mut job.move_type)
                                .default_field("Max Level", |job| &mut job.max_level)
                                .default_field("Internal Level", |job| &mut job.internal_level)
                                .field("Advanced Class 1", |ui, job| {
                                    ui.add(model_drop_down(
                                        self.cache.get(),
                                        &(),
                                        &mut job.high_job_1,
                                    ))
                                })
                                .field("Advanced Class 2", |ui, job| {
                                    ui.add(model_drop_down(
                                        self.cache.get(),
                                        &(),
                                        &mut job.high_job_2,
                                    ))
                                })
                                .default_field("Base Class", |job| &mut job.low_job)
                                .new_section_with_columns("Weapons", 2)
                                .field("None", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_none))
                                })
                                .field("None (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_none))
                                })
                                .field("Swords", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_sword))
                                })
                                .field("Swords (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_sword))
                                })
                                .field("Axe", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_axe))
                                })
                                .field("Axe (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_axe))
                                })
                                .field("Lance", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_lance))
                                })
                                .field("Lance (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_lance))
                                })
                                .field("Bow", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_bow))
                                })
                                .field("Bow (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_bow))
                                })
                                .field("Dagger", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_dagger))
                                })
                                .field("Dagger (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_dagger))
                                })
                                .field("Magic", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_magic))
                                })
                                .field("Magic (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_magic))
                                })
                                .field("Staff", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_rod))
                                })
                                .field("Staff (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_rod))
                                })
                                .field("Fist", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_fist))
                                })
                                .field("Fist (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_fist))
                                })
                                .field("Special", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_special))
                                })
                                .field("Special (Max)", |ui, job| {
                                    ui.add(weapon_rank_drop_down(&mut job.max_weapon_level_special))
                                })
                                .field("Tool", |ui, job| {
                                    ui.add(weapon_rank_numbered_drop_down(&mut job.weapon_tool))
                                })
                                .new_section("Skills")
                                .field("Skills", |ui, job| {
                                    ui.add(editable_list(&mut job.skills, |_, value, ui| {
                                        state.skill.read(|data| {
                                            ui.add(model_drop_down(data, state, value))
                                        })
                                    }))
                                })
                                .field("Learned Skill", |ui, job| {
                                    state.skill.read(|data| {
                                        ui.add(model_drop_down(
                                            data,
                                            state,
                                            &mut job.learning_skill,
                                        ))
                                    })
                                })
                                .field("Lunatic Skill", |ui, job| {
                                    state.skill.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut job.lunatic_skill))
                                    })
                                })
                                .new_section("Items")
                                .field("Class Change Items", |ui, job| {
                                    ui.add(editable_list(&mut job.cc_items, |_, value, ui| {
                                        state.item.read(|data| {
                                            ModelDropDown::default()
                                                .transform(
                                                    &item_key_transform,
                                                    &item_key_reverse_transform,
                                                )
                                                .show(ui, data, state, value)
                                        })
                                    }))
                                })
                                .field("Unique Items", |ui, job| {
                                    ui.add(editable_list(&mut job.unique_items, |_, value, ui| {
                                        state.item.read(|data| {
                                            ModelDropDown::default()
                                                .transform(
                                                    &item_key_transform,
                                                    &item_key_reverse_transform,
                                                )
                                                .show(ui, data, state, value)
                                        })
                                    }))
                                })
                                .new_section("Visuals")
                                .default_field("AID", |job| &mut job.aid)
                                .default_field("Icon (M)", |job| &mut job.unit_icon_id_m)
                                .default_field("Icon (F)", |job| &mut job.unit_icon_id_f)
                                .default_field("Icon Weapon", |job| &mut job.unit_icon_weapon_id)
                                .default_field("Step Frame", |job| &mut job.step_frame)
                                .show(ui)
                                .changed();
                        });
                    });
                changed
            })
        });
    }
}
