use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct RingBook {
    pub ring_data: Sheet<Vec<RingData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct RingData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Rnid")]
    pub rnid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@Gid")]
    pub gid: String,
    #[astra(key = "@RingModel")]
    pub ring_model: String,
    #[astra(key = "@Rank")]
    pub rank: Option<i8>,
    #[astra(key = "@Icon")]
    pub icon: String,
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
    #[astra(key = "@EquipSids")]
    pub equip_sids: Vec<String>,
    #[astra(key = "@IsSingleRank")]
    pub is_single_rank: Option<bool>,
    #[astra(key = "@JewelColorR")]
    pub jewel_color_r: Option<u8>,
    #[astra(key = "@JewelColorG")]
    pub jewel_color_g: Option<u8>,
    #[astra(key = "@JewelColorB")]
    pub jewel_color_b: Option<u8>,
    #[astra(key = "@RimColorR")]
    pub rim_color_r: Option<u8>,
    #[astra(key = "@RimColorG")]
    pub rim_color_g: Option<u8>,
    #[astra(key = "@RimColorB")]
    pub rim_color_b: Option<u8>,
}

#[derive(AstraBook)]
pub struct RingCleaningVoiceBook {
    pub ring_data: Sheet<IndexMap<String, Vec<RingPolishVoiceData>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct RingPolishVoiceData {
    #[astra(key = "@Gid", public_array)]
    pub gid: String,
    #[astra(key = "@Label")]
    pub label: Option<i8>,
    #[astra(key = "@PlaySituation")]
    pub play_situation: Option<i8>,
    #[astra(key = "@IsPlayCompleted")]
    pub is_play_completed: Option<bool>,
    #[astra(key = "@UnitFaceAnim")]
    pub unit_face_anim: String,
    #[astra(key = "@GodFaceAnim")]
    pub god_face_anim: String,
}
