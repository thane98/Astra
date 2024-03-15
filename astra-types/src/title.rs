use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct TitleBook {
    pub call_data: Sheet<Vec<TitleCallData>>,
    pub pedestal_data: Sheet<Vec<TitlePedestalData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TitleCallData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@PidOrGid")]
    pub pid_or_gid: String,
    #[astra(key = "@Cid")]
    pub cid: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TitlePedestalData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@PedestalName")]
    pub pedestal_name: String,
    #[astra(key = "@Cid")]
    pub cid: String,
}