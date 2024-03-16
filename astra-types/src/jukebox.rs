use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct JukeboxBook {
    pub jukebox_data: Sheet<IndexMap<String, JukeboxData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct JukeboxData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@EventName", id)]
    pub event_name: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Condition")]
    pub condition: String,
}