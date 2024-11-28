use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct RelayBook {
    pub relay_data: Sheet<IndexMap<String, RelayData>>,
    pub relay_stamp_data: Sheet<IndexMap<String, RelayStampData>>,
    pub relay_clear_award_data: Sheet<IndexMap<String, Vec<RelayClearAwardData>>>,
    pub relay_award_data: Sheet<IndexMap<String, RelayAwardData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct RelayData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Cid", id)]
    pub cid: String,
    #[astra(key = "@Difficulty")]
    pub difficulty: i8,
    #[astra(key = "@MaxTurn")]
    pub max_turn: i32,
    #[astra(key = "@MaxUnit")]
    pub max_unit: i32,
    #[astra(key = "@NewTurn")]
    pub new_turn: i32,
    #[astra(key = "@TakeOverTurn")]
    pub take_over_turn: i32,
    #[astra(key = "@TakeOverUnit")]
    pub take_over_unit: i32,
    #[astra(key = "@CompletionAwardMain")]
    pub completion_award_main: String,
    #[astra(key = "@CompletionAwardSub")]
    pub completion_award_sub: String,
    #[astra(key = "@GameOverAward")]
    pub game_over_award: String,
    #[astra(key = "@UnlockCid")]
    pub unlock_cid: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct RelayStampData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Name", id)]
    pub name: String,
    #[astra(key = "@SerialNo")]
    pub serial_no: u8,
    #[astra(key = "@Pid")]
    pub pid: String,
    #[astra(key = "@Gid")]
    pub gid: String,
    #[astra(key = "@Sort")]
    pub sort: i32,
    #[astra(key = "@Flag")]
    pub flag: i32,
    #[astra(key = "@Voice")]
    pub voice: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct RelayClearAwardData {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Iid")]
    pub iid: String,
    #[astra(key = "@Rate")]
    pub rate: f32,
    #[astra(key = "@MinCount")]
    pub min_count: i32,
    #[astra(key = "@MaxCount")]
    pub max_count: i32,
    #[astra(key = "@Flag")]
    pub flag: i32,
    #[astra(key = "@Condition")]
    pub condition: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct RelayAwardData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Raid", id)]
    pub raid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@ResultText")]
    pub result_text: String,
    #[astra(key = "@Awards")]
    pub awards: Vec<String>,
    #[astra(key = "@Flag")]
    pub flag: i32,
}
