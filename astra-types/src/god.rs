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
    pub level: u8,
    #[astra(key = "@ForceType")]
    pub force_type: i8,
    #[astra(key = "@Female")]
    pub female: i8,
    #[astra(key = "@GoodWeapon")]
    pub good_weapon: i8,
    #[astra(key = "@Sort")]
    pub sort: i16,
    #[astra(key = "@EngageCount")]
    pub engage_count: u8,
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
    pub level_cap: u8,
    #[astra(key = "@UnlockLevelCapVarName")]
    pub unlock_level_cap_var_name: String,
    #[astra(key = "@EngraveWord")]
    pub engrave_word: String,
    #[astra(key = "@EngravePower")]
    pub engrave_power: i8,
    #[astra(key = "@EngraveWeight")]
    pub engrave_weight: i8,
    #[astra(key = "@EngraveHit")]
    pub engrave_hit: i8,
    #[astra(key = "@EngraveCritical")]
    pub engrave_critical: i8,
    #[astra(key = "@EngraveAvoid")]
    pub engrave_avoid: i8,
    #[astra(key = "@EngraveSecure")]
    pub engrave_secure: i8,
    #[astra(key = "@SynchroEnhance.Hp")]
    pub synchro_enhance_hp: i8,
    #[astra(key = "@SynchroEnhance.Str")]
    pub synchro_enhance_str: i8,
    #[astra(key = "@SynchroEnhance.Tech")]
    pub synchro_enhance_tech: i8,
    #[astra(key = "@SynchroEnhance.Quick")]
    pub synchro_enhance_quick: i8,
    #[astra(key = "@SynchroEnhance.Luck")]
    pub synchro_enhance_luck: i8,
    #[astra(key = "@SynchroEnhance.Def")]
    pub synchro_enhance_def: i8,
    #[astra(key = "@SynchroEnhance.Magic")]
    pub synchro_enhance_magic: i8,
    #[astra(key = "@SynchroEnhance.Mdef")]
    pub synchro_enhance_mdef: i8,
    #[astra(key = "@SynchroEnhance.Phys")]
    pub synchro_enhance_phys: i8,
    #[astra(key = "@SynchroEnhance.Move")]
    pub synchro_enhance_move: i8,
    #[astra(key = "@Flag")]
    pub flag: i32,
    #[astra(key = "@NetRankingIndex")]
    pub net_ranking_index: u8,
    #[astra(key = "@AIEngageAttackType")]
    pub ai_engage_attack_type: i8,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct GodLevelData {
    #[astra(key = "@Ggid", public_array)]
    pub ggid: String,
    #[astra(key = "@Level")]
    pub level: u8,
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
    pub aptitude: i32,
    #[astra(key = "@AptitudeCostNone")]
    pub aptitude_cost_none: u16,
    #[astra(key = "@AptitudeCostSword")]
    pub aptitude_cost_sword: u16,
    #[astra(key = "@AptitudeCostLance")]
    pub aptitude_cost_lance: u16,
    #[astra(key = "@AptitudeCostAxe")]
    pub aptitude_cost_axe: u16,
    #[astra(key = "@AptitudeCostBow")]
    pub aptitude_cost_bow: u16,
    #[astra(key = "@AptitudeCostDagger")]
    pub aptitude_cost_dagger: u16,
    #[astra(key = "@AptitudeCostMagic")]
    pub aptitude_cost_magic: u16,
    #[astra(key = "@AptitudeCostRod")]
    pub aptitude_cost_rod: u16,
    #[astra(key = "@AptitudeCostFist")]
    pub aptitude_cost_fist: u16,
    #[astra(key = "@AptitudeCostSpecial")]
    pub aptitude_cost_special: u16,
    #[astra(key = "@Flag")]
    pub flag: i32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct GodBondLevelData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Level")]
    pub level: String,
    #[astra(key = "@Exp")]
    pub exp: i32,
    #[astra(key = "@RelianceLevel")]
    pub reliance_level: String,
    #[astra(key = "@Cost")]
    pub cost: i32,
}
