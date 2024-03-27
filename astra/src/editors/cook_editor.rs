use std::borrow::Cow;

use astra_types::{
    CookBook, CookData, FoodData, FoodNamingConfig, IngredientData, Person, TasteConditionData,
    TasteData,
};
use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editable_list, editor_tab_strip, id_field, keyed_add_modal_content, model_drop_down,
    msbt_key_value_multiline, msbt_key_value_singleline, nation_drop_down, rgb_color_picker,
    sheet_retriever, standard_keyed_display, CachedView, DropDownModal, EditorState, KeyedViewItem,
    ListEditorContent, PropertyGrid, ViewItem,
};

sheet_retriever!(Cook, CookBook, cook_data, IndexMap<String, CookData>);

impl ViewItem for CookData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies.person.read(|data| {
            data.get(&self.pid)
                .map(|p| Cow::Owned(p.text(dependencies).to_string()))
                .unwrap_or(Cow::Borrowed("{unknown person}"))
        })
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Person::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.person.read(|data| {
            data.get(&self.pid)
                .and_then(|p| p.decoration(dependencies, kind))
        })
    }
}

impl KeyedViewItem for CookData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.pid)
    }

    fn set_key(&mut self, key: String) {
        self.pid = key;
    }
}

sheet_retriever!(Food, CookBook, food_data, IndexMap<String, FoodData>);

impl ViewItem for FoodData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, fid, name)
    }
}

impl KeyedViewItem for FoodData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.fid)
    }

    fn set_key(&mut self, key: String) {
        self.fid = key;
    }
}

sheet_retriever!(Taste, CookBook, taste_data, IndexMap<String, TasteData>);

impl ViewItem for TasteData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, tid, name)
    }
}

impl KeyedViewItem for TasteData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.tid)
    }

    fn set_key(&mut self, key: String) {
        self.tid = key;
    }
}

sheet_retriever!(TasteCondition, CookBook, taste_condition_data, IndexMap<String, TasteConditionData>);

impl ViewItem for TasteConditionData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, cid, name)
    }
}

impl KeyedViewItem for TasteConditionData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.cid)
    }

    fn set_key(&mut self, key: String) {
        self.cid = key;
    }
}

sheet_retriever!(Ingredient, CookBook, ingredient_data, IndexMap<String, IngredientData>);

impl ViewItem for IngredientData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, iid, name)
    }

    fn decorated(_: crate::DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        _: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        let decoration_id = self.name.trim_start_matches("MIID_");
        let mut cache = dependencies.texture_cache.borrow_mut();
        if let Some(texture) = cache.get_system(decoration_id) {
            return Some((texture, 0.5));
        }
        let decoration_id = if decoration_id == "Nuts" {
            "Nut"
        } else {
            match self.category.as_str() {
                "IID_肉" => "Meat",
                "IID_野菜" => "Onion",
                "IID_果物" => "Apple",
                "IID_魚" => "Fish",
                _ => "Rare",
            }
        };
        cache
            .get_system(decoration_id)
            .map(|texture| (texture, 0.5))
    }
}

impl KeyedViewItem for IngredientData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.iid)
    }

    fn set_key(&mut self, key: String) {
        self.iid = key;
    }
}

sheet_retriever!(FoodNaming, CookBook, food_naming_configs, IndexMap<String, FoodNamingConfig>);

