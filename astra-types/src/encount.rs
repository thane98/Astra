use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct EncountBook {
    pub encount_equipment: Sheet<IndexMap<String, Vec<EncountEquipment>>>,
    pub encount_weapon_categories: Sheet<IndexMap<String, Vec<EncountWeaponCategory>>>,
    pub encount_enemy_types: Sheet<IndexMap<String, EncountEnemyType>>,
    pub encount_rarity_configs: Sheet<IndexMap<String, Vec<EncountRarityConfig>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct EncountEquipment {
    #[astra(key = "@Name", public_array)]
    pub name: String,
    #[astra(key = "@Category")]
    pub category: String,
    #[astra(key = "@Percentage")]
    pub percentage: u8,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct EncountWeaponCategory {
    #[astra(key = "@r8", public_array)]
    pub r_8: String,
    #[astra(key = "@Iid")]
    pub iid: String,
    #[astra(key = "@RankConditionMore")]
    pub rank_condition_more: u8,
    #[astra(key = "@RankConditionLess")]
    pub rank_condition_less: u8,
    #[astra(key = "@Percentage")]
    pub percentage: u8,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct EncountEnemyType {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@EJid", id)]
    pub e_jid: String,
    #[astra(key = "@Jobs")]
    pub jobs: Vec<String>,
    #[astra(key = "@Flag")]
    pub flag: i32,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct EncountRarityConfig {
    #[astra(key = "@Name", public_array)]
    pub name: String,
    #[astra(key = "@NationLevel")]
    pub nation_level: u8,
    #[astra(key = "@Iid")]
    pub iid: String,
}
