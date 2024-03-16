use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct MapEditorBook {
    pub objects: Sheet<IndexMap<String, MapEditorObject>>,
    pub categories: Sheet<IndexMap<String, MapEditorCategory>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MapEditorObject {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@ObjectName", id)]
    pub object_name: String,
    #[astra(key = "@SoundEvent")]
    pub sound_event: String,
    #[astra(key = "@Category")]
    pub category: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MapEditorCategory {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Cid", id)]
    pub cid: String,
    #[astra(key = "@CountMax")]
    pub count_max: Option<i32>,
    #[astra(key = "@IconName")]
    pub icon_name: String,
}