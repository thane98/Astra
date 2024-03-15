use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct HubAreaBook {
    pub hub_area_data: Sheet<Vec<HubAreaData>>,
    pub hub_facility_data: Sheet<Vec<HubFacilityData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubAreaData {
    #[astra(key = "@AID")]
    pub aid: String,
    #[astra(key = "@MID")]
    pub mid: String,
    #[astra(key = "@MID_H")]
    pub mid_h: String,
    #[astra(key = "@ConditionCID")]
    pub condition_cid: String,
    #[astra(key = "@SceneName")]
    pub scene_name: String,
    #[astra(key = "@LocatorName")]
    pub locator_name: String,
    #[astra(key = "@MapPointNo")]
    pub map_point_no: Option<u8>,
    #[astra(key = "@FacilityAidList")]
    pub facility_aid_list: Vec<String>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubFacilityData {
    #[astra(key = "@AID")]
    pub aid: String,
    #[astra(key = "@MID")]
    pub mid: String,
    #[astra(key = "@ConditionCID")]
    pub condition_cid: String,
    #[astra(key = "@IconName")]
    pub icon_name: String,
}

#[derive(AstraBook)]
pub struct HubDemoBook {
    pub hub_demo_data: Sheet<Vec<HubDemoData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubDemoData {
    #[astra(key = "@Locator")]
    pub locator: String,
    #[astra(key = "@MID")]
    pub mid: String,
    #[astra(key = "@CameraName")]
    pub camera_name: String,
    #[astra(key = "@Tutorial")]
    pub tutorial: String,
    #[astra(key = "@Condition")]
    pub condition: String,
    #[astra(key = "@Timezone")]
    pub timezone: Option<i8>,
    #[astra(key = "@FlagName")]
    pub flag_name: String,
    #[astra(key = "@ManualCullingName")]
    pub manual_culling_name: String,
    #[astra(key = "@LodBias")]
    pub lod_bias: Option<f32>,
    #[astra(key = "@IsDisabledLodCrossfadeAnime")]
    pub is_disabled_lod_crossfade_anime: Option<bool>,
}

#[derive(AstraBook)]
pub struct HubDisposBook {
    pub spawns: Sheet<Vec<HubSpawn>>,
    pub random_sets: Sheet<IndexMap<String, Vec<HubSpawnRandomSet>>>,
    pub unity_behavior: Sheet<IndexMap<String, Vec<HubUnityBehavior>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubSpawn {
    #[astra(key = "@HID", public_array)]
    pub hid: String,
    #[astra(key = "@Locator")]
    pub locator: String,
    #[astra(key = "@ParentLocator")]
    pub parent_locator: String,
    #[astra(key = "@IsMustChild")]
    pub is_must_child: Option<bool>,
    #[astra(key = "@FadeDistance")]
    pub fade_distance: Option<f32>,
    #[astra(key = "@Priority")]
    pub priority: Option<i8>,
    #[astra(key = "@Chapter")]
    pub chapter: String,
    #[astra(key = "@Phase")]
    pub phase: Option<i8>,
    #[astra(key = "@TimezoneFlag")]
    pub timezone_flag: Option<i32>,
    #[astra(key = "@FlagName")]
    pub flag_name: String,
    #[astra(key = "@AnyCondition")]
    pub any_condition: String,
    #[astra(key = "@ContentType")]
    pub content_type: Option<i8>,
    #[astra(key = "@AID")]
    pub aid: String,
    #[astra(key = "@TalkPattern")]
    pub talk_pattern: String,
    #[astra(key = "@HelpLabel")]
    pub help_label: String,
    #[astra(key = "@MainLabel")]
    pub main_label: String,
    #[astra(key = "@ScriptName")]
    pub script_name: String,
    #[astra(key = "@AccessType")]
    pub access_type: Option<i8>,
    #[astra(key = "@IdleBodyName")]
    pub idle_body_name: String,
    #[astra(key = "@IdleFaceName")]
    pub idle_face_name: String,
    #[astra(key = "@IdleType")]
    pub idle_type: Option<i8>,
    #[astra(key = "@DisabledAnim")]
    pub disabled_anim: Option<bool>,
    #[astra(key = "@DisabledTalk")]
    pub disabled_talk: Option<bool>,
    #[astra(key = "@IgnoreStory")]
    pub ignore_story: Option<bool>,
    #[astra(key = "@Bind")]
    pub bind: String,
    #[astra(key = "@DisposType")]
    pub dispos_type: Option<i8>,
    #[astra(key = "@AccessAngle")]
    pub access_angle: Option<f32>,
    #[astra(key = "@MoveName")]
    pub move_name: String,
    #[astra(key = "@Area")]
    pub area: String,
    #[astra(key = "@Layer")]
    pub layer: Option<i8>,
    #[astra(key = "@DisabledMiniMap")]
    pub disabled_mini_map: Option<bool>,
    #[astra(key = "@Weight")]
    pub weight: Option<f32>,
    #[astra(key = "@OptimizeType")]
    pub optimize_type: Option<i8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubSpawnRandomSet {
    #[astra(key = "@RID", public_array)]
    pub rid: String,
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@Rate")]
    pub rate: Option<i32>,
    #[astra(key = "@Count")]
    pub count: Option<i32>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubUnityBehavior {
    #[astra(key = "@MID", public_array)]
    pub mid: String,
    #[astra(key = "@MoveType")]
    pub move_type: String,
    #[astra(key = "@Locator")]
    pub locator: String,
    #[astra(key = "@BodyName")]
    pub body_name: String,
    #[astra(key = "@FaceName")]
    pub face_name: String,
    #[astra(key = "@IsTurn")]
    pub is_turn: Option<bool>,
    #[astra(key = "@MoveSec")]
    pub move_sec: String,
    #[astra(key = "@MoveSpeed")]
    pub move_speed: Option<f32>,
}

#[derive(AstraBook)]
pub struct HubFortuneTellingBook {
    pub fortune_telling_data: Sheet<Vec<HubFortuneTellingData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubFortuneTellingData {
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@TextureName")]
    pub texture_name: String,
    #[astra(key = "@PrimaryText")]
    pub primary_text: String,
    #[astra(key = "@PrimaryTextEx")]
    pub primary_text_ex: String,
    #[astra(key = "@ReverseText")]
    pub reverse_text: String,
    #[astra(key = "@ReverseTextEx")]
    pub reverse_text_ex: String,
}

#[derive(AstraBook)]
pub struct HubInvestmentBook {
    pub nation_data: Sheet<Vec<HubNationData>>,
    pub material_bonuses: Sheet<IndexMap<String, Vec<HubMaterialBonus>>>,
    pub ingredient_bonuses: Sheet<IndexMap<String, Vec<HubIngredientBonus>>>,
    pub animal_bonuses: Sheet<IndexMap<String, Vec<HubAnimalBonus>>>,
    pub item_bonuses: Sheet<IndexMap<String, Vec<HubItemBonus>>>,
    pub ingredient_bonus_groups: Sheet<IndexMap<String, Vec<HubIngredientBonusGroup>>>,
    pub animal_bonus_groups: Sheet<IndexMap<String, Vec<HubAnimalBonusGroup>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubNationData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@ID")]
    pub id: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Chapter")]
    pub chapter: String,
    #[astra(key = "@IsNotLevel")]
    pub is_not_level: Option<bool>,
    #[astra(key = "@SymbolTexture")]
    pub symbol_texture: String,
    #[astra(key = "@LevelInfo")]
    pub level_info: String,
    #[astra(key = "@FoodstuffInfo")]
    pub foodstuff_info: String,
    #[astra(key = "@AnimalInfo")]
    pub animal_info: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubMaterialBonus {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Cost")]
    pub cost: Option<i32>,
    #[astra(key = "@BonusName")]
    pub bonus_name: String,
    #[astra(key = "@BonusItem")]
    pub bonus_item: String,
    #[astra(key = "@BonusFood")]
    pub bonus_food: String,
    #[astra(key = "@BonusAnimal")]
    pub bonus_animal: String,
    #[astra(key = "@BonusAccessoryAid")]
    pub bonus_accessory_aid: String,
    #[astra(key = "@BonusIron")]
    pub bonus_iron: Option<i32>,
    #[astra(key = "@BonusSteel")]
    pub bonus_steel: Option<i32>,
    #[astra(key = "@BonusSilver")]
    pub bonus_silver: Option<i32>,
    #[astra(key = "@BonusPieceOfBond")]
    pub bonus_piece_of_bond: Option<i32>,
    #[astra(key = "@GoldEnemyRate")]
    pub gold_enemy_rate: Option<i8>,
    #[astra(key = "@ExpEnemyRate")]
    pub exp_enemy_rate: Option<i8>,
    #[astra(key = "@Iron")]
    pub iron: Option<u8>,
    #[astra(key = "@Steel")]
    pub steel: Option<u8>,
    #[astra(key = "@Silver")]
    pub silver: Option<u8>,
    #[astra(key = "@PieceOfBond")]
    pub piece_of_bond: Option<u8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubIngredientBonus {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Foodstuff")]
    pub foodstuff: String,
    #[astra(key = "@Lv1")]
    pub lv_1: Option<u8>,
    #[astra(key = "@Lv2")]
    pub lv_2: Option<u8>,
    #[astra(key = "@Lv3")]
    pub lv_3: Option<u8>,
    #[astra(key = "@Lv4")]
    pub lv_4: Option<u8>,
    #[astra(key = "@Lv5")]
    pub lv_5: Option<u8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubAnimalBonus {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@ANID")]
    pub anid: String,
    #[astra(key = "@AppearRateLv1")]
    pub appear_rate_lv_1: Option<u8>,
    #[astra(key = "@AppearRateLv2")]
    pub appear_rate_lv_2: Option<u8>,
    #[astra(key = "@AppearRateLv3")]
    pub appear_rate_lv_3: Option<u8>,
    #[astra(key = "@AppearRateLv4")]
    pub appear_rate_lv_4: Option<u8>,
    #[astra(key = "@AppearRateLv5")]
    pub appear_rate_lv_5: Option<u8>,
    #[astra(key = "@CaptureLevel")]
    pub capture_level: Option<u8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubItemBonus {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@ItemId")]
    pub item_id: String,
    #[astra(key = "@Num")]
    pub num: Option<u8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubIngredientBonusGroup {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Foodstuff")]
    pub foodstuff: String,
    #[astra(key = "@Num")]
    pub num: Option<u8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubAnimalBonusGroup {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@AnimalId")]
    pub animal_id: String,
    #[astra(key = "@Num")]
    pub num: Option<u8>,
}

#[derive(AstraBook)]
pub struct HubMapIconBook {
    pub map_icon_data: Sheet<Vec<HubMapIconData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubMapIconData {
    #[astra(key = "@DisposName")]
    pub dispos_name: String,
    #[astra(key = "@IconName")]
    pub icon_name: String,
    #[astra(key = "@LargeScale")]
    pub large_scale: Option<f32>,
    #[astra(key = "@SmallScale")]
    pub small_scale: Option<f32>,
}

#[derive(AstraBook)]
pub struct HubMyRoomBook {
    pub my_room_data: Sheet<Vec<HubMyRoomData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubMyRoomData {
    #[astra(key = "@PID")]
    pub pid: String,
    #[astra(key = "@C1")]
    pub c_1: Option<i8>,
    #[astra(key = "@C2")]
    pub c_2: Option<i8>,
    #[astra(key = "@B1")]
    pub b_1: Option<i8>,
    #[astra(key = "@B2")]
    pub b_2: Option<i8>,
    #[astra(key = "@A1")]
    pub a_1: Option<i8>,
    #[astra(key = "@A2")]
    pub a_2: Option<i8>,
    #[astra(key = "@S1")]
    pub s_1: Option<i8>,
    #[astra(key = "@S2")]
    pub s_2: Option<i8>,
}

#[derive(AstraBook)]
pub struct HubResourceBook {
    pub resources: Sheet<Vec<HubResourceData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubResourceData {
    #[astra(key = "@Name")]
    pub name: String,
}

#[derive(AstraBook)]
pub struct HubTalkBook {
    pub talk_data: Sheet<Vec<HubTalkData>>,
    pub relative_data: Sheet<Vec<HubTalkRelativeData>>,
    pub talk_facility_data: Sheet<Vec<HubTalkFacilityData>>,
    pub crystal_data: Sheet<Vec<HubCrystalData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubTalkData {
    #[astra(key = "@KRID")]
    pub krid: String,
    #[astra(key = "@Count")]
    pub count: Option<u8>,
    #[astra(key = "@Args0")]
    pub args_0: Option<u8>,
    #[astra(key = "@Args1")]
    pub args_1: Option<u8>,
    #[astra(key = "@Item")]
    pub item: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubTalkRelativeData {
    #[astra(key = "@PID")]
    pub pid: String,
    #[astra(key = "@PID1")]
    pub pid_1: String,
    #[astra(key = "@PID2")]
    pub pid_2: String,
    #[astra(key = "@PID3")]
    pub pid_3: String,
    #[astra(key = "@PID4")]
    pub pid_4: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubTalkFacilityData {
    #[astra(key = "@Pattern")]
    pub pattern: String,
    #[astra(key = "@PID")]
    pub pid: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct HubCrystalData {
    #[astra(key = "@CID")]
    pub cid: String,
    #[astra(key = "@Count")]
    pub count: Option<u8>,
}
