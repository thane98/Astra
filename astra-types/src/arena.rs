use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct ArenaBook {
    pub arena_data: Sheet<IndexMap<String, Vec<ArenaData>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct ArenaData {
    #[astra(key = "@Name", public_array)]
    pub name: String,
    #[astra(key = "@Rate")]
    pub rate: i32,
    #[astra(key = "@Pid")]
    pub pid: String,
    #[astra(key = "@Iid")]
    pub iid: String,
}
