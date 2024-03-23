use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct GodBook {
    pub gods: Sheet<IndexMap<String, GodData>>,
    pub level_data: Sheet<IndexMap<String, Vec<GodLevelData>>>,
    pub bond_level_data: Sheet<Vec<GodBondLevelData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct GodData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Gid", id)]
    pub gid: String,
    #[astra(key = "@Mid")]
    pub mid: String,
    #[astra(key = "@Nickname")]
    pub nickname: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@AsciiName")]
    pub ascii_name: String,
    #[astra(key = "@SoundID")]
    pub sound_id: String,
    #[astra(key = "@AssetID")]
    pub asset_id: String,
    #[astra(key = "@FaceIconName")]
    pub face_icon_name: String,
    #[astra(key = "@FaceIconNameDarkness")]
    pub face_icon_name_darkness: String,
    #[astra(key = "@Ringname")]
    pub ringname: String,
    #[astra(key = "@Ringhelp")]
    pub ringhelp: String,
    #[astra(key = "@UnitIconID")]
    pub unit_icon_id: String,
    #[astra(key = "@Change")]
    pub change: Vec<String>,
    #[astra(key = "@Link")]
    pub link: String,
    #[astra(key = "@EngageHaunt")]
    pub engage_haunt: String,
    #[astra(key = "@Level")]
    pub level: Option<u8>,
    #[astra(key = "@ForceType")]
    pub force_type: Option<i8>,
    #[astra(key = "@Female")]
    pub female: Option<i8>,
    #[astra(key = "@GoodWeapon")]
    pub good_weapon: Option<i8>,
    #[astra(key = "@Sort")]
    pub sort: Option<i16>,
    #[astra(key = "@EngageCount")]
    pub engage_count: Option<u8>,
    #[astra(key = "@EngageAttack")]
    pub engage_attack: String,
    #[astra(key = "@EngageAttackRampage")]
    pub engage_attack_rampage: String,
    #[astra(key = "@EngageAttackLink")]
    pub engage_attack_link: String,
    #[astra(key = "@LinkGid")]
    pub link_gid: String,
    #[astra(key = "@Gbid")]
    pub gbid: String,
    #[astra(key = "@GrowTable")]
    pub grow_table: String,
    #[astra(key = "@LevelCap")]
    pub level_cap: Option<u8>,
    #[astra(key = "@UnlockLevelCapVarName")]
    pub unlock_level_cap_var_name: String,
    #[astra(key = "@EngraveWord")]
    pub engrave_word: String,
    #[astra(key = "@EngravePower")]
    pub engrave_power: Option<i8>,
    #[astra(key = "@EngraveWeight")]
    pub engrave_weight: Option<i8>,
    #[astra(key = "@EngraveHit")]
    pub engrave_hit: Option<i8>,
    #[astra(key = "@EngraveCritical")]
    pub engrave_critical: Option<i8>,
    #[astra(key = "@EngraveAvoid")]
    pub engrave_avoid: Option<i8>,
    #[astra(key = "@EngraveSecure")]
    pub engrave_secure: Option<i8>,
    #[astra(key = "@SynchroEnhance.Hp")]
    pub synchro_enhance_hp: Option<i8>,
    #[astra(key = "@SynchroEnhance.Str")]
    pub synchro_enhance_str: Option<i8>,
    #[astra(key = "@SynchroEnhance.Tech")]
    pub synchro_enhance_tech: Option<i8>,
    #[astra(key = "@SynchroEnhance.Quick")]
    pub synchro_enhance_quick: Option<i8>,
    #[astra(key = "@SynchroEnhance.Luck")]
    pub synchro_enhance_luck: Option<i8>,
    #[astra(key = "@SynchroEnhance.Def")]
    pub synchro_enhance_def: Option<i8>,
    #[astra(key = "@SynchroEnhance.Magic")]
    pub synchro_enhance_magic: Option<i8>,
    #[astra(key = "@SynchroEnhance.Mdef")]
    pub synchro_enhance_mdef: Option<i8>,
    #[astra(key = "@SynchroEnhance.Phys")]
    pub synchro_enhance_phys: Option<i8>,
    #[astra(key = "@SynchroEnhance.Move")]
    pub synchro_enhance_move: Option<i8>,
    #[astra(key = "@Flag")]
    pub flag: Option<i32>,
    #[astra(key = "@NetRankingIndex")]
    pub net_ranking_index: Option<u8>,
    #[astra(key = "@AIEngageAttackType")]
    pub ai_engage_attack_type: Option<i8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct GodLevelData {
    #[astra(key = "@Ggid", public_array)]
    pub ggid: String,
    #[astra(key = "@Level")]
    pub level: Option<u8>,
    #[astra(key = "@InheritanceSkills")]
    pub inheritance_skills: Vec<String>,
    #[astra(key = "@SynchroSkills")]
    pub synchro_skills: Vec<String>,
    #[astra(key = "@EngageSkills")]
    pub engage_skills: Vec<String>,
    #[astra(key = "@EngageItems")]
    pub engage_items: Vec<String>,
    #[astra(key = "@EngageCooperations")]
    pub engage_cooperations: Vec<String>,
    #[astra(key = "@EngageHorses")]
    pub engage_horses: Vec<String>,
    #[astra(key = "@EngageCoverts")]
    pub engage_coverts: Vec<String>,
    #[astra(key = "@EngageHeavys")]
    pub engage_heavys: Vec<String>,
    #[astra(key = "@EngageFlys")]
    pub engage_flys: Vec<String>,
    #[astra(key = "@EngageMagics")]
    pub engage_magics: Vec<String>,
    #[astra(key = "@EngagePranas")]
    pub engage_pranas: Vec<String>,
    #[astra(key = "@EngageDragons")]
    pub engage_dragons: Vec<String>,
    #[astra(key = "@Aptitude")]
    pub aptitude: Option<i32>,
    #[astra(key = "@AptitudeCostNone")]
    pub aptitude_cost_none: Option<u16>,
    #[astra(key = "@AptitudeCostSword")]
    pub aptitude_cost_sword: Option<u16>,
    #[astra(key = "@AptitudeCostLance")]
    pub aptitude_cost_lance: Option<u16>,
    #[astra(key = "@AptitudeCostAxe")]
    pub aptitude_cost_axe: Option<u16>,
    #[astra(key = "@AptitudeCostBow")]
    pub aptitude_cost_bow: Option<u16>,
    #[astra(key = "@AptitudeCostDagger")]
    pub aptitude_cost_dagger: Option<u16>,
    #[astra(key = "@AptitudeCostMagic")]
    pub aptitude_cost_magic: Option<u16>,
    #[astra(key = "@AptitudeCostRod")]
    pub aptitude_cost_rod: Option<u16>,
    #[astra(key = "@AptitudeCostFist")]
    pub aptitude_cost_fist: Option<u16>,
    #[astra(key = "@AptitudeCostSpecial")]
    pub aptitude_cost_special: Option<u16>,
    #[astra(key = "@Flag")]
    pub flag: Option<i32>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct GodBondLevelData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Level")]
    pub level: String,
    #[astra(key = "@Exp")]
    pub exp: Option<i32>,
    #[astra(key = "@RelianceLevel")]
    pub reliance_level: String,
    #[astra(key = "@Cost")]
    pub cost: Option<i32>,
}
