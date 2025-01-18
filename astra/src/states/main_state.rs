use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, OnceLock};

use egui_notify::Toasts;
use parking_lot::{Mutex, RwLock};

use astra_core::{Astra, RomSource};

use crate::widgets::{about_modal, config_editor_modal};
use crate::{
    AccessoryEditor, AchieveEditor, AiEditor, AnimSetEditor, AnimalEditor, AppConfig, AppState,
    ArenaEditor, AssetTableEditor, CalculatorEditor, ChapterEditor, ChartEditor, CookEditor,
    DragonRideEditor, EditorState, EffectEditor, EncountEditor, FishingFishEditor, ForgeEditor,
    FriendListEditor, GameParamEditor, GodDataSheetRetriever, GodEditor, HubAreaEditor, ItemEditor,
    JobEditor, KillBonusEditor, LaterTalkEditor, MapEditorEditor, MascotEditor, MessageDb,
    MessageDbWrapper, MiscEditor, MovieEditor, MuscleExerciseDataEditor, MusicEditor, PersonEditor,
    PhotographSpotEditor, ProfileCardEditor, RelayEditor, RelianceEditor, RingEditor, SaveScreen,
    ScriptManager, SheetHandle, ShopEditor, SkillEditor, TerrainDataEditor, TextDataEditor,
    TextureCache, Theme, TitleEditor, TutorialEditor, NEXT_TAB_SHORTCUT, PREV_TAB_SHORTCUT,
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
    Achieve,
    Ai,
    AnimSet,
    Animal,
    Arena,
    AssetTable,
    Calculator,
    Chart,
    Chapter,
    Cook,
    DragonRide,
    Effect,
    Encount,
    Fishing,
    Forge,
    FriendList,
    God,
    Hub,
    Item,
    Job,
    KillBonus,
    LaterTalk,
    MapEditor,
    Mascot,
    Misc,
    Movie,
    MuscleExercise,
    Music,
    Param,
    Person,
    Photograph,
    ProfileCard,
    Relay,
    Reliance,
    Ring,
    Save,
    Scripts,
    Shop,
    Skill,
    Terrain,
    Text,
    Title,
    Tutorial,
}

impl Screens {
    pub fn from_tab_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Screens::Accessory),
            1 => Some(Screens::Achieve),
            2 => Some(Screens::Ai),
            3 => Some(Screens::AnimSet),
            4 => Some(Screens::Animal),
            5 => Some(Screens::Arena),
            6 => Some(Screens::AssetTable),
            7 => Some(Screens::Calculator),
            8 => Some(Screens::Chapter),
            9 => Some(Screens::Person),
            10 => Some(Screens::Chart),
            11 => Some(Screens::Job),
            12 => Some(Screens::Cook),
            13 => Some(Screens::Effect),
            14 => Some(Screens::Encount),
            15 => Some(Screens::MuscleExercise),
            16 => Some(Screens::Fishing),
            17 => Some(Screens::Forge),
            18 => Some(Screens::FriendList),
            19 => Some(Screens::God),
            20 => Some(Screens::Hub),
            21 => Some(Screens::Item),
            22 => Some(Screens::KillBonus),
            23 => Some(Screens::MapEditor),
            24 => Some(Screens::Mascot),
            25 => Some(Screens::Misc),
            26 => Some(Screens::Movie),
            27 => Some(Screens::Music),
            28 => Some(Screens::Param),
            29 => Some(Screens::Photograph),
            30 => Some(Screens::LaterTalk),
            31 => Some(Screens::ProfileCard),
            32 => Some(Screens::Relay),
            33 => Some(Screens::Reliance),
            34 => Some(Screens::Ring),
            35 => Some(Screens::Scripts),
            36 => Some(Screens::Shop),
            37 => Some(Screens::Skill),
            38 => Some(Screens::Terrain),
            39 => Some(Screens::Text),
            40 => Some(Screens::Title),
            41 => Some(Screens::Tutorial),
            42 => Some(Screens::DragonRide),
            _ => None,
        }
    }

    pub fn get_tab_index(&self) -> Option<usize> {
        match self {
            Screens::Accessory => Some(0),
            Screens::Achieve => Some(1),
            Screens::Ai => Some(2),
            Screens::AnimSet => Some(3),
            Screens::Animal => Some(4),
            Screens::Arena => Some(5),
            Screens::AssetTable => Some(6),
            Screens::Calculator => Some(7),
            Screens::Chapter => Some(8),
            Screens::Person => Some(9),
            Screens::Chart => Some(10),
            Screens::Job => Some(11),
            Screens::Cook => Some(12),
            Screens::Effect => Some(13),
            Screens::Encount => Some(14),
            Screens::MuscleExercise => Some(15),
            Screens::Fishing => Some(16),
            Screens::Forge => Some(17),
            Screens::FriendList => Some(18),
            Screens::God => Some(19),
            Screens::Hub => Some(20),
            Screens::Item => Some(21),
            Screens::KillBonus => Some(22),
            Screens::MapEditor => Some(23),
            Screens::Mascot => Some(24),
            Screens::Misc => Some(25),
            Screens::Movie => Some(26),
            Screens::Music => Some(27),
            Screens::Param => Some(28),
            Screens::Photograph => Some(29),
            Screens::LaterTalk => Some(30),
            Screens::ProfileCard => Some(31),
            Screens::Relay => Some(32),
            Screens::Reliance => Some(33),
            Screens::Ring => Some(34),
            Screens::Save => None,
            Screens::Scripts => Some(35),
            Screens::Shop => Some(36),
            Screens::Skill => Some(37),
            Screens::Terrain => Some(38),
            Screens::Text => Some(39),
            Screens::Title => Some(40),
            Screens::Tutorial => Some(41),
            Screens::DragonRide => Some(42),
        }
    }

    pub fn next_tab(&self) -> Option<Self> {
        self.get_tab_index()
            .and_then(|index| Self::from_tab_index(if index + 1 < 43 { index + 1 } else { 0 }))
    }

    pub fn prev_tab(&self) -> Option<Self> {
        self.get_tab_index()
            .and_then(|index| Self::from_tab_index(if index > 0 { index - 1 } else { 42 }))
    }
}

