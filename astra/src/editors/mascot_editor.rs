use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, model_drop_down, rgb_color_picker, sheet_retriever, EditorState,
    KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{
    Accessory, IngredientData, MascotAccessoryData, MascotBook, MascotColorData, MascotFoodData, MascotParamData
};

sheet_retriever!(MascotAccessoryData, MascotBook, accessory_data, IndexMap<String, MascotAccessoryData>);

impl ViewItem for MascotAccessoryData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .accessory
            .read(|data| {
                data.get(&self.aid)
                    .map(|accessory| accessory.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.aid))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Accessory::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.accessory.read(|data| {
            data.get(&self.aid)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

impl KeyedViewItem for MascotAccessoryData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.aid)
    }

    fn set_key(&mut self, key: String) {
        self.aid = key;
    }
}

sheet_retriever!(
    MascotColorData,
    MascotBook,
    color_data,
    Vec<MascotColorData>
);

// TODO: Can we show the color as a decoration?
impl ViewItem for MascotColorData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!(
            "#{:X}{:X}{:X}",
            self.r.unwrap_or_default(),
            self.g.unwrap_or_default(),
            self.b.unwrap_or_default()
        ))
    }
}

sheet_retriever!(MascotParamData, MascotBook, param_data, IndexMap<String, MascotParamData>);

impl ViewItem for MascotParamData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.param_name)
    }
}

impl KeyedViewItem for MascotParamData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.param_name)
    }

    fn set_key(&mut self, key: String) {
        self.param_name = key;
    }
}

sheet_retriever!(MascotFoodData, MascotBook, food_data, IndexMap<String, MascotFoodData>);

impl ViewItem for MascotFoodData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .item
            .read(|data| {
                data.get(&self.iid)
                    .map(|item| item.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.iid))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        IngredientData::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.ingredient.read(|data| {
            data.get(&self.iid)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

impl KeyedViewItem for MascotFoodData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.iid)
    }

    fn set_key(&mut self, key: String) {
        self.iid = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Accessory,
    Color,
    Param,
    Food,
}

pub struct MascotEditor {
    tab: Tab,
    accessory_data: MascotAccessoryDataSheet,
    color_data: MascotColorDataSheet,
    param_data: MascotParamDataSheet,
    food_data: MascotFoodDataSheet,
    accessory_data_content:
        ListEditorContent<IndexMap<String, MascotAccessoryData>, MascotAccessoryData>,
    color_data_content: ListEditorContent<Vec<MascotColorData>, MascotColorData>,
    param_data_content: ListEditorContent<IndexMap<String, MascotParamData>, MascotParamData>,
    food_data_content: ListEditorContent<IndexMap<String, MascotFoodData>, MascotFoodData>,
}

impl MascotEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Accessory,
            accessory_data: state.mascot_accessory_data.clone(),
            color_data: state.mascot_color_data.clone(),
            param_data: state.mascot_param_data.clone(),
            food_data: state.mascot_food_data.clone(),
            accessory_data_content: ListEditorContent::new("accessory_data_editor"),
            color_data_content: ListEditorContent::new("color_data_editor"),
            param_data_content: ListEditorContent::new("param_data_editor"),
            food_data_content: ListEditorContent::new("food_data_editor"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Accessory, "Accessory");
            ui.selectable_value(&mut self.tab, Tab::Color, "Color");
            ui.selectable_value(&mut self.tab, Tab::Param, "Param");
            ui.selectable_value(&mut self.tab, Tab::Food, "Food");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::Accessory => {
                self.accessory_data_content
                    .left_panel(ctx, &self.accessory_data, state);
                self.accessory_data.write(|data| {
                    self.accessory_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("accessory_data", selection)
                                .new_section("")
                                .field("Accessory", |ui, d| {
                                    state.accessory.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.aid))
                                    })
                                })
                                .default_field("Ty", |d| &mut d.ty)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Color => {
                self.color_data_content
                    .left_panel(ctx, &self.color_data, state);
                self.color_data.write(|data| {
                    self.color_data_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("color_data", selection)
                            .new_section("")
                            .field("Color", |ui, d| {
                                ui.add(rgb_color_picker(&mut d.r, &mut d.g, &mut d.b))
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::Param => {
                self.param_data_content
                    .left_panel(ctx, &self.param_data, state);
                self.param_data.write(|data| {
                    self.param_data_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("param_data", selection)
                            .new_section("")
                            .default_field("Param Name", |d| &mut d.param_name)
                            .default_field("Value", |d| &mut d.value)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::Food => {
                self.food_data_content
                    .left_panel(ctx, &self.food_data, state);
                self.food_data.write(|data| {
                    self.food_data_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("food_data", selection)
                            .new_section("")
                            .field("Item", |ui, d| {
                                state
                                    .item
                                    .read(|data| ui.add(model_drop_down(data, state, &mut d.iid)))
                            })
                            .default_field("Value", |d| &mut d.value)
                            .show(ui)
                            .changed()
                    })
                });
            }
        }
    }
}
