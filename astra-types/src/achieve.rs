use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct AchievementBook {
    pub achievements: Sheet<IndexMap<String, AchieveData>>,
    pub belong: Sheet<IndexMap<String, BelongData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AchieveData {
    #[astra(key = "@Aid", id)]
    pub aid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Category")]
    pub category: Option<i8>,
    #[astra(key = "@Kind")]
    pub kind: Option<i16>,
    #[astra(key = "@Count")]
    pub count: Option<i32>,
    #[astra(key = "@Arg")]
    pub arg: String,
    #[astra(key = "@CountUnit")]
    pub count_unit: String,
    #[astra(key = "@KizunaReward")]
    pub kizuna_reward: Option<i32>,
    #[astra(key = "@Chapter")]
    pub chapter: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct BelongData {
    #[astra(key = "@Bid", id)]
    pub bid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@DefeatAchieve")]
    pub defeat_achieve: Option<i8>,
}