pub struct MainState {
    editor_state: EditorState,
    active_screen: Screens,
    toasts: Toasts,

    accessory_editor: AccessoryEditor,
    achieve_editor: AchieveEditor,
    ai_editor: AiEditor,
    anim_set_editor: AnimSetEditor,
    animal_editor: AnimalEditor,
    arena_editor: ArenaEditor,
    asset_table_editor: AssetTableEditor,
    calculator_editor: CalculatorEditor,
    chart_editor: ChartEditor,
    chapter_editor: ChapterEditor,
    cook_editor: CookEditor,
    dragon_ride_editor: DragonRideEditor,
    effect_editor: EffectEditor,
    encount_editor: EncountEditor,
    fishing_editor: FishingFishEditor,
    forge_editor: ForgeEditor,
    friend_list_editor: FriendListEditor,
    god_editor: GodEditor,
    hub_editor: HubAreaEditor,
    item_editor: ItemEditor,
    job_editor: JobEditor,
    kill_bonus_editor: KillBonusEditor,
    later_talk_editor: LaterTalkEditor,
    map_editor_editor: MapEditorEditor,
    mascot_editor: MascotEditor,
    misc_editor: MiscEditor,
    movie_editor: MovieEditor,
    muscle_exercise_editor: MuscleExerciseDataEditor,
    music_editor: MusicEditor,
    param_editor: GameParamEditor,
    person_editor: PersonEditor,
    photograph_editor: PhotographSpotEditor,
    profile_card_editor: ProfileCardEditor,
    relay_editor: RelayEditor,
    reliance_editor: RelianceEditor,
    ring_editor: RingEditor,
    save_screen: SaveScreen,
    script_manager: ScriptManager,
    shop_editor: ShopEditor,
    skill_editor: SkillEditor,
    terrain_editor: TerrainDataEditor,
    text_data_editor: TextDataEditor,
    title_editor: TitleEditor,
    tutorial_editor: TutorialEditor,
}

