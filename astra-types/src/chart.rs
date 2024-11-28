use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct ChartBook {
    pub chart_data: Sheet<IndexMap<String, Vec<ChartData>>>,
    pub chart_god_data: Sheet<IndexMap<String, ChartGodData>>,
    pub chart_params: Sheet<IndexMap<String, Vec<ChartParam>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct ChartData {
    #[astra(key = "@Chapter", public_array)]
    pub chapter: String,
    #[astra(key = "@Pid")]
    pub pid: String,
    #[astra(key = "@LevelN")]
    pub level_n: u8,
    #[astra(key = "@LevelH")]
    pub level_h: u8,
    #[astra(key = "@LevelL")]
    pub level_l: u8,
    #[astra(key = "@Jid")]
    pub jid: String,
    #[astra(key = "@Item1.Iid")]
    pub item_1_iid: String,
    #[astra(key = "@Item2.Iid")]
    pub item_2_iid: String,
    #[astra(key = "@Item3.Iid")]
    pub item_3_iid: String,
    #[astra(key = "@Item4.Iid")]
    pub item_4_iid: String,
    #[astra(key = "@Item5.Iid")]
    pub item_5_iid: String,
    #[astra(key = "@GodId")]
    pub god_id: String,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct ChartGodData {
    #[astra(key = "@Chapter", id)]
    pub chapter: String,
    #[astra(key = "@MarthLevel")]
    pub marth_level: i32,
    #[astra(key = "@SigludLevel")]
    pub siglud_level: i32,
    #[astra(key = "@CelicaLevel")]
    pub celica_level: i32,
    #[astra(key = "@MicaiahLevel")]
    pub micaiah_level: i32,
    #[astra(key = "@RoyLevel")]
    pub roy_level: i32,
    #[astra(key = "@LeafLevel")]
    pub leaf_level: i32,
    #[astra(key = "@LucinaLevel")]
    pub lucina_level: i32,
    #[astra(key = "@LinLevel")]
    pub lin_level: i32,
    #[astra(key = "@IkeLevel")]
    pub ike_level: i32,
    #[astra(key = "@BylethLevel")]
    pub byleth_level: i32,
    #[astra(key = "@KamuiLevel")]
    pub kamui_level: i32,
    #[astra(key = "@EirikLevel")]
    pub eirik_level: i32,
    #[astra(key = "@Flag")]
    pub flag: i32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct ChartParam {
    #[astra(key = "@Chapter", public_array)]
    pub chapter: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@Value")]
    pub value: i32,
}
