use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use astra_core::{Astra, OpenBook};
use astra_types::{
    Accessory, AccessoryShopInventory, AnimSet, AnimSetBook, AssetDef, AssetTableBook, Chapter,
    ChapterBook, DisposBook, ForgeEvolveData, ForgeExchangeData, ForgeImproveData, GameParam,
    GodBondLevelData, GodBook, GodData, GodLevelData, Item, ItemBook, Job, JobBook, ParamsBook,
    Person, PersonBook, RelianceBonusData, RelianceBook, RelianceData, RelianceExpData, ShopBook,
    ShopInventory, Skill, SkillBook, Spawn, TerrainBook, TerrainData,
};
use egui::TextureHandle;
use indexmap::IndexMap;
use itertools::Itertools;
use parking_lot::RwLock;

use crate::{
    AchievementSheet, AiSheet, AmiiboSheet, AnimalSheet, ArenaSheet, BelongSheet, CalculatorSheet,
    ChartGodDataSheet, ChartParamSheet, ChartSheet, CookSheet, DecorationKind,
    DragonRidePresetParamSheet, DragonRidePrizeSheet, DragonRideTargetPatternSheet,
    EffectSequenceSheet, EffectSheet, EncountEnemyTypeSheet, EncountEquipmentSheet,
    EncountRarityConfigSheet, EncountWeaponCategorySheet, EndRollDataSheet, ExpTableSheet,
    FishSizeDataSheet, FishSpawnSheet, FishingAssistDataSheet, FishingFishDataSheet,
    FishingRadicalParamDataSheet, FishingTargetListDataSheet, FoodNamingSheet, FoodSheet,
    FriendListDataSheet, GroundAttributeSheet, HubAnimalBonusGroupSheet, HubAnimalBonusSheet,
    HubAreaDataSheet, HubCrystalDataSheet, HubDemoDataSheet, HubFacilityDataSheet,
    HubFortuneTellingDataSheet, HubIngredientBonusGroupSheet, HubIngredientBonusSheet,
    HubItemBonusSheet, HubMapIconDataSheet, HubMaterialBonusSheet, HubMyRoomDataSheet,
    HubNationDataSheet, HubResourceDataSheet, HubSpawnRandomSetSheet, HubSpawnSheet,
    HubTalkDataSheet, HubTalkFacilityDataSheet, HubTalkRelativeDataSheet, HubUnityBehaviorSheet,
    IngredientSheet, JukeboxDataSheet, KeyHelpDataSheet, KeyedViewItem, KillBonus1Sheet,
    KillBonus2Sheet, MapEditorCategorySheet, MapEditorObjectSheet, MapHistorySheet,
    MascotAccessoryDataSheet, MascotColorDataSheet, MascotFoodDataSheet, MascotParamDataSheet,
    MessageDbWrapper, MovieSheet, MuscleAssistDataSheet, MuscleExerciseDifficultySheet,
    MuscleExercisePrizeDataSheet, MuscleExerciseSetupSheet, MusclePushUpSpeedDataSheet,
    MuscleSitUpFallDataSheet, MuscleSquatJudgeAreaDataSheet, MuscleSquatMusicSheetSheet,
    MuscleSquatScoreListDataSheet, MusicDataSheet, PhotographPoseSheet, PhotographSpotSheet,
    PostBattleConversationSheet, ProfileCardCategorizedComponentSheet,
    ProfileCardCategorizedImageComponentSheet, ProfileCardColorComponentSheet,
    ProfileCardDefaultCommentDataSheet, ProfileCardFavoriteMapDataSheet,
    ProfileCardImageComponentSheet, ProfileCardNameComponentSheet, RangeDataSheet,
    RelayAwardDataSheet, RelayClearAwardDataSheet, RelayDataSheet, RelayStampDataSheet,
    RingDataSheet, RingPolishVoiceDataSheet, Screens, SoundEventSheet, TasteConditionSheet,
    TasteSheet, TextureCache, TipDataSheet, TitleCallDataSheet, TitlePedestalDataSheet,
    TutorialDataSheet, VibrationDefineDataSheet, ViewItem,
};

