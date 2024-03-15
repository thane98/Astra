use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct AmiiboBook {
    pub ai_data: Sheet<Vec<AmiiboData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AmiiboData {
    #[astra(key = "@NumberingID")]
    pub numbering_id: String,
    #[astra(key = "@IIDs")]
    pub ii_ds: Vec<String>,
    #[astra(key = "@AID")]
    pub aid: String,
    #[astra(key = "@BGM")]
    pub bgm: String,
    #[astra(key = "@TicketNum")]
    pub ticket_num: Option<i32>,
    #[astra(key = "@KizunaNum")]
    pub kizuna_num: Option<i32>,
}