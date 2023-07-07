use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct AnimSetBook {
    pub sets: Sheet<IndexMap<String, AnimSet>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AnimSet {
    #[astra(key = "@Name", id)]
    pub name: String,
    #[astra(key = "@Attack1")]
    pub attack_1: String,
    #[astra(key = "@Attack2")]
    pub attack_2: String,
    #[astra(key = "@Attack3")]
    pub attack_3: String,
    #[astra(key = "@Attack4")]
    pub attack_4: String,
    #[astra(key = "@Attack5")]
    pub attack_5: String,
    #[astra(key = "@AttackC")]
    pub attack_c: String,
    #[astra(key = "@AttackT")]
    pub attack_t: String,
    #[astra(key = "@DamageHigh")]
    pub damage_high: String,
    #[astra(key = "@DamageMidB")]
    pub damage_mid_b: String,
    #[astra(key = "@DamageMidDU")]
    pub damage_mid_du: String,
    #[astra(key = "@DamageMidUD")]
    pub damage_mid_ud: String,
    #[astra(key = "@DieB")]
    pub die_b: String,
    #[astra(key = "@DieL")]
    pub die_l: String,
    #[astra(key = "@DieR")]
    pub die_r: String,
    #[astra(key = "@Dive")]
    pub dive: String,
    #[astra(key = "@Engage1")]
    pub engage_1: String,
    #[astra(key = "@Engage2")]
    pub engage_2: String,
    #[astra(key = "@Engage3")]
    pub engage_3: String,
    #[astra(key = "@EvasionB")]
    pub evasion_b: String,
    #[astra(key = "@EvasionL")]
    pub evasion_l: String,
    #[astra(key = "@EvasionR")]
    pub evasion_r: String,
    #[astra(key = "@Guard")]
    pub guard: String,
    #[astra(key = "@HoveringLoop")]
    pub hovering_loop: String,
    #[astra(key = "@IdleDying")]
    pub idle_dying: String,
    #[astra(key = "@IdleNormal")]
    pub idle_normal: String,
    #[astra(key = "@ParryL")]
    pub parry_l: String,
    #[astra(key = "@ParryR")]
    pub parry_r: String,
    #[astra(key = "@Ready")]
    pub ready: String,
    #[astra(key = "@RelaxLoop")]
    pub relax_loop: String,
    #[astra(key = "@Repelled")]
    pub repelled: String,
    #[astra(key = "@RunLoop")]
    pub run_loop: String,
    #[astra(key = "@RunStart")]
    pub run_start: String,
    #[astra(key = "@Special1")]
    pub special_1: String,
    #[astra(key = "@Start")]
    pub start: String,
    #[astra(key = "@Win")]
    pub win: String,
    #[astra(key = "@WinLoop")]
    pub win_loop: String,
}

#[derive(AstraBook)]
pub struct AssetTableBook {
    pub asset_defs: Sheet<Vec<AssetDef>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AssetDef {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@PresetName")]
    pub preset_name: String,
    #[astra(key = "@Mode")]
    pub mode: Option<i8>,
    #[astra(key = "@Conditions")]
    pub conditions: Vec<String>,
    #[astra(key = "@BodyModel")]
    pub body_model: String,
    #[astra(key = "@DressModel")]
    pub dress_model: String,
    #[astra(key = "@MaskColor100R")]
    pub mask_color_100_r: Option<u8>,
    #[astra(key = "@MaskColor100G")]
    pub mask_color_100_g: Option<u8>,
    #[astra(key = "@MaskColor100B")]
    pub mask_color_100_b: Option<u8>,
    #[astra(key = "@MaskColor075R")]
    pub mask_color_075_r: Option<u8>,
    #[astra(key = "@MaskColor075G")]
    pub mask_color_075_g: Option<u8>,
    #[astra(key = "@MaskColor075B")]
    pub mask_color_075_b: Option<u8>,
    #[astra(key = "@MaskColor050R")]
    pub mask_color_050_r: Option<u8>,
    #[astra(key = "@MaskColor050G")]
    pub mask_color_050_g: Option<u8>,
    #[astra(key = "@MaskColor050B")]
    pub mask_color_050_b: Option<u8>,
    #[astra(key = "@MaskColor025R")]
    pub mask_color_025_r: Option<u8>,
    #[astra(key = "@MaskColor025G")]
    pub mask_color_025_g: Option<u8>,
    #[astra(key = "@MaskColor025B")]
    pub mask_color_025_b: Option<u8>,
    #[astra(key = "@HeadModel")]
    pub head_model: String,
    #[astra(key = "@HairModel")]
    pub hair_model: String,
    #[astra(key = "@HairR")]
    pub hair_r: Option<u8>,
    #[astra(key = "@HairG")]
    pub hair_g: Option<u8>,
    #[astra(key = "@HairB")]
    pub hair_b: Option<u8>,
    #[astra(key = "@GradR")]
    pub grad_r: Option<u8>,
    #[astra(key = "@GradG")]
    pub grad_g: Option<u8>,
    #[astra(key = "@GradB")]
    pub grad_b: Option<u8>,
    #[astra(key = "@SkinR")]
    pub skin_r: Option<u8>,
    #[astra(key = "@SkinG")]
    pub skin_g: Option<u8>,
    #[astra(key = "@SkinB")]
    pub skin_b: Option<u8>,
    #[astra(key = "@ToonR")]
    pub toon_r: Option<u8>,
    #[astra(key = "@ToonG")]
    pub toon_g: Option<u8>,
    #[astra(key = "@ToonB")]
    pub toon_b: Option<u8>,
    #[astra(key = "@RideModel")]
    pub ride_model: String,
    #[astra(key = "@RideDressModel")]
    pub ride_dress_model: String,
    #[astra(key = "@LeftHand")]
    pub left_hand: String,
    #[astra(key = "@RightHand")]
    pub right_hand: String,
    #[astra(key = "@Trail")]
    pub trail: String,
    #[astra(key = "@Magic")]
    pub magic: String,
    #[astra(key = "@Acc1.Locator")]
    pub acc_1_locator: String,
    #[astra(key = "@Acc1.Model")]
    pub acc_1_model: String,
    #[astra(key = "@Acc2.Locator")]
    pub acc_2_locator: String,
    #[astra(key = "@Acc2.Model")]
    pub acc_2_model: String,
    #[astra(key = "@Acc3.Locator")]
    pub acc_3_locator: String,
    #[astra(key = "@Acc3.Model")]
    pub acc_3_model: String,
    #[astra(key = "@Acc4.Locator")]
    pub acc_4_locator: String,
    #[astra(key = "@Acc4.Model")]
    pub acc_4_model: String,
    #[astra(key = "@Acc5.Locator")]
    pub acc_5_locator: String,
    #[astra(key = "@Acc5.Model")]
    pub acc_5_model: String,
    #[astra(key = "@Acc6.Locator")]
    pub acc_6_locator: String,
    #[astra(key = "@Acc6.Model")]
    pub acc_6_model: String,
    #[astra(key = "@Acc7.Locator")]
    pub acc_7_locator: String,
    #[astra(key = "@Acc7.Model")]
    pub acc_7_model: String,
    #[astra(key = "@Acc8.Locator")]
    pub acc_8_locator: String,
    #[astra(key = "@Acc8.Model")]
    pub acc_8_model: String,
    #[astra(key = "@BodyAnim")]
    pub body_anim: String,
    #[astra(key = "@InfoAnim")]
    pub info_anim: String,
    #[astra(key = "@TalkAnim")]
    pub talk_anim: String,
    #[astra(key = "@DemoAnim")]
    pub demo_anim: String,
    #[astra(key = "@HubAnim")]
    pub hub_anim: String,
    #[astra(key = "@ScaleAll")]
    pub scale_all: Option<f32>,
    #[astra(key = "@ScaleHead")]
    pub scale_head: Option<f32>,
    #[astra(key = "@ScaleNeck")]
    pub scale_neck: Option<f32>,
    #[astra(key = "@ScaleTorso")]
    pub scale_torso: Option<f32>,
    #[astra(key = "@ScaleShoulders")]
    pub scale_shoulders: Option<f32>,
    #[astra(key = "@ScaleArms")]
    pub scale_arms: Option<f32>,
    #[astra(key = "@ScaleHands")]
    pub scale_hands: Option<f32>,
    #[astra(key = "@ScaleLegs")]
    pub scale_legs: Option<f32>,
    #[astra(key = "@ScaleFeet")]
    pub scale_feet: Option<f32>,
    #[astra(key = "@VolumeArms")]
    pub volume_arms: Option<f32>,
    #[astra(key = "@VolumeLegs")]
    pub volume_legs: Option<f32>,
    #[astra(key = "@VolumeBust")]
    pub volume_bust: Option<f32>,
    #[astra(key = "@VolumeAbdomen")]
    pub volume_abdomen: Option<f32>,
    #[astra(key = "@VolumeTorso")]
    pub volume_torso: Option<f32>,
    #[astra(key = "@VolumeScaleArms")]
    pub volume_scale_arms: Option<f32>,
    #[astra(key = "@VolumeScaleLegs")]
    pub volume_scale_legs: Option<f32>,
    #[astra(key = "@MapScaleAll")]
    pub map_scale_all: Option<f32>,
    #[astra(key = "@MapScaleHead")]
    pub map_scale_head: Option<f32>,
    #[astra(key = "@MapScaleWing")]
    pub map_scale_wing: Option<f32>,
    #[astra(key = "@Voice")]
    pub voice: String,
    #[astra(key = "@FootStep")]
    pub foot_step: String,
    #[astra(key = "@Material")]
    pub material: String,
    #[astra(key = "@Comment")]
    pub comment: String,
}

#[derive(AstraBook)]
pub struct ChapterBook {
    pub chapters: Sheet<IndexMap<String, Chapter>>,
    pub challenges: Sheet<Vec<Challenge>>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct Chapter {
    #[astra(key = "@Cid", id)]
    pub cid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Field")]
    pub field: String,
    #[astra(key = "@Mess")]
    pub mess: String,
    #[astra(key = "@Event")]
    pub event: String,
    #[astra(key = "@ScriptBmap")]
    pub script_bmap: String,
    #[astra(key = "@ScriptEncount")]
    pub script_encount: String,
    #[astra(key = "@ScriptKizuna")]
    pub script_kizuna: String,
    #[astra(key = "@ChapterTitle")]
    pub chapter_title: String,
    #[astra(key = "@Alpha")]
    pub alpha: Option<f32>,
    #[astra(key = "@Terrain")]
    pub terrain: String,
    #[astra(key = "@Dispos")]
    pub dispos: String,
    #[astra(key = "@NextChapter")]
    pub next_chapter: String,
    #[astra(key = "@GmapSpot")]
    pub gmap_spot: String,
    #[astra(key = "@GmapSpotState")]
    pub gmap_spot_state: Option<i8>,
    #[astra(key = "@GmapSpotOpenCondition")]
    pub gmap_spot_open_condition: String,
    #[astra(key = "@GmapSpotEncount")]
    pub gmap_spot_encount: Option<i8>,
    #[astra(key = "@EncountJobs")]
    pub encount_jobs: Vec<String>,
    #[astra(key = "@Reward")]
    pub reward: String,
    #[astra(key = "@Progress")]
    pub progress: Option<u8>,
    #[astra(key = "@HoldLevel")]
    pub hold_level: Option<u8>,
    #[astra(key = "@Flag")]
    pub flag: Option<i32>,
    #[astra(key = "@SoundFieldSituation")]
    pub sound_field_situation: String,
    #[astra(key = "@PlayerPhaseBgm")]
    pub player_phase_bgm: String,
    #[astra(key = "@EnemyPhaseBgm")]
    pub enemy_phase_bgm: String,
    #[astra(key = "@AllyPhaseBgm")]
    pub ally_phase_bgm: String,
    #[astra(key = "@PlayerEncountBgm")]
    pub player_encount_bgm: String,
    #[astra(key = "@EnemyEncountBgm")]
    pub enemy_encount_bgm: String,
    #[astra(key = "@SortieBgm")]
    pub sortie_bgm: String,
    #[astra(key = "@KizunaBgm")]
    pub kizuna_bgm: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@RecommendedLevel")]
    pub recommended_level: Option<u8>,
    #[astra(key = "@Nation")]
    pub nation: String,
    #[astra(key = "@NetKillBonusIndex")]
    pub net_kill_bonus_index: Option<u8>,
    #[astra(key = "@NetRankingIndex")]
    pub net_ranking_index: Option<u8>,
}

#[derive(Astra, Debug)]
pub struct Challenge {
    #[astra(key = "@Cid")]
    pub cid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Stage1")]
    pub stage_1: Vec<String>,
    #[astra(key = "@Stage2")]
    pub stage_2: Vec<String>,
    #[astra(key = "@Stage3")]
    pub stage_3: Vec<String>,
    #[astra(key = "@Reward")]
    pub reward: String,
    #[astra(key = "@UnlockCid")]
    pub unlock_cid: String,
    #[astra(key = "@SortieCount")]
    pub sortie_count: Option<i32>,
}

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

#[derive(AstraBook)]
pub struct PersonBook {
    pub persons: Sheet<IndexMap<String, Person>>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct Person {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Pid", id)]
    pub pid: String,
    #[astra(key = "@Fid")]
    pub fid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Jid")]
    pub jid: String,
    #[astra(key = "@Aid")]
    pub aid: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@Die")]
    pub die: String,
    #[astra(key = "@Belong")]
    pub belong: String,
    #[astra(key = "@UnitIconID")]
    pub unit_icon_id: String,
    #[astra(key = "@Age")]
    pub age: Option<i16>,
    #[astra(key = "@Gender")]
    pub gender: Option<i8>,
    #[astra(key = "@BirthMonth")]
    pub birth_month: Option<u8>,
    #[astra(key = "@BirthDay")]
    pub birth_day: Option<u8>,
    #[astra(key = "@Level")]
    pub level: Option<u8>,
    #[astra(key = "@InternalLevel")]
    pub internal_level: Option<i8>,
    #[astra(key = "@AutoGrowOffsetN")]
    pub auto_grow_offset_n: Option<i8>,
    #[astra(key = "@AutoGrowOffsetH")]
    pub auto_grow_offset_h: Option<i8>,
    #[astra(key = "@AutoGrowOffsetL")]
    pub auto_grow_offset_l: Option<i8>,
    #[astra(key = "@AssetForce")]
    pub asset_force: Option<i8>,
    #[astra(key = "@SupportCategory")]
    pub support_category: String,
    #[astra(key = "@SkillPoint")]
    pub skill_point: Option<i32>,
    #[astra(key = "@BmapSize")]
    pub bmap_size: Option<u8>,
    #[astra(key = "@Flag")]
    pub flag: Option<u8>,
    #[astra(key = "@Aptitude")]
    pub aptitude: Option<i32>,
    #[astra(key = "@SubAptitude")]
    pub sub_aptitude: Option<i32>,
    #[astra(key = "@OffsetN.Hp")]
    pub offset_n_hp: Option<i8>,
    #[astra(key = "@OffsetN.Str")]
    pub offset_n_str: Option<i8>,
    #[astra(key = "@OffsetN.Tech")]
    pub offset_n_tech: Option<i8>,
    #[astra(key = "@OffsetN.Quick")]
    pub offset_n_quick: Option<i8>,
    #[astra(key = "@OffsetN.Luck")]
    pub offset_n_luck: Option<i8>,
    #[astra(key = "@OffsetN.Def")]
    pub offset_n_def: Option<i8>,
    #[astra(key = "@OffsetN.Magic")]
    pub offset_n_magic: Option<i8>,
    #[astra(key = "@OffsetN.Mdef")]
    pub offset_n_mdef: Option<i8>,
    #[astra(key = "@OffsetN.Phys")]
    pub offset_n_phys: Option<i8>,
    #[astra(key = "@OffsetN.Sight")]
    pub offset_n_sight: Option<i8>,
    #[astra(key = "@OffsetN.Move")]
    pub offset_n_move: Option<i8>,
    #[astra(key = "@OffsetH.Hp")]
    pub offset_h_hp: Option<i8>,
    #[astra(key = "@OffsetH.Str")]
    pub offset_h_str: Option<i8>,
    #[astra(key = "@OffsetH.Tech")]
    pub offset_h_tech: Option<i8>,
    #[astra(key = "@OffsetH.Quick")]
    pub offset_h_quick: Option<i8>,
    #[astra(key = "@OffsetH.Luck")]
    pub offset_h_luck: Option<i8>,
    #[astra(key = "@OffsetH.Def")]
    pub offset_h_def: Option<i8>,
    #[astra(key = "@OffsetH.Magic")]
    pub offset_h_magic: Option<i8>,
    #[astra(key = "@OffsetH.Mdef")]
    pub offset_h_mdef: Option<i8>,
    #[astra(key = "@OffsetH.Phys")]
    pub offset_h_phys: Option<i8>,
    #[astra(key = "@OffsetH.Sight")]
    pub offset_h_sight: Option<i8>,
    #[astra(key = "@OffsetH.Move")]
    pub offset_h_move: Option<i8>,
    #[astra(key = "@OffsetL.Hp")]
    pub offset_l_hp: Option<i8>,
    #[astra(key = "@OffsetL.Str")]
    pub offset_l_str: Option<i8>,
    #[astra(key = "@OffsetL.Tech")]
    pub offset_l_tech: Option<i8>,
    #[astra(key = "@OffsetL.Quick")]
    pub offset_l_quick: Option<i8>,
    #[astra(key = "@OffsetL.Luck")]
    pub offset_l_luck: Option<i8>,
    #[astra(key = "@OffsetL.Def")]
    pub offset_l_def: Option<i8>,
    #[astra(key = "@OffsetL.Magic")]
    pub offset_l_magic: Option<i8>,
    #[astra(key = "@OffsetL.Mdef")]
    pub offset_l_mdef: Option<i8>,
    #[astra(key = "@OffsetL.Phys")]
    pub offset_l_phys: Option<i8>,
    #[astra(key = "@OffsetL.Sight")]
    pub offset_l_sight: Option<i8>,
    #[astra(key = "@OffsetL.Move")]
    pub offset_l_move: Option<i8>,
    #[astra(key = "@Limit.Hp")]
    pub limit_hp: Option<i8>,
    #[astra(key = "@Limit.Str")]
    pub limit_str: Option<i8>,
    #[astra(key = "@Limit.Tech")]
    pub limit_tech: Option<i8>,
    #[astra(key = "@Limit.Quick")]
    pub limit_quick: Option<i8>,
    #[astra(key = "@Limit.Luck")]
    pub limit_luck: Option<i8>,
    #[astra(key = "@Limit.Def")]
    pub limit_def: Option<i8>,
    #[astra(key = "@Limit.Magic")]
    pub limit_magic: Option<i8>,
    #[astra(key = "@Limit.Mdef")]
    pub limit_mdef: Option<i8>,
    #[astra(key = "@Limit.Phys")]
    pub limit_phys: Option<i8>,
    #[astra(key = "@Limit.Sight")]
    pub limit_sight: Option<i8>,
    #[astra(key = "@Limit.Move")]
    pub limit_move: Option<i8>,
    #[astra(key = "@Grow.Hp")]
    pub grow_hp: Option<u8>,
    #[astra(key = "@Grow.Str")]
    pub grow_str: Option<u8>,
    #[astra(key = "@Grow.Tech")]
    pub grow_tech: Option<u8>,
    #[astra(key = "@Grow.Quick")]
    pub grow_quick: Option<u8>,
    #[astra(key = "@Grow.Luck")]
    pub grow_luck: Option<u8>,
    #[astra(key = "@Grow.Def")]
    pub grow_def: Option<u8>,
    #[astra(key = "@Grow.Magic")]
    pub grow_magic: Option<u8>,
    #[astra(key = "@Grow.Mdef")]
    pub grow_mdef: Option<u8>,
    #[astra(key = "@Grow.Phys")]
    pub grow_phys: Option<u8>,
    #[astra(key = "@Grow.Sight")]
    pub grow_sight: Option<u8>,
    #[astra(key = "@Grow.Move")]
    pub grow_move: Option<u8>,
    #[astra(key = "@Items")]
    pub items: Vec<String>,
    #[astra(key = "@DropItem")]
    pub drop_item: String,
    #[astra(key = "@DropRatio")]
    pub drop_ratio: Option<f32>,
    #[astra(key = "@Attrs")]
    pub attrs: Option<i32>,
    #[astra(key = "@CommonSids")]
    pub common_sids: Vec<String>,
    #[astra(key = "@NormalSids")]
    pub normal_sids: Vec<String>,
    #[astra(key = "@HardSids")]
    pub hard_sids: Vec<String>,
    #[astra(key = "@LunaticSids")]
    pub lunatic_sids: Vec<String>,
    #[astra(key = "@EngageSid")]
    pub engage_sid: String,
    #[astra(key = "@TalkPauseDelayMin")]
    pub talk_pause_delay_min: Option<f32>,
    #[astra(key = "@TalkPauseDelayMax")]
    pub talk_pause_delay_max: Option<f32>,
    #[astra(key = "@TalkPauseSpeed")]
    pub talk_pause_speed: Option<f32>,
    #[astra(key = "@CombatBgm")]
    pub combat_bgm: String,
    #[astra(key = "@ExistDieCid")]
    pub exist_die_cid: String,
    #[astra(key = "@ExistDieTiming")]
    pub exist_die_timing: Option<i8>,
    #[astra(key = "@Hometown")]
    pub hometown: Option<i8>,
    #[astra(key = "@NetRankingIndex")]
    pub net_ranking_index: Option<u8>,
    #[astra(key = "@NotLvUpTalkPids")]
    pub not_lv_up_talk_pids: Vec<String>,
    #[astra(key = "@SummonColor")]
    pub summon_color: Option<i8>,
    #[astra(key = "@SummonRank")]
    pub summon_rank: Option<i8>,
    #[astra(key = "@SummonRate")]
    pub summon_rate: Option<i32>,
    #[astra(key = "@SummonGod")]
    pub summon_god: String,
}

#[derive(AstraBook)]
pub struct ItemBook {
    pub items: Sheet<IndexMap<String, Item>>,
    pub categories: Sheet<Vec<ItemCategory>>,
    pub improve_data: Sheet<IndexMap<String, Vec<ForgeImproveData>>>,
    pub evolve_data: Sheet<IndexMap<String, Vec<ForgeEvolveData>>>,
    pub exchange_data: Sheet<Vec<ForgeExchangeData>>,
    pub weapon_rank_data: Sheet<Vec<WeaponRankData>>,
    pub item_interact_data: Sheet<Vec<ItemInteractData>>,
    pub accessories: Sheet<IndexMap<String, Accessory>>,
    pub gifts: Sheet<Vec<GiftData>>,
    pub reward_data: Sheet<IndexMap<String, Vec<RewardData>>>,
    pub engage_weapon_enhancement_data: Sheet<IndexMap<String, Vec<EngageWeaponEnhancementData>>>,
    pub versus_reward_data: Sheet<Vec<VersusRewardData>>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct Item {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Iid", id)]
    pub iid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@Tutorial")]
    pub tutorial: String,
    #[astra(key = "@Aid")]
    pub aid: String,
    #[astra(key = "@Kind")]
    pub kind: Option<i8>,
    #[astra(key = "@UseType")]
    pub use_type: Option<i8>,
    #[astra(key = "@WeaponAttr")]
    pub weapon_attr: Option<i8>,
    #[astra(key = "@Icon")]
    pub icon: String,
    #[astra(key = "@Endurance")]
    pub endurance: Option<u8>,
    #[astra(key = "@Power")]
    pub power: Option<u8>,
    #[astra(key = "@Weight")]
    pub weight: Option<u8>,
    #[astra(key = "@RangeI")]
    pub range_i: Option<u8>,
    #[astra(key = "@RangeO")]
    pub range_o: Option<u8>,
    #[astra(key = "@Distance")]
    pub distance: Option<u8>,
    #[astra(key = "@Hit")]
    pub hit: Option<i16>,
    #[astra(key = "@Critical")]
    pub critical: Option<i16>,
    #[astra(key = "@Avoid")]
    pub avoid: Option<i16>,
    #[astra(key = "@Secure")]
    pub secure: Option<i16>,
    #[astra(key = "@Enhance.Hp")]
    pub enhance_hp: Option<i8>,
    #[astra(key = "@Enhance.Str")]
    pub enhance_str: Option<i8>,
    #[astra(key = "@Enhance.Tech")]
    pub enhance_tech: Option<i8>,
    #[astra(key = "@Enhance.Quick")]
    pub enhance_quick: Option<i8>,
    #[astra(key = "@Enhance.Luck")]
    pub enhance_luck: Option<i8>,
    #[astra(key = "@Enhance.Def")]
    pub enhance_def: Option<i8>,
    #[astra(key = "@Enhance.Magic")]
    pub enhance_magic: Option<i8>,
    #[astra(key = "@Enhance.Mdef")]
    pub enhance_mdef: Option<i8>,
    #[astra(key = "@Enhance.Phys")]
    pub enhance_phys: Option<i8>,
    #[astra(key = "@Enhance.Move")]
    pub enhance_move: Option<i8>,
    #[astra(key = "@GrowRatio.Hp")]
    pub grow_ratio_hp: Option<i8>,
    #[astra(key = "@GrowRatio.Str")]
    pub grow_ratio_str: Option<i8>,
    #[astra(key = "@GrowRatio.Tech")]
    pub grow_ratio_tech: Option<i8>,
    #[astra(key = "@GrowRatio.Quick")]
    pub grow_ratio_quick: Option<i8>,
    #[astra(key = "@GrowRatio.Luck")]
    pub grow_ratio_luck: Option<i8>,
    #[astra(key = "@GrowRatio.Def")]
    pub grow_ratio_def: Option<i8>,
    #[astra(key = "@GrowRatio.Magic")]
    pub grow_ratio_magic: Option<i8>,
    #[astra(key = "@GrowRatio.Mdef")]
    pub grow_ratio_mdef: Option<i8>,
    #[astra(key = "@GrowRatio.Phys")]
    pub grow_ratio_phys: Option<i8>,
    #[astra(key = "@GrowRatio.Move")]
    pub grow_ratio_move: Option<i8>,
    #[astra(key = "@Price")]
    pub price: Option<i32>,
    #[astra(key = "@WeaponLevel")]
    pub weapon_level: String,
    #[astra(key = "@RodType")]
    pub rod_type: Option<i8>,
    #[astra(key = "@RodExp")]
    pub rod_exp: Option<u8>,
    #[astra(key = "@RateArena")]
    pub rate_arena: Option<u8>,
    #[astra(key = "@ShootEffect")]
    pub shoot_effect: String,
    #[astra(key = "@HitEffect")]
    pub hit_effect: String,
    #[astra(key = "@CannonEffect")]
    pub cannon_effect: String,
    #[astra(key = "@AttackMotion")]
    pub attack_motion: Option<i8>,
    #[astra(key = "@OverlapTerrain")]
    pub overlap_terrain: String,
    #[astra(key = "@EquipCondition")]
    pub equip_condition: String,
    #[astra(key = "@Flag")]
    pub flag: Option<i32>,
    #[astra(key = "@EquipSids")]
    pub equip_sids: Vec<String>,
    #[astra(key = "@PassiveSids")]
    pub passive_sids: Vec<String>,
    #[astra(key = "@GiveSids")]
    pub give_sids: Vec<String>,
    #[astra(key = "@AddTarget")]
    pub add_target: Option<i8>,
    #[astra(key = "@AddRange")]
    pub add_range: Option<u8>,
    #[astra(key = "@AddType")]
    pub add_type: Option<i8>,
    #[astra(key = "@AddPower")]
    pub add_power: Option<u8>,
    #[astra(key = "@AddSids")]
    pub add_sids: Vec<String>,
    #[astra(key = "@AddEffect")]
    pub add_effect: String,
    #[astra(key = "@AddHelp")]
    pub add_help: String,
    #[astra(key = "@HighRankItem")]
    pub high_rank_item: String,
}

#[derive(Astra, Debug)]
pub struct ItemCategory {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Category")]
    pub category: String,
    #[astra(key = "@Help")]
    pub help: String,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct ForgeImproveData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Rid", public_array)]
    pub rid: String,
    #[astra(key = "@Iron")]
    pub iron: Option<u16>,
    #[astra(key = "@Steel")]
    pub steel: Option<u16>,
    #[astra(key = "@Silver")]
    pub silver: Option<u16>,
    #[astra(key = "@Price")]
    pub price: Option<u16>,
    #[astra(key = "@Power")]
    pub power: Option<i8>,
    #[astra(key = "@Weight")]
    pub weight: Option<i8>,
    #[astra(key = "@Hit")]
    pub hit: Option<i8>,
    #[astra(key = "@Critical")]
    pub critical: Option<i8>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct ForgeEvolveData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Eid", public_array)]
    pub eid: String,
    #[astra(key = "@Iid")]
    pub iid: String,
    #[astra(key = "@Iron")]
    pub iron: Option<u16>,
    #[astra(key = "@Steel")]
    pub steel: Option<u16>,
    #[astra(key = "@Silver")]
    pub silver: Option<u16>,
    #[astra(key = "@Price")]
    pub price: Option<u16>,
    #[astra(key = "@RefineLevel")]
    pub refine_level: Option<u8>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct ForgeExchangeData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Operation")]
    pub operation: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Icon")]
    pub icon: String,
    #[astra(key = "@ToIron")]
    pub to_iron: Option<u16>,
    #[astra(key = "@ToSteel")]
    pub to_steel: Option<u16>,
    #[astra(key = "@ToSilver")]
    pub to_silver: Option<u16>,
    #[astra(key = "@ForIron")]
    pub for_iron: Option<u16>,
    #[astra(key = "@ForSteel")]
    pub for_steel: Option<u16>,
    #[astra(key = "@ForSilver")]
    pub for_silver: Option<u16>,
}

#[derive(Astra, Debug)]
pub struct WeaponRankData {
    #[astra(key = "@Level")]
    pub level: String,
    #[astra(key = "@Exp")]
    pub exp: Option<u8>,
    #[astra(key = "@Mastery")]
    pub mastery: Option<u8>,
    #[astra(key = "@Attack")]
    pub attack: Option<u8>,
    #[astra(key = "@Hit")]
    pub hit: Option<u8>,
    #[astra(key = "@Critical")]
    pub critical: Option<u8>,
    #[astra(key = "@Recover")]
    pub recover: Option<u8>,
}

#[derive(Astra, Debug)]
pub struct ItemInteractData {
    #[astra(key = "@Kind")]
    pub kind: String,
    #[astra(key = "@Flag")]
    pub flag: Option<u32>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct Accessory {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Aid", id)]
    pub aid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@NameM")]
    pub name_m: String,
    #[astra(key = "@HelpM")]
    pub help_m: String,
    #[astra(key = "@NameF")]
    pub name_f: String,
    #[astra(key = "@HelpF")]
    pub help_f: String,
    #[astra(key = "@First")]
    pub first: Option<bool>,
    #[astra(key = "@Amiibo")]
    pub amiibo: Option<bool>,
    #[astra(key = "@Asset")]
    pub asset: String,
    #[astra(key = "@CondtionCid")]
    pub condtion_cid: String,
    #[astra(key = "@CondtionSkills")]
    pub condtion_skills: Vec<String>,
    #[astra(key = "@CondtionGender")]
    pub condtion_gender: Option<i8>,
    #[astra(key = "@Gid")]
    pub gid: String,
    #[astra(key = "@Price")]
    pub price: Option<i32>,
    #[astra(key = "@Iron")]
    pub iron: Option<i32>,
    #[astra(key = "@Steel")]
    pub steel: Option<i32>,
    #[astra(key = "@Silver")]
    pub silver: Option<i32>,
    #[astra(key = "@Mask")]
    pub mask: Option<i32>,
}

#[derive(Astra, Debug)]
pub struct GiftData {
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@V00")]
    pub v_00: Option<i8>,
    #[astra(key = "@V01")]
    pub v_01: Option<i8>,
    #[astra(key = "@V02")]
    pub v_02: Option<i8>,
    #[astra(key = "@V03")]
    pub v_03: Option<i8>,
    #[astra(key = "@V04")]
    pub v_04: Option<i8>,
    #[astra(key = "@V05")]
    pub v_05: Option<i8>,
    #[astra(key = "@V06")]
    pub v_06: Option<i8>,
    #[astra(key = "@V07")]
    pub v_07: Option<i8>,
    #[astra(key = "@V08")]
    pub v_08: Option<i8>,
    #[astra(key = "@V09")]
    pub v_09: Option<i8>,
    #[astra(key = "@V10")]
    pub v_10: Option<i8>,
    #[astra(key = "@V11")]
    pub v_11: Option<i8>,
    #[astra(key = "@V12")]
    pub v_12: Option<i8>,
    #[astra(key = "@V13")]
    pub v_13: Option<i8>,
    #[astra(key = "@V14")]
    pub v_14: Option<i8>,
    #[astra(key = "@V15")]
    pub v_15: Option<i8>,
    #[astra(key = "@V16")]
    pub v_16: Option<i8>,
    #[astra(key = "@V17")]
    pub v_17: Option<i8>,
    #[astra(key = "@V18")]
    pub v_18: Option<i8>,
    #[astra(key = "@V19")]
    pub v_19: Option<i8>,
    #[astra(key = "@V20")]
    pub v_20: Option<i8>,
    #[astra(key = "@V21")]
    pub v_21: Option<i8>,
    #[astra(key = "@V22")]
    pub v_22: Option<i8>,
    #[astra(key = "@V23")]
    pub v_23: Option<i8>,
    #[astra(key = "@V24")]
    pub v_24: Option<i8>,
    #[astra(key = "@V25")]
    pub v_25: Option<i8>,
    #[astra(key = "@V26")]
    pub v_26: Option<i8>,
    #[astra(key = "@V27")]
    pub v_27: Option<i8>,
    #[astra(key = "@V28")]
    pub v_28: Option<i8>,
    #[astra(key = "@V29")]
    pub v_29: Option<i8>,
    #[astra(key = "@V30")]
    pub v_30: Option<i8>,
    #[astra(key = "@V31")]
    pub v_31: Option<i8>,
    #[astra(key = "@V32")]
    pub v_32: Option<i8>,
    #[astra(key = "@V33")]
    pub v_33: Option<i8>,
    #[astra(key = "@V34")]
    pub v_34: Option<i8>,
    #[astra(key = "@V35")]
    pub v_35: Option<i8>,
    #[astra(key = "@V36")]
    pub v_36: Option<i8>,
    #[astra(key = "@V37")]
    pub v_37: Option<i8>,
    #[astra(key = "@V38")]
    pub v_38: Option<i8>,
    #[astra(key = "@V39")]
    pub v_39: Option<i8>,
    #[astra(key = "@V40")]
    pub v_40: Option<i8>,
    #[astra(key = "@V41")]
    pub v_41: Option<i8>,
    #[astra(key = "@V42")]
    pub v_42: Option<i8>,
    #[astra(key = "@V43")]
    pub v_43: Option<i8>,
    #[astra(key = "@V44")]
    pub v_44: Option<i8>,
    #[astra(key = "@V45")]
    pub v_45: Option<i8>,
    #[astra(key = "@V46")]
    pub v_46: Option<i8>,
    #[astra(key = "@V47")]
    pub v_47: Option<i8>,
    #[astra(key = "@V48")]
    pub v_48: Option<i8>,
    #[astra(key = "@V49")]
    pub v_49: Option<i8>,
}

#[derive(Astra, Debug)]
pub struct RewardData {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Iid")]
    pub iid: String,
    #[astra(key = "@Ratio")]
    pub ratio: Option<f32>,
    #[astra(key = "@Factor")]
    pub factor: Option<f32>,
    #[astra(key = "@Min")]
    pub min: Option<f32>,
    #[astra(key = "@Max")]
    pub max: Option<f32>,
    #[astra(key = "@IsShow")]
    pub is_show: Option<bool>,
    #[astra(key = "@Condition")]
    pub condition: String,
}

#[derive(Astra, Debug)]
pub struct EngageWeaponEnhancementData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Iid", public_array)]
    pub iid: String,
    #[astra(key = "@PowerMat")]
    pub power_mat: Option<u16>,
    #[astra(key = "@HitMat")]
    pub hit_mat: Option<u16>,
    #[astra(key = "@CriticalMat")]
    pub critical_mat: Option<u16>,
    #[astra(key = "@AvoidMat")]
    pub avoid_mat: Option<u16>,
    #[astra(key = "@SecureMat")]
    pub secure_mat: Option<u16>,
    #[astra(key = "@TechMat")]
    pub tech_mat: Option<u16>,
    #[astra(key = "@QuickMat")]
    pub quick_mat: Option<u16>,
    #[astra(key = "@DefMat")]
    pub def_mat: Option<u16>,
    #[astra(key = "@MdefMat")]
    pub mdef_mat: Option<u16>,
    #[astra(key = "@EfficacyHorseMat")]
    pub efficacy_horse_mat: Option<u16>,
    #[astra(key = "@EfficacyArmorMat")]
    pub efficacy_armor_mat: Option<u16>,
    #[astra(key = "@EfficacyFlyMat")]
    pub efficacy_fly_mat: Option<u16>,
    #[astra(key = "@EfficacyDragonMat")]
    pub efficacy_dragon_mat: Option<u16>,
    #[astra(key = "@EfficacyMorphMat")]
    pub efficacy_morph_mat: Option<u16>,
    #[astra(key = "@PowerCapa")]
    pub power_capa: Option<u16>,
    #[astra(key = "@HitCapa")]
    pub hit_capa: Option<u16>,
    #[astra(key = "@CriticalCapa")]
    pub critical_capa: Option<u16>,
    #[astra(key = "@AvoidCapa")]
    pub avoid_capa: Option<u16>,
    #[astra(key = "@SecureCapa")]
    pub secure_capa: Option<u16>,
    #[astra(key = "@TechCapa")]
    pub tech_capa: Option<u16>,
    #[astra(key = "@QuickCapa")]
    pub quick_capa: Option<u16>,
    #[astra(key = "@DefCapa")]
    pub def_capa: Option<u16>,
    #[astra(key = "@MdefCapa")]
    pub mdef_capa: Option<u16>,
    #[astra(key = "@EfficacyHorseCapa")]
    pub efficacy_horse_capa: Option<u16>,
    #[astra(key = "@EfficacyArmorCapa")]
    pub efficacy_armor_capa: Option<u16>,
    #[astra(key = "@EfficacyFlyCapa")]
    pub efficacy_fly_capa: Option<u16>,
    #[astra(key = "@EfficacyDragonCapa")]
    pub efficacy_dragon_capa: Option<u16>,
    #[astra(key = "@EfficacyMorphCapa")]
    pub efficacy_morph_capa: Option<u16>,
    #[astra(key = "@Power")]
    pub power: Option<i8>,
    #[astra(key = "@Hit")]
    pub hit: Option<i8>,
    #[astra(key = "@Critical")]
    pub critical: Option<i8>,
    #[astra(key = "@Avoid")]
    pub avoid: Option<i8>,
    #[astra(key = "@Secure")]
    pub secure: Option<i8>,
    #[astra(key = "@Tech")]
    pub tech: Option<i8>,
    #[astra(key = "@Quick")]
    pub quick: Option<i8>,
    #[astra(key = "@Def")]
    pub def: Option<i8>,
    #[astra(key = "@Mdef")]
    pub mdef: Option<i8>,
}

#[derive(Astra, Debug)]
pub struct VersusRewardData {
    #[astra(key = "@TypeID")]
    pub type_id: String,
    #[astra(key = "@Iids")]
    pub iids: Vec<String>,
    #[astra(key = "@Nums")]
    pub nums: Vec<i32>,
    #[astra(key = "@Conditions")]
    pub conditions: Vec<String>,
}

#[derive(AstraBook)]
pub struct JobBook {
    pub jobs: Sheet<IndexMap<String, Job>>,
    pub fighting_styles: Sheet<Vec<FightingStyle>>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct Job {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Jid", id)]
    pub jid: String,
    #[astra(key = "@Aid")]
    pub aid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@UnitIconID_M")]
    pub unit_icon_id_m: String,
    #[astra(key = "@UnitIconID_F")]
    pub unit_icon_id_f: String,
    #[astra(key = "@UnitIconWeaponID")]
    pub unit_icon_weapon_id: String,
    #[astra(key = "@Rank")]
    pub rank: Option<i8>,
    #[astra(key = "@StyleName")]
    pub style_name: String,
    #[astra(key = "@MoveType")]
    pub move_type: Option<i8>,
    #[astra(key = "@StepFrame")]
    pub step_frame: Option<u8>,
    #[astra(key = "@MaxLevel")]
    pub max_level: Option<u8>,
    #[astra(key = "@InternalLevel")]
    pub internal_level: Option<i8>,
    #[astra(key = "@Sort")]
    pub sort: Option<u16>,
    #[astra(key = "@Flag")]
    pub flag: Option<u8>,
    #[astra(key = "@WeaponNone")]
    pub weapon_none: Option<i8>,
    #[astra(key = "@WeaponSword")]
    pub weapon_sword: Option<i8>,
    #[astra(key = "@WeaponLance")]
    pub weapon_lance: Option<i8>,
    #[astra(key = "@WeaponAxe")]
    pub weapon_axe: Option<i8>,
    #[astra(key = "@WeaponBow")]
    pub weapon_bow: Option<i8>,
    #[astra(key = "@WeaponDagger")]
    pub weapon_dagger: Option<i8>,
    #[astra(key = "@WeaponMagic")]
    pub weapon_magic: Option<i8>,
    #[astra(key = "@WeaponRod")]
    pub weapon_rod: Option<i8>,
    #[astra(key = "@WeaponFist")]
    pub weapon_fist: Option<i8>,
    #[astra(key = "@WeaponSpecial")]
    pub weapon_special: Option<i8>,
    #[astra(key = "@WeaponTool")]
    pub weapon_tool: Option<i8>,
    #[astra(key = "@MaxWeaponLevelNone")]
    pub max_weapon_level_none: String,
    #[astra(key = "@MaxWeaponLevelSword")]
    pub max_weapon_level_sword: String,
    #[astra(key = "@MaxWeaponLevelLance")]
    pub max_weapon_level_lance: String,
    #[astra(key = "@MaxWeaponLevelAxe")]
    pub max_weapon_level_axe: String,
    #[astra(key = "@MaxWeaponLevelBow")]
    pub max_weapon_level_bow: String,
    #[astra(key = "@MaxWeaponLevelDagger")]
    pub max_weapon_level_dagger: String,
    #[astra(key = "@MaxWeaponLevelMagic")]
    pub max_weapon_level_magic: String,
    #[astra(key = "@MaxWeaponLevelRod")]
    pub max_weapon_level_rod: String,
    #[astra(key = "@MaxWeaponLevelFist")]
    pub max_weapon_level_fist: String,
    #[astra(key = "@MaxWeaponLevelSpecial")]
    pub max_weapon_level_special: String,
    #[astra(key = "@Base.Hp")]
    pub base_hp: Option<u8>,
    #[astra(key = "@Base.Str")]
    pub base_str: Option<u8>,
    #[astra(key = "@Base.Tech")]
    pub base_tech: Option<u8>,
    #[astra(key = "@Base.Quick")]
    pub base_quick: Option<u8>,
    #[astra(key = "@Base.Luck")]
    pub base_luck: Option<u8>,
    #[astra(key = "@Base.Def")]
    pub base_def: Option<u8>,
    #[astra(key = "@Base.Magic")]
    pub base_magic: Option<u8>,
    #[astra(key = "@Base.Mdef")]
    pub base_mdef: Option<u8>,
    #[astra(key = "@Base.Phys")]
    pub base_phys: Option<u8>,
    #[astra(key = "@Base.Sight")]
    pub base_sight: Option<u8>,
    #[astra(key = "@Base.Move")]
    pub base_move: Option<u8>,
    #[astra(key = "@Limit.Hp")]
    pub limit_hp: Option<u8>,
    #[astra(key = "@Limit.Str")]
    pub limit_str: Option<u8>,
    #[astra(key = "@Limit.Tech")]
    pub limit_tech: Option<u8>,
    #[astra(key = "@Limit.Quick")]
    pub limit_quick: Option<u8>,
    #[astra(key = "@Limit.Luck")]
    pub limit_luck: Option<u8>,
    #[astra(key = "@Limit.Def")]
    pub limit_def: Option<u8>,
    #[astra(key = "@Limit.Magic")]
    pub limit_magic: Option<u8>,
    #[astra(key = "@Limit.Mdef")]
    pub limit_mdef: Option<u8>,
    #[astra(key = "@Limit.Phys")]
    pub limit_phys: Option<u8>,
    #[astra(key = "@Limit.Sight")]
    pub limit_sight: Option<u8>,
    #[astra(key = "@Limit.Move")]
    pub limit_move: Option<u8>,
    #[astra(key = "@BaseGrow.Hp")]
    pub base_grow_hp: Option<u8>,
    #[astra(key = "@BaseGrow.Str")]
    pub base_grow_str: Option<u8>,
    #[astra(key = "@BaseGrow.Tech")]
    pub base_grow_tech: Option<u8>,
    #[astra(key = "@BaseGrow.Quick")]
    pub base_grow_quick: Option<u8>,
    #[astra(key = "@BaseGrow.Luck")]
    pub base_grow_luck: Option<u8>,
    #[astra(key = "@BaseGrow.Def")]
    pub base_grow_def: Option<u8>,
    #[astra(key = "@BaseGrow.Magic")]
    pub base_grow_magic: Option<u8>,
    #[astra(key = "@BaseGrow.Mdef")]
    pub base_grow_mdef: Option<u8>,
    #[astra(key = "@BaseGrow.Phys")]
    pub base_grow_phys: Option<u8>,
    #[astra(key = "@BaseGrow.Sight")]
    pub base_grow_sight: Option<u8>,
    #[astra(key = "@BaseGrow.Move")]
    pub base_grow_move: Option<u8>,
    #[astra(key = "@DiffGrow.Hp")]
    pub diff_grow_hp: Option<i8>,
    #[astra(key = "@DiffGrow.Str")]
    pub diff_grow_str: Option<i8>,
    #[astra(key = "@DiffGrow.Tech")]
    pub diff_grow_tech: Option<i8>,
    #[astra(key = "@DiffGrow.Quick")]
    pub diff_grow_quick: Option<i8>,
    #[astra(key = "@DiffGrow.Luck")]
    pub diff_grow_luck: Option<i8>,
    #[astra(key = "@DiffGrow.Def")]
    pub diff_grow_def: Option<i8>,
    #[astra(key = "@DiffGrow.Magic")]
    pub diff_grow_magic: Option<i8>,
    #[astra(key = "@DiffGrow.Mdef")]
    pub diff_grow_mdef: Option<i8>,
    #[astra(key = "@DiffGrow.Phys")]
    pub diff_grow_phys: Option<i8>,
    #[astra(key = "@DiffGrow.Sight")]
    pub diff_grow_sight: Option<i8>,
    #[astra(key = "@DiffGrow.Move")]
    pub diff_grow_move: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Hp")]
    pub diff_grow_normal_hp: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Str")]
    pub diff_grow_normal_str: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Tech")]
    pub diff_grow_normal_tech: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Quick")]
    pub diff_grow_normal_quick: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Luck")]
    pub diff_grow_normal_luck: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Def")]
    pub diff_grow_normal_def: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Magic")]
    pub diff_grow_normal_magic: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Mdef")]
    pub diff_grow_normal_mdef: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Phys")]
    pub diff_grow_normal_phys: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Sight")]
    pub diff_grow_normal_sight: Option<i8>,
    #[astra(key = "@DiffGrowNormal.Move")]
    pub diff_grow_normal_move: Option<i8>,
    #[astra(key = "@DiffGrowHard.Hp")]
    pub diff_grow_hard_hp: Option<i8>,
    #[astra(key = "@DiffGrowHard.Str")]
    pub diff_grow_hard_str: Option<i8>,
    #[astra(key = "@DiffGrowHard.Tech")]
    pub diff_grow_hard_tech: Option<i8>,
    #[astra(key = "@DiffGrowHard.Quick")]
    pub diff_grow_hard_quick: Option<i8>,
    #[astra(key = "@DiffGrowHard.Luck")]
    pub diff_grow_hard_luck: Option<i8>,
    #[astra(key = "@DiffGrowHard.Def")]
    pub diff_grow_hard_def: Option<i8>,
    #[astra(key = "@DiffGrowHard.Magic")]
    pub diff_grow_hard_magic: Option<i8>,
    #[astra(key = "@DiffGrowHard.Mdef")]
    pub diff_grow_hard_mdef: Option<i8>,
    #[astra(key = "@DiffGrowHard.Phys")]
    pub diff_grow_hard_phys: Option<i8>,
    #[astra(key = "@DiffGrowHard.Sight")]
    pub diff_grow_hard_sight: Option<i8>,
    #[astra(key = "@DiffGrowHard.Move")]
    pub diff_grow_hard_move: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Hp")]
    pub diff_grow_lunatic_hp: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Str")]
    pub diff_grow_lunatic_str: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Tech")]
    pub diff_grow_lunatic_tech: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Quick")]
    pub diff_grow_lunatic_quick: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Luck")]
    pub diff_grow_lunatic_luck: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Def")]
    pub diff_grow_lunatic_def: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Magic")]
    pub diff_grow_lunatic_magic: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Mdef")]
    pub diff_grow_lunatic_mdef: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Phys")]
    pub diff_grow_lunatic_phys: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Sight")]
    pub diff_grow_lunatic_sight: Option<i8>,
    #[astra(key = "@DiffGrowLunatic.Move")]
    pub diff_grow_lunatic_move: Option<i8>,
    #[astra(key = "@HighJob1")]
    pub high_job_1: String,
    #[astra(key = "@HighJob2")]
    pub high_job_2: String,
    #[astra(key = "@LowJob")]
    pub low_job: String,
    #[astra(key = "@CCItems")]
    pub cc_items: Vec<String>,
    #[astra(key = "@ShortName")]
    pub short_name: String,
    #[astra(key = "@UniqueItems")]
    pub unique_items: Vec<String>,
    #[astra(key = "@Skills")]
    pub skills: Vec<String>,
    #[astra(key = "@LearningSkill")]
    pub learning_skill: String,
    #[astra(key = "@LunaticSkill")]
    pub lunatic_skill: String,
    #[astra(key = "@Attrs")]
    pub attrs: Option<i32>,
}

#[derive(Astra, Debug)]
pub struct FightingStyle {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Style")]
    pub style: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@Skills")]
    pub skills: Vec<String>,
}

#[derive(AstraBook)]
pub struct RelianceBook {
    pub reliance_data: Sheet<IndexMap<String, RelianceData>>,
    pub reliance_exp_data: Sheet<IndexMap<String, RelianceExpData>>,
    pub relianace_bonus_data: Sheet<IndexMap<String, Vec<RelianceBonusData>>>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct RelianceData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Pid", id)]
    pub pid: String,
    #[astra(key = "@ExpType0")]
    pub exp_type_0: Option<u8>,
    #[astra(key = "@ExpType1")]
    pub exp_type_1: Option<u8>,
    #[astra(key = "@ExpType2")]
    pub exp_type_2: Option<u8>,
    #[astra(key = "@ExpType3")]
    pub exp_type_3: Option<u8>,
    #[astra(key = "@ExpType4")]
    pub exp_type_4: Option<u8>,
    #[astra(key = "@ExpType5")]
    pub exp_type_5: Option<u8>,
    #[astra(key = "@ExpType6")]
    pub exp_type_6: Option<u8>,
    #[astra(key = "@ExpType7")]
    pub exp_type_7: Option<u8>,
    #[astra(key = "@ExpType8")]
    pub exp_type_8: Option<u8>,
    #[astra(key = "@ExpType9")]
    pub exp_type_9: Option<u8>,
    #[astra(key = "@ExpType10")]
    pub exp_type_10: Option<u8>,
    #[astra(key = "@ExpType11")]
    pub exp_type_11: Option<u8>,
    #[astra(key = "@ExpType12")]
    pub exp_type_12: Option<u8>,
    #[astra(key = "@ExpType13")]
    pub exp_type_13: Option<u8>,
    #[astra(key = "@ExpType14")]
    pub exp_type_14: Option<u8>,
    #[astra(key = "@ExpType15")]
    pub exp_type_15: Option<u8>,
    #[astra(key = "@ExpType16")]
    pub exp_type_16: Option<u8>,
    #[astra(key = "@ExpType17")]
    pub exp_type_17: Option<u8>,
    #[astra(key = "@ExpType18")]
    pub exp_type_18: Option<u8>,
    #[astra(key = "@ExpType19")]
    pub exp_type_19: Option<u8>,
    #[astra(key = "@ExpType20")]
    pub exp_type_20: Option<u8>,
    #[astra(key = "@ExpType21")]
    pub exp_type_21: Option<u8>,
    #[astra(key = "@ExpType22")]
    pub exp_type_22: Option<u8>,
    #[astra(key = "@ExpType23")]
    pub exp_type_23: Option<u8>,
    #[astra(key = "@ExpType24")]
    pub exp_type_24: Option<u8>,
    #[astra(key = "@ExpType25")]
    pub exp_type_25: Option<u8>,
    #[astra(key = "@ExpType26")]
    pub exp_type_26: Option<u8>,
    #[astra(key = "@ExpType27")]
    pub exp_type_27: Option<u8>,
    #[astra(key = "@ExpType28")]
    pub exp_type_28: Option<u8>,
    #[astra(key = "@ExpType29")]
    pub exp_type_29: Option<u8>,
    #[astra(key = "@ExpType30")]
    pub exp_type_30: Option<u8>,
    #[astra(key = "@ExpType31")]
    pub exp_type_31: Option<u8>,
    #[astra(key = "@ExpType32")]
    pub exp_type_32: Option<u8>,
    #[astra(key = "@ExpType33")]
    pub exp_type_33: Option<u8>,
    #[astra(key = "@ExpType34")]
    pub exp_type_34: Option<u8>,
    #[astra(key = "@ExpType35")]
    pub exp_type_35: Option<u8>,
    #[astra(key = "@ExpType36")]
    pub exp_type_36: Option<u8>,
    #[astra(key = "@ExpType37")]
    pub exp_type_37: Option<u8>,
    #[astra(key = "@ExpType38")]
    pub exp_type_38: Option<u8>,
    #[astra(key = "@ExpType39")]
    pub exp_type_39: Option<u8>,
    #[astra(key = "@ExpType40")]
    pub exp_type_40: Option<u8>,
    #[astra(key = "@ExpType41")]
    pub exp_type_41: Option<u8>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct RelianceExpData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Rexid", id)]
    pub rexid: String,
    #[astra(key = "@ExpC")]
    pub exp_c: Option<u8>,
    #[astra(key = "@ExpB")]
    pub exp_b: Option<u8>,
    #[astra(key = "@ExpA")]
    pub exp_a: Option<u8>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct RelianceBonusData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Name", public_array)]
    pub name: String,
    #[astra(key = "@Level")]
    pub level: Option<i8>,
    #[astra(key = "@Hit")]
    pub hit: Option<i8>,
    #[astra(key = "@Critical")]
    pub critical: Option<i8>,
    #[astra(key = "@Avoid")]
    pub avoid: Option<i8>,
    #[astra(key = "@Secure")]
    pub secure: Option<i8>,
}

#[derive(AstraBook)]
pub struct ShopBook {
    pub armory_shop_inventory: Sheet<IndexMap<String, Vec<ShopInventory>>>,
    pub item_shop_inventory: Sheet<IndexMap<String, Vec<ShopInventory>>>,
    pub flea_market_shop_inventory: Sheet<IndexMap<String, Vec<ShopInventory>>>,
    pub accessory_shop_inventory: Sheet<IndexMap<String, Vec<AccessoryShopInventory>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct ShopInventory {
    #[astra(key = "@Condition", public_array)]
    pub condition: String,
    #[astra(key = "@Iid")]
    pub iid: String,
    #[astra(key = "@Stock")]
    pub stock: Option<i16>,
    #[astra(key = "@Attribute")]
    pub attribute: Option<i8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AccessoryShopInventory {
    #[astra(key = "@Condition", public_array)]
    pub condition: String,
    #[astra(key = "@Aid")]
    pub aid: String,
}

#[derive(AstraBook)]
pub struct SkillBook {
    pub skills: Sheet<IndexMap<String, Skill>>,
}

#[derive(Astra, Debug, Default, Clone)]
pub struct Skill {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Sid", id)]
    pub sid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@CommandName")]
    pub command_name: String,
    #[astra(key = "@CommandHelp")]
    pub command_help: String,
    #[astra(key = "@CommandWarning")]
    pub command_warning: String,
    #[astra(key = "@RootCommandSid")]
    pub root_command_sid: String,
    #[astra(key = "@IconKind")]
    pub icon_kind: Option<i8>,
    #[astra(key = "@IconLabel")]
    pub icon_label: String,
    #[astra(key = "@IconBmap")]
    pub icon_bmap: String,
    #[astra(key = "@Priority")]
    pub priority: Option<u8>,
    #[astra(key = "@Layer")]
    pub layer: Option<i32>,
    #[astra(key = "@Order")]
    pub order: Option<i8>,
    #[astra(key = "@Cycle")]
    pub cycle: Option<i8>,
    #[astra(key = "@Life")]
    pub life: Option<u8>,
    #[astra(key = "@Timing")]
    pub timing: Option<i8>,
    #[astra(key = "@Target")]
    pub target: Option<i8>,
    #[astra(key = "@Frequency")]
    pub frequency: Option<i8>,
    #[astra(key = "@Stand")]
    pub stand: Option<i8>,
    #[astra(key = "@Action")]
    pub action: Option<i8>,
    #[astra(key = "@Condition")]
    pub condition: String,
    #[astra(key = "@ActNames")]
    pub act_names: Vec<String>,
    #[astra(key = "@ActOperations")]
    pub act_operations: Vec<String>,
    #[astra(key = "@ActValues")]
    pub act_values: Vec<String>,
    #[astra(key = "@AroundCenter")]
    pub around_center: Option<i8>,
    #[astra(key = "@AroundTarget")]
    pub around_target: Option<i8>,
    #[astra(key = "@AroundCondition")]
    pub around_condition: String,
    #[astra(key = "@AroundName")]
    pub around_name: String,
    #[astra(key = "@AroundOperation")]
    pub around_operation: String,
    #[astra(key = "@AroundValue")]
    pub around_value: String,
    #[astra(key = "@GiveTarget")]
    pub give_target: Option<i8>,
    #[astra(key = "@GiveCondition")]
    pub give_condition: String,
    #[astra(key = "@GiveSids")]
    pub give_sids: Vec<String>,
    #[astra(key = "@RemoveSids")]
    pub remove_sids: Vec<String>,
    #[astra(key = "@SyncConditions")]
    pub sync_conditions: Vec<String>,
    #[astra(key = "@SyncSids")]
    pub sync_sids: Vec<String>,
    #[astra(key = "@RebirthSid")]
    pub rebirth_sid: String,
    #[astra(key = "@EngageSid")]
    pub engage_sid: String,
    #[astra(key = "@ChangeSids")]
    pub change_sids: Vec<String>,
    #[astra(key = "@CooperationSkill")]
    pub cooperation_skill: String,
    #[astra(key = "@HorseSkill")]
    pub horse_skill: String,
    #[astra(key = "@CovertSkill")]
    pub covert_skill: String,
    #[astra(key = "@HeavySkill")]
    pub heavy_skill: String,
    #[astra(key = "@FlySkill")]
    pub fly_skill: String,
    #[astra(key = "@MagicSkill")]
    pub magic_skill: String,
    #[astra(key = "@PranaSkill")]
    pub prana_skill: String,
    #[astra(key = "@DragonSkill")]
    pub dragon_skill: String,
    #[astra(key = "@AttackRange")]
    pub attack_range: String,
    #[astra(key = "@OverlapRange")]
    pub overlap_range: String,
    #[astra(key = "@OverlapTerrain")]
    pub overlap_terrain: String,
    #[astra(key = "@ZocRange")]
    pub zoc_range: String,
    #[astra(key = "@ZocType")]
    pub zoc_type: Option<i8>,
    #[astra(key = "@Work")]
    pub work: Option<i8>,
    #[astra(key = "@WorkOperation")]
    pub work_operation: String,
    #[astra(key = "@WorkValue")]
    pub work_value: Option<f32>,
    #[astra(key = "@Power")]
    pub power: Option<i8>,
    #[astra(key = "@Rewarp")]
    pub rewarp: Option<u8>,
    #[astra(key = "@Removable")]
    pub removable: Option<u8>,
    #[astra(key = "@VisionCount")]
    pub vision_count: Option<u8>,
    #[astra(key = "@Cost")]
    pub cost: Option<u8>,
    #[astra(key = "@MoveSelf")]
    pub move_self: Option<i8>,
    #[astra(key = "@MoveTarget")]
    pub move_target: Option<i8>,
    #[astra(key = "@RangeTarget")]
    pub range_target: Option<i8>,
    #[astra(key = "@RangeI")]
    pub range_i: Option<u8>,
    #[astra(key = "@RangeO")]
    pub range_o: Option<u8>,
    #[astra(key = "@RangeAdd")]
    pub range_add: Option<u8>,
    #[astra(key = "@RangeExtend")]
    pub range_extend: Option<u8>,
    #[astra(key = "@Flag")]
    pub flag: Option<u64>,
    #[astra(key = "@Efficacy")]
    pub efficacy: Option<i32>,
    #[astra(key = "@EfficacyValue")]
    pub efficacy_value: Option<u8>,
    #[astra(key = "@EfficacyIgnore")]
    pub efficacy_ignore: Option<i32>,
    #[astra(key = "@BadState")]
    pub bad_state: Option<i32>,
    #[astra(key = "@BadIgnore")]
    pub bad_ignore: Option<i32>,
    #[astra(key = "@WeaponProhibit")]
    pub weapon_prohibit: Option<i32>,
    #[astra(key = "@EnhanceLevel")]
    pub enhance_level: Option<i8>,
    #[astra(key = "@EnhanceValue.Hp")]
    pub enhance_value_hp: Option<i8>,
    #[astra(key = "@EnhanceValue.Str")]
    pub enhance_value_str: Option<i8>,
    #[astra(key = "@EnhanceValue.Tech")]
    pub enhance_value_tech: Option<i8>,
    #[astra(key = "@EnhanceValue.Quick")]
    pub enhance_value_quick: Option<i8>,
    #[astra(key = "@EnhanceValue.Luck")]
    pub enhance_value_luck: Option<i8>,
    #[astra(key = "@EnhanceValue.Def")]
    pub enhance_value_def: Option<i8>,
    #[astra(key = "@EnhanceValue.Magic")]
    pub enhance_value_magic: Option<i8>,
    #[astra(key = "@EnhanceValue.Mdef")]
    pub enhance_value_mdef: Option<i8>,
    #[astra(key = "@EnhanceValue.Phys")]
    pub enhance_value_phys: Option<i8>,
    #[astra(key = "@EnhanceValue.Move")]
    pub enhance_value_move: Option<i8>,
    #[astra(key = "@WeaponLevel.None")]
    pub weapon_level_none: Option<i8>,
    #[astra(key = "@WeaponLevel.Sword")]
    pub weapon_level_sword: Option<i8>,
    #[astra(key = "@WeaponLevel.Lance")]
    pub weapon_level_lance: Option<i8>,
    #[astra(key = "@WeaponLevel.Axe")]
    pub weapon_level_axe: Option<i8>,
    #[astra(key = "@WeaponLevel.Bow")]
    pub weapon_level_bow: Option<i8>,
    #[astra(key = "@WeaponLevel.Dagger")]
    pub weapon_level_dagger: Option<i8>,
    #[astra(key = "@WeaponLevel.Magic")]
    pub weapon_level_magic: Option<i8>,
    #[astra(key = "@WeaponLevel.Rod")]
    pub weapon_level_rod: Option<i8>,
    #[astra(key = "@WeaponLevel.Fist")]
    pub weapon_level_fist: Option<i8>,
    #[astra(key = "@WeaponLevel.Special")]
    pub weapon_level_special: Option<i8>,
    #[astra(key = "@EquipIids")]
    pub equip_iids: Vec<String>,
    #[astra(key = "@Effect")]
    pub effect: String,
    #[astra(key = "@InheritanceCost")]
    pub inheritance_cost: Option<u16>,
    #[astra(key = "@InheritanceSort")]
    pub inheritance_sort: Option<u16>,
}

#[derive(Debug, AstraBook)]
pub struct DisposBook {
    pub spawns: Sheet<IndexMap<String, Vec<Spawn>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct Spawn {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Pid")]
    pub pid: String,
    #[astra(key = "@Force")]
    pub force: Option<i8>,
    #[astra(key = "@Flag")]
    pub flag: Option<u16>,
    #[astra(key = "@AppearX")]
    pub appear_x: Option<i8>,
    #[astra(key = "@AppearY")]
    pub appear_y: Option<i8>,
    #[astra(key = "@DisposX")]
    pub dispos_x: Option<i8>,
    #[astra(key = "@DisposY")]
    pub dispos_y: Option<i8>,
    #[astra(key = "@Direction")]
    pub direction: Option<i8>,
    #[astra(key = "@LevelN")]
    pub level_n: Option<u8>,
    #[astra(key = "@LevelH")]
    pub level_h: Option<u8>,
    #[astra(key = "@LevelL")]
    pub level_l: Option<u8>,
    #[astra(key = "@Jid")]
    pub jid: String,
    #[astra(key = "@Item1.Iid")]
    pub item_1_iid: String,
    #[astra(key = "@Item1.Drop")]
    pub item_1_drop: Option<i8>,
    #[astra(key = "@Item2.Iid")]
    pub item_2_iid: String,
    #[astra(key = "@Item2.Drop")]
    pub item_2_drop: Option<i8>,
    #[astra(key = "@Item3.Iid")]
    pub item_3_iid: String,
    #[astra(key = "@Item3.Drop")]
    pub item_3_drop: Option<i8>,
    #[astra(key = "@Item4.Iid")]
    pub item_4_iid: String,
    #[astra(key = "@Item4.Drop")]
    pub item_4_drop: Option<i8>,
    #[astra(key = "@Item5.Iid")]
    pub item_5_iid: String,
    #[astra(key = "@Item5.Drop")]
    pub item_5_drop: Option<i8>,
    #[astra(key = "@Item6.Iid")]
    pub item_6_iid: String,
    #[astra(key = "@Item6.Drop")]
    pub item_6_drop: Option<i8>,
    #[astra(key = "@Sid")]
    pub sid: String,
    #[astra(key = "@Bid")]
    pub bid: String,
    #[astra(key = "@Gid")]
    pub gid: String,
    #[astra(key = "@HpStockCount")]
    pub hp_stock_count: Option<u8>,
    #[astra(key = "@State0")]
    pub state_0: Option<i8>,
    #[astra(key = "@State1")]
    pub state_1: Option<i8>,
    #[astra(key = "@State2")]
    pub state_2: Option<i8>,
    #[astra(key = "@State3")]
    pub state_3: Option<i8>,
    #[astra(key = "@State4")]
    pub state_4: Option<i8>,
    #[astra(key = "@State5")]
    pub state_5: Option<i8>,
    #[astra(key = "@AI_ActionName")]
    pub ai_action_name: String,
    #[astra(key = "@AI_ActionVal")]
    pub ai_action_val: String,
    #[astra(key = "@AI_MindName")]
    pub ai_mind_name: String,
    #[astra(key = "@AI_MindVal")]
    pub ai_mind_val: String,
    #[astra(key = "@AI_AttackName")]
    pub ai_attack_name: String,
    #[astra(key = "@AI_AttackVal")]
    pub ai_attack_val: String,
    #[astra(key = "@AI_MoveName")]
    pub ai_move_name: String,
    #[astra(key = "@AI_MoveVal")]
    pub ai_move_val: String,
    #[astra(key = "@AI_BattleRate")]
    pub ai_battle_rate: String,
    #[astra(key = "@AI_Priority")]
    pub ai_priority: Option<u8>,
    #[astra(key = "@AI_HealRateA")]
    pub ai_heal_rate_a: Option<i8>,
    #[astra(key = "@AI_HealRateB")]
    pub ai_heal_rate_b: Option<i8>,
    #[astra(key = "@AI_BandNo")]
    pub ai_band_no: Option<u32>,
    #[astra(key = "@AI_MoveLimit")]
    pub ai_move_limit: String,
    #[astra(key = "@AI_Flag")]
    pub ai_flag: Option<u32>,
}

#[derive(AstraBook)]
pub struct TerrainBook {
    pub terrain_data: Sheet<IndexMap<String, TerrainData>>,
    pub terrain_cost_data: Sheet<Vec<TerrainCostData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TerrainData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Tid", id)]
    pub tid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@CostName")]
    pub cost_name: String,
    #[astra(key = "@Layer")]
    pub layer: Option<i8>,
    #[astra(key = "@Prohibition")]
    pub prohibition: Option<i8>,
    #[astra(key = "@Sight")]
    pub sight: Option<u8>,
    #[astra(key = "@Destroyer")]
    pub destroyer: Option<i8>,
    #[astra(key = "@Hp_N")]
    pub hp_n: Option<u8>,
    #[astra(key = "@Hp_H")]
    pub hp_h: Option<u8>,
    #[astra(key = "@Hp_L")]
    pub hp_l: Option<u8>,
    #[astra(key = "@Defense")]
    pub defense: Option<i8>,
    #[astra(key = "@Avoid")]
    pub avoid: Option<i8>,
    #[astra(key = "@PlayerDefense")]
    pub player_defense: Option<i8>,
    #[astra(key = "@EnemyDefense")]
    pub enemy_defense: Option<i8>,
    #[astra(key = "@PlayerAvoid")]
    pub player_avoid: Option<i8>,
    #[astra(key = "@EnemyAvoid")]
    pub enemy_avoid: Option<i8>,
    #[astra(key = "@Heal")]
    pub heal: Option<i8>,
    #[astra(key = "@Life")]
    pub life: Option<u8>,
    #[astra(key = "@MoveCost")]
    pub move_cost: Option<u8>,
    #[astra(key = "@FlyCost")]
    pub fly_cost: Option<u8>,
    #[astra(key = "@MoveFirst")]
    pub move_first: Option<i8>,
    #[astra(key = "@Offset")]
    pub offset: Option<f32>,
    #[astra(key = "@PutEffect")]
    pub put_effect: String,
    #[astra(key = "@Minimap")]
    pub minimap: String,
    #[astra(key = "@CannonSkill")]
    pub cannon_skill: String,
    #[astra(key = "@CannonShellsN")]
    pub cannon_shells_n: Option<u8>,
    #[astra(key = "@CannonShellsH")]
    pub cannon_shells_h: Option<u8>,
    #[astra(key = "@CannonShellsL")]
    pub cannon_shells_l: Option<u8>,
    #[astra(key = "@ChangeTid")]
    pub change_tid: String,
    #[astra(key = "@ChangeEncount")]
    pub change_encount: String,
    #[astra(key = "@Command")]
    pub command: Option<i8>,
    #[astra(key = "@Flag")]
    pub flag: Option<i32>,
    #[astra(key = "@PutAllow")]
    pub put_allow: Option<u8>,
    #[astra(key = "@Height")]
    pub height: Option<f32>,
    #[astra(key = "@ColorR")]
    pub color_r: Option<u8>,
    #[astra(key = "@ColorG")]
    pub color_g: Option<u8>,
    #[astra(key = "@ColorB")]
    pub color_b: Option<u8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TerrainCostData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@None")]
    pub none: Option<u8>,
    #[astra(key = "@Foot")]
    pub foot: Option<u8>,
    #[astra(key = "@Horse")]
    pub horse: Option<u8>,
    #[astra(key = "@Fly")]
    pub fly: Option<u8>,
    #[astra(key = "@Dragon")]
    pub dragon: Option<u8>,
    #[astra(key = "@Pad")]
    pub pad: Option<u8>,
    #[astra(key = "@ColorR")]
    pub color_r: Option<u8>,
    #[astra(key = "@ColorG")]
    pub color_g: Option<u8>,
    #[astra(key = "@ColorB")]
    pub color_b: Option<u8>,
    #[astra(key = "@ColorA")]
    pub color_a: Option<u8>,
}

#[derive(AstraBook)]
pub struct ParamsBook {
    pub game_params: Sheet<Vec<GameParam>>,
}

#[derive(Debug, Default, Astra, Clone)]
pub struct GameParam {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@English")]
    pub english: String,
    #[astra(key = "@Value")]
    pub value: Option<f32>,
    #[astra(key = "@Min")]
    pub min: Option<f32>,
    #[astra(key = "@Max")]
    pub max: Option<f32>,
    #[astra(key = "@Step")]
    pub step: Option<f32>,
    #[astra(key = "@Enum")]
    pub en: String,
}
