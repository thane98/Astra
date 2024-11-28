use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

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
    pub kind: i8,
    #[astra(key = "@UseType")]
    pub use_type: i8,
    #[astra(key = "@WeaponAttr")]
    pub weapon_attr: i8,
    #[astra(key = "@Icon")]
    pub icon: String,
    #[astra(key = "@Endurance")]
    pub endurance: u8,
    #[astra(key = "@Power")]
    pub power: u8,
    #[astra(key = "@Weight")]
    pub weight: u8,
    #[astra(key = "@RangeI")]
    pub range_i: u8,
    #[astra(key = "@RangeO")]
    pub range_o: u8,
    #[astra(key = "@Distance")]
    pub distance: u8,
    #[astra(key = "@Hit")]
    pub hit: i16,
    #[astra(key = "@Critical")]
    pub critical: i16,
    #[astra(key = "@Avoid")]
    pub avoid: i16,
    #[astra(key = "@Secure")]
    pub secure: i16,
    #[astra(key = "@Enhance.Hp")]
    pub enhance_hp: i8,
    #[astra(key = "@Enhance.Str")]
    pub enhance_str: i8,
    #[astra(key = "@Enhance.Tech")]
    pub enhance_tech: i8,
    #[astra(key = "@Enhance.Quick")]
    pub enhance_quick: i8,
    #[astra(key = "@Enhance.Luck")]
    pub enhance_luck: i8,
    #[astra(key = "@Enhance.Def")]
    pub enhance_def: i8,
    #[astra(key = "@Enhance.Magic")]
    pub enhance_magic: i8,
    #[astra(key = "@Enhance.Mdef")]
    pub enhance_mdef: i8,
    #[astra(key = "@Enhance.Phys")]
    pub enhance_phys: i8,
    #[astra(key = "@Enhance.Move")]
    pub enhance_move: i8,
    #[astra(key = "@GrowRatio.Hp")]
    pub grow_ratio_hp: i8,
    #[astra(key = "@GrowRatio.Str")]
    pub grow_ratio_str: i8,
    #[astra(key = "@GrowRatio.Tech")]
    pub grow_ratio_tech: i8,
    #[astra(key = "@GrowRatio.Quick")]
    pub grow_ratio_quick: i8,
    #[astra(key = "@GrowRatio.Luck")]
    pub grow_ratio_luck: i8,
    #[astra(key = "@GrowRatio.Def")]
    pub grow_ratio_def: i8,
    #[astra(key = "@GrowRatio.Magic")]
    pub grow_ratio_magic: i8,
    #[astra(key = "@GrowRatio.Mdef")]
    pub grow_ratio_mdef: i8,
    #[astra(key = "@GrowRatio.Phys")]
    pub grow_ratio_phys: i8,
    #[astra(key = "@GrowRatio.Move")]
    pub grow_ratio_move: i8,
    #[astra(key = "@Price")]
    pub price: i32,
    #[astra(key = "@WeaponLevel")]
    pub weapon_level: String,
    #[astra(key = "@RodType")]
    pub rod_type: i8,
    #[astra(key = "@RodExp")]
    pub rod_exp: u8,
    #[astra(key = "@RateArena")]
    pub rate_arena: u8,
    #[astra(key = "@ShootEffect")]
    pub shoot_effect: String,
    #[astra(key = "@HitEffect")]
    pub hit_effect: String,
    #[astra(key = "@CannonEffect")]
    pub cannon_effect: String,
    #[astra(key = "@AttackMotion")]
    pub attack_motion: i8,
    #[astra(key = "@OverlapTerrain")]
    pub overlap_terrain: String,
    #[astra(key = "@EquipCondition")]
    pub equip_condition: String,
    #[astra(key = "@Flag")]
    pub flag: i32,
    #[astra(key = "@EquipSids")]
    pub equip_sids: Vec<String>,
    #[astra(key = "@PassiveSids")]
    pub passive_sids: Vec<String>,
    #[astra(key = "@GiveSids")]
    pub give_sids: Vec<String>,
    #[astra(key = "@AddTarget")]
    pub add_target: i8,
    #[astra(key = "@AddRange")]
    pub add_range: u8,
    #[astra(key = "@AddType")]
    pub add_type: i8,
    #[astra(key = "@AddPower")]
    pub add_power: u8,
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
    pub iron: u16,
    #[astra(key = "@Steel")]
    pub steel: u16,
    #[astra(key = "@Silver")]
    pub silver: u16,
    #[astra(key = "@Price")]
    pub price: u16,
    #[astra(key = "@Power")]
    pub power: i8,
    #[astra(key = "@Weight")]
    pub weight: i8,
    #[astra(key = "@Hit")]
    pub hit: i8,
    #[astra(key = "@Critical")]
    pub critical: i8,
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
    pub iron: u16,
    #[astra(key = "@Steel")]
    pub steel: u16,
    #[astra(key = "@Silver")]
    pub silver: u16,
    #[astra(key = "@Price")]
    pub price: u16,
    #[astra(key = "@RefineLevel")]
    pub refine_level: u8,
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
    pub to_iron: u16,
    #[astra(key = "@ToSteel")]
    pub to_steel: u16,
    #[astra(key = "@ToSilver")]
    pub to_silver: u16,
    #[astra(key = "@ForIron")]
    pub for_iron: u16,
    #[astra(key = "@ForSteel")]
    pub for_steel: u16,
    #[astra(key = "@ForSilver")]
    pub for_silver: u16,
}

