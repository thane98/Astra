use astra_types::{ForgeEvolveData, ForgeExchangeData, ForgeImproveData, Item, ItemBook};
use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, model_drop_down, CacheItem, CachedView, EditorState, ForgeEvolveDataSheet,
    ForgeExchangeDataSheet, ForgeImproveDataSheet, GroupEditorContent, ItemSheetRetriever,
    ListEditorContent, PropertyGrid,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Refine,
    Evolve,
    Exchange,
}

pub struct ForgeEditor {
    tab: Tab,
    refine: ForgeImproveDataSheet,
    evolve: ForgeEvolveDataSheet,
    exchange: ForgeExchangeDataSheet,
    refine_content: GroupEditorContent,
    evolve_content: GroupEditorContent,
    exchange_content: ListEditorContent<Vec<ForgeExchangeData>, ForgeExchangeData>,

    // Must use a cached view of items since they are contained in the same book.
    cache: CachedView<ItemSheetRetriever, ItemBook, Item>,
}

impl ForgeEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Refine,
            refine: state.forge_improve.clone(),
            evolve: state.forge_evolve.clone(),
            exchange: state.forge_exchange.clone(),
            refine_content: GroupEditorContent::new("forge_improve"),
            evolve_content: GroupEditorContent::new("forge_evolve"),
            exchange_content: ListEditorContent::new("forge_exchange"),
            cache: CachedView::new(state.item.clone(), state),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Refine, "Refine");
            ui.selectable_value(&mut self.tab, Tab::Evolve, "Evolve");
            ui.selectable_value(&mut self.tab, Tab::Exchange, "Exchange");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        self.cache.refresh(state);

        match self.tab {
            Tab::Refine => self.refine_content.left_panel(ctx, &self.refine, &()),
            Tab::Evolve => self.evolve_content.left_panel(ctx, &self.evolve, state),
            Tab::Exchange => self.exchange_content.side_panel(ctx, &self.exchange, &()),
        }

        match self.tab {
            Tab::Refine => self.refine.write(|data| {
                self.refine_content
                    .content(ctx, data, |ui, data| Self::refine_property_grid(ui, data))
            }),
            Tab::Evolve => self.evolve.write(|data| {
                self.evolve_content.content(ctx, data, |ui, data| {
                    Self::evolve_property_grid(self.cache.get(), ui, data)
                })
            }),
            Tab::Exchange => self.exchange.write(|data| {
                self.exchange_content
                    .content(ctx, data, |ui, data| Self::exchange_property_grid(ui, data))
            }),
        }
    }

    fn refine_property_grid(ui: &mut Ui, data: &mut ForgeImproveData) -> bool {
        PropertyGrid::new("forge_refine", data)
            .new_section("Cost")
            .default_field("Money", |d| &mut d.price)
            .default_field("Iron", |d| &mut d.iron)
            .default_field("Steel", |d| &mut d.steel)
            .default_field("Silver", |d| &mut d.silver)
            .new_section("Bonuses")
            .default_field("Mt", |d| &mut d.power)
            .default_field("Wt", |d| &mut d.weight)
            .default_field("Hit", |d| &mut d.hit)
            .default_field("Crit", |d| &mut d.critical)
            .show(ui)
            .changed()
    }

    fn evolve_property_grid(
        cache: &IndexMap<String, CacheItem<Item>>,
        ui: &mut Ui,
        data: &mut ForgeEvolveData,
    ) -> bool {
        PropertyGrid::new("forge_evolve", data)
            .new_section("Data")
            .field("Item", |ui, d| {
                ui.add(model_drop_down(cache, &(), &mut d.iid))
            })
            .default_field("Level", |d| &mut d.refine_level)
            .default_field("Money", |d| &mut d.price)
            .default_field("Iron", |d| &mut d.iron)
            .default_field("Steel", |d| &mut d.steel)
            .default_field("Silver", |d| &mut d.silver)
            .show(ui)
            .changed()
    }

    fn exchange_property_grid(ui: &mut Ui, data: &mut ForgeExchangeData) -> bool {
        PropertyGrid::new("forge_refine", data)
            .new_section("Data")
            .default_field("Name", |d| &mut d.name)
            .default_field("Operation", |d| &mut d.operation)
            .default_field("Icon", |d| &mut d.icon)
            .default_field("To Iron", |d| &mut d.to_iron)
            .default_field("To Steel", |d| &mut d.to_steel)
            .default_field("To Silver", |d| &mut d.to_silver)
            .default_field("For Iron", |d| &mut d.for_iron)
            .default_field("For Steel", |d| &mut d.for_steel)
            .default_field("For Silver", |d| &mut d.for_silver)
            .show(ui)
            .changed()
    }
}
