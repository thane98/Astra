use astra_types::{ForgeExchangeData, Item, ItemBook};
use egui::Ui;

use crate::{
    editor_tab_strip, gold_field, iron_field_i8, model_drop_down, silver_field, steel_field,
    system_icon_field, CachedView, EditorState, ForgeEvolveDataSheet, ForgeExchangeDataSheet,
    ForgeImproveDataSheet, GroupEditorContent, ItemSheetRetriever, ListEditorContent, PropertyGrid,
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
            Tab::Refine => self.refine_content.left_panel(ctx, &self.refine, state),
            Tab::Evolve => self.evolve_content.left_panel(ctx, &self.evolve, state),
            Tab::Exchange => self.exchange_content.left_panel(ctx, &self.exchange, &()),
        }

        match self.tab {
            Tab::Refine => self.refine.write(|data| {
                self.refine_content.content(ctx, data, |ui, selection| {
                    PropertyGrid::new("forge_refine", selection)
                        .new_section("Cost")
                        .field("Gold", |ui, d| gold_field(ui, state, &mut d.price))
                        .field("Iron", |ui, d| iron_field_i8(ui, state, &mut d.iron))
                        .field("Steel", |ui, d| steel_field(ui, state, &mut d.steel))
                        .field("Silver", |ui, d| silver_field(ui, state, &mut d.silver))
                        .new_section("Bonuses")
                        .default_field("Mt", |d| &mut d.power)
                        .default_field("Wt", |d| &mut d.weight)
                        .default_field("Hit", |d| &mut d.hit)
                        .default_field("Crit", |d| &mut d.critical)
                        .show(ui)
                        .changed()
                })
            }),
            Tab::Evolve => self.evolve.write(|data| {
                self.evolve_content.content(ctx, data, |ui, data| {
                    PropertyGrid::new("forge_evolve", data)
                        .new_section("Data")
                        .field("Item", |ui, d| {
                            ui.add(model_drop_down(self.cache.get(), &(), &mut d.iid))
                        })
                        .default_field("Level", |d| &mut d.refine_level)
                        .field("Gold", |ui, d| gold_field(ui, state, &mut d.price))
                        .field("Iron", |ui, d| iron_field_i8(ui, state, &mut d.iron))
                        .field("Steel", |ui, d| steel_field(ui, state, &mut d.steel))
                        .field("Silver", |ui, d| silver_field(ui, state, &mut d.silver))
                        .show(ui)
                        .changed()
                })
            }),
            Tab::Exchange => self.exchange.write(|data| {
                self.exchange_content.content(ctx, data, |ui, selection| {
                    PropertyGrid::new("forge_refine", selection)
                        .new_section("Data")
                        .default_field("Name", |d| &mut d.name)
                        .default_field("Operation", |d| &mut d.operation)
                        .field("Icon", |ui, d| {
                            let icon = d.icon.clone();
                            system_icon_field(ui, state, &mut d.icon, &icon)
                        })
                        .field("To Iron", |ui, d| iron_field_i8(ui, state, &mut d.to_iron))
                        .field("To Steel", |ui, d| steel_field(ui, state, &mut d.to_steel))
                        .field("To Silver", |ui, d| {
                            silver_field(ui, state, &mut d.to_silver)
                        })
                        .field("For Iron", |ui, d| {
                            iron_field_i8(ui, state, &mut d.for_iron)
                        })
                        .field("For Steel", |ui, d| {
                            steel_field(ui, state, &mut d.for_steel)
                        })
                        .field("For Silver", |ui, d| {
                            silver_field(ui, state, &mut d.for_silver)
                        })
                        .show(ui)
                        .changed()
                })
            }),
        }
    }
}
