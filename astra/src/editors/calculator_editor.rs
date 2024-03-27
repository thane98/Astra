use std::borrow::Cow;

use astra_types::{CalculatorBook, CalculatorCommon, ExpTableEntry};
use egui::Ui;

use crate::{
    editable_list, editor_tab_strip, sheet_retriever, EditorState, ListEditorContent, PropertyGrid,
    ViewItem,
};

sheet_retriever!(
    Calculator,
    CalculatorBook,
    common_functions,
    Vec<CalculatorCommon>
);

impl ViewItem for CalculatorCommon {
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

sheet_retriever!(ExpTable, CalculatorBook, exp_table, Vec<ExpTableEntry>);

impl ViewItem for ExpTableEntry {
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    CommonFunctions,
    ExpTable,
}

pub struct CalculatorEditor {
    tab: Tab,
    common_functions: CalculatorSheet,
    exp_table: ExpTableSheet,
    common_functions_content: ListEditorContent<Vec<CalculatorCommon>, CalculatorCommon, ()>,
    exp_table_content: ListEditorContent<Vec<ExpTableEntry>, ExpTableEntry, ()>,
}

impl CalculatorEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::CommonFunctions,
            common_functions: state.calculator.clone(),
            exp_table: state.exp_table.clone(),
            common_functions_content: ListEditorContent::new("common_functions_editor"),
            exp_table_content: ListEditorContent::new("exp_table_editor"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::CommonFunctions, "Common Functions");
            ui.selectable_value(&mut self.tab, Tab::ExpTable, "Exp. Table");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        match self.tab {
            Tab::CommonFunctions => {
                self.common_functions_content
                    .left_panel(ctx, &self.common_functions, &());
                self.common_functions.write(|data| {
                    self.common_functions_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("calculator", selection)
                                .new_section("")
                                .default_field("Name", |d| &mut d.name)
                                .field("Condition", |ui, d| {
                                    ui.add(editable_list(&mut d.condition, |_, i, ui| {
                                        ui.text_edit_singleline(i)
                                    }))
                                })
                                .field("Function", |ui, d| {
                                    ui.add(editable_list(&mut d.function, |_, i, ui| {
                                        ui.text_edit_singleline(i)
                                    }))
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::ExpTable => {
                self.exp_table_content.left_panel(ctx, &self.exp_table, &());
                self.exp_table.write(|data| {
                    self.exp_table_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("exp_table", selection)
                            .new_section("")
                            .default_field("Name", |d| &mut d.name)
                            .default_field("M 39", |d| &mut d.m_39)
                            .default_field("M 38", |d| &mut d.m_38)
                            .default_field("M 37", |d| &mut d.m_37)
                            .default_field("M 36", |d| &mut d.m_36)
                            .default_field("M 35", |d| &mut d.m_35)
                            .default_field("M 34", |d| &mut d.m_34)
                            .default_field("M 33", |d| &mut d.m_33)
                            .default_field("M 32", |d| &mut d.m_32)
                            .default_field("M 31", |d| &mut d.m_31)
                            .default_field("M 30", |d| &mut d.m_30)
                            .default_field("M 29", |d| &mut d.m_29)
                            .default_field("M 28", |d| &mut d.m_28)
                            .default_field("M 27", |d| &mut d.m_27)
                            .default_field("M 26", |d| &mut d.m_26)
                            .default_field("M 25", |d| &mut d.m_25)
                            .default_field("M 24", |d| &mut d.m_24)
                            .default_field("M 23", |d| &mut d.m_23)
                            .default_field("M 22", |d| &mut d.m_22)
                            .default_field("M 21", |d| &mut d.m_21)
                            .default_field("M 20", |d| &mut d.m_20)
                            .default_field("M 19", |d| &mut d.m_19)
                            .default_field("M 18", |d| &mut d.m_18)
                            .default_field("M 17", |d| &mut d.m_17)
                            .default_field("M 16", |d| &mut d.m_16)
                            .default_field("M 15", |d| &mut d.m_15)
                            .default_field("M 14", |d| &mut d.m_14)
                            .default_field("M 13", |d| &mut d.m_13)
                            .default_field("M 12", |d| &mut d.m_12)
                            .default_field("M 11", |d| &mut d.m_11)
                            .default_field("M 10", |d| &mut d.m_10)
                            .default_field("M 09", |d| &mut d.m_09)
                            .default_field("M 08", |d| &mut d.m_08)
                            .default_field("M 07", |d| &mut d.m_07)
                            .default_field("M 06", |d| &mut d.m_06)
                            .default_field("M 05", |d| &mut d.m_05)
                            .default_field("M 04", |d| &mut d.m_04)
                            .default_field("M 03", |d| &mut d.m_03)
                            .default_field("M 02", |d| &mut d.m_02)
                            .default_field("M 01", |d| &mut d.m_01)
                            .default_field("N 00", |d| &mut d.n_00)
                            .default_field("P 01", |d| &mut d.p_01)
                            .default_field("P 02", |d| &mut d.p_02)
                            .default_field("P 03", |d| &mut d.p_03)
                            .default_field("P 04", |d| &mut d.p_04)
                            .default_field("P 05", |d| &mut d.p_05)
                            .default_field("P 06", |d| &mut d.p_06)
                            .default_field("P 07", |d| &mut d.p_07)
                            .default_field("P 08", |d| &mut d.p_08)
                            .default_field("P 09", |d| &mut d.p_09)
                            .default_field("P 10", |d| &mut d.p_10)
                            .default_field("P 11", |d| &mut d.p_11)
                            .default_field("P 12", |d| &mut d.p_12)
                            .default_field("P 13", |d| &mut d.p_13)
                            .default_field("P 14", |d| &mut d.p_14)
                            .default_field("P 15", |d| &mut d.p_15)
                            .default_field("P 16", |d| &mut d.p_16)
                            .default_field("P 17", |d| &mut d.p_17)
                            .default_field("P 18", |d| &mut d.p_18)
                            .default_field("P 19", |d| &mut d.p_19)
                            .default_field("P 20", |d| &mut d.p_20)
                            .default_field("P 21", |d| &mut d.p_21)
                            .default_field("P 22", |d| &mut d.p_22)
                            .default_field("P 23", |d| &mut d.p_23)
                            .default_field("P 24", |d| &mut d.p_24)
                            .default_field("P 25", |d| &mut d.p_25)
                            .default_field("P 26", |d| &mut d.p_26)
                            .default_field("P 27", |d| &mut d.p_27)
                            .default_field("P 28", |d| &mut d.p_28)
                            .default_field("P 29", |d| &mut d.p_29)
                            .default_field("P 30", |d| &mut d.p_30)
                            .default_field("P 31", |d| &mut d.p_31)
                            .default_field("P 32", |d| &mut d.p_32)
                            .default_field("P 33", |d| &mut d.p_33)
                            .default_field("P 34", |d| &mut d.p_34)
                            .default_field("P 35", |d| &mut d.p_35)
                            .default_field("P 36", |d| &mut d.p_36)
                            .default_field("P 37", |d| &mut d.p_37)
                            .default_field("P 38", |d| &mut d.p_38)
                            .default_field("P 39", |d| &mut d.p_39)
                            .default_field("P 40", |d| &mut d.p_40)
                            .show(ui)
                            .changed()
                    })
                });
            }
        }
    }
}
