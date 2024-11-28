use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct CookBook {
    pub cook_data: Sheet<IndexMap<String, CookData>>,
    pub food_data: Sheet<IndexMap<String, FoodData>>,
    pub taste_data: Sheet<IndexMap<String, TasteData>>,
    pub taste_condition_data: Sheet<IndexMap<String, TasteConditionData>>,
    pub ingredient_data: Sheet<IndexMap<String, IngredientData>>,
    pub food_naming_configs: Sheet<IndexMap<String, FoodNamingConfig>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct CookData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Pid", id)]
    pub pid: String,
    #[astra(key = "@Taste1")]
    pub taste_1: String,
    #[astra(key = "@Taste2")]
    pub taste_2: String,
    #[astra(key = "@Taste3")]
    pub taste_3: String,
    #[astra(key = "@VeryGoodFood")]
    pub very_good_food: Vec<String>,
    #[astra(key = "@GoodFood")]
    pub good_food: Vec<String>,
    #[astra(key = "@HaveCookedFood")]
    pub have_cooked_food: Vec<String>,
    #[astra(key = "@ChallengingFood")]
    pub challenging_food: Vec<String>,
    #[astra(key = "@LikeFood")]
    pub like_food: Vec<String>,
    #[astra(key = "@DislikeFood")]
    pub dislike_food: Vec<String>,
    #[astra(key = "@BentoIid")]
    pub bento_iid: String,
    #[astra(key = "@MaskColor100R")]
    pub mask_color_100_r: u8,
    #[astra(key = "@MaskColor100G")]
    pub mask_color_100_g: u8,
    #[astra(key = "@MaskColor100B")]
    pub mask_color_100_b: u8,
    #[astra(key = "@MaskColor075R")]
    pub mask_color_075_r: u8,
    #[astra(key = "@MaskColor075G")]
    pub mask_color_075_g: u8,
    #[astra(key = "@MaskColor075B")]
    pub mask_color_075_b: u8,
    #[astra(key = "@SeEvent")]
    pub se_event: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct FoodData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Fid", id)]
    pub fid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Message")]
    pub message: String,
    #[astra(key = "@Enhance.Str")]
    pub enhance_str: i8,
    #[astra(key = "@Enhance.Quick")]
    pub enhance_quick: i8,
    #[astra(key = "@Enhance.Def")]
    pub enhance_def: i8,
    #[astra(key = "@Enhance.Magic")]
    pub enhance_magic: i8,
    #[astra(key = "@Enhance.Mdef")]
    pub enhance_mdef: i8,
    #[astra(key = "@Foodstuffs")]
    pub foodstuffs: Vec<String>,
    #[astra(key = "@Country")]
    pub country: i8,
    #[astra(key = "@PrefabName")]
    pub prefab_name: String,
    #[astra(key = "@SeEvent")]
    pub se_event: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TasteData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Tid", id)]
    pub tid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Grade")]
    pub grade: i8,
    #[astra(key = "@Augment")]
    pub augment: i8,
    #[astra(key = "@OtherEnhance")]
    pub other_enhance: i8,
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
    #[astra(key = "@Flag")]
    pub flag: i32,
    #[astra(key = "@Cid")]
    pub cid: String,
    #[astra(key = "@AlternativeTaste")]
    pub alternative_taste: String,
    #[astra(key = "@DerivingProbability")]
    pub deriving_probability: i8,
    #[astra(key = "@DerivedTid")]
    pub derived_tid: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TasteConditionData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Cid", id)]
    pub cid: String,
    #[astra(key = "@Name")]
    pub name: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct IngredientData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Iid", id)]
    pub iid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Flag")]
    pub flag: i32,
    #[astra(key = "@Category")]
    pub category: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct FoodNamingConfig {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Pid", id)]
    pub pid: String,
    #[astra(key = "@NameType0")]
    pub name_type_0: i8,
    #[astra(key = "@NameType1")]
    pub name_type_1: i8,
    #[astra(key = "@NameType2")]
    pub name_type_2: i8,
    #[astra(key = "@NameType3")]
    pub name_type_3: i8,
    #[astra(key = "@NameType4")]
    pub name_type_4: i8,
    #[astra(key = "@NameType5")]
    pub name_type_5: i8,
    #[astra(key = "@NameType6")]
    pub name_type_6: i8,
    #[astra(key = "@NameType7")]
    pub name_type_7: i8,
    #[astra(key = "@NameType8")]
    pub name_type_8: i8,
    #[astra(key = "@NameType9")]
    pub name_type_9: i8,
    #[astra(key = "@NameType10")]
    pub name_type_10: i8,
    #[astra(key = "@NameType11")]
    pub name_type_11: i8,
    #[astra(key = "@NameType12")]
    pub name_type_12: i8,
    #[astra(key = "@NameType13")]
    pub name_type_13: i8,
    #[astra(key = "@NameType14")]
    pub name_type_14: i8,
    #[astra(key = "@NameType15")]
    pub name_type_15: i8,
    #[astra(key = "@NameType16")]
    pub name_type_16: i8,
    #[astra(key = "@NameType17")]
    pub name_type_17: i8,
    #[astra(key = "@NameType18")]
    pub name_type_18: i8,
    #[astra(key = "@NameType19")]
    pub name_type_19: i8,
    #[astra(key = "@NameType20")]
    pub name_type_20: i8,
    #[astra(key = "@NameType21")]
    pub name_type_21: i8,
    #[astra(key = "@NameType22")]
    pub name_type_22: i8,
    #[astra(key = "@NameType23")]
    pub name_type_23: i8,
    #[astra(key = "@NameType24")]
    pub name_type_24: i8,
    #[astra(key = "@NameType25")]
    pub name_type_25: i8,
    #[astra(key = "@NameType26")]
    pub name_type_26: i8,
    #[astra(key = "@NameType27")]
    pub name_type_27: i8,
    #[astra(key = "@NameType28")]
    pub name_type_28: i8,
    #[astra(key = "@NameType29")]
    pub name_type_29: i8,
    #[astra(key = "@NameType30")]
    pub name_type_30: i8,
    #[astra(key = "@NameType31")]
    pub name_type_31: i8,
    #[astra(key = "@NameType32")]
    pub name_type_32: i8,
    #[astra(key = "@NameType33")]
    pub name_type_33: i8,
    #[astra(key = "@NameType34")]
    pub name_type_34: i8,
    #[astra(key = "@NameType35")]
    pub name_type_35: i8,
    #[astra(key = "@NameType36")]
    pub name_type_36: i8,
    #[astra(key = "@NameType37")]
    pub name_type_37: i8,
    #[astra(key = "@NameType38")]
    pub name_type_38: i8,
    #[astra(key = "@NameType39")]
    pub name_type_39: i8,
}
