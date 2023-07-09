use std::sync::Arc;

use astra_core::{Astra, OpenMessageScript, OpenTerrain};
use astra_types::{Chapter, ChapterBook, Spawn};
use egui::{Button, CentralPanel, ComboBox, Ui};
use egui_modal::{Icon, Modal};
use indexmap::IndexMap;
use parking_lot::RwLock;

use crate::widgets::{
    bitgrid_i32, bitgrid_u16, chapter_encount_type, chapter_spot_state, id_field,
    keyed_add_modal_content,
};
use crate::{
    blank_slate, dispos_grid, editor_tab_strip, f32_drag, i8_drag, indexed_model_drop_down,
    model_drop_down, msbt_key_value_singleline, msbt_script_editor, terrain_grid, u32_drag,
    u8_drag, AppConfig, CacheItem, CachedView, ChapterSheet, ChapterSheetRetriever, EditorState,
    GroupEditorContent, ListEditorContent, ListModel, PropertyGrid, SpawnSheet,
};

const CHAPTER_FLAG_LABELS: &[&str] = &[
    "Sortie",
    "Can Back Out",
    "Sight",
    "Kizuna",
    "Hub",
    "GMap",
    "Continue",
    "Serious",
    "Casual",
    "Challenge",
    "Relay",
    "Versus",
    "Test Map",
    "Opposition",
    "High Rank Item",
    "Can Slope",
];

const SPAWN_FLAG_LABELS: &[&str] = &[
    "Normal",
    "Hard",
    "Lunatic",
    "Create",
    "Boss",
    "Cannot Move",
    "Edge",
    "Deployment Slot",
    "Must Deploy",
    "Fixed",
    "Guest",
];

