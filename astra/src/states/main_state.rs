use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, OnceLock};

use egui_notify::Toasts;
use parking_lot::{Mutex, RwLock};

use astra_core::Astra;

use crate::widgets::{about_modal, config_editor_modal};
use crate::{
    AccessoryEditor, AccessorySheetRetriever, AccessoryShopSheetRetriever, AnimSetEditor,
    AnimSetSheetRetriever, AppConfig, AppState, ArmoryShopSheetRetriever, AssetTableEditor,
    AssetTableSheetRetriever, ChapterEditor, ChapterSheetRetriever, EditorState,
    FleaMarketSheetRetriever, ForgeEditor, ForgeEvolveDataSheetRetriever,
    ForgeExchangeDataSheetRetriever, ForgeImproveDataSheetRetriever, GameParamEditor,
    GameParamSheetRetriever, GodBondLevelDataSheetRetriever, GodDataSheetRetriever, GodEditor,
    GodLevelDataSheetRetriever, ItemEditor, ItemSheetRetriever, ItemShopSheetRetriever, JobEditor,
    JobSheetRetriever, MessageDb, MessageDbWrapper, PersonEditor, PersonSheetRetriever,
    RelianceBonusDataSheetRetriever, RelianceDataSheetRetriever, RelianceEditor,
    RelianceExpDataSheetRetriever, SaveScreen, ScriptManager, SheetHandle, ShopEditor, SkillEditor,
    SkillSheetRetriever, TerrainDataEditor, TerrainDataSheetRetriever, TextDataEditor,
    TextureCache, Theme, NEXT_TAB_SHORTCUT, PREV_TAB_SHORTCUT,
};

static TRANSITION: OnceLock<Mutex<Option<Transition>>> = OnceLock::new();

pub fn queue_transition(transition: Transition) {
    let lock = TRANSITION.get_or_init(|| Mutex::new(None));
    *lock.lock() = Some(transition);
}

#[derive(Debug)]
pub struct Transition {
    screen: Screens,
    index: usize,
}

impl Transition {
    pub fn new(screen: Screens, index: usize) -> Self {
        Self { screen, index }
    }

    pub fn act(&self, state: &mut MainState) {
        let index = Some(self.index);
        state.active_screen = self.screen;
        match self.screen {
            Screens::Accessory => state.accessory_editor.select(index),
            Screens::AnimSet => state.anim_set_editor.select(index),
            Screens::Chapter => state.chapter_editor.select(index),
            Screens::God => state.god_editor.select(index),
            Screens::Item => state.item_editor.select(index),
            Screens::Job => state.job_editor.select(index),
            Screens::Person => state.person_editor.select(index),
            Screens::Skill => state.skill_editor.select(index),
            Screens::Terrain => state.terrain_editor.select(index),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screens {
    Accessory,
    AnimSet,
    AssetTable,
    Chapter,
    Forge,
    God,
    Item,
    Job,
    Param,
    Person,
    Reliance,
    Save,
    Scripts,
    Shop,
    Skill,
    Terrain,
    Text,
}

impl Screens {
    pub fn from_tab_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Screens::Accessory),
            1 => Some(Screens::AnimSet),
            2 => Some(Screens::AssetTable),
            3 => Some(Screens::Chapter),
            4 => Some(Screens::Person),
            5 => Some(Screens::Job),
            6 => Some(Screens::Forge),
            7 => Some(Screens::God),
            8 => Some(Screens::Item),
            9 => Some(Screens::Param),
            10 => Some(Screens::Reliance),
            11 => Some(Screens::Scripts),
            12 => Some(Screens::Shop),
            13 => Some(Screens::Skill),
            14 => Some(Screens::Terrain),
            15 => Some(Screens::Text),
            _ => None,
        }
    }

