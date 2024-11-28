use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct TutorialBook {
    pub tutorials: Sheet<IndexMap<String, Vec<TutorialData>>>,
    pub tips: Sheet<IndexMap<String, TipData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TutorialData {
    #[astra(key = "@TUTID", public_array)]
    pub tutid: String,
    #[astra(key = "@MID")]
    pub mid: String,
    #[astra(key = "@Title")]
    pub title: String,
    #[astra(key = "@SpriteAtlas")]
    pub sprite_atlas: String,
    #[astra(key = "@Type")]
    pub ty: i8,
    #[astra(key = "@Notice")]
    pub notice: i8,
    #[astra(key = "@Cid")]
    pub cid: String,
    #[astra(key = "@No")]
    pub no: i8,
    #[astra(key = "@SSType")]
    pub ss_type: i8,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TipData {
    #[astra(key = "@ID", id)]
    pub id: String,
    #[astra(key = "@Title")]
    pub title: String,
    #[astra(key = "@Tips")]
    pub tips: String,
    #[astra(key = "@OwnID")]
    pub own_id: String,
    #[astra(key = "@IconInfoID")]
    pub icon_info_id: String,
    #[astra(key = "@Chapter")]
    pub chapter: String,
    #[astra(key = "@Variable")]
    pub variable: String,
    #[astra(key = "@Allow")]
    pub allow: i32,
}
