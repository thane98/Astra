use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct MusicBook {
    pub music: Sheet<IndexMap<String, MusicData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MusicData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@EventName", id)]
    pub event_name: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Help")]
    pub help: String,
    #[astra(key = "@Condition")]
    pub condition: String,
    #[astra(key = "@Amiibo")]
    pub amiibo: String,
    #[astra(key = "@ChangeEventName")]
    pub change_event_name: String,
    #[astra(key = "@IsChange")]
    pub is_change: Option<bool>,
    #[astra(key = "@Gid")]
    pub gid: String,
}
