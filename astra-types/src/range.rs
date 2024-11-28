use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct RangeBook {
    pub ranges: Sheet<IndexMap<String, Vec<RangeData>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct RangeData {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Value1")]
    pub value_1: i8,
    #[astra(key = "@Value2")]
    pub value_2: i8,
    #[astra(key = "@Value3")]
    pub value_3: i8,
    #[astra(key = "@Value4")]
    pub value_4: i8,
    #[astra(key = "@Value5")]
    pub value_5: i8,
    #[astra(key = "@Value6")]
    pub value_6: i8,
    #[astra(key = "@Value7")]
    pub value_7: i8,
    #[astra(key = "@Value8")]
    pub value_8: i8,
}
