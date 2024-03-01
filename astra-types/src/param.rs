use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct ParamsBook {
    pub game_params: Sheet<Vec<GameParam>>,
}

#[derive(Debug, Default, Astra, Clone)]
pub struct GameParam {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@English")]
    pub english: String,
    #[astra(key = "@Value")]
    pub value: Option<f32>,
    #[astra(key = "@Min")]
    pub min: Option<f32>,
    #[astra(key = "@Max")]
    pub max: Option<f32>,
    #[astra(key = "@Step")]
    pub step: Option<f32>,
    #[astra(key = "@Enum")]
    pub en: String,
}
