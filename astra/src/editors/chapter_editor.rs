use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::Arc;

use astra_core::{Astra, OpenTerrain};
use astra_types::{Chapter, ChapterBook, Spawn, TerrainData};
use egui::{Button, CentralPanel, ComboBox, Slider, TopBottomPanel, Ui};
use egui_modal::{Icon, Modal};
use indexmap::IndexMap;
use parking_lot::RwLock;

use crate::widgets::{
    bitgrid_i32, bitgrid_u16, chapter_encount_type, chapter_spot_state, force_drop_down, id_field,
    keyed_add_modal_content,
};
use crate::{
    blank_slate, dispos_grid, editor_tab_strip, f32_drag, i8_drag, indexed_model_drop_down,
    model_drop_down, msbt_key_value_singleline, terrain_grid, u32_drag, u8_drag, AppConfig,
    CacheItem, CachedView, ChapterSheet, ChapterSheetRetriever, EditorState, GroupEditorContent,
    ListEditorContent, PropertyGrid, SheetHandle, SpawnSheet, SpawnSheetRetriever,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    All,
    Normal,
    Hard,
    Lunatic,
}

struct OpenChapterState {
    dispos: Option<SpawnSheet>,
    encount_dispos: Option<SpawnSheet>,
    terrain: Option<OpenTerrain>,
    script: String,
    encount_script: String,
    kizuna_script: String,
}

impl OpenChapterState {
    pub fn load(
        chapter: &Chapter,
        astra: &mut Astra,
        spawn_cache: &mut HashMap<String, SpawnSheet>,
    ) -> Self {
        let cid_part = chapter.cid.trim_start_matches("CID_");
        let dispos_stem = chapter.dispos.replace('*', cid_part).to_lowercase();
        let encount_stem = format!("{}e", dispos_stem);
        let terrain =
            astra.get_chapter_terrain(&chapter.terrain.replace('*', cid_part).to_lowercase());
        Self {
            dispos: load_dispos_sheet(spawn_cache, astra, dispos_stem),
            encount_dispos: load_dispos_sheet(spawn_cache, astra, encount_stem),
            terrain,
            script: chapter.script_bmap.replace('*', cid_part).to_lowercase(),
            encount_script: chapter.script_encount.replace('*', cid_part).to_lowercase(),
            kizuna_script: chapter.script_kizuna.replace('*', cid_part).to_lowercase(),
        }
    }
}

fn load_dispos_sheet(
    cache: &mut HashMap<String, SpawnSheet>,
    astra: &mut Astra,
    key: String,
) -> Option<SpawnSheet> {
    if let Entry::Vacant(e) = cache.entry(key.clone()) {
        let dispos = astra.get_dispos(&key)?;
        let sheet = SheetHandle::new(dispos, SpawnSheetRetriever);
        // TODO: We clone today and it's fine, but I'm not sure if this is safe long term...
        e.insert(sheet.clone());
        Some(sheet)
    } else {
        cache.get(&key).cloned()
    }
}

#[derive(Default)]
enum ChapterLoader {
    #[default]
    NoChapterSelected,
    Loading(Receiver<Option<OpenChapterState>>),
    Loaded(Option<OpenChapterState>),
    Error(String),
}

impl ChapterLoader {
    pub fn load(&mut self, state: &mut EditorState, chapter_index: Option<usize>) {
        *self = match chapter_index {
            Some(chapter_index) => {
                let (sender, receiver) = std::sync::mpsc::channel();
                let spawn_cache = state.spawns.clone();
                let astra = state.astra.clone();
                std::thread::spawn(move || {
                    let mut spawn_cache = spawn_cache.write();
                    let mut astra = astra.write();
                    let chapter = astra.get_chapter_book();
                    sender.send(chapter.read(|data| {
                        data.chapters
                            .data
                            .get_index(chapter_index)
                            .map(|(_, chapter)| {
                                OpenChapterState::load(chapter, &mut astra, &mut spawn_cache)
                            })
                    }))
                });
                Self::Loading(receiver)
            }
            None => Self::NoChapterSelected,
        };
    }

