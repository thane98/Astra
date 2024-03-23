use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct MapHistoryBook {
    pub history: Sheet<IndexMap<String, MapHistory>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MapHistory {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Mhid", id)]
    pub mhid: String,
    #[astra(key = "@Action")]
    pub action: String,
    #[astra(key = "@Priority")]
    pub priority: Option<u8>,
}
