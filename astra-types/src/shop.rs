use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct ShopBook {
    pub armory_shop_inventory: Sheet<IndexMap<String, Vec<ShopInventory>>>,
    pub item_shop_inventory: Sheet<IndexMap<String, Vec<ShopInventory>>>,
    pub flea_market_shop_inventory: Sheet<IndexMap<String, Vec<ShopInventory>>>,
    pub accessory_shop_inventory: Sheet<IndexMap<String, Vec<AccessoryShopInventory>>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct ShopInventory {
    #[astra(key = "@Condition", public_array)]
    pub condition: String,
    #[astra(key = "@Iid")]
    pub iid: String,
    #[astra(key = "@Stock")]
    pub stock: Option<i16>,
    #[astra(key = "@Attribute")]
    pub attribute: Option<i8>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct AccessoryShopInventory {
    #[astra(key = "@Condition", public_array)]
    pub condition: String,
    #[astra(key = "@Aid")]
    pub aid: String,
}
