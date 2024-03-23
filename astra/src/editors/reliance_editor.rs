use astra_types::{RelianceBonusData, RelianceExpData};
use egui::Ui;
use indexmap::IndexMap;

use crate::widgets::{id_field, keyed_add_modal_content};
use crate::{
    editor_tab_strip, i8_drag, u8_drag, EditorState, GroupEditorContent, ListEditorContent,
    PropertyGrid, RelianceBonusDataSheet, RelianceExpDataSheet,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    ExpData,
    Bonuses,
}

pub struct RelianceEditor {
    tab: Tab,
    exp_data: RelianceExpDataSheet,
    bonuses: RelianceBonusDataSheet,
    exp_data_content: ListEditorContent<IndexMap<String, RelianceExpData>, RelianceExpData>,
    bonuses_content: GroupEditorContent,
}

impl RelianceEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::ExpData,
            exp_data: state.reliance_exp_data.clone(),
            bonuses: state.reliance_bonus_data.clone(),
            exp_data_content: ListEditorContent::new("reliance_exp_data")
                .with_add_modal_content(keyed_add_modal_content),
            bonuses_content: GroupEditorContent::new("reliance_bonuses"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::ExpData, "Exp Data");
            ui.selectable_value(&mut self.tab, Tab::Bonuses, "Bonuses");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        match self.tab {
            Tab::ExpData => self.exp_data_content.left_panel(ctx, &self.exp_data, &()),
            Tab::Bonuses => self.bonuses_content.left_panel(ctx, &self.bonuses, &()),
        }

        match self.tab {
            Tab::ExpData => self.exp_data.write(|data| {
                self.exp_data_content
                    .content(ctx, data, Self::exp_data_property_grid)
            }),
            Tab::Bonuses => self.bonuses.write(|data| {
                self.bonuses_content
                    .content(ctx, data, Self::bonuses_property_grid)
            }),
        }
    }

    fn exp_data_property_grid(ui: &mut Ui, data: &mut RelianceExpData) -> bool {
        PropertyGrid::new("reliance_exp_data", data)
            .new_section("Data")
            .field("REXID", |ui, data| ui.add(id_field(&mut data.rexid)))
            .field("C", |ui, data| ui.add(u8_drag(&mut data.exp_c)))
            .field("B", |ui, data| ui.add(u8_drag(&mut data.exp_b)))
            .field("A", |ui, data| ui.add(u8_drag(&mut data.exp_a)))
            .show(ui)
            .changed()
    }

    fn bonuses_property_grid(ui: &mut Ui, data: &mut RelianceBonusData) -> bool {
        PropertyGrid::new("reliance_bonuses_data", data)
            .new_section("Data")
            .field("Level", |ui, data| ui.add(i8_drag(&mut data.level)))
            .field("Hit", |ui, data| ui.add(i8_drag(&mut data.hit)))
            .field("Crit", |ui, data| ui.add(i8_drag(&mut data.critical)))
            .field("Avoid", |ui, data| ui.add(i8_drag(&mut data.avoid)))
            .field("Dodge", |ui, data| ui.add(i8_drag(&mut data.secure)))
            .show(ui)
            .changed()
    }
}
