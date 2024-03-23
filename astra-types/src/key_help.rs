use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct KeyHelpDataBook {
    pub key_help_data: Sheet<IndexMap<String, Vec<KeyHelpData>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct KeyHelpData {
    #[astra(key = "@KHID", public_array)]
    pub khid: String,
    #[astra(key = "@ButtonIndex")]
    pub button_index: Option<i8>,
    #[astra(key = "@MID")]
    pub mid: String,
}
