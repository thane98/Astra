use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct AmiiboBook {
    pub ai_data: Sheet<IndexMap<String, AmiiboData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AmiiboData {
    #[astra(key = "@NumberingID", id)]
    pub numbering_id: String,
    #[astra(key = "@IIDs")]
    pub items: Vec<String>,
    #[astra(key = "@AID")]
    pub aid: String,
    #[astra(key = "@BGM")]
    pub bgm: String,
    #[astra(key = "@TicketNum")]
    pub ticket_num: Option<i32>,
    #[astra(key = "@KizunaNum")]
    pub kizuna_num: Option<i32>,
}