use super::GroupViewItem;

pub struct EditorState {
    pub message_db: MessageDbWrapper,
    pub texture_cache: Rc<RefCell<TextureCache>>,
    pub spawns: Arc<RwLock<HashMap<String, SpawnSheet>>>,
    pub astra: Arc<RwLock<Astra>>,

    pub accessory: AccessorySheet,
    pub accessory_shop: AccessoryShopSheet,
    pub achieve: AchievementSheet,
    pub ai: AiSheet,
    pub amiibo: AmiiboSheet,
    pub anim_set: AnimSetSheet,
    pub animal: AnimalSheet,
    pub arena: ArenaSheet,
    pub armory_shop: ArmoryShopSheet,
    pub asset_table: AssetTableSheet,
    pub belong: BelongSheet,
    pub calculator: CalculatorSheet,
    pub chapter: ChapterSheet,
    pub chart: ChartSheet,
    pub chart_god: ChartGodDataSheet,
    pub chart_param: ChartParamSheet,
    pub cook: CookSheet,
    pub dragon_ride_presets: DragonRidePresetParamSheet,
    pub dragon_ride_prizes: DragonRidePrizeSheet,
    pub dragon_ride_target_patterns: DragonRideTargetPatternSheet,
    pub effect: EffectSheet,
    pub effect_sequence: EffectSequenceSheet,
    pub encount_equipment: EncountEquipmentSheet,
    pub encount_weapon_categories: EncountWeaponCategorySheet,
    pub encount_enemy_types: EncountEnemyTypeSheet,
    pub encount_rarity_configs: EncountRarityConfigSheet,
    pub end_roll_data: EndRollDataSheet,
    pub exp_table: ExpTableSheet,
    pub fishing_fish_data: FishingFishDataSheet,
    pub fishing_size_data: FishSizeDataSheet,
    pub fish_spawns: FishSpawnSheet,
    pub fishing_target_list: FishingTargetListDataSheet,
    pub fishing_assist_data: FishingAssistDataSheet,
    pub fishing_radical_param_data: FishingRadicalParamDataSheet,
    pub flea_market: FleaMarketSheet,
    pub food: FoodSheet,
    pub food_naming: FoodNamingSheet,
    pub forge_improve: ForgeImproveDataSheet,
    pub forge_evolve: ForgeEvolveDataSheet,
    pub forge_exchange: ForgeExchangeDataSheet,
    pub friend_list_data: FriendListDataSheet,
    pub god: GodDataSheet,
    pub god_level_data: GodLevelDataSheet,
    pub god_bond_level_data: GodBondLevelDataSheet,
    pub ground_attributes: GroundAttributeSheet,
    pub hub_area_data: HubAreaDataSheet,
    pub hub_facility_data: HubFacilityDataSheet,
    pub hub_demo_data: HubDemoDataSheet,
    pub hub_spawns: HubSpawnSheet,
    pub hub_random_sets: HubSpawnRandomSetSheet,
    pub hub_unity_behavior: HubUnityBehaviorSheet,
    pub hub_fortune_telling_data: HubFortuneTellingDataSheet,
    pub hub_nation_data: HubNationDataSheet,
    pub hub_material_bonuses: HubMaterialBonusSheet,
    pub hub_ingredient_bonuses: HubIngredientBonusSheet,
    pub hub_animal_bonuses: HubAnimalBonusSheet,
    pub hub_item_bonuses: HubItemBonusSheet,
    pub hub_ingredient_bonus_groups: HubIngredientBonusGroupSheet,
    pub hub_animal_bonus_groups: HubAnimalBonusGroupSheet,
    pub hub_map_icon_data: HubMapIconDataSheet,
    pub hub_my_room_data: HubMyRoomDataSheet,
    pub hub_resources: HubResourceDataSheet,
    pub hub_talk_data: HubTalkDataSheet,
    pub hub_relative_data: HubTalkRelativeDataSheet,
    pub hub_talk_facility_data: HubTalkFacilityDataSheet,
    pub hub_crystal_data: HubCrystalDataSheet,
    pub item: ItemSheet,
    pub item_shop: ItemShopSheet,
    pub ingredient: IngredientSheet,
    pub job: JobSheet,
    pub jukebox_data: JukeboxDataSheet,
    pub key_help_data: KeyHelpDataSheet,
    pub kill_bonuses_1: KillBonus1Sheet,
    pub kill_bonuses_2: KillBonus2Sheet,
    pub map_editor_objects: MapEditorObjectSheet,
    pub map_editor_categories: MapEditorCategorySheet,
    pub map_history: MapHistorySheet,
    pub mascot_accessory_data: MascotAccessoryDataSheet,
    pub mascot_color_data: MascotColorDataSheet,
    pub mascot_param_data: MascotParamDataSheet,
    pub mascot_food_data: MascotFoodDataSheet,
    pub movies: MovieSheet,
    pub muscle_exercise_difficulty: MuscleExerciseDifficultySheet,
    pub muscle_exercise_setups: MuscleExerciseSetupSheet,
    pub muscle_exercise_prizes: MuscleExercisePrizeDataSheet,
    pub muscle_exercise_sit_up_fall_data: MuscleSitUpFallDataSheet,
    pub muscle_exercise_push_up_speed: MusclePushUpSpeedDataSheet,
    pub muscle_exercise_squat_judge_area: MuscleSquatJudgeAreaDataSheet,
    pub muscle_exercise_score_list_data: MuscleSquatScoreListDataSheet,
    pub muscle_exercise_music_sheets: MuscleSquatMusicSheetSheet,
    pub muscle_exercise_assist_data: MuscleAssistDataSheet,
    pub music: MusicDataSheet,
    pub param: GameParamSheet,
    pub person: PersonSheet,
    pub post_battle_conversations: PostBattleConversationSheet,
    pub photograph_spots: PhotographSpotSheet,
    pub photograph_poses: PhotographPoseSheet,
    pub profile_card_bg: ProfileCardImageComponentSheet,
    pub profile_card_frames: ProfileCardImageComponentSheet,
    pub profile_card_lettering: ProfileCardImageComponentSheet,
    pub profile_card_text_colors: ProfileCardColorComponentSheet,
    pub profile_card_stamp_data_1: ProfileCardImageComponentSheet,
    pub profile_card_stamp_data_2: ProfileCardCategorizedImageComponentSheet,
    pub profile_card_title: ProfileCardNameComponentSheet,
    pub profile_card_favorite_character: ProfileCardNameComponentSheet,
    pub profile_card_favorite_map: ProfileCardFavoriteMapDataSheet,
    pub profile_card_comment: ProfileCardCategorizedComponentSheet,
    pub profile_card_favorite_map_editor_theme: ProfileCardCategorizedComponentSheet,
    pub profile_card_default_comment: ProfileCardDefaultCommentDataSheet,
    pub ranges: RangeDataSheet,
    pub relay_data: RelayDataSheet,
    pub relay_stamp_data: RelayStampDataSheet,
    pub relay_clear_award_data: RelayClearAwardDataSheet,
    pub relay_award_data: RelayAwardDataSheet,
    pub reliance: RelianceDataSheet,
    pub reliance_exp_data: RelianceExpDataSheet,
    pub reliance_bonus_data: RelianceBonusDataSheet,
    pub ring_data: RingDataSheet,
    pub ring_polish_voice: RingPolishVoiceDataSheet,
    pub skill: SkillSheet,
    pub sound_events: SoundEventSheet,
    pub taste: TasteSheet,
    pub taste_condition: TasteConditionSheet,
    pub title_call_data: TitleCallDataSheet,
    pub title_pedestal_data: TitlePedestalDataSheet,
    pub tips: TipDataSheet,
    pub tutorials: TutorialDataSheet,
    pub terrain: TerrainDataSheet,
    pub vibration_data: VibrationDefineDataSheet,
}

