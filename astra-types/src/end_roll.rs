use astra_derive::{Astra, AstraBook};
use astra_formats::Sheet;


#[derive(AstraBook)]
pub struct EndRollBook {
    pub end_roll_data: Sheet<Vec<EndRollData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct EndRollData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Type")]
    pub ty: Option<i8>,
    #[astra(key = "@Text1")]
    pub text_1: String,
    #[astra(key = "@Text2")]
    pub text_2: String,
    #[astra(key = "@Text3")]
    pub text_3: String,
}