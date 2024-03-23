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