impl ViewItem for FoodNamingConfig {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies.person.read(|data| {
            data.get(&self.pid)
                .map(|p| Cow::Owned(p.text(dependencies).to_string()))
                .unwrap_or(Cow::Borrowed("{unknown person}"))
        })
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Person::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.person.read(|data| {
            data.get(&self.pid)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

impl KeyedViewItem for FoodNamingConfig {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.pid)
    }

    fn set_key(&mut self, key: String) {
        self.pid = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Main,
    Food,
    Taste,
    TasteCondition,
    Ingredient,
    FoodNaming,
}

pub struct CookEditor {
    tab: Tab,
    cook: CookSheet,
    food: FoodSheet,
    taste: TasteSheet,
    taste_condition: TasteConditionSheet,
    ingredient: IngredientSheet,
    food_naming: FoodNamingSheet,
    food_cache: CachedView<FoodSheetRetriever, CookBook, FoodData>,
    taste_cache: CachedView<TasteSheetRetriever, CookBook, TasteData>,
    taste_condition_cache: CachedView<TasteConditionSheetRetriever, CookBook, TasteConditionData>,
    ingredient_cache: CachedView<IngredientSheetRetriever, CookBook, IngredientData>,
    cook_content: ListEditorContent<IndexMap<String, CookData>, CookData, EditorState>,
    food_content: ListEditorContent<IndexMap<String, FoodData>, FoodData, EditorState>,
    taste_content: ListEditorContent<IndexMap<String, TasteData>, TasteData, EditorState>,
    taste_condition_content:
        ListEditorContent<IndexMap<String, TasteConditionData>, TasteConditionData, EditorState>,
    ingredient_content:
        ListEditorContent<IndexMap<String, IngredientData>, IngredientData, EditorState>,
    food_naming_content:
        ListEditorContent<IndexMap<String, FoodNamingConfig>, FoodNamingConfig, EditorState>,
}

impl CookEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Main,
            cook: state.cook.clone(),
            food: state.food.clone(),
            taste: state.taste.clone(),
            taste_condition: state.taste_condition.clone(),
            ingredient: state.ingredient.clone(),
            food_naming: state.food_naming.clone(),
            food_cache: CachedView::new(state.food.clone(), state),
            taste_cache: CachedView::new(state.taste.clone(), state),
            taste_condition_cache: CachedView::new(state.taste_condition.clone(), state),
            ingredient_cache: CachedView::new(state.ingredient.clone(), state),
            cook_content: ListEditorContent::new("cook_editor")
                .with_add_modal_content(DropDownModal::new(state.person.clone())),
            food_content: ListEditorContent::new("food_editor")
                .with_add_modal_content(keyed_add_modal_content),
            taste_content: ListEditorContent::new("taste_editor")
                .with_add_modal_content(keyed_add_modal_content),
            taste_condition_content: ListEditorContent::new("taste_condition_editor")
                .with_add_modal_content(keyed_add_modal_content),
            ingredient_content: ListEditorContent::new("ingredient_data")
                .with_add_modal_content(keyed_add_modal_content),
            food_naming_content: ListEditorContent::new("food_naming_editor")
                .with_add_modal_content(DropDownModal::new(state.person.clone())),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Main, "Main");
            ui.selectable_value(&mut self.tab, Tab::Food, "Food");
            ui.selectable_value(&mut self.tab, Tab::Taste, "Taste");
            ui.selectable_value(&mut self.tab, Tab::TasteCondition, "Taste Condition");
            ui.selectable_value(&mut self.tab, Tab::Ingredient, "Ingredient");
            ui.selectable_value(&mut self.tab, Tab::FoodNaming, "Food Naming");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.food_cache.refresh(state);
        self.taste_cache.refresh(state);
        self.taste_condition_cache.refresh(state);
        self.ingredient_cache.refresh(state);

