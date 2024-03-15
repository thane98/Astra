use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct ProfileCardBook {
    pub bg: Sheet<Vec<ProfileCardImageComponent>>,
    pub frames: Sheet<Vec<ProfileCardImageComponent>>,
    pub lettering: Sheet<Vec<ProfileCardImageComponent>>,
    pub text_colors: Sheet<Vec<ProfileCardColorComponent>>,
    pub stamp_data_1: Sheet<Vec<ProfileCardImageComponent>>,
    pub stamp_data_2: Sheet<Vec<ProfileCardCategorizedImageComponent>>,
    pub title: Sheet<Vec<ProfileCardNameComponent>>,
    pub favorite_character: Sheet<Vec<ProfileCardNameComponent>>,
    pub favorite_map: Sheet<Vec<ProfileCardFavoriteMapData>>,
    pub comment: Sheet<Vec<ProfileCardCategorizedComponent>>,
    pub favorite_map_editor_theme: Sheet<Vec<ProfileCardCategorizedComponent>>,
    pub default_comment: Sheet<Vec<ProfileCardDefaultCommentData>>,
}


#[derive(Debug, Default, Clone, Astra)]
pub struct ProfileCardImageComponent {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Id")]
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
    #[astra(key = "@Id")]
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
    #[astra(key = "@Id")]
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
    #[astra(key = "@Id")]
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
    #[astra(key = "@Id")]
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
    #[astra(key = "@Id")]
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
    #[astra(key = "@Language")]
    pub language: String,
    #[astra(key = "@Id1")]
    pub id_1: String,
    #[astra(key = "@Id2")]
    pub id_2: String,
    #[astra(key = "@Id3")]
    pub id_3: String,
}