/// Strategy for retrieving a sheet from its containing book.
pub trait SheetRetriever<B, S> {
    fn retrieve<'a>(&self, book: &'a B) -> &'a S;
    fn retrieve_mut<'a>(&self, book: &'a mut B) -> &'a mut S;
}

/// Utility for editing a sheet contained in some book and tracking changes to it.
pub struct SheetHandle<R, B, S> {
    book: OpenBook<B>,
    retriever: R,
    revision_number: Arc<AtomicUsize>,
    phantom: PhantomData<S>,
}

impl<R, B, S> Clone for SheetHandle<R, B, S>
where
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            book: self.book.clone(),
            retriever: self.retriever.clone(),
            revision_number: self.revision_number.clone(),
            phantom: PhantomData,
        }
    }
}

impl<R, B, S> SheetHandle<R, B, S>
where
    R: SheetRetriever<B, S>,
{
    /// Create a new handle to a sheet from the given book and retriever.
    pub fn new(book: OpenBook<B>, retriever: R) -> Self {
        Self {
            book,
            retriever,
            revision_number: Default::default(),
            phantom: PhantomData,
        }
    }

    /// Perform a read operation on the sheet.
    pub fn read<V>(&self, consumer: impl FnOnce(&S) -> V) -> V {
        self.book
            .read(|book| consumer(self.retriever.retrieve(book)))
    }

    /// Perform a write operation on the sheet.
    /// The operation must return true if the sheet was modified.
    pub fn write(&self, consumer: impl FnOnce(&mut S) -> bool) {
        let mut changed = false;
        self.book.write(|book| {
            changed = consumer(self.retriever.retrieve_mut(book));
        });
        if changed {
            self.book.mark_dirty();
            self.revision_number.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Retrieve the revision number for the sheet.
    /// This is incremented every time a write operation modifies it.
    pub fn revision_number(&self) -> usize {
        self.revision_number.load(Ordering::Relaxed)
    }
}

#[macro_export]
macro_rules! sheet_retriever {
    ($name:ident, $book:ty, $sheet:ident, $con:ty) => {
        paste::paste! {
            #[derive(Debug, Clone, Default)]
            pub struct [<$name SheetRetriever>];

            impl $crate::SheetRetriever<$book, $con> for [<$name SheetRetriever>] {
                fn retrieve<'a>(&self, book: &'a $book) -> &'a $con {
                    &book.$sheet.data
                }

                fn retrieve_mut<'a>(&self, book: &'a mut $book) -> &'a mut $con {
                    &mut book.$sheet.data
                }
            }

            pub type [<$name Sheet>] = $crate::SheetHandle<[<$name SheetRetriever>], $book, $con>;
        }
    };
}