    pub fn get_tab_index(&self) -> Option<usize> {
        match self {
            Screens::Accessory => Some(0),
            Screens::AnimSet => Some(1),
            Screens::AssetTable => Some(2),
            Screens::Chapter => Some(3),
            Screens::Person => Some(4),
            Screens::Job => Some(5),
            Screens::Forge => Some(6),
            Screens::God => Some(7),
            Screens::Item => Some(8),
            Screens::Param => Some(9),
            Screens::Reliance => Some(10),
            Screens::Save => None,
            Screens::Scripts => Some(11),
            Screens::Shop => Some(12),
            Screens::Skill => Some(13),
            Screens::Terrain => Some(14),
            Screens::Text => Some(15),
        }
    }

    pub fn next_tab(&self) -> Option<Self> {
        self.get_tab_index()
            .and_then(|index| Self::from_tab_index(if index + 1 < 16 { index + 1 } else { 0 }))
    }

    pub fn prev_tab(&self) -> Option<Self> {
        self.get_tab_index()
            .and_then(|index| Self::from_tab_index(if index > 0 { index - 1 } else { 15 }))
    }
}

pub struct MainState {
    editor_state: EditorState,
    active_screen: Screens,
    toasts: Toasts,

    accessory_editor: AccessoryEditor,
    anim_set_editor: AnimSetEditor,
    asset_table_editor: AssetTableEditor,
    chapter_editor: ChapterEditor,
    forge_editor: ForgeEditor,
    god_editor: GodEditor,
    item_editor: ItemEditor,
    job_editor: JobEditor,
    param_editor: GameParamEditor,
    person_editor: PersonEditor,
    reliance_editor: RelianceEditor,
    save_screen: SaveScreen,
    script_manager: ScriptManager,
    shop_editor: ShopEditor,
    skill_editor: SkillEditor,
    terrain_editor: TerrainDataEditor,
    text_data_editor: TextDataEditor,
}

impl MainState {
    pub fn new(
        astra: Arc<RwLock<Astra>>,
        message_db: MessageDb,
        texture_cache: TextureCache,
    ) -> Self {
        let astra_tmp = astra.clone();
        let state = EditorState {
            accessory: SheetHandle::new(astra.read().get_item_book(), AccessorySheetRetriever),
            anim_set: SheetHandle::new(astra.read().get_anim_set_book(), AnimSetSheetRetriever),
            asset_table: SheetHandle::new(
                astra.read().get_asset_table_book(),
                AssetTableSheetRetriever,
            ),
            chapter: SheetHandle::new(astra.read().get_chapter_book(), ChapterSheetRetriever),
            forge_improve: SheetHandle::new(
                astra.read().get_item_book(),
                ForgeImproveDataSheetRetriever,
            ),
            forge_evolve: SheetHandle::new(
                astra.read().get_item_book(),
                ForgeEvolveDataSheetRetriever,
            ),
            forge_exchange: SheetHandle::new(
                astra.read().get_item_book(),
                ForgeExchangeDataSheetRetriever,
            ),
            god: SheetHandle::new(astra.read().get_god_book(), GodDataSheetRetriever),
            god_level_data: SheetHandle::new(
                astra.read().get_god_book(),
                GodLevelDataSheetRetriever,
            ),
            god_bond_level_data: SheetHandle::new(
                astra.read().get_god_book(),
                GodBondLevelDataSheetRetriever,
            ),
            job: SheetHandle::new(astra.read().get_job_book(), JobSheetRetriever),
            param: SheetHandle::new(astra.read().get_param_book(), GameParamSheetRetriever),
            person: SheetHandle::new(astra.read().get_person_book(), PersonSheetRetriever),
            item: SheetHandle::new(astra.read().get_item_book(), ItemSheetRetriever),
            reliance: SheetHandle::new(
                astra.read().get_reliance_book(),
                RelianceDataSheetRetriever,
            ),
            reliance_exp_data: SheetHandle::new(
                astra.read().get_reliance_book(),
                RelianceExpDataSheetRetriever,
            ),
            reliance_bonus_data: SheetHandle::new(
                astra.read().get_reliance_book(),
                RelianceBonusDataSheetRetriever,
            ),
            armory_shop: SheetHandle::new(astra.read().get_shop_book(), ArmoryShopSheetRetriever),
            item_shop: SheetHandle::new(astra.read().get_shop_book(), ItemShopSheetRetriever),
            flea_market: SheetHandle::new(astra.read().get_shop_book(), FleaMarketSheetRetriever),
            accessory_shop: SheetHandle::new(
                astra.read().get_shop_book(),
                AccessoryShopSheetRetriever,
            ),
            skill: SheetHandle::new(astra.read().get_skill_book(), SkillSheetRetriever),
            spawns: Default::default(),
            terrain: SheetHandle::new(astra.read().get_terrain_book(), TerrainDataSheetRetriever),
            message_db: MessageDbWrapper::new(message_db),
            texture_cache: Rc::new(RefCell::new(texture_cache)),
            astra: astra_tmp,
        };
        Self {
            accessory_editor: AccessoryEditor::new(&state),
            anim_set_editor: AnimSetEditor::new(&state),
            asset_table_editor: AssetTableEditor::new(&state),
            chapter_editor: ChapterEditor::new(&state),
            forge_editor: ForgeEditor::new(&state),
            god_editor: GodEditor::new(&state),
            item_editor: ItemEditor::new(&state),
            param_editor: GameParamEditor::new(&state),
            person_editor: PersonEditor::new(&state),
            job_editor: JobEditor::new(&state),
            reliance_editor: RelianceEditor::new(&state),
            shop_editor: ShopEditor::new(&state),
            skill_editor: SkillEditor::new(&state),
            terrain_editor: TerrainDataEditor::new(&state),
            text_data_editor: TextDataEditor::new(&state),
            editor_state: state,
            save_screen: SaveScreen::new(astra.clone()),
            script_manager: ScriptManager::new(astra),
            active_screen: Screens::Person,
            toasts: Toasts::default(),
        }
    }

