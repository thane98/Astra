use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct LaterTalkBook {
    pub post_battle_conversations: Sheet<IndexMap<String, Vec<PostBattleConversation>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct PostBattleConversation {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Person")]
    pub person: String,
    #[astra(key = "@Field")]
    pub field: String,
    #[astra(key = "@BackDegree")]
    pub back_degree: Option<i32>,
    #[astra(key = "@LightDegree")]
    pub light_degree: Option<i32>,
}