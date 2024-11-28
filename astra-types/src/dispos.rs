use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(Debug, AstraBook)]
pub struct DisposBook {
    pub spawns: Sheet<IndexMap<String, Vec<Spawn>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct Spawn {
    #[astra(key = "@Group", public_array)]
    pub group: String,
    #[astra(key = "@Pid")]
    pub pid: String,
    #[astra(key = "@Force")]
    pub force: i8,
    #[astra(key = "@Flag")]
    pub flag: u16,
    #[astra(key = "@AppearX")]
    pub appear_x: i8,
    #[astra(key = "@AppearY")]
    pub appear_y: i8,
    #[astra(key = "@DisposX")]
    pub dispos_x: i8,
    #[astra(key = "@DisposY")]
    pub dispos_y: i8,
    #[astra(key = "@Direction")]
    pub direction: i8,
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
    #[astra(key = "@Item1.Drop")]
    pub item_1_drop: i8,
    #[astra(key = "@Item2.Iid")]
    pub item_2_iid: String,
    #[astra(key = "@Item2.Drop")]
    pub item_2_drop: i8,
    #[astra(key = "@Item3.Iid")]
    pub item_3_iid: String,
    #[astra(key = "@Item3.Drop")]
    pub item_3_drop: i8,
    #[astra(key = "@Item4.Iid")]
    pub item_4_iid: String,
    #[astra(key = "@Item4.Drop")]
    pub item_4_drop: i8,
    #[astra(key = "@Item5.Iid")]
    pub item_5_iid: String,
    #[astra(key = "@Item5.Drop")]
    pub item_5_drop: i8,
    #[astra(key = "@Item6.Iid")]
    pub item_6_iid: String,
    #[astra(key = "@Item6.Drop")]
    pub item_6_drop: i8,
    #[astra(key = "@Sid")]
    pub sid: String,
    #[astra(key = "@Bid")]
    pub bid: String,
    #[astra(key = "@Gid")]
    pub gid: String,
    #[astra(key = "@HpStockCount")]
    pub hp_stock_count: u8,
    #[astra(key = "@State0")]
    pub state_0: i8,
    #[astra(key = "@State1")]
    pub state_1: i8,
    #[astra(key = "@State2")]
    pub state_2: i8,
    #[astra(key = "@State3")]
    pub state_3: i8,
    #[astra(key = "@State4")]
    pub state_4: i8,
    #[astra(key = "@State5")]
    pub state_5: i8,
    #[astra(key = "@AI_ActionName")]
    pub ai_action_name: String,
    #[astra(key = "@AI_ActionVal")]
    pub ai_action_val: String,
    #[astra(key = "@AI_MindName")]
    pub ai_mind_name: String,
    #[astra(key = "@AI_MindVal")]
    pub ai_mind_val: String,
    #[astra(key = "@AI_AttackName")]
    pub ai_attack_name: String,
    #[astra(key = "@AI_AttackVal")]
    pub ai_attack_val: String,
    #[astra(key = "@AI_MoveName")]
    pub ai_move_name: String,
    #[astra(key = "@AI_MoveVal")]
    pub ai_move_val: String,
    #[astra(key = "@AI_BattleRate")]
    pub ai_battle_rate: String,
    #[astra(key = "@AI_Priority")]
    pub ai_priority: u8,
    #[astra(key = "@AI_HealRateA")]
    pub ai_heal_rate_a: i8,
    #[astra(key = "@AI_HealRateB")]
    pub ai_heal_rate_b: i8,
    #[astra(key = "@AI_BandNo")]
    pub ai_band_no: u32,
    #[astra(key = "@AI_MoveLimit")]
    pub ai_move_limit: String,
    #[astra(key = "@AI_Flag")]
    pub ai_flag: u32,
}