#[macro_export]
macro_rules! standard_keyed_display {
    ($this:ident, $dependencies:ident, $key:ident) => {
        $dependencies
            .message_db
            .message(&$this.name)
            .map(|name| Cow::Owned(format!("{} ({})", name, $this.$key)))
            .unwrap_or_else(|| Cow::Borrowed($this.$key.as_str()))
    };
    ($this:ident, $dependencies:ident, $key:ident, $key_ident:ident) => {
        $dependencies
            .message_db
            .message(&$this.$key_ident)
            .map(|name| Cow::Owned(format!("{} ({})", name, $this.$key)))
            .unwrap_or_else(|| Cow::Borrowed($this.$key.as_str()))
    };
}

sheet_retriever!(Accessory, ItemBook, accessories, IndexMap<String, Accessory>);

impl ViewItem for Accessory {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, aid)
    }

    fn decorated(_: DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        _: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        let decoration_id = if self.mask & 1 == 0 {
            "Face"
        } else {
            "Clothes"
        };
        dependencies
            .texture_cache
            .borrow_mut()
            .get_system(decoration_id)
            .map(|texture| (texture, 0.5))
    }

    fn screen() -> Option<Screens> {
        Some(Screens::Accessory)
    }
}

impl KeyedViewItem for Accessory {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.aid)
    }

    fn set_key(&mut self, key: String) {
        self.aid = key;
    }
}

