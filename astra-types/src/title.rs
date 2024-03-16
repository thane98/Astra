use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct TitleBook {
    pub call_data: Sheet<IndexMap<String, TitleCallData>>,
    pub pedestal_data: Sheet<IndexMap<String, TitlePedestalData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TitleCallData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@PidOrGid", id)]
    pub pid_or_gid: String,
    #[astra(key = "@Cid")]
    pub cid: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TitlePedestalData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@PedestalName", id)]
    pub pedestal_name: String,
    #[astra(key = "@Cid")]
    pub cid: String,
}