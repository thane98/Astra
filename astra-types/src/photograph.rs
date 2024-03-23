use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct PhotographSpotBook {
    pub spots: Sheet<IndexMap<String, PhotographSpot>>,
    pub poses: Sheet<IndexMap<String, Vec<PhotographPose>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct PhotographSpot {
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@MID", id)]
    pub mid: String,
    #[astra(key = "@ConditionCid")]
    pub condition_cid: String,
    #[astra(key = "@LocatorCount")]
    pub locator_count: Option<i32>,
    #[astra(key = "@PauseGroupNameList1")]
    pub pause_group_name_list_1: Vec<String>,
    #[astra(key = "@PauseGroupNameList2")]
    pub pause_group_name_list_2: Vec<String>,
    #[astra(key = "@PauseGroupNameList3")]
    pub pause_group_name_list_3: Vec<String>,
    #[astra(key = "@PauseGroupNameList4")]
    pub pause_group_name_list_4: Vec<String>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct PhotographPose {
    #[astra(key = "@GroupName", public_array)]
    pub group_name: String,
    #[astra(key = "@PauseName")]
    pub pause_name: String,
    #[astra(key = "@Mid")]
    pub mid: String,
    #[astra(key = "@No")]
    pub no: Option<i32>,
    #[astra(key = "@AnimeFrame")]
    pub anime_frame: Option<i32>,
    #[astra(key = "@FaceAnime")]
    pub face_anime: String,
    #[astra(key = "@CharaIdList")]
    pub chara_id_list: Vec<String>,
}
