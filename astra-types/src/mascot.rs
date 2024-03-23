use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct MascotBook {
    pub accessory_data: Sheet<IndexMap<String, MascotAccessoryData>>,
    pub color_data: Sheet<Vec<MascotColorData>>,
    pub param_data: Sheet<IndexMap<String, MascotParamData>>,
    pub food_data: Sheet<IndexMap<String, MascotFoodData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MascotAccessoryData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Aid", id)]
    pub aid: String,
    #[astra(key = "@Type")]
    pub ty: Option<i8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MascotColorData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@R")]
    pub r: Option<u8>,
    #[astra(key = "@G")]
    pub g: Option<u8>,
    #[astra(key = "@B")]
    pub b: Option<u8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MascotParamData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@ParamName", id)]
    pub param_name: String,
    #[astra(key = "@Value")]
    pub value: Option<i8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MascotFoodData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Iid", id)]
    pub iid: String,
    #[astra(key = "@Value")]
    pub value: Option<i8>,
}