#[derive(Astra, Debug)]
pub struct WeaponRankData {
    #[astra(key = "@Level")]
    pub level: String,
    #[astra(key = "@Exp")]
    pub exp: u8,
    #[astra(key = "@Mastery")]
    pub mastery: u8,
    #[astra(key = "@Attack")]
    pub attack: u8,
    #[astra(key = "@Hit")]
    pub hit: u8,
    #[astra(key = "@Critical")]
    pub critical: u8,
    #[astra(key = "@Recover")]
    pub recover: u8,
}

#[derive(Astra, Debug)]
pub struct ItemInteractData {
    #[astra(key = "@Kind")]
    pub kind: String,
    #[astra(key = "@Flag")]
    pub flag: u32,
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
    pub first: bool,
    #[astra(key = "@Amiibo")]
    pub amiibo: bool,
    #[astra(key = "@Asset")]
    pub asset: String,
    #[astra(key = "@CondtionCid")]
    pub condtion_cid: String,
    #[astra(key = "@CondtionSkills")]
    pub condtion_skills: Vec<String>,
    #[astra(key = "@CondtionGender")]
    pub condtion_gender: i8,
    #[astra(key = "@Gid")]
    pub gid: String,
    #[astra(key = "@Price")]
    pub price: i32,
    #[astra(key = "@Iron")]
    pub iron: i32,
    #[astra(key = "@Steel")]
    pub steel: i32,
    #[astra(key = "@Silver")]
    pub silver: i32,
    #[astra(key = "@Mask")]
    pub mask: i32,
}

#[derive(Astra, Debug)]
pub struct GiftData {
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@V00")]
    pub v_00: i8,
    #[astra(key = "@V01")]
    pub v_01: i8,
    #[astra(key = "@V02")]
    pub v_02: i8,
    #[astra(key = "@V03")]
    pub v_03: i8,
    #[astra(key = "@V04")]
    pub v_04: i8,
    #[astra(key = "@V05")]
    pub v_05: i8,
    #[astra(key = "@V06")]
    pub v_06: i8,
    #[astra(key = "@V07")]
    pub v_07: i8,
    #[astra(key = "@V08")]
    pub v_08: i8,
    #[astra(key = "@V09")]
    pub v_09: i8,
    #[astra(key = "@V10")]
    pub v_10: i8,
    #[astra(key = "@V11")]
    pub v_11: i8,
    #[astra(key = "@V12")]
    pub v_12: i8,
    #[astra(key = "@V13")]
    pub v_13: i8,
    #[astra(key = "@V14")]
    pub v_14: i8,
    #[astra(key = "@V15")]
    pub v_15: i8,
    #[astra(key = "@V16")]
    pub v_16: i8,
    #[astra(key = "@V17")]
    pub v_17: i8,
    #[astra(key = "@V18")]
    pub v_18: i8,
    #[astra(key = "@V19")]
    pub v_19: i8,
    #[astra(key = "@V20")]
    pub v_20: i8,
    #[astra(key = "@V21")]
    pub v_21: i8,
    #[astra(key = "@V22")]
    pub v_22: i8,
    #[astra(key = "@V23")]
    pub v_23: i8,
    #[astra(key = "@V24")]
    pub v_24: i8,
    #[astra(key = "@V25")]
    pub v_25: i8,
    #[astra(key = "@V26")]
    pub v_26: i8,
    #[astra(key = "@V27")]
    pub v_27: i8,
    #[astra(key = "@V28")]
    pub v_28: i8,
    #[astra(key = "@V29")]
    pub v_29: i8,
    #[astra(key = "@V30")]
    pub v_30: i8,
    #[astra(key = "@V31")]
    pub v_31: i8,
    #[astra(key = "@V32")]
    pub v_32: i8,
    #[astra(key = "@V33")]
    pub v_33: i8,
    #[astra(key = "@V34")]
    pub v_34: i8,
    #[astra(key = "@V35")]
    pub v_35: i8,
    #[astra(key = "@V36")]
    pub v_36: i8,
    #[astra(key = "@V37")]
    pub v_37: i8,
    #[astra(key = "@V38")]
    pub v_38: i8,
    #[astra(key = "@V39")]
    pub v_39: i8,
    #[astra(key = "@V40")]
    pub v_40: i8,
    #[astra(key = "@V41")]
    pub v_41: i8,
    #[astra(key = "@V42")]
    pub v_42: i8,
    #[astra(key = "@V43")]
    pub v_43: i8,
    #[astra(key = "@V44")]
    pub v_44: i8,
    #[astra(key = "@V45")]
    pub v_45: i8,
    #[astra(key = "@V46")]
    pub v_46: i8,
    #[astra(key = "@V47")]
    pub v_47: i8,
    #[astra(key = "@V48")]
    pub v_48: i8,
    #[astra(key = "@V49")]
    pub v_49: i8,
}

