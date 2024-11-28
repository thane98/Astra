use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct RingBook {
    pub ring_data: Sheet<IndexMap<String, RingData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct RingData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Rnid", id)]
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
    pub rank: i8,
    #[astra(key = "@Icon")]
    pub icon: String,
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
    #[astra(key = "@EquipSids")]
    pub equip_sids: Vec<String>,
    #[astra(key = "@IsSingleRank")]
    pub is_single_rank: bool,
    #[astra(key = "@JewelColorR")]
    pub jewel_color_r: u8,
    #[astra(key = "@JewelColorG")]
    pub jewel_color_g: u8,
    #[astra(key = "@JewelColorB")]
    pub jewel_color_b: u8,
    #[astra(key = "@RimColorR")]
    pub rim_color_r: u8,
    #[astra(key = "@RimColorG")]
    pub rim_color_g: u8,
    #[astra(key = "@RimColorB")]
    pub rim_color_b: u8,
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
    pub label: i8,
    #[astra(key = "@PlaySituation")]
    pub play_situation: i8,
    #[astra(key = "@IsPlayCompleted")]
    pub is_play_completed: bool,
    #[astra(key = "@UnitFaceAnim")]
    pub unit_face_anim: String,
    #[astra(key = "@GodFaceAnim")]
    pub god_face_anim: String,
}
