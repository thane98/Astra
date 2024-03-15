use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct AiBook {
    pub ai_data: Sheet<IndexMap<String, Vec<AiData>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AiData {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Active")]
    pub active: Option<i8>,
    #[astra(key = "@Code")]
    pub code: Option<i8>,
    #[astra(key = "@Mind")]
    pub mind: Option<i8>,
    #[astra(key = "@StrValue0")]
    pub str_value_0: String,
    #[astra(key = "@StrValue1")]
    pub str_value_1: String,
    #[astra(key = "@Trans")]
    pub trans: Option<i8>,
}