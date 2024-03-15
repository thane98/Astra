use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct MascotBook {
    pub accessory_data: Sheet<Vec<MascotAccessoryData>>,
    pub color_data: Sheet<Vec<MascotColorData>>,
    pub param_data: Sheet<Vec<MascotParamData>>,
    pub food_data: Sheet<Vec<MascotFoodData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MascotAccessoryData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Aid")]
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
    #[astra(key = "@ParamName")]
    pub param_name: String,
    #[astra(key = "@Value")]
    pub value: Option<i8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct MascotFoodData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Iid")]
    pub iid: String,
    #[astra(key = "@Value")]
    pub value: Option<i8>,
}