    pub fn update(&mut self) {
        if let Self::Loading(receiver) = self {
            match receiver.try_recv() {
                Ok(result) => *self = Self::Loaded(result),
                Err(err) => {
                    if let TryRecvError::Disconnected = err {
                        *self = Self::Error("Thread disconnected unexpectedly.".to_string());
                    }
                }
            }
        }
    }

    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading(_))
    }
}

pub struct ChapterEditor {
    tab: Tab,
    dispos_kind: DisposKind,
    coordinate_kind: CoordinateKind,
    dispos_difficulty: Difficulty,
    hovered_tile: Option<String>,
    hovered_spawn: Option<String>,
    script_open_error: Option<String>,
    selected_chapter_index: Option<usize>,

    terrain_content: ListEditorContent<IndexMap<String, TerrainData>, TerrainData, EditorState>,
    dispos_content: GroupEditorContent,

    astra: Arc<RwLock<Astra>>,
    chapter: ChapterSheet,
    cache: CachedView<ChapterSheetRetriever, ChapterBook, Chapter>,
    loader: ChapterLoader,
}

impl ChapterEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Core,
            dispos_kind: DisposKind::Main,
            coordinate_kind: CoordinateKind::Dispos,
            dispos_difficulty: Difficulty::All,
            hovered_tile: None,
            hovered_spawn: None,
            script_open_error: None,
            selected_chapter_index: None,

            terrain_content: ListEditorContent::new("chapter_terrain_list_editor")
                .with_add_modal_content(keyed_add_modal_content),
            dispos_content: GroupEditorContent::new("chapter_dispos_group_editor"),

            astra: state.astra.clone(),
            chapter: state.chapter.clone(),
            cache: CachedView::new(state.chapter.clone(), state),
            loader: Default::default(),
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected_chapter_index = index;
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState, config: &mut AppConfig) {
        self.loader.update();

        if self.loader.is_loading() {
            CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Spinner::new().size(96.0));
                });
            });
            return;
        }

        if self.selected_chapter_index.is_some() {
            let confirm_delete_modal = Modal::new(ctx, "chapter_delete_confirm_modal");
            confirm_delete_modal.show(|ui| {
                confirm_delete_modal.title(ui, "Confirm Delete");
                confirm_delete_modal.body_and_icon(
                    ui,
                    "Are you sure you want to delete this chapter?",
                    Icon::Error,
                );
                confirm_delete_modal.buttons(ui, |ui| {
                    confirm_delete_modal.button(ui, "Cancel");
                    if confirm_delete_modal.caution_button(ui, "Confirm").clicked() {
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
                        self.loader.load(state, self.selected_chapter_index);
                    }
                });
            });
        }

        if !matches!(self.loader, ChapterLoader::Loaded(_)) {
            CentralPanel::default().show(ctx, |ui| {
                blank_slate(ui);
            });
            return;
        }

        self.cache.refresh(state);

        match self.tab {
            Tab::Core => self.core_tab_content(ctx, state, config),
            Tab::Dispos => self.dispos_tab_content(ctx, state, config),
            Tab::Terrain => self.terrain_tab_content(ctx, state, config),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui, state: &mut EditorState) {
        ui.set_enabled(!self.loader.is_loading());
        editor_tab_strip(ui, |ui| {
            if ui
                .add_enabled(
                    self.selected_chapter_index.is_some(),
                    Button::new("-").min_size([30., 0.].into()),
                )
                .clicked()
            {
                let modal = Modal::new(ui.ctx(), "chapter_delete_confirm_modal");
                modal.open();
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
                    self.loader.load(state, self.selected_chapter_index);
                }
            });
            ui.selectable_value(&mut self.tab, Tab::Core, "Core");
            ui.selectable_value(&mut self.tab, Tab::Dispos, "Dispos");
            ui.selectable_value(&mut self.tab, Tab::Terrain, "Terrain");
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
            ComboBox::from_id_source("dispos_difficulty_drop_down")
                .selected_text(match self.dispos_difficulty {
                    Difficulty::All => "All Difficulties",
                    Difficulty::Normal => "Normal",
                    Difficulty::Hard => "Hard",
                    Difficulty::Lunatic => "Lunatic",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.dispos_difficulty,
                        Difficulty::All,
                        "All Difficulties",
                    );
                    ui.selectable_value(&mut self.dispos_difficulty, Difficulty::Normal, "Normal");
                    ui.selectable_value(&mut self.dispos_difficulty, Difficulty::Hard, "Hard");
                    ui.selectable_value(
                        &mut self.dispos_difficulty,
                        Difficulty::Lunatic,
                        "Lunatic",
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
        let (script, encount_script, kizuna_script) = match &self.loader {
            ChapterLoader::Loaded(Some(state)) => (
                Some(state.script.as_str()),
                Some(state.encount_script.as_str()),
                Some(state.kizuna_script.as_str()),
            ),
            _ => (None, None, None),
        };

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
            .add_enabled(can_open_script(script_name, config), Button::new(label))
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

    fn dispos_tab_content(
        &mut self,
        ctx: &egui::Context,
        state: &EditorState,
        config: &mut AppConfig,
    ) {
        let dispos = match &self.loader {
            ChapterLoader::Loaded(Some(state)) => match self.dispos_kind {
                DisposKind::Main => state.dispos.as_ref(),
                DisposKind::Encount => state.encount_dispos.as_ref(),
            },
            _ => None,
        };
        if let Some(dispos) = dispos {
            TopBottomPanel::bottom("dispos_bottom_panel").show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    ui.label("Tile Brightness");
                    ui.add(Slider::new(&mut config.terrain_brightness, 0.0..=1.0));
                    if let Some(tile) = self.hovered_tile.as_deref() {
                        ui.label(format!("Tile: {}", tile));
                    }
                    if let Some(spawn) = self.hovered_spawn.as_deref() {
                        ui.label(format!("Spawn: {}", spawn));
                    }
                });
            });

            self.dispos_content.left_panel(ctx, dispos, state);

            dispos.write(|data| {
                let mut changed = self.dispos_content.right_panel(ctx, data, |ui, spawn| {
                    Self::spawn_property_grid(ui, spawn, state)
                });

                CentralPanel::default().show(ctx, |ui| {
                    let terrain = match &self.loader {
                        ChapterLoader::Loaded(Some(state)) => state.terrain.as_ref(),
                        _ => None,
                    };
                    if let Some(chapter_terrain) = terrain {
                        chapter_terrain.read(|terrain_data| {
                            let result = dispos_grid(
                                ui,
                                terrain_data,
                                state,
                                data,
                                self.dispos_content.selection_mut(),
                                self.coordinate_kind,
                                self.dispos_difficulty,
                                config,
                            );
                            changed |= result.changed;
                            self.hovered_tile = result.hovered_tile;
                            self.hovered_spawn = result.hovered_spawn;
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
            .field("Force", |ui, spawn| {
                ui.add(force_drop_down(&mut spawn.force))
            })
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

    fn terrain_tab_content(
        &mut self,
        ctx: &egui::Context,
        state: &EditorState,
        config: &mut AppConfig,
    ) {
        let terrain = match &self.loader {
            ChapterLoader::Loaded(Some(state)) => state.terrain.as_ref(),
            _ => None,
        };
        if let Some(chapter_terrain) = terrain {
            TopBottomPanel::bottom("dispos_bottom_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Tile Brightness");
                    ui.add(Slider::new(&mut config.terrain_brightness, 0.0..=1.0));
                    if let Some(tile) = self.hovered_tile.as_deref() {
                        ui.label(format!("Tile: {}", tile));
                    }
                });
            });

            self.terrain_content.left_panel(ctx, &state.terrain, state);

            CentralPanel::default().show(ctx, |ui| {
                chapter_terrain.write(|terrain_data| {
                    let result = terrain_grid(
                        ui,
                        terrain_data,
                        self.terrain_content.selection(),
                        state,
                        config,
                    );
                    self.hovered_tile = result.hovered_tile;
                    result.changed
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
}