sheet_retriever!(AnimSet, AnimSetBook, sets, IndexMap<String, AnimSet>);

impl ViewItem for AnimSet {
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    fn screen() -> Option<Screens> {
        Some(Screens::AnimSet)
    }
}

impl KeyedViewItem for AnimSet {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    fn set_key(&mut self, key: String) {
        self.name = key;
    }
}

sheet_retriever!(AssetTable, AssetTableBook, asset_defs, Vec<AssetDef>);

impl ViewItem for AssetDef {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        let condition_text = self.conditions.iter().join(" and ");

        if self.preset_name.is_empty() && condition_text.is_empty() {
            Cow::Borrowed("{unnamed}")
        } else if self.preset_name.is_empty() {
            Cow::Owned(condition_text)
        } else {
            Cow::Owned(format!("{}, ({})", self.preset_name, condition_text))
        }
    }
}

sheet_retriever!(Chapter, ChapterBook, chapters, IndexMap<String, Chapter>);

impl ViewItem for Chapter {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        let name = self.name.replace('*', self.cid.trim_start_matches("CID_"));
        dependencies
            .message_db
            .message(&name)
            .map(|name| Cow::Owned(format!("{} ({})", name, self.cid)))
            .unwrap_or_else(|| Cow::Borrowed(self.cid.as_str()))
    }

    fn screen() -> Option<Screens> {
        Some(Screens::Chapter)
    }
}

impl KeyedViewItem for Chapter {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.cid)
    }

    fn set_key(&mut self, key: String) {
        self.cid = key;
    }
}

sheet_retriever!(ForgeImproveData, ItemBook, improve_data, IndexMap<String, Vec<ForgeImproveData>>);

impl GroupViewItem for IndexMap<String, Vec<ForgeImproveData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, dependencies: &'a Self::Dependencies) -> Cow<'a, str> {
        dependencies.item.read(|data| {
            data.get(&key.replace("RID_", "IID_"))
                .and_then(|item| dependencies.message_db.message(&item.name))
                .map(|name| Cow::Owned(format!("{} ({})", name, key)))
                .unwrap_or_else(|| key.into())
        })
    }

    fn decorated(kind: DecorationKind<'_>) -> bool {
        matches!(kind, DecorationKind::List)
    }

    fn decoration(
        key: &str,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        dependencies.item.read(|data| {
            data.get(&key.replace("RID_", "IID_"))
                .and_then(|item| item.decoration(dependencies, kind))
        })
    }
}

impl ViewItem for ForgeImproveData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!(
            "Mt {}, Wt {}, Hit {}, Crit {}",
            self.power, self.weight, self.hit, self.critical
        ))
    }
}

sheet_retriever!(ForgeEvolveData, ItemBook, evolve_data, IndexMap<String, Vec<ForgeEvolveData>>);

impl GroupViewItem for IndexMap<String, Vec<ForgeEvolveData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, dependencies: &'a Self::Dependencies) -> Cow<'a, str> {
        dependencies.item.read(|data| {
            data.get(&key.replace("EID_", "IID_"))
                .and_then(|item| dependencies.message_db.message(&item.name))
                .map(|name| Cow::Owned(format!("{} ({})", name, key)))
                .unwrap_or_else(|| key.into())
        })
    }

    fn decorated(kind: DecorationKind<'_>) -> bool {
        matches!(kind, DecorationKind::List)
    }

    fn decoration(
        key: &str,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        dependencies.item.read(|data| {
            data.get(&key.replace("EID_", "IID_"))
                .and_then(|item| item.decoration(dependencies, kind))
        })
    }
}

