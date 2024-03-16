use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct ProfileCardBook {
    pub bg: Sheet<IndexMap<String, ProfileCardImageComponent>>,
    pub frames: Sheet<IndexMap<String, ProfileCardImageComponent>>,
    pub lettering: Sheet<IndexMap<String, ProfileCardImageComponent>>,
    pub text_colors: Sheet<IndexMap<String, ProfileCardColorComponent>>,
    pub stamp_data_1: Sheet<IndexMap<String, ProfileCardImageComponent>>,
    pub stamp_data_2: Sheet<IndexMap<String, ProfileCardCategorizedImageComponent>>,
    pub title: Sheet<IndexMap<String, ProfileCardNameComponent>>,
    pub favorite_character: Sheet<IndexMap<String, ProfileCardNameComponent>>,
    pub favorite_map: Sheet<IndexMap<String, ProfileCardFavoriteMapData>>,
    pub comment: Sheet<IndexMap<String, ProfileCardCategorizedComponent>>,
    pub favorite_map_editor_theme: Sheet<IndexMap<String, ProfileCardCategorizedComponent>>,
    pub default_comment: Sheet<IndexMap<String, ProfileCardDefaultCommentData>>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct ProfileCardImageComponent {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Id", id)]
    pub id: String,
    #[astra(key = "@Image")]
    pub image: String,
    #[astra(key = "@Condition")]
    pub condition: Option<i8>,
    #[astra(key = "@Arg")]
    pub arg: String,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct ProfileCardColorComponent {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Id", id)]
    pub id: String,
    #[astra(key = "@Color")]
    pub color: String,
    #[astra(key = "@Condition")]
    pub condition: Option<i8>,
    #[astra(key = "@Arg")]
    pub arg: String,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct ProfileCardNameComponent {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Id", id)]
    pub id: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Condition")]
    pub condition: Option<i8>,
    #[astra(key = "@Arg")]
    pub arg: String,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct ProfileCardCategorizedComponent {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Id", id)]
    pub id: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Category")]
    pub category: Option<i8>,
    #[astra(key = "@Condition")]
    pub condition: Option<i8>,
    #[astra(key = "@Arg")]
    pub arg: String,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct ProfileCardCategorizedImageComponent {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Id", id)]
    pub id: String,
    #[astra(key = "@Image")]
    pub image: String,
    #[astra(key = "@Category")]
    pub category: Option<i8>,
    #[astra(key = "@Condition")]
    pub condition: Option<i8>,
    #[astra(key = "@Arg")]
    pub arg: String,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct ProfileCardFavoriteMapData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Id", id)]
    pub id: String,
    #[astra(key = "@Cid")]
    pub cid: String,
    #[astra(key = "@Condition")]
    pub condition: Option<i8>,
    #[astra(key = "@Arg")]
    pub arg: String,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct ProfileCardDefaultCommentData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Language", id)]
    pub language: String,
    #[astra(key = "@Id1")]
    pub id_1: String,
    #[astra(key = "@Id2")]
    pub id_2: String,
    #[astra(key = "@Id3")]
    pub id_3: String,
}