impl MainState {
    pub fn new(
        astra: Arc<RwLock<Astra>>,
        message_db: MessageDb,
        texture_cache: TextureCache,
    ) -> Self {
        let astra_tmp = astra.clone();
        let state = EditorState {
            accessory: SheetHandle::new(astra.read().get_item_book(), Default::default()),
            achieve: SheetHandle::new(astra.read().get_achieve_book(), Default::default()),
            ai: SheetHandle::new(astra.read().get_ai_book(), Default::default()),
            amiibo: SheetHandle::new(astra.read().get_amiibo_book(), Default::default()),
            anim_set: SheetHandle::new(astra.read().get_anim_set_book(), Default::default()),
            animal: SheetHandle::new(astra.read().get_animal_book(), Default::default()),
            arena: SheetHandle::new(astra.read().get_arena_book(), Default::default()),
            asset_table: SheetHandle::new(astra.read().get_asset_table_book(), Default::default()),
            belong: SheetHandle::new(astra.read().get_achieve_book(), Default::default()),
            calculator: SheetHandle::new(astra.read().get_calculator_book(), Default::default()),
            chapter: SheetHandle::new(astra.read().get_chapter_book(), Default::default()),
            chart: SheetHandle::new(astra.read().get_chart_book(), Default::default()),
            chart_god: SheetHandle::new(astra.read().get_chart_book(), Default::default()),
            chart_param: SheetHandle::new(astra.read().get_chart_book(), Default::default()),
            cook: SheetHandle::new(astra.read().get_cook_book(), Default::default()),
            dragon_ride_presets: SheetHandle::new(
                astra.read().get_dragon_ride_preset_param_book(),
                Default::default(),
            ),
            dragon_ride_prizes: SheetHandle::new(
                astra.read().get_dragon_ride_prize_list_book(),
                Default::default(),
            ),
            dragon_ride_target_patterns: SheetHandle::new(
                astra.read().get_dragon_ride_target_pattern_book(),
                Default::default(),
            ),
            effect: SheetHandle::new(astra.read().get_effect_book(), Default::default()),
            effect_sequence: SheetHandle::new(astra.read().get_effect_book(), Default::default()),
            encount_equipment: SheetHandle::new(
                astra.read().get_encount_book(),
                Default::default(),
            ),
            encount_weapon_categories: SheetHandle::new(
                astra.read().get_encount_book(),
                Default::default(),
            ),
            encount_enemy_types: SheetHandle::new(
                astra.read().get_encount_book(),
                Default::default(),
            ),
            encount_rarity_configs: SheetHandle::new(
                astra.read().get_encount_book(),
                Default::default(),
            ),
            end_roll_data: SheetHandle::new(astra.read().get_end_roll_book(), Default::default()),
            exp_table: SheetHandle::new(astra.read().get_calculator_book(), Default::default()),
            fishing_fish_data: SheetHandle::new(
                astra.read().get_fishing_book(),
                Default::default(),
            ),
            fishing_size_data: SheetHandle::new(
                astra.read().get_fishing_book(),
                Default::default(),
            ),
            fish_spawns: SheetHandle::new(astra.read().get_fishing_book(), Default::default()),
            fishing_target_list: SheetHandle::new(
                astra.read().get_fishing_book(),
                Default::default(),
            ),
            fishing_assist_data: SheetHandle::new(
                astra.read().get_fishing_book(),
                Default::default(),
            ),
            fishing_radical_param_data: SheetHandle::new(
                astra.read().get_fishing_book(),
                Default::default(),
            ),
            food: SheetHandle::new(astra.read().get_cook_book(), Default::default()),
            food_naming: SheetHandle::new(astra.read().get_cook_book(), Default::default()),
            forge_improve: SheetHandle::new(astra.read().get_item_book(), Default::default()),
            forge_evolve: SheetHandle::new(astra.read().get_item_book(), Default::default()),
            forge_exchange: SheetHandle::new(astra.read().get_item_book(), Default::default()),
            friend_list_data: SheetHandle::new(
                astra.read().get_friend_list_book(),
                Default::default(),
            ),
            god: SheetHandle::new(astra.read().get_god_book(), GodDataSheetRetriever),
            god_level_data: SheetHandle::new(astra.read().get_god_book(), Default::default()),
            god_bond_level_data: SheetHandle::new(astra.read().get_god_book(), Default::default()),
            ground_attributes: SheetHandle::new(
                astra.read().get_ground_attribute_book(),
                Default::default(),
            ),
            hub_area_data: SheetHandle::new(astra.read().get_hub_area_book(), Default::default()),
            hub_facility_data: SheetHandle::new(
                astra.read().get_hub_area_book(),
                Default::default(),
            ),
            hub_demo_data: SheetHandle::new(astra.read().get_hub_demo_book(), Default::default()),
            hub_spawns: SheetHandle::new(astra.read().get_hub_dispos_book(), Default::default()),
            hub_random_sets: SheetHandle::new(
                astra.read().get_hub_dispos_book(),
                Default::default(),
            ),
            hub_unity_behavior: SheetHandle::new(
                astra.read().get_hub_dispos_book(),
                Default::default(),
            ),
            hub_fortune_telling_data: SheetHandle::new(
                astra.read().get_hub_fortune_telling_book(),
                Default::default(),
            ),
            hub_nation_data: SheetHandle::new(
                astra.read().get_hub_investment_book(),
                Default::default(),
            ),
            hub_material_bonuses: SheetHandle::new(
                astra.read().get_hub_investment_book(),
                Default::default(),
            ),
            hub_ingredient_bonuses: SheetHandle::new(
                astra.read().get_hub_investment_book(),
                Default::default(),
            ),
            hub_animal_bonuses: SheetHandle::new(
                astra.read().get_hub_investment_book(),
                Default::default(),
            ),
            hub_item_bonuses: SheetHandle::new(
                astra.read().get_hub_investment_book(),
                Default::default(),
            ),
            hub_ingredient_bonus_groups: SheetHandle::new(
                astra.read().get_hub_investment_book(),
                Default::default(),
            ),
            hub_animal_bonus_groups: SheetHandle::new(
                astra.read().get_hub_investment_book(),
                Default::default(),
            ),
            hub_map_icon_data: SheetHandle::new(
                astra.read().get_hub_map_icon_book(),
                Default::default(),
            ),
            hub_my_room_data: SheetHandle::new(
                astra.read().get_hub_my_room_book(),
                Default::default(),
            ),
            hub_resources: SheetHandle::new(
                astra.read().get_hub_resource_book(),
                Default::default(),
            ),
            hub_talk_data: SheetHandle::new(astra.read().get_hub_talk_book(), Default::default()),
            hub_relative_data: SheetHandle::new(
                astra.read().get_hub_talk_book(),
                Default::default(),
            ),
            hub_talk_facility_data: SheetHandle::new(
                astra.read().get_hub_talk_book(),
                Default::default(),
            ),
            hub_crystal_data: SheetHandle::new(
                astra.read().get_hub_talk_book(),
                Default::default(),
            ),
            item: SheetHandle::new(astra.read().get_item_book(), Default::default()),
            ingredient: SheetHandle::new(astra.read().get_cook_book(), Default::default()),
            job: SheetHandle::new(astra.read().get_job_book(), Default::default()),
            jukebox_data: SheetHandle::new(astra.read().get_jukebox_book(), Default::default()),
            key_help_data: SheetHandle::new(astra.read().get_key_help_book(), Default::default()),
            kill_bonuses_1: SheetHandle::new(
                astra.read().get_kill_bonus_book(),
                Default::default(),
            ),
            kill_bonuses_2: SheetHandle::new(
                astra.read().get_kill_bonus_book(),
                Default::default(),
            ),
            map_editor_objects: SheetHandle::new(
                astra.read().get_map_editor_book(),
                Default::default(),
            ),
            map_editor_categories: SheetHandle::new(
                astra.read().get_map_editor_book(),
                Default::default(),
            ),
            map_history: SheetHandle::new(astra.read().get_map_history_book(), Default::default()),
            mascot_accessory_data: SheetHandle::new(
                astra.read().get_mascot_book(),
                Default::default(),
            ),
            mascot_color_data: SheetHandle::new(astra.read().get_mascot_book(), Default::default()),
            mascot_param_data: SheetHandle::new(astra.read().get_mascot_book(), Default::default()),
            mascot_food_data: SheetHandle::new(astra.read().get_mascot_book(), Default::default()),
            movies: SheetHandle::new(astra.read().get_movie_book(), Default::default()),
            muscle_exercise_difficulty: SheetHandle::new(
                astra.read().get_muscle_exercise_book(),
                Default::default(),
            ),
            muscle_exercise_setups: SheetHandle::new(
                astra.read().get_muscle_exercise_book(),
                Default::default(),
            ),
            muscle_exercise_prizes: SheetHandle::new(
                astra.read().get_muscle_exercise_book(),
                Default::default(),
            ),
            muscle_exercise_sit_up_fall_data: SheetHandle::new(
                astra.read().get_muscle_exercise_book(),
                Default::default(),
            ),
            muscle_exercise_push_up_speed: SheetHandle::new(
                astra.read().get_muscle_exercise_book(),
                Default::default(),
            ),
            muscle_exercise_squat_judge_area: SheetHandle::new(
                astra.read().get_muscle_exercise_book(),
                Default::default(),
            ),
            muscle_exercise_score_list_data: SheetHandle::new(
                astra.read().get_muscle_exercise_book(),
                Default::default(),
            ),
            muscle_exercise_music_sheets: SheetHandle::new(
                astra.read().get_muscle_exercise_book(),
                Default::default(),
            ),
            muscle_exercise_assist_data: SheetHandle::new(
                astra.read().get_muscle_exercise_book(),
                Default::default(),
            ),
            music: SheetHandle::new(astra.read().get_music_book(), Default::default()),
            param: SheetHandle::new(astra.read().get_param_book(), Default::default()),
            person: SheetHandle::new(astra.read().get_person_book(), Default::default()),
            post_battle_conversations: SheetHandle::new(
                astra.read().get_later_talk_book(),
                Default::default(),
            ),
            photograph_spots: SheetHandle::new(
                astra.read().get_photograph_book(),
                Default::default(),
            ),
            photograph_poses: SheetHandle::new(
                astra.read().get_photograph_book(),
                Default::default(),
            ),
            profile_card_bg: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_frames: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_lettering: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_text_colors: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_stamp_data_1: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_stamp_data_2: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_title: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_favorite_character: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_favorite_map: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_comment: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_favorite_map_editor_theme: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            profile_card_default_comment: SheetHandle::new(
                astra.read().get_profile_card_book(),
                Default::default(),
            ),
            ranges: SheetHandle::new(astra.read().get_range_book(), Default::default()),
            relay_data: SheetHandle::new(astra.read().get_relay_book(), Default::default()),
            relay_stamp_data: SheetHandle::new(astra.read().get_relay_book(), Default::default()),
            relay_clear_award_data: SheetHandle::new(
                astra.read().get_relay_book(),
                Default::default(),
            ),
            relay_award_data: SheetHandle::new(astra.read().get_relay_book(), Default::default()),
            reliance: SheetHandle::new(astra.read().get_reliance_book(), Default::default()),
            reliance_exp_data: SheetHandle::new(
                astra.read().get_reliance_book(),
                Default::default(),
            ),
            reliance_bonus_data: SheetHandle::new(
                astra.read().get_reliance_book(),
                Default::default(),
            ),
            ring_data: SheetHandle::new(astra.read().get_ring_book(), Default::default()),
            ring_polish_voice: SheetHandle::new(
                astra.read().get_ring_cleaning_voice_book(),
                Default::default(),
            ),
            armory_shop: SheetHandle::new(astra.read().get_shop_book(), Default::default()),
            item_shop: SheetHandle::new(astra.read().get_shop_book(), Default::default()),
            flea_market: SheetHandle::new(astra.read().get_shop_book(), Default::default()),
            accessory_shop: SheetHandle::new(astra.read().get_shop_book(), Default::default()),
            skill: SheetHandle::new(astra.read().get_skill_book(), Default::default()),
            sound_events: SheetHandle::new(astra.read().get_sound_event_book(), Default::default()),
            taste: SheetHandle::new(astra.read().get_cook_book(), Default::default()),
            taste_condition: SheetHandle::new(astra.read().get_cook_book(), Default::default()),
            title_call_data: SheetHandle::new(astra.read().get_title_book(), Default::default()),
            title_pedestal_data: SheetHandle::new(
                astra.read().get_title_book(),
                Default::default(),
            ),
            tips: SheetHandle::new(astra.read().get_tutorial_book(), Default::default()),
            tutorials: SheetHandle::new(astra.read().get_tutorial_book(), Default::default()),
            terrain: SheetHandle::new(astra.read().get_terrain_book(), Default::default()),
            vibration_data: SheetHandle::new(astra.read().get_vibration_book(), Default::default()),

            spawns: Default::default(),
            message_db: MessageDbWrapper::new(message_db),
            texture_cache: Rc::new(RefCell::new(texture_cache)),
            astra: astra_tmp,
        };
        Self {
            accessory_editor: AccessoryEditor::new(&state),
            achieve_editor: AchieveEditor::new(&state),
            ai_editor: AiEditor::new(&state),
            anim_set_editor: AnimSetEditor::new(&state),
            animal_editor: AnimalEditor::new(&state),
            arena_editor: ArenaEditor::new(&state),
            asset_table_editor: AssetTableEditor::new(&state),
            calculator_editor: CalculatorEditor::new(&state),
            chart_editor: ChartEditor::new(&state),
            chapter_editor: ChapterEditor::new(&state),
            cook_editor: CookEditor::new(&state),
            dragon_ride_editor: DragonRideEditor::new(&state),
            effect_editor: EffectEditor::new(&state),
            encount_editor: EncountEditor::new(&state),
            fishing_editor: FishingFishEditor::new(&state),
            forge_editor: ForgeEditor::new(&state),
            friend_list_editor: FriendListEditor::new(&state),
            god_editor: GodEditor::new(&state),
            hub_editor: HubAreaEditor::new(&state),
            item_editor: ItemEditor::new(&state),
            job_editor: JobEditor::new(&state),
            kill_bonus_editor: KillBonusEditor::new(&state),
            later_talk_editor: LaterTalkEditor::new(&state),
            map_editor_editor: MapEditorEditor::new(&state),
            mascot_editor: MascotEditor::new(&state),
            misc_editor: MiscEditor::new(&state),
            movie_editor: MovieEditor::new(&state),
            muscle_exercise_editor: MuscleExerciseDataEditor::new(&state),
            music_editor: MusicEditor::new(&state),
            param_editor: GameParamEditor::new(&state),
            person_editor: PersonEditor::new(&state),
            photograph_editor: PhotographSpotEditor::new(&state),
            profile_card_editor: ProfileCardEditor::new(&state),
            relay_editor: RelayEditor::new(&state),
            reliance_editor: RelianceEditor::new(&state),
            ring_editor: RingEditor::new(&state),
            shop_editor: ShopEditor::new(&state),
            skill_editor: SkillEditor::new(&state),
            terrain_editor: TerrainDataEditor::new(&state),
            text_data_editor: TextDataEditor::new(&state),
            title_editor: TitleEditor::new(&state),
            tutorial_editor: TutorialEditor::new(&state),
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
                ui.menu_button("Open", |ui| {
                    let astra = state.editor_state.astra.read();
                    if let RomSource::Directory(path) = &astra.project().rom_source {
                        ui.add_enabled_ui(path.exists(), |ui| {
                            if ui.button("ROM Directory").clicked() {
                                let _ = open::that_detached(path);
                                ui.close_menu();
                            }
                        });
                    }
                    ui.add_enabled_ui(astra.project().output_dir.exists(), |ui| {
                        if ui.button("LayeredFS Directory").clicked() {
                            let _ = open::that_detached(&astra.project().output_dir);
                            ui.close_menu();
                        }
                    });
                    if let Some(path) = &astra.project().cobalt_dir {
                        ui.add_enabled_ui(path.exists(), |ui| {
                            if ui.button("Cobalt Directory").clicked() {
                                let _ = open::that_detached(path);
                                ui.close_menu();
                            }
                        });
                    }
                    ui.add_enabled_ui(astra.project().backup_dir.exists(), |ui| {
                        if ui.button("Backup Directory").clicked() {
                            let _ = open::that_detached(&astra.project().backup_dir);
                            ui.close_menu();
                        }
                    });
                });
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
            ui.selectable_value(&mut state.active_screen, Screens::Achieve, "Achieve");
            ui.selectable_value(&mut state.active_screen, Screens::Ai, "AI");
            ui.selectable_value(&mut state.active_screen, Screens::AnimSet, "Anim Set");
            ui.selectable_value(&mut state.active_screen, Screens::Animal, "Animal");
            ui.selectable_value(&mut state.active_screen, Screens::Arena, "Arena");
            ui.selectable_value(&mut state.active_screen, Screens::AssetTable, "Asset Table");
            ui.selectable_value(&mut state.active_screen, Screens::Calculator, "Calculator");
            ui.selectable_value(&mut state.active_screen, Screens::Chapter, "Chapters");
            ui.selectable_value(&mut state.active_screen, Screens::Person, "Characters");
            ui.selectable_value(&mut state.active_screen, Screens::Chart, "Chart");
            ui.selectable_value(&mut state.active_screen, Screens::Job, "Classes");
            ui.selectable_value(&mut state.active_screen, Screens::Cook, "Cook");
            ui.selectable_value(&mut state.active_screen, Screens::Effect, "Effect");
            ui.selectable_value(&mut state.active_screen, Screens::Encount, "Encount");
            ui.selectable_value(
                &mut state.active_screen,
                Screens::MuscleExercise,
                "Exercise",
            );
            ui.selectable_value(&mut state.active_screen, Screens::Fishing, "Fishing");
            ui.selectable_value(&mut state.active_screen, Screens::Forge, "Forge");
            ui.selectable_value(&mut state.active_screen, Screens::FriendList, "Friend List");
            ui.selectable_value(&mut state.active_screen, Screens::God, "God");
            ui.selectable_value(&mut state.active_screen, Screens::Hub, "Hub");
            ui.selectable_value(&mut state.active_screen, Screens::Item, "Items");
            ui.selectable_value(&mut state.active_screen, Screens::KillBonus, "Kill Bonus");
            ui.selectable_value(&mut state.active_screen, Screens::MapEditor, "Map Editor");
            ui.selectable_value(&mut state.active_screen, Screens::Mascot, "Mascot");
            ui.selectable_value(&mut state.active_screen, Screens::Misc, "Misc.");
            ui.selectable_value(&mut state.active_screen, Screens::Movie, "Movie");
            ui.selectable_value(&mut state.active_screen, Screens::Music, "Music");
            ui.selectable_value(&mut state.active_screen, Screens::Param, "Param");
            ui.selectable_value(&mut state.active_screen, Screens::Photograph, "Photograph");
            ui.selectable_value(&mut state.active_screen, Screens::LaterTalk, "Post Battle");
            ui.selectable_value(&mut state.active_screen, Screens::ProfileCard, "Profile");
            ui.selectable_value(&mut state.active_screen, Screens::Relay, "Relay");
            ui.selectable_value(&mut state.active_screen, Screens::Reliance, "Reliance");
            ui.selectable_value(&mut state.active_screen, Screens::Ring, "Ring");
            ui.selectable_value(&mut state.active_screen, Screens::Scripts, "Scripts");
            ui.selectable_value(&mut state.active_screen, Screens::Shop, "Shop");
            ui.selectable_value(&mut state.active_screen, Screens::Skill, "Skills");
            ui.selectable_value(&mut state.active_screen, Screens::Terrain, "Terrain");
            ui.selectable_value(&mut state.active_screen, Screens::Text, "Text");
            ui.selectable_value(&mut state.active_screen, Screens::Title, "Title");
            ui.selectable_value(&mut state.active_screen, Screens::Tutorial, "Tutorial");
            ui.selectable_value(&mut state.active_screen, Screens::DragonRide, "Wyvern Ride");
        });
        if state.active_screen != prev {
            state.on_leave_tab(prev);
        }
        match state.active_screen {
            Screens::Achieve => state.achieve_editor.tab_strip(ui),
            Screens::Calculator => state.calculator_editor.tab_strip(ui),
            Screens::Chapter => state.chapter_editor.tab_strip(ui, &mut state.editor_state),
            Screens::Chart => state.chart_editor.tab_strip(ui),
            Screens::Cook => state.cook_editor.tab_strip(ui),
            Screens::DragonRide => state.dragon_ride_editor.tab_strip(ui),
            Screens::Effect => state.effect_editor.tab_strip(ui),
            Screens::Encount => state.encount_editor.tab_strip(ui),
            Screens::Fishing => state.fishing_editor.tab_strip(ui),
            Screens::Forge => state.forge_editor.tab_strip(ui),
            Screens::God => state.god_editor.tab_strip(ui),
            Screens::Hub => state.hub_editor.tab_strip(ui),
            Screens::KillBonus => state.kill_bonus_editor.tab_strip(ui),
            Screens::MapEditor => state.map_editor_editor.tab_strip(ui),
            Screens::Mascot => state.mascot_editor.tab_strip(ui),
            Screens::Misc => state.misc_editor.tab_strip(ui),
            Screens::MuscleExercise => state.muscle_exercise_editor.tab_strip(ui),
            Screens::Photograph => state.photograph_editor.tab_strip(ui),
            Screens::ProfileCard => state.profile_card_editor.tab_strip(ui),
            Screens::Relay => state.relay_editor.tab_strip(ui),
            Screens::Reliance => state.reliance_editor.tab_strip(ui),
            Screens::Ring => state.ring_editor.tab_strip(ui),
            Screens::Shop => state.shop_editor.tab_strip(ui),
            Screens::Title => state.title_editor.tab_strip(ui),
            Screens::Tutorial => state.tutorial_editor.tab_strip(ui),
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
        Screens::Achieve => state.achieve_editor.show(ctx, &state.editor_state),
        Screens::Ai => state.ai_editor.show(ctx, &state.editor_state),
        Screens::AnimSet => state.anim_set_editor.show(ctx),
        Screens::Animal => state.animal_editor.show(ctx, &state.editor_state),
        Screens::Arena => state.arena_editor.show(ctx, &state.editor_state),
        Screens::AssetTable => state.asset_table_editor.show(ctx, &state.editor_state),
        Screens::Calculator => state.calculator_editor.show(ctx),
        Screens::Chapter => state
            .chapter_editor
            .show(ctx, &mut state.editor_state, config),
        Screens::Chart => state.chart_editor.show(ctx, &state.editor_state),
        Screens::Cook => state.cook_editor.show(ctx, &state.editor_state),
        Screens::DragonRide => state.dragon_ride_editor.show(ctx, &state.editor_state),
        Screens::Effect => state.effect_editor.show(ctx, &state.editor_state),
        Screens::Encount => state.encount_editor.show(ctx, &state.editor_state),
        Screens::Fishing => state.fishing_editor.show(ctx, &state.editor_state),
        Screens::Forge => state.forge_editor.show(ctx, &mut state.editor_state),
        Screens::FriendList => state.friend_list_editor.show(ctx, &state.editor_state),
        Screens::God => state.god_editor.show(ctx, &mut state.editor_state),
        Screens::Hub => state.hub_editor.show(ctx, &state.editor_state),
        Screens::Item => state.item_editor.show(ctx, &mut state.editor_state),
        Screens::Job => state.job_editor.show(ctx, &mut state.editor_state),
        Screens::KillBonus => state.kill_bonus_editor.show(ctx, &state.editor_state),
        Screens::LaterTalk => state.later_talk_editor.show(ctx, &state.editor_state),
        Screens::MapEditor => state.map_editor_editor.show(ctx, &state.editor_state),
        Screens::Mascot => state.mascot_editor.show(ctx, &state.editor_state),
        Screens::Misc => state.misc_editor.show(ctx, &state.editor_state),
        Screens::Movie => state.movie_editor.show(ctx, &state.editor_state),
        Screens::MuscleExercise => state.muscle_exercise_editor.show(ctx, &state.editor_state),
        Screens::Music => state.music_editor.show(ctx, &state.editor_state),
        Screens::Param => state.param_editor.show(ctx),
        Screens::Person => state.person_editor.show(ctx, &mut state.editor_state),
        Screens::Photograph => state.photograph_editor.show(ctx, &state.editor_state),
        Screens::ProfileCard => state.profile_card_editor.show(ctx, &state.editor_state),
        Screens::Relay => state.relay_editor.show(ctx, &state.editor_state),
        Screens::Reliance => state.reliance_editor.show(ctx),
        Screens::Ring => state.ring_editor.show(ctx, &state.editor_state),
        Screens::Save => state
            .save_screen
            .ui(&mut state.active_screen, ctx, &mut state.toasts),
        Screens::Scripts => state.script_manager.ui(ctx, &config, &mut state.toasts),
        Screens::Shop => state.shop_editor.show(ctx, &mut state.editor_state),
        Screens::Skill => state.skill_editor.show(ctx, &mut state.editor_state),
        Screens::Terrain => state.terrain_editor.show(ctx, &mut state.editor_state),
        Screens::Text => state
            .text_data_editor
            .show(ctx, &mut state.editor_state, config),
        Screens::Title => state.title_editor.show(ctx, &state.editor_state),
        Screens::Tutorial => state.tutorial_editor.show(ctx, &state.editor_state),
    }

    state.toasts.show(ctx);
}

fn update_theme(config: &mut AppConfig, ctx: &egui::Context, new_theme: Theme) {
    config.theme = new_theme;
    catppuccin_egui::set_theme(ctx, new_theme.into());
}
