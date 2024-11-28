use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

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
    pub alpha: f32,
    #[astra(key = "@Terrain")]
    pub terrain: String,
    #[astra(key = "@Dispos")]
    pub dispos: String,
    #[astra(key = "@NextChapter")]
    pub next_chapter: String,
    #[astra(key = "@GmapSpot")]
    pub gmap_spot: String,
    #[astra(key = "@GmapSpotState")]
    pub gmap_spot_state: i8,
    #[astra(key = "@GmapSpotOpenCondition")]
    pub gmap_spot_open_condition: String,
    #[astra(key = "@GmapSpotEncount")]
    pub gmap_spot_encount: i8,
    #[astra(key = "@EncountJobs")]
    pub encount_jobs: Vec<String>,
    #[astra(key = "@Reward")]
    pub reward: String,
    #[astra(key = "@Progress")]
    pub progress: u8,
    #[astra(key = "@HoldLevel")]
    pub hold_level: u8,
    #[astra(key = "@Flag")]
    pub flag: i32,
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
    pub recommended_level: u8,
    #[astra(key = "@Nation")]
    pub nation: String,
    #[astra(key = "@NetKillBonusIndex")]
    pub net_kill_bonus_index: u8,
    #[astra(key = "@NetRankingIndex")]
    pub net_ranking_index: u8,
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
    pub sortie_count: i32,
}