    fn on_leave_tab(&mut self, prev: Screens) {
        #[allow(clippy::single_match)]
        match prev {
            Screens::Text => self.text_data_editor.on_leave(&self.editor_state),
            _ => {}
        }
    }
}

pub fn main_window(
    state: &mut MainState,
    next_state: &mut Option<AppState>,
    config: &mut AppConfig,
    ctx: &egui::Context,
) {
    let about_modal = about_modal(ctx);
    let config_editor_modal = config_editor_modal(ctx, config);

    if let Some(lock) = TRANSITION.get() {
        let mut data = lock.lock();
        if let Some(transition) = &*data {
            transition.act(state);
        }
        *data = None;
    }

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.set_enabled(!matches!(state.active_screen, Screens::Save));
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Save").clicked() {
                    state.save_screen.set_return_screen(state.active_screen);
                    state.active_screen = Screens::Save;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Preferences").clicked() {
                    config_editor_modal.open();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Close").clicked() {
                    // TODO: Prompt before closing?
                    *next_state = Some(AppState::SelectProject);
                    ui.close_menu();
                }
            });
            ui.menu_button("View", |ui| {
                ui.menu_button("Theme", |ui| {
                    if ui
                        .selectable_label(matches!(config.theme, Theme::Latte), "Latte")
                        .clicked()
                    {
                        update_theme(config, ctx, Theme::Latte);
                        ui.close_menu();
                    }
                    if ui
                        .selectable_label(matches!(config.theme, Theme::Frappe), "Frappé")
                        .clicked()
                    {
                        update_theme(config, ctx, Theme::Frappe);
                        ui.close_menu();
                    }
                    if ui
                        .selectable_label(matches!(config.theme, Theme::Macchiato), "Macchiato")
                        .clicked()
                    {
                        update_theme(config, ctx, Theme::Macchiato);
                        ui.close_menu();
                    }
                    if ui
                        .selectable_label(matches!(config.theme, Theme::Mocha), "Mocha")
                        .clicked()
                    {
                        update_theme(config, ctx, Theme::Mocha);
                        ui.close_menu();
                    }
                });
            });
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    about_modal.open();
                    ui.close_menu();
                }
            });
        });
        ui.separator();
        let prev = state.active_screen;
        ui.horizontal_wrapped(|ui| {
            ui.selectable_value(&mut state.active_screen, Screens::Accessory, "Accessory");
            ui.selectable_value(&mut state.active_screen, Screens::AnimSet, "Anim Set");
            ui.selectable_value(&mut state.active_screen, Screens::AssetTable, "Asset Table");
            ui.selectable_value(&mut state.active_screen, Screens::Chapter, "Chapters");
            ui.selectable_value(&mut state.active_screen, Screens::Person, "Characters");
            ui.selectable_value(&mut state.active_screen, Screens::Job, "Classes");
            ui.selectable_value(&mut state.active_screen, Screens::Forge, "Forge");
            ui.selectable_value(&mut state.active_screen, Screens::God, "God");
            ui.selectable_value(&mut state.active_screen, Screens::Item, "Items");
            ui.selectable_value(&mut state.active_screen, Screens::Param, "Param");
            ui.selectable_value(&mut state.active_screen, Screens::Reliance, "Reliance");
            ui.selectable_value(&mut state.active_screen, Screens::Scripts, "Scripts");
            ui.selectable_value(&mut state.active_screen, Screens::Shop, "Shop");
            ui.selectable_value(&mut state.active_screen, Screens::Skill, "Skills");
            ui.selectable_value(&mut state.active_screen, Screens::Terrain, "Terrain");
            ui.selectable_value(&mut state.active_screen, Screens::Text, "Text");
        });
        if state.active_screen != prev {
            state.on_leave_tab(prev);
        }
        match state.active_screen {
            Screens::Chapter => state.chapter_editor.tab_strip(ui, &mut state.editor_state),
            Screens::Forge => state.forge_editor.tab_strip(ui),
            Screens::God => state.god_editor.tab_strip(ui),
            Screens::Reliance => state.reliance_editor.tab_strip(ui),
            Screens::Shop => state.shop_editor.tab_strip(ui),
            _ => {}
        }
    });

    ctx.input_mut(|input| {
        if input.consume_shortcut(&PREV_TAB_SHORTCUT) {
            if let Some(screen) = state.active_screen.prev_tab() {
                state.active_screen = screen;
            }
        } else if input.consume_shortcut(&NEXT_TAB_SHORTCUT) {
            if let Some(screen) = state.active_screen.next_tab() {
                state.active_screen = screen;
            }
        }
    });

    match &mut state.active_screen {
        Screens::Accessory => state.accessory_editor.show(ctx, &mut state.editor_state),
        Screens::AnimSet => state.anim_set_editor.show(ctx),
        Screens::AssetTable => state.asset_table_editor.show(ctx, &state.editor_state),
        Screens::Chapter => state
            .chapter_editor
            .show(ctx, &mut state.editor_state, config),
        Screens::Forge => state.forge_editor.show(ctx, &mut state.editor_state),
        Screens::God => state.god_editor.show(ctx, &mut state.editor_state),
        Screens::Item => state.item_editor.show(ctx, &mut state.editor_state),
        Screens::Job => state.job_editor.show(ctx, &mut state.editor_state),
        Screens::Param => state.param_editor.show(ctx),
        Screens::Person => state.person_editor.show(ctx, &mut state.editor_state),
        Screens::Reliance => state.reliance_editor.show(ctx),
        Screens::Save => state
            .save_screen
            .ui(&mut state.active_screen, ctx, &mut state.toasts),
        Screens::Scripts => state.script_manager.ui(ctx),
        Screens::Shop => state.shop_editor.show(ctx, &mut state.editor_state),
        Screens::Skill => state.skill_editor.show(ctx, &mut state.editor_state),
        Screens::Terrain => state.terrain_editor.show(ctx, &mut state.editor_state),
        Screens::Text => state.text_data_editor.show(ctx, &mut state.editor_state, config),
    }

    state.toasts.show(ctx);
}

fn update_theme(config: &mut AppConfig, ctx: &egui::Context, new_theme: Theme) {
    config.theme = new_theme;
    catppuccin_egui::set_theme(ctx, new_theme.into());
}
