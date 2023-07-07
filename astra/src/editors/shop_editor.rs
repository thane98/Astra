use astra_types::{AccessoryShopInventory, ShopInventory};
use egui::Ui;

use crate::{
    editor_tab_strip, model_drop_down, AccessoryShopSheet, ArmoryShopSheet, EditorState,
    FleaMarketSheet, GroupEditorContent, ItemShopSheet, PropertyGrid,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Armory,
    ItemShop,
    FleaMarket,
    AccessoryShop,
}

pub struct ShopEditor {
    tab: Tab,
    armory_shop: ArmoryShopSheet,
    item_shop: ItemShopSheet,
    flea_market: FleaMarketSheet,
    accessory_shop: AccessoryShopSheet,
    armory_content: GroupEditorContent,
    item_shop_content: GroupEditorContent,
    flea_market_content: GroupEditorContent,
    accessory_shop_content: GroupEditorContent,
}

impl ShopEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Armory,
            armory_shop: state.armory_shop.clone(),
            item_shop: state.item_shop.clone(),
            flea_market: state.flea_market.clone(),
            accessory_shop: state.accessory_shop.clone(),
            armory_content: GroupEditorContent::new("armory"),
            item_shop_content: GroupEditorContent::new("item_shop"),
            flea_market_content: GroupEditorContent::new("flea_market"),
            accessory_shop_content: GroupEditorContent::new("accessory_shop"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Armory, "Armory");
            ui.selectable_value(&mut self.tab, Tab::ItemShop, "Items");
            ui.selectable_value(&mut self.tab, Tab::FleaMarket, "Flea Market");
            ui.selectable_value(&mut self.tab, Tab::AccessoryShop, "Accessory");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        match self.tab {
            Tab::Armory => self
                .armory_content
                .left_panel(ctx, &self.armory_shop, state),
            Tab::ItemShop => self
                .item_shop_content
                .left_panel(ctx, &self.item_shop, state),
            Tab::FleaMarket => self
                .flea_market_content
                .left_panel(ctx, &self.flea_market, state),
            Tab::AccessoryShop => {
                self.accessory_shop_content
                    .left_panel(ctx, &self.accessory_shop, state)
            }
        }

        match self.tab {
            Tab::Armory => self.armory_shop.write(|data| {
                self.armory_content.content(ctx, data, |ui, data| {
                    Self::shop_inventory_property_grid("armory", ui, data, state)
                })
            }),
            Tab::ItemShop => self.item_shop.write(|data| {
                self.item_shop_content.content(ctx, data, |ui, data| {
                    Self::shop_inventory_property_grid("item_shop", ui, data, state)
                })
            }),
            Tab::FleaMarket => self.flea_market.write(|data| {
                self.flea_market_content.content(ctx, data, |ui, data| {
                    Self::shop_inventory_property_grid("flea_market", ui, data, state)
                })
            }),
            Tab::AccessoryShop => self.accessory_shop.write(|data| {
                self.accessory_shop_content.content(ctx, data, |ui, data| {
                    Self::accessory_shop_inventory_property_grid(ui, data, state)
                })
            }),
        }
    }

    fn shop_inventory_property_grid(
        id_source: &'static str,
        ui: &mut Ui,
        data: &mut ShopInventory,
        state: &EditorState,
    ) -> bool {
        PropertyGrid::new(id_source, data)
            .new_section("Data")
            .field("Item", |ui, d| {
                state
                    .item
                    .read(|data| ui.add(model_drop_down(data, state, &mut d.iid)))
            })
            .default_field("Condition", |d| &mut d.condition)
            .default_field("Stock", |d| &mut d.stock)
            .default_field("Attribute", |d| &mut d.attribute)
            .show(ui)
            .changed()
    }

    fn accessory_shop_inventory_property_grid(
        ui: &mut Ui,
        data: &mut AccessoryShopInventory,
        state: &EditorState,
    ) -> bool {
        PropertyGrid::new("accessory_shop", data)
            .new_section("Data")
            .field("Item", |ui, d| {
                state
                    .accessory
                    .read(|data| ui.add(model_drop_down(data, state, &mut d.aid)))
            })
            .default_field("Condition", |d| &mut d.condition)
            .show(ui)
            .changed()
    }
}