#[derive(Astra, Debug)]
pub struct RewardData {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Iid")]
    pub iid: String,
    #[astra(key = "@Ratio")]
    pub ratio: f32,
    #[astra(key = "@Factor")]
    pub factor: f32,
    #[astra(key = "@Min")]
    pub min: f32,
    #[astra(key = "@Max")]
    pub max: f32,
    #[astra(key = "@IsShow")]
    pub is_show: bool,
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
    pub power_mat: u16,
    #[astra(key = "@HitMat")]
    pub hit_mat: u16,
    #[astra(key = "@CriticalMat")]
    pub critical_mat: u16,
    #[astra(key = "@AvoidMat")]
    pub avoid_mat: u16,
    #[astra(key = "@SecureMat")]
    pub secure_mat: u16,
    #[astra(key = "@TechMat")]
    pub tech_mat: u16,
    #[astra(key = "@QuickMat")]
    pub quick_mat: u16,
    #[astra(key = "@DefMat")]
    pub def_mat: u16,
    #[astra(key = "@MdefMat")]
    pub mdef_mat: u16,
    #[astra(key = "@EfficacyHorseMat")]
    pub efficacy_horse_mat: u16,
    #[astra(key = "@EfficacyArmorMat")]
    pub efficacy_armor_mat: u16,
    #[astra(key = "@EfficacyFlyMat")]
    pub efficacy_fly_mat: u16,
    #[astra(key = "@EfficacyDragonMat")]
    pub efficacy_dragon_mat: u16,
    #[astra(key = "@EfficacyMorphMat")]
    pub efficacy_morph_mat: u16,
    #[astra(key = "@PowerCapa")]
    pub power_capa: u16,
    #[astra(key = "@HitCapa")]
    pub hit_capa: u16,
    #[astra(key = "@CriticalCapa")]
    pub critical_capa: u16,
    #[astra(key = "@AvoidCapa")]
    pub avoid_capa: u16,
    #[astra(key = "@SecureCapa")]
    pub secure_capa: u16,
    #[astra(key = "@TechCapa")]
    pub tech_capa: u16,
    #[astra(key = "@QuickCapa")]
    pub quick_capa: u16,
    #[astra(key = "@DefCapa")]
    pub def_capa: u16,
    #[astra(key = "@MdefCapa")]
    pub mdef_capa: u16,
    #[astra(key = "@EfficacyHorseCapa")]
    pub efficacy_horse_capa: u16,
    #[astra(key = "@EfficacyArmorCapa")]
    pub efficacy_armor_capa: u16,
    #[astra(key = "@EfficacyFlyCapa")]
    pub efficacy_fly_capa: u16,
    #[astra(key = "@EfficacyDragonCapa")]
    pub efficacy_dragon_capa: u16,
    #[astra(key = "@EfficacyMorphCapa")]
    pub efficacy_morph_capa: u16,
    #[astra(key = "@Power")]
    pub power: i8,
    #[astra(key = "@Hit")]
    pub hit: i8,
    #[astra(key = "@Critical")]
    pub critical: i8,
    #[astra(key = "@Avoid")]
    pub avoid: i8,
    #[astra(key = "@Secure")]
    pub secure: i8,
    #[astra(key = "@Tech")]
    pub tech: i8,
    #[astra(key = "@Quick")]
    pub quick: i8,
    #[astra(key = "@Def")]
    pub def: i8,
    #[astra(key = "@Mdef")]
    pub mdef: i8,
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
