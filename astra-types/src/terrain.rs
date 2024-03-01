use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct TerrainBook {
    pub terrain_data: Sheet<IndexMap<String, TerrainData>>,
    pub terrain_cost_data: Sheet<Vec<TerrainCostData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TerrainData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Tid", id)]
    pub tid: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@CostName")]
    pub cost_name: String,
    #[astra(key = "@Layer")]
    pub layer: Option<i8>,
    #[astra(key = "@Prohibition")]
    pub prohibition: Option<i8>,
    #[astra(key = "@Sight")]
    pub sight: Option<u8>,
    #[astra(key = "@Destroyer")]
    pub destroyer: Option<i8>,
    #[astra(key = "@Hp_N")]
    pub hp_n: Option<u8>,
    #[astra(key = "@Hp_H")]
    pub hp_h: Option<u8>,
    #[astra(key = "@Hp_L")]
    pub hp_l: Option<u8>,
    #[astra(key = "@Defense")]
    pub defense: Option<i8>,
    #[astra(key = "@Avoid")]
    pub avoid: Option<i8>,
    #[astra(key = "@PlayerDefense")]
    pub player_defense: Option<i8>,
    #[astra(key = "@EnemyDefense")]
    pub enemy_defense: Option<i8>,
    #[astra(key = "@PlayerAvoid")]
    pub player_avoid: Option<i8>,
    #[astra(key = "@EnemyAvoid")]
    pub enemy_avoid: Option<i8>,
    #[astra(key = "@Heal")]
    pub heal: Option<i8>,
    #[astra(key = "@Life")]
    pub life: Option<u8>,
    #[astra(key = "@MoveCost")]
    pub move_cost: Option<u8>,
    #[astra(key = "@FlyCost")]
    pub fly_cost: Option<u8>,
    #[astra(key = "@MoveFirst")]
    pub move_first: Option<i8>,
    #[astra(key = "@Offset")]
    pub offset: Option<f32>,
    #[astra(key = "@PutEffect")]
    pub put_effect: String,
    #[astra(key = "@Minimap")]
    pub minimap: String,
    #[astra(key = "@CannonSkill")]
    pub cannon_skill: String,
    #[astra(key = "@CannonShellsN")]
    pub cannon_shells_n: Option<u8>,
    #[astra(key = "@CannonShellsH")]
    pub cannon_shells_h: Option<u8>,
    #[astra(key = "@CannonShellsL")]
    pub cannon_shells_l: Option<u8>,
    #[astra(key = "@ChangeTid")]
    pub change_tid: String,
    #[astra(key = "@ChangeEncount")]
    pub change_encount: String,
    #[astra(key = "@Command")]
    pub command: Option<i8>,
    #[astra(key = "@Flag")]
    pub flag: Option<i32>,
    #[astra(key = "@PutAllow")]
    pub put_allow: Option<u8>,
    #[astra(key = "@Height")]
    pub height: Option<f32>,
    #[astra(key = "@ColorR")]
    pub color_r: Option<u8>,
    #[astra(key = "@ColorG")]
    pub color_g: Option<u8>,
    #[astra(key = "@ColorB")]
    pub color_b: Option<u8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct TerrainCostData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@Name")]
    pub name: String,
    #[astra(key = "@None")]
    pub none: Option<u8>,
    #[astra(key = "@Foot")]
    pub foot: Option<u8>,
    #[astra(key = "@Horse")]
    pub horse: Option<u8>,
    #[astra(key = "@Fly")]
    pub fly: Option<u8>,
    #[astra(key = "@Dragon")]
    pub dragon: Option<u8>,
    #[astra(key = "@Pad")]
    pub pad: Option<u8>,
    #[astra(key = "@ColorR")]
    pub color_r: Option<u8>,
    #[astra(key = "@ColorG")]
    pub color_g: Option<u8>,
    #[astra(key = "@ColorB")]
    pub color_b: Option<u8>,
    #[astra(key = "@ColorA")]
    pub color_a: Option<u8>,
}