// TODO: Can we make this keyed even though it's in a group
impl ViewItem for ForgeEvolveData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies.item.read(|data| {
            data.get(&self.iid)
                .map(|item| item.text(dependencies).into_owned().into())
                .unwrap_or_else(|| Cow::Borrowed("{unknown item}"))
        })
    }

    fn decorated(kind: DecorationKind<'_>) -> bool {
        matches!(kind, DecorationKind::List)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        if matches!(kind, DecorationKind::List) {
            dependencies.item.read(|data| {
                data.get(&self.iid)
                    .and_then(|item| item.decoration(dependencies, kind))
            })
        } else {
            None
        }
    }
}

sheet_retriever!(
    ForgeExchangeData,
    ItemBook,
    exchange_data,
    Vec<ForgeExchangeData>
);

impl ViewItem for ForgeExchangeData {
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

sheet_retriever!(GodData, GodBook, gods, IndexMap<String, GodData>);

impl ViewItem for GodData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, gid, mid)
    }

    fn decorated(_: DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        _: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        let darkness = self.flag & 0b100 != 0;
        let decoration_id = if self.face_icon_name == "Face_Lueur" {
            "Face_Lueur_Female"
        } else if darkness {
            &self.face_icon_name_darkness
        } else {
            &self.face_icon_name
        };
        dependencies
            .texture_cache
            .borrow_mut()
            .get_map_status(decoration_id)
            .map(|texture| (texture, 1.))
    }

    fn screen() -> Option<Screens> {
        Some(Screens::God)
    }
}

impl KeyedViewItem for GodData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.gid)
    }

    fn set_key(&mut self, key: String) {
        self.gid = key;
    }
}

sheet_retriever!(GodLevelData, GodBook, level_data, IndexMap<String, Vec<GodLevelData>>);

impl GroupViewItem for IndexMap<String, Vec<GodLevelData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, dependencies: &'a Self::Dependencies) -> Cow<'a, str> {
        dependencies.god.read(|data| {
            data.get(&key.replace("GGID_", "GID_"))
                .map(|item| item.text(dependencies).into_owned().into())
                .unwrap_or_else(|| Cow::Borrowed("{unknown item}"))
        })
    }
}

impl ViewItem for GodLevelData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("Level {}", self.level))
    }
}

sheet_retriever!(
    GodBondLevelData,
    GodBook,
    bond_level_data,
    Vec<GodBondLevelData>
);

impl ViewItem for GodBondLevelData {
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("Level {}", self.level))
    }
}

sheet_retriever!(Item, ItemBook, items, IndexMap<String, Item>);

impl ViewItem for Item {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, iid)
    }

    fn decorated(_: DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        let mut texture_cache = dependencies.texture_cache.borrow_mut();
        let decoration = texture_cache
            .get_item(&self.icon)
            .or_else(|| texture_cache.get_item("Vulnerary"))?;
        match kind {
            DecorationKind::Other("portrait") => Some((decoration, 1.)),
            _ => Some((decoration, 0.5)),
        }
    }

    fn screen() -> Option<Screens> {
        Some(Screens::Item)
    }
}

impl KeyedViewItem for Item {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.iid)
    }

    fn set_key(&mut self, key: String) {
        self.iid = key;
    }
}

sheet_retriever!(Job, JobBook, jobs, IndexMap<String, Job>);

