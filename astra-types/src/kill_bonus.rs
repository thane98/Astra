use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct KillBonusBook {
    pub kill_bonuses_1: Sheet<IndexMap<String, Vec<KillBonus1>>>,
    pub kill_bonuses_2: Sheet<IndexMap<String, Vec<KillBonus2>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct KillBonus1 {
    #[astra(key = "@Name", public_array)]
    pub name: String,
    #[astra(key = "@Iid")]
    pub iid: String,
    #[astra(key = "@Rate")]
    pub rate: Option<u8>,
    #[astra(key = "@Cid")]
    pub cid: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct KillBonus2 {
    #[astra(key = "@Name", public_array)]
    pub name: String,
    #[astra(key = "@Kind")]
    pub kind: Option<i8>,
    #[astra(key = "@Value")]
    pub value: Option<i32>,
    #[astra(key = "@Rate")]
    pub rate: Option<u8>,
    #[astra(key = "@Flag")]
    pub flag: Option<i32>,
    #[astra(key = "@Cid")]
    pub cid: String,
}