        match self.tab {
            Tab::Main => {
                self.cook_content.left_panel(ctx, &self.cook, state);
                self.cook.write(|data| {
                    self.cook_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("cook", selection)
                            .new_section("")
                            .field("PID", |ui, d| ui.add(id_field(&mut d.pid)))
                            .field("Taste 1", |ui, d| {
                                ui.add(model_drop_down(self.taste_cache.get(), &(), &mut d.taste_1))
                            })
                            .field("Taste 2", |ui, d| {
                                ui.add(model_drop_down(self.taste_cache.get(), &(), &mut d.taste_2))
                            })
                            .field("Taste 3", |ui, d| {
                                ui.add(model_drop_down(self.taste_cache.get(), &(), &mut d.taste_3))
                            })
                            .field("Very Good Food", |ui, d| {
                                ui.add(editable_list(&mut d.very_good_food, |_, d, ui| {
                                    ui.add(model_drop_down(self.food_cache.get(), &(), d))
                                }))
                            })
                            .field("Good Food", |ui, d| {
                                ui.add(editable_list(&mut d.good_food, |_, d, ui| {
                                    ui.add(model_drop_down(self.food_cache.get(), &(), d))
                                }))
                            })
                            .field("Have Cooked Food", |ui, d| {
                                ui.add(editable_list(&mut d.have_cooked_food, |_, d, ui| {
                                    ui.add(model_drop_down(self.food_cache.get(), &(), d))
                                }))
                            })
                            .field("Challenging Food", |ui, d| {
                                ui.add(editable_list(&mut d.challenging_food, |_, d, ui| {
                                    ui.add(model_drop_down(self.food_cache.get(), &(), d))
                                }))
                            })
                            .field("Like Food", |ui, d| {
                                ui.add(editable_list(&mut d.like_food, |_, d, ui| {
                                    ui.add(model_drop_down(self.food_cache.get(), &(), d))
                                }))
                            })
                            .field("Dislike Food", |ui, d| {
                                ui.add(editable_list(&mut d.dislike_food, |_, d, ui| {
                                    ui.add(model_drop_down(self.food_cache.get(), &(), d))
                                }))
                            })
                            .field("Bento", |ui, d| {
                                state.item.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.bento_iid))
                                })
                            })
                            .field("Mask Color 100 (RGB)", |ui, d| {
                                ui.add(rgb_color_picker(
                                    &mut d.mask_color_100_r,
                                    &mut d.mask_color_100_g,
                                    &mut d.mask_color_100_b,
                                ))
                            })
                            .field("Mask Color 075 (RGB)", |ui, d| {
                                ui.add(rgb_color_picker(
                                    &mut d.mask_color_075_r,
                                    &mut d.mask_color_075_g,
                                    &mut d.mask_color_075_b,
                                ))
                            })
                            .default_field("Se Event", |d| &mut d.se_event)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::Food => {
                self.food_content.left_panel(ctx, &self.food, state);
                self.food.write(|data| {
                    self.food_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("food", selection)
                            .new_section("")
                            .field("FID", |ui, d| ui.add(id_field(&mut d.fid)))
                            .field("Name", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "cook", d.name)
                            })
                            .field("Message", |ui, d| {
                                msbt_key_value_multiline!(ui, state, "cook", d.message)
                            })
                            .default_field("Enhance Str", |d| &mut d.enhance_str)
                            .default_field("Enhance Quick", |d| &mut d.enhance_quick)
                            .default_field("Enhance Def", |d| &mut d.enhance_def)
                            .default_field("Enhance Magic", |d| &mut d.enhance_magic)
                            .default_field("Enhance Mdef", |d| &mut d.enhance_mdef)
                            .field("Foodstuffs", |ui, d| {
                                ui.add(editable_list(&mut d.foodstuffs, |_, d, ui| {
                                    ui.add(model_drop_down(self.ingredient_cache.get(), &(), d))
                                }))
                            })
                            .field("Country", |ui, d| ui.add(nation_drop_down(&mut d.country)))
                            .default_field("Prefab Name", |d| &mut d.prefab_name)
                            .default_field("Se Event", |d| &mut d.se_event)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::Taste => {
                self.taste_content.left_panel(ctx, &self.taste, state);
                self.taste.write(|data| {
                    self.taste_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("taste", selection)
                            .new_section("")
                            .field("TID", |ui, d| ui.add(id_field(&mut d.tid)))
                            .field("Name", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "cook", d.name)
                            })
                            .default_field("Grade", |d| &mut d.grade)
                            .default_field("Augment", |d| &mut d.augment)
                            .default_field("Other Enhance", |d| &mut d.other_enhance)
                            .default_field("Enhance Str", |d| &mut d.enhance_str)
                            .default_field("Enhance Tech", |d| &mut d.enhance_tech)
                            .default_field("Enhance Quick", |d| &mut d.enhance_quick)
                            .default_field("Enhance Luck", |d| &mut d.enhance_luck)
                            .default_field("Enhance Def", |d| &mut d.enhance_def)
                            .default_field("Enhance Magic", |d| &mut d.enhance_magic)
                            .default_field("Enhance Mdef", |d| &mut d.enhance_mdef)
                            .default_field("Flag", |d| &mut d.flag)
                            .field("Condition", |ui, d| {
                                ui.add(model_drop_down(
                                    self.taste_condition_cache.get(),
                                    &(),
                                    &mut d.cid,
                                ))
                            })
                            .field("Alternative Taste", |ui, d| {
                                ui.add(model_drop_down(
                                    self.taste_cache.get(),
                                    &(),
                                    &mut d.alternative_taste,
                                ))
                            })
                            .default_field("Deriving Probability", |d| &mut d.deriving_probability)
                            .field("Derived TID", |ui, d| {
                                ui.add(model_drop_down(
                                    self.taste_cache.get(),
                                    &(),
                                    &mut d.derived_tid,
                                ))
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::TasteCondition => {
                self.taste_condition_content
                    .left_panel(ctx, &self.taste_condition, state);
                self.taste_condition.write(|data| {
                    self.taste_condition_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("taste_condition", selection)
                                .new_section("")
                                .field("CID", |ui, d| ui.add(id_field(&mut d.cid)))
                                .field("Name", |ui, d| {
                                    msbt_key_value_singleline!(ui, state, "cook", d.name)
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Ingredient => {
                self.ingredient_content
                    .left_panel(ctx, &self.ingredient, state);
                self.ingredient.write(|data| {
                    self.ingredient_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("ingredient", selection)
                            .new_section("")
                            .field("IID", |ui, d| ui.add(id_field(&mut d.iid)))
                            .field("Name", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "item", d.name)
                            })
                            .default_field("Flag", |d| &mut d.flag)
                            .default_field("Category", |d| &mut d.category)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::FoodNaming => {
                self.food_naming_content
                    .left_panel(ctx, &self.food_naming, state);
                self.food_naming.write(|data| {
                    self.food_naming_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("food_naming", selection)
                                .new_section("")
                                .field("PID", |ui, d| ui.add(id_field(&mut d.pid)))
                                .default_field("Name Type 0", |d| &mut d.name_type_0)
                                .default_field("Name Type 1", |d| &mut d.name_type_1)
                                .default_field("Name Type 2", |d| &mut d.name_type_2)
                                .default_field("Name Type 3", |d| &mut d.name_type_3)
                                .default_field("Name Type 4", |d| &mut d.name_type_4)
                                .default_field("Name Type 5", |d| &mut d.name_type_5)
                                .default_field("Name Type 6", |d| &mut d.name_type_6)
                                .default_field("Name Type 7", |d| &mut d.name_type_7)
                                .default_field("Name Type 8", |d| &mut d.name_type_8)
                                .default_field("Name Type 9", |d| &mut d.name_type_9)
                                .default_field("Name Type 10", |d| &mut d.name_type_10)
                                .default_field("Name Type 11", |d| &mut d.name_type_11)
                                .default_field("Name Type 12", |d| &mut d.name_type_12)
                                .default_field("Name Type 13", |d| &mut d.name_type_13)
                                .default_field("Name Type 14", |d| &mut d.name_type_14)
                                .default_field("Name Type 15", |d| &mut d.name_type_15)
                                .default_field("Name Type 16", |d| &mut d.name_type_16)
                                .default_field("Name Type 17", |d| &mut d.name_type_17)
                                .default_field("Name Type 18", |d| &mut d.name_type_18)
                                .default_field("Name Type 19", |d| &mut d.name_type_19)
                                .default_field("Name Type 20", |d| &mut d.name_type_20)
                                .default_field("Name Type 21", |d| &mut d.name_type_21)
                                .default_field("Name Type 22", |d| &mut d.name_type_22)
                                .default_field("Name Type 23", |d| &mut d.name_type_23)
                                .default_field("Name Type 24", |d| &mut d.name_type_24)
                                .default_field("Name Type 25", |d| &mut d.name_type_25)
                                .default_field("Name Type 26", |d| &mut d.name_type_26)
                                .default_field("Name Type 27", |d| &mut d.name_type_27)
                                .default_field("Name Type 28", |d| &mut d.name_type_28)
                                .default_field("Name Type 29", |d| &mut d.name_type_29)
                                .default_field("Name Type 30", |d| &mut d.name_type_30)
                                .default_field("Name Type 31", |d| &mut d.name_type_31)
                                .default_field("Name Type 32", |d| &mut d.name_type_32)
                                .default_field("Name Type 33", |d| &mut d.name_type_33)
                                .default_field("Name Type 34", |d| &mut d.name_type_34)
                                .default_field("Name Type 35", |d| &mut d.name_type_35)
                                .default_field("Name Type 36", |d| &mut d.name_type_36)
                                .default_field("Name Type 37", |d| &mut d.name_type_37)
                                .default_field("Name Type 38", |d| &mut d.name_type_38)
                                .default_field("Name Type 39", |d| &mut d.name_type_39)
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