impl ViewItem for Job {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, jid)
    }

    fn decorated(_: DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        let mut texture_cache = dependencies.texture_cache.borrow_mut();
        let decoration = texture_cache
            .get_unit(
                "800SoldierMG",
                &self.unit_icon_id_m,
                &self.unit_icon_weapon_id,
            )
            .or_else(|| {
                texture_cache.get_unit(
                    "850SoldierFG",
                    &self.unit_icon_id_f,
                    &self.unit_icon_weapon_id,
                )
            })
            .or_else(|| texture_cache.get_unit("000Dummy", "000Dummy", "Dummy"))?;
        match kind {
            DecorationKind::List | DecorationKind::DropDown => Some((decoration, 1.)),
            DecorationKind::Other("portrait") => Some((decoration, 2.)),
            _ => None,
        }
    }

    fn screen() -> Option<Screens> {
        Some(Screens::Job)
    }
}

impl KeyedViewItem for Job {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.jid)
    }

    fn set_key(&mut self, key: String) {
        self.jid = key;
    }
}

sheet_retriever!(GameParam, ParamsBook, game_params, Vec<GameParam>);

impl ViewItem for GameParam {
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.english)
    }
}

sheet_retriever!(Person, PersonBook, persons, IndexMap<String, Person>);

impl ViewItem for Person {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, pid)
    }

    fn decorated(kind: DecorationKind<'_>) -> bool {
        matches!(kind, DecorationKind::List)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        match kind {
            DecorationKind::List => {
                let mut texture_cache = dependencies.texture_cache.borrow_mut();
                let decoration = dependencies.job.read(|data| {
                    data.get(&self.jid)
                        .and_then(|job| {
                            texture_cache.get_unit(
                                &self.unit_icon_id,
                                if self.gender == 2 {
                                    &job.unit_icon_id_f
                                } else {
                                    &job.unit_icon_id_m
                                },
                                &job.unit_icon_weapon_id,
                            )
                        })
                        .or_else(|| texture_cache.get_unit("000Dummy", "000Dummy", "Dummy"))
                });
                decoration.map(|tex| (tex, 1.))
            }
            DecorationKind::Other("portrait") => dependencies
                .texture_cache
                .borrow_mut()
                .get_facethumb(self.name.trim_start_matches("MPID_"))
                .map(|texture| (texture, 1.)),
            _ => None,
        }
    }

    fn screen() -> Option<Screens> {
        Some(Screens::Person)
    }
}

impl KeyedViewItem for Person {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.pid)
    }

    fn set_key(&mut self, key: String) {
        self.pid = key;
    }
}

sheet_retriever!(RelianceData, RelianceBook, reliance_data, IndexMap<String, RelianceData>);

impl ViewItem for RelianceData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .person
            .read(|data| {
                data.get(&self.pid)
                    .map(|person| Cow::Owned(person.text(dependencies).to_string()))
            })
            .unwrap_or(Cow::Borrowed("{unknown PID}"))
    }
}

impl KeyedViewItem for RelianceData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.pid)
    }

    fn set_key(&mut self, key: String) {
        self.pid = key;
    }
}

sheet_retriever!(RelianceExpData, RelianceBook, reliance_exp_data, IndexMap<String, RelianceExpData>);

impl ViewItem for RelianceExpData {
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.rexid)
    }
}

impl KeyedViewItem for RelianceExpData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.rexid)
    }

    fn set_key(&mut self, key: String) {
        self.rexid = key;
    }
}

sheet_retriever!(RelianceBonusData, RelianceBook, relianace_bonus_data, IndexMap<String, Vec<RelianceBonusData>>);

impl GroupViewItem for IndexMap<String, Vec<RelianceBonusData>> {
    type Dependencies = ();

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for RelianceBonusData {
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("Level {}", self.level))
    }
}

sheet_retriever!(ArmoryShop, ShopBook, armory_shop_inventory, IndexMap<String, Vec<ShopInventory>>);

sheet_retriever!(ItemShop, ShopBook, item_shop_inventory, IndexMap<String, Vec<ShopInventory>>);

sheet_retriever!(FleaMarket, ShopBook, flea_market_shop_inventory, IndexMap<String, Vec<ShopInventory>>);

