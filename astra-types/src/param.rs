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
    pub value: f32,
    #[astra(key = "@Min")]
    pub min: f32,
    #[astra(key = "@Max")]
    pub max: f32,
    #[astra(key = "@Step")]
    pub step: f32,
    #[astra(key = "@Enum")]
    pub en: String,
}