fn can_open_script(script_name: &str, config: &AppConfig) -> bool {
    !(script_name.is_empty()
        || config.script_editor_process.is_empty()
        || config.script_editor_command_args.is_empty())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Core,
    Dispos,
    Terrain,
    Dialogue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DisposKind {
    Main,
    Encount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateKind {
    Dispos,
    Appear,
}

struct OpenChapterState {
    dispos: Option<SpawnSheet>,
    encount_dispos: Option<SpawnSheet>,
    terrain: Option<OpenTerrain>,
    message: Option<OpenMessageScript>,
    script: String,
    encount_script: String,
    kizuna_script: String,
}

impl OpenChapterState {
    pub fn load(state: &mut EditorState, chapter_index: Option<usize>) -> Option<Self> {
        chapter_index.and_then(|index| {
            state.chapter.clone().read(|data| {
                data.item(index).map(|chapter| {
                    let cid_part = chapter.cid.trim_start_matches("CID_");
                    let dispos_stem = chapter.dispos.replace('*', cid_part).to_lowercase();
                    let terrain = state.astra.write().get_chapter_terrain(
                        &chapter.terrain.replace('*', cid_part).to_lowercase(),
                    );
                    let message = state
                        .astra
                        .write()
                        .open_msbt_script(&chapter.mess.replace('*', cid_part).to_lowercase())
                        .ok();
                    Self {
                        dispos: state.load_spawn_sheet(&dispos_stem),
                        encount_dispos: state.load_spawn_sheet(&format!("{}e", dispos_stem)),
                        terrain,
                        message,
                        script: chapter.script_bmap.replace('*', cid_part).to_lowercase(),
                        encount_script: chapter
                            .script_encount
                            .replace('*', cid_part)
                            .to_lowercase(),
                        kizuna_script: chapter.script_kizuna.replace('*', cid_part).to_lowercase(),
                    }
                })
            })
        })
    }
}

pub struct ChapterEditor {
    tab: Tab,
    dispos_kind: DisposKind,
    coordinate_kind: CoordinateKind,
    script_open_error: Option<String>,
    selected_chapter_index: Option<usize>,

    terrain_content:
        ListEditorContent<IndexMap<String, astra_types::TerrainData>, astra_types::TerrainData>,
    dispos_content: GroupEditorContent,

    astra: Arc<RwLock<Astra>>,
    chapter: ChapterSheet,
    cache: CachedView<ChapterSheetRetriever, ChapterBook, Chapter>,
    chapter_state: Option<OpenChapterState>,
}

impl ChapterEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Core,
            dispos_kind: DisposKind::Main,
            coordinate_kind: CoordinateKind::Dispos,
            script_open_error: None,
            selected_chapter_index: None,

            terrain_content: ListEditorContent::new("chapter_terrain_list_editor")
                .with_add_modal_content(keyed_add_modal_content),
            dispos_content: GroupEditorContent::new("chapter_dispos_group_editor"),

            astra: state.astra.clone(),
            chapter: state.chapter.clone(),
            cache: CachedView::new(state.chapter.clone(), state),
            chapter_state: None,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState, config: &AppConfig) {
        if self.chapter_state.is_none() {
            CentralPanel::default().show(ctx, |ui| {
                blank_slate(ui);
            });
            return;
        }

        self.cache.refresh(state);

        match self.tab {
            Tab::Core => self.core_tab_content(ctx, state, config),
            Tab::Dispos => self.dispos_tab_content(ctx, state),
            Tab::Terrain => self.terrain_tab_content(ctx, state),
            Tab::Dialogue => self.dialogue_tab_content(ctx),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui, state: &mut EditorState) {
        editor_tab_strip(ui, |ui| {
            if ui
                .add_enabled(
                    self.selected_chapter_index.is_some(),
                    Button::new("-").min_size([30., 0.].into()),
                )
                .clicked()
            {
                let index = self.selected_chapter_index.unwrap();
                self.chapter.write(|data| {
                    if index < data.len() {
                        data.shift_remove_index(index);
                        if data.is_empty() {
                            self.selected_chapter_index = None;
                        } else if index >= data.len() {
                            self.selected_chapter_index = Some(index - 1);
                        }
                        true
                    } else {
                        false
                    }
                });
                self.chapter_state = OpenChapterState::load(state, self.selected_chapter_index);
            }
            self.chapter.read(|data| {
                if ui
                    .add(indexed_model_drop_down(
                        data,
                        state,
                        &mut self.selected_chapter_index,
                    ))
                    .changed()
                {
                    self.chapter_state = OpenChapterState::load(state, self.selected_chapter_index);
                }
            });
            ui.selectable_value(&mut self.tab, Tab::Core, "Core");
            ui.selectable_value(&mut self.tab, Tab::Dispos, "Dispos");
            ui.selectable_value(&mut self.tab, Tab::Terrain, "Terrain");
            ui.selectable_value(&mut self.tab, Tab::Dialogue, "Dialogue");
            ComboBox::from_id_source("dispos_kind")
                .selected_text(match self.dispos_kind {
                    DisposKind::Main => "Main",
                    DisposKind::Encount => "Encount",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.dispos_kind, DisposKind::Main, "Main");
                    ui.selectable_value(&mut self.dispos_kind, DisposKind::Encount, "Encount");
                });
            ComboBox::from_id_source("coordinate_kind")
                .selected_text(match self.coordinate_kind {
                    CoordinateKind::Dispos => "Dispos",
                    CoordinateKind::Appear => "Appear",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.coordinate_kind,
                        CoordinateKind::Dispos,
                        "Dispos",
                    );
                    ui.selectable_value(
                        &mut self.coordinate_kind,
                        CoordinateKind::Appear,
                        "Appear",
                    );
                });
        });
    }

    fn core_tab_content(&mut self, ctx: &egui::Context, state: &EditorState, config: &AppConfig) {
        CentralPanel::default().show(ctx, |ui| {
            self.script_buttons(ui, config);
            self.chapter.write(|data| {
                if let Some((_, chapter)) = data.get_index_mut(self.selected_chapter_index.unwrap())
                {
                    Self::chapter_property_grid(self.cache.get(), ui, state, chapter)
                } else {
                    false
                }
            });
        });
    }

    fn script_buttons(&mut self, ui: &mut Ui, config: &AppConfig) {
        let (script, encount_script, kizuna_script) = self
            .chapter_state
            .as_ref()
            .map(|state| {
                (
                    Some(state.script.as_str()),
                    Some(state.encount_script.as_str()),
                    Some(state.kizuna_script.as_str()),
                )
            })
            .unwrap_or((None, None, None));

        let error_modal = Modal::new(ui.ctx(), "script_open_error_modal");
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let mut astra = self.astra.write();
                Self::script_button(
                    &mut astra,
                    ui,
                    config,
                    script,
                    "Open Main Script",
                    &mut self.script_open_error,
                );
                Self::script_button(
                    &mut astra,
                    ui,
                    config,
                    encount_script,
                    "Open Encount Script",
                    &mut self.script_open_error,
                );
                Self::script_button(
                    &mut astra,
                    ui,
                    config,
                    kizuna_script,
                    "Open Kizuna Script",
                    &mut self.script_open_error,
                );
            });
        });
        if let Some(error) = self.script_open_error.clone() {
            error_modal.show(|ui| {
                error_modal.title(ui, "Failed to open script");
                error_modal.body_and_icon(ui, &error, Icon::Error);
                error_modal.buttons(ui, |ui| {
                    if error_modal.button(ui, "Close").clicked() {
                        self.script_open_error = None;
                    }
                    if error_modal.button(ui, "Copy Error").clicked() {
                        ui.output_mut(|out| {
                            out.copied_text = error.to_string();
                        });
                    }
                });
            });
            error_modal.open();
        }
    }

    fn script_button(
        astra: &mut Astra,
        ui: &mut Ui,
        config: &AppConfig,
        script_name: Option<&str>,
        label: &str,
        error_message: &mut Option<String>,
    ) {
        if script_name.is_none() {
            ui.add_enabled(false, Button::new(label));
            return;
        }
        let script_name = script_name.unwrap();
        if ui
            .add_enabled(can_open_script(&script_name, config), Button::new(label))
            .clicked()
        {
            let result = astra.open_script(
                script_name,
                &config.script_editor_process,
                &config.script_editor_command_args,
            );
            if let Err(error) = result {
                *error_message = Some(format!("{:?}", error));
            }
        }
    }

    fn chapter_property_grid(
        cache: &IndexMap<String, CacheItem<Chapter>>,
        ui: &mut Ui,
        state: &EditorState,
        chapter: &mut Chapter,
    ) -> bool {
        PropertyGrid::new("chapter", chapter)
            .new_section("Core")
            .field("CID", |ui, chapter| ui.add(id_field(&mut chapter.cid)))
            .field("Name", |ui, chapter| {
                msbt_key_value_singleline!(ui, state, "gamedata", chapter.name)
            })
            .field("Help", |ui, chapter| {
                // TODO: Figure out the right way to do this
                ui.text_edit_singleline(&mut chapter.help)
            })
            .field("Recommended Level", |ui, chapter| {
                ui.add(u8_drag(&mut chapter.recommended_level))
            })
            .field("Nation", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.nation)
            })
            .field("Title", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.chapter_title)
            })
            .field("Next Chapter", |ui, chapter| {
                ui.add(model_drop_down(cache, &(), &mut chapter.next_chapter))
            })
            .field("Reward", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.reward)
            })
            .field("Field", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.field)
            })
            .field("Message", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.mess)
            })
            .field("Event", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.event)
            })
            .field("BMap Script", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.script_bmap)
            })
            .field("Encount Script", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.script_encount)
            })
            .field("Kizuna Script", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.script_kizuna)
            })
            .field("Terrain", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.terrain)
            })
            .field("Dispos", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.dispos)
            })
            .field("Flags", |ui, chapter| {
                ui.add(bitgrid_i32(CHAPTER_FLAG_LABELS, 3, &mut chapter.flag))
            })
            .new_section("GMap")
            .field("Spot", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.gmap_spot)
            })
            .field("Open Condition", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.gmap_spot_open_condition)
            })
            .field("State", |ui, chapter| {
                ui.add(chapter_spot_state(&mut chapter.gmap_spot_state))
            })
            .field("Encount", |ui, chapter| {
                ui.add(chapter_encount_type(&mut chapter.gmap_spot_encount))
            })
            .new_section("Sound")
            .field("Field Situation", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.sound_field_situation)
            })
            .field("Player Phase BGM", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.player_phase_bgm)
            })
            .field("Enemy Phase BGM", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.enemy_phase_bgm)
            })
            .field("Ally Phase BGM", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.ally_phase_bgm)
            })
            .field("Player Encount BGM", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.player_encount_bgm)
            })
            .field("Enemy Encount BGM", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.enemy_encount_bgm)
            })
            .field("Sortie BGM", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.sortie_bgm)
            })
            .field("Kizuna BGM", |ui, chapter| {
                ui.text_edit_singleline(&mut chapter.kizuna_bgm)
            })
            .new_section("Misc.")
            .field("Progress", |ui, chapter| {
                ui.add(u8_drag(&mut chapter.progress))
            })
            .field("Hold Level", |ui, chapter| {
                ui.add(u8_drag(&mut chapter.hold_level))
            })
            .field("Alpha", |ui, chapter| ui.add(f32_drag(&mut chapter.alpha)))
            .field("Net Kill Bonus Index", |ui, chapter| {
                ui.add(u8_drag(&mut chapter.net_kill_bonus_index))
            })
            .field("Net Kill Ranking Index", |ui, chapter| {
                ui.add(u8_drag(&mut chapter.net_ranking_index))
            })
            .show(ui)
            .changed()
    }

    fn dispos_tab_content(&mut self, ctx: &egui::Context, state: &EditorState) {
        if let Some(dispos) = self
            .chapter_state
            .as_ref()
            .and_then(|state| match self.dispos_kind {
                DisposKind::Main => state.dispos.as_ref(),
                DisposKind::Encount => state.encount_dispos.as_ref(),
            })
        {
            self.dispos_content.left_panel(ctx, dispos, state);

            dispos.write(|data| {
                let mut changed = self.dispos_content.right_panel(ctx, data, |ui, spawn| {
                    Self::spawn_property_grid(ui, spawn, state)
                });

                CentralPanel::default().show(ctx, |ui| {
                    if let Some(chapter_terrain) = self
                        .chapter_state
                        .as_ref()
                        .and_then(|state| state.terrain.as_ref())
                    {
                        chapter_terrain.read(|terrain_data| {
                            changed = dispos_grid(
                                ui,
                                terrain_data,
                                state,
                                data,
                                self.dispos_content.selection_mut(),
                                self.coordinate_kind,
                            );
                        });
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.heading("Terrain not found.");
                        });
                    }
                });

                changed
            });
        } else {
            CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.heading("Dispos not found.");
                });
            });
        }
    }

    fn spawn_property_grid(ui: &mut Ui, spawn: &mut Spawn, state: &EditorState) -> bool {
        PropertyGrid::new("spawn", spawn)
            .new_section("Core")
            .field("PID", |ui, spawn| ui.text_edit_singleline(&mut spawn.pid))
            .field("Class", |ui, spawn| {
                state
                    .job
                    .read(|data| ui.add(model_drop_down(data, state, &mut spawn.jid)))
            })
            .field("Skill", |ui, spawn| {
                state
                    .skill
                    .read(|data| ui.add(model_drop_down(data, state, &mut spawn.sid)))
            })
            .field("BID", |ui, spawn| ui.text_edit_singleline(&mut spawn.bid))
            .field("GID", |ui, spawn| {
                state
                    .god
                    .read(|data| ui.add(model_drop_down(data, state, &mut spawn.gid)))
            })
            .field("Force", |ui, spawn| ui.add(i8_drag(&mut spawn.force)))
            .field("Flag", |ui, spawn| {
                ui.add(bitgrid_u16(SPAWN_FLAG_LABELS, 1, &mut spawn.flag))
            })
            .field("Appear X", |ui, spawn| ui.add(i8_drag(&mut spawn.appear_x)))
            .field("Appear Y", |ui, spawn| ui.add(i8_drag(&mut spawn.appear_y)))
            .field("Dispos X", |ui, spawn| ui.add(i8_drag(&mut spawn.dispos_x)))
            .field("Dispos Y", |ui, spawn| ui.add(i8_drag(&mut spawn.dispos_y)))
            .field("Direction", |ui, spawn| {
                ui.add(i8_drag(&mut spawn.direction))
            })
            .field("Level (N)", |ui, spawn| ui.add(u8_drag(&mut spawn.level_n)))
            .field("Level (H)", |ui, spawn| ui.add(u8_drag(&mut spawn.level_h)))
            .field("Level (L)", |ui, spawn| ui.add(u8_drag(&mut spawn.level_l)))
            .field("HP Stock Count", |ui, spawn| {
                ui.add(u8_drag(&mut spawn.hp_stock_count))
            })
            .new_section("Items")
            .field("Item 1", |ui, spawn| {
                state
                    .item
                    .read(|data| ui.add(model_drop_down(data, state, &mut spawn.item_1_iid)))
            })
            .field("Item 1 (Drop)", |ui, spawn| {
                ui.add(i8_drag(&mut spawn.item_1_drop))
            })
            .field("Item 2", |ui, spawn| {
                state
                    .item
                    .read(|data| ui.add(model_drop_down(data, state, &mut spawn.item_2_iid)))
            })
            .field("Item 2 (Drop)", |ui, spawn| {
                ui.add(i8_drag(&mut spawn.item_2_drop))
            })
            .field("Item 3", |ui, spawn| {
                state
                    .item
                    .read(|data| ui.add(model_drop_down(data, state, &mut spawn.item_3_iid)))
            })
            .field("Item 3 (Drop)", |ui, spawn| {
                ui.add(i8_drag(&mut spawn.item_3_drop))
            })
            .field("Item 4", |ui, spawn| {
                state
                    .item
                    .read(|data| ui.add(model_drop_down(data, state, &mut spawn.item_4_iid)))
            })
            .field("Item 4 (Drop)", |ui, spawn| {
                ui.add(i8_drag(&mut spawn.item_4_drop))
            })
            .field("Item 5", |ui, spawn| {
                state
                    .item
                    .read(|data| ui.add(model_drop_down(data, state, &mut spawn.item_5_iid)))
            })
            .field("Item 5 (Drop)", |ui, spawn| {
                ui.add(i8_drag(&mut spawn.item_5_drop))
            })
            .field("Item 6", |ui, spawn| {
                state
                    .item
                    .read(|data| ui.add(model_drop_down(data, state, &mut spawn.item_6_iid)))
            })
            .field("Item 6 (Drop)", |ui, spawn| {
                ui.add(i8_drag(&mut spawn.item_6_drop))
            })
            .new_section("States")
            .field("State 0", |ui, spawn| ui.add(i8_drag(&mut spawn.state_0)))
            .field("State 1", |ui, spawn| ui.add(i8_drag(&mut spawn.state_1)))
            .field("State 2", |ui, spawn| ui.add(i8_drag(&mut spawn.state_2)))
            .field("State 3", |ui, spawn| ui.add(i8_drag(&mut spawn.state_3)))
            .field("State 4", |ui, spawn| ui.add(i8_drag(&mut spawn.state_4)))
            .field("State 5", |ui, spawn| ui.add(i8_drag(&mut spawn.state_5)))
            .new_section("AI")
            .field("AI Action Name", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_action_name)
            })
            .field("AI Action Val", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_action_val)
            })
            .field("AI Mind Name", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_mind_name)
            })
            .field("AI Mind Val", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_mind_val)
            })
            .field("AI Attack Name", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_attack_name)
            })
            .field("AI Attack Val", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_attack_val)
            })
            .field("AI Move Name", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_move_name)
            })
            .field("AI Move Val", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_move_val)
            })
            .field("AI Battle Rate", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_battle_rate)
            })
            .field("AI Priority", |ui, spawn| {
                ui.add(u8_drag(&mut spawn.ai_priority))
            })
            .field("AI Heal Rate A", |ui, spawn| {
                ui.add(i8_drag(&mut spawn.ai_heal_rate_a))
            })
            .field("AI Heal Rate B", |ui, spawn| {
                ui.add(i8_drag(&mut spawn.ai_heal_rate_b))
            })
            .field("AI Band #", |ui, spawn| {
                ui.add(u32_drag(&mut spawn.ai_band_no))
            })
            .field("AI Move Limit", |ui, spawn| {
                ui.text_edit_singleline(&mut spawn.ai_move_limit)
            })
            .field("AI Flag", |ui, spawn| ui.add(u32_drag(&mut spawn.ai_flag)))
            .show(ui)
            .changed()
    }

    fn terrain_tab_content(&mut self, ctx: &egui::Context, state: &EditorState) {
        if let Some(chapter_terrain) = self
            .chapter_state
            .as_ref()
            .and_then(|state| state.terrain.as_ref())
        {
            self.terrain_content.side_panel(ctx, &state.terrain, state);

            CentralPanel::default().show(ctx, |ui| {
                chapter_terrain.write(|terrain_data| {
                    terrain_grid(ui, terrain_data, self.terrain_content.selection(), state)
                });
            });
        } else {
            CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.heading("Terrain not found.");
                });
            });
        }
    }

    fn dialogue_tab_content(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            if let Some(script) = self
                .chapter_state
                .as_ref()
                .and_then(|state| state.message.as_ref())
            {
                msbt_script_editor(ui, script)
            } else {
                ui.centered_and_justified(|ui| {
                    ui.heading("Archive not found.");
                });
            }
        });
    }
}