impl GroupViewItem for IndexMap<String, Vec<ShopInventory>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

// TODO: Make this keyed after adding keyed group support.
impl ViewItem for ShopInventory {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies.item.read(|data| {
            data.get(&self.iid)
                .map(|item| item.text(dependencies).into_owned().into())
                .unwrap_or_else(|| Cow::Borrowed("{unknown item}"))
        })
    }

    fn decorated(kind: DecorationKind<'_>) -> bool {
        matches!(kind, DecorationKind::List)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        matches!(kind, DecorationKind::List)
            .then(|| {
                dependencies.item.read(|data| {
                    data.get(&self.iid)
                        .and_then(|item| item.decoration(dependencies, kind))
                })
            })
            .flatten()
    }
}

sheet_retriever!(AccessoryShop, ShopBook, accessory_shop_inventory, IndexMap<String, Vec<AccessoryShopInventory>>);

impl GroupViewItem for IndexMap<String, Vec<AccessoryShopInventory>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

// TODO: Make this keyed after adding keyed group support.
impl ViewItem for AccessoryShopInventory {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies.accessory.read(|data| {
            data.get(&self.aid)
                .map(|acc| acc.text(dependencies).into_owned().into())
                .unwrap_or_else(|| Cow::Borrowed("{unknown accessory}"))
        })
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Accessory::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.accessory.read(|data| {
            data.get(&self.aid)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

sheet_retriever!(Skill, SkillBook, skills, IndexMap<String, Skill>);

impl ViewItem for Skill {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, sid)
    }

    fn decorated(_: DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        let mut texture_cache = dependencies.texture_cache.borrow_mut();
        let decoration = texture_cache
            .get_skill(&self.icon_label)
            .or_else(|| texture_cache.get_skill(self.sid.trim_start_matches("SID_")))
            .or_else(|| texture_cache.get_skill("Empty"))?;
        match kind {
            DecorationKind::Other("portrait") => Some((decoration, 1.)),
            _ => Some((decoration, 0.5)),
        }
    }

    fn screen() -> Option<Screens> {
        Some(Screens::Skill)
    }

    fn matches_filter(&self, filter_expr: &str, display_text: &str) -> bool {
        display_text.to_lowercase().contains(filter_expr)
            || self.name.to_lowercase().contains(filter_expr)
    }
}

impl KeyedViewItem for Skill {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.sid)
    }

    fn set_key(&mut self, key: String) {
        self.sid = key;
    }
}

sheet_retriever!(Spawn, DisposBook, spawns, IndexMap<String, Vec<Spawn>>);

impl GroupViewItem for IndexMap<String, Vec<Spawn>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for Spawn {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .person
            .read(|data| {
                data.get(&self.pid)
                    .map(|person| Cow::Owned(person.text(dependencies).to_string()))
            })
            .unwrap_or(Cow::Borrowed("{unknown PID}"))
    }

    fn decorated(kind: DecorationKind<'_>) -> bool {
        matches!(kind, DecorationKind::Other(kind) if kind == "spawn_grid")
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        match kind {
            DecorationKind::Other("spawn_grid") => dependencies
                .person
                .read(|data| {
                    data.get(&self.pid)
                        .and_then(|person| person.decoration(dependencies, DecorationKind::List))
                })
                .or_else(|| {
                    dependencies
                        .texture_cache
                        .borrow_mut()
                        .get_unit("000Dummy", "000Dummy", "Dummy")
                        .map(|decoration| (decoration, 1.))
                }),
            _ => None,
        }
    }
}

sheet_retriever!(TerrainData, TerrainBook, terrain_data, IndexMap<String, TerrainData>);

impl ViewItem for TerrainData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, tid)
    }

    fn screen() -> Option<Screens> {
        Some(Screens::Terrain)
    }
}

impl KeyedViewItem for TerrainData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.tid)
    }

    fn set_key(&mut self, key: String) {
        self.tid = key;
    }
}
