use egui::{ComboBox, Grid, Response, Ui, Widget};

use crate::{DecorationKind, KeyedListModel, KeyedViewItem, ListModel, ViewItem};

pub fn model_drop_down<'a, M, I, D>(
    model: &'a M,
    dependencies: &'a D,
    selected: &'a mut String,
) -> impl Widget + 'a
where
    M: KeyedListModel<I>,
    I: KeyedViewItem<Dependencies = D>,
{
    move |ui: &mut Ui| ModelDropDown::default().show(ui, model, dependencies, selected)
}

pub fn indexed_model_drop_down<'a, M, I, D>(
    model: &'a M,
    dependencies: &'a D,
    selected: &'a mut Option<usize>,
) -> impl Widget + 'a
where
    M: ListModel<I>,
    I: ViewItem<Dependencies = D>,
{
    move |ui: &mut Ui| ModelDropDown::default().show_indexed(ui, model, dependencies, selected)
}

fn drop_down_item_ui<M, I, D>(
    ui: &mut Ui,
    model: &M,
    dependencies: &D,
    index: usize,
    selected: bool,
) -> Response
where
    M: ListModel<I>,
    I: ViewItem<Dependencies = D>,
{
    if let Some(text) = model.item(index).map(|item| item.text(dependencies)) {
        if let Some((decoration, scale)) = model
            .item(index)
            .and_then(|item| item.decoration(dependencies, DecorationKind::DropDown))
        {
            ui.image(decoration.id(), decoration.size_vec2() * scale);
        } else {
            ui.label("");
        }
        ui.selectable_label(selected, text)
    } else {
        // Out of bounds - fill with empty space.
        ui.label("");
        ui.label("")
    }
}

#[derive(Default)]
pub struct ModelDropDown<'a> {
    key_transform: Option<&'a dyn Fn(&str) -> String>,
    key_reverse_transform: Option<&'a dyn Fn(&str) -> String>,
}

impl<'a> ModelDropDown<'a> {
    pub fn transform(
        mut self,
        transform: &'a dyn Fn(&str) -> String,
        reverse_transform: &'a dyn Fn(&str) -> String,
    ) -> Self {
        self.key_transform = Some(transform);
        self.key_reverse_transform = Some(reverse_transform);
        self
    }

    pub fn show<M, I, D>(
        self,
        ui: &mut Ui,
        model: &M,
        dependencies: &D,
        key: &mut String,
    ) -> Response
    where
        M: KeyedListModel<I>,
        I: KeyedViewItem<Dependencies = D>,
    {
        let mut changed = false;
        let id = ui.auto_id_with("model_combo_box");
        let index = match self.key_transform {
            Some(transform) => model.index_of(&transform(key)),
            None => model.index_of(key),
        };
        let inner_response = ComboBox::from_id_source(id)
            .selected_text(
                index
                    .and_then(|index| model.item(index))
                    .map(|item| item.text(dependencies))
                    .unwrap_or_default(),
            )
            .width(ui.spacing().text_edit_width)
            .show_ui(ui, |ui| {
                if I::decorated(DecorationKind::DropDown) {
                    Grid::new(ui.auto_id_with("__model_combo_box_grid"))
                        .num_columns(2)
                        .show(ui, |ui| {
                            for i in 0..model.len() {
                                let response =
                                    drop_down_item_ui(ui, model, dependencies, i, Some(i) == index);
                                ui.end_row();
                                if response.clicked() {
                                    if let Some(new_key) = model.item(i).map(|item| item.key()) {
                                        *key = match self.key_reverse_transform {
                                            Some(transform) => transform(&new_key),
                                            None => new_key.to_string(),
                                        };
                                        changed = true;
                                    }
                                }
                            }
                        });
                } else {
                    for i in 0..model.len() {
                        ui.vertical(|ui| {
                            if ui
                                .selectable_label(
                                    Some(i) == index,
                                    model
                                        .item(i)
                                        .map(|i| i.text(dependencies))
                                        .unwrap_or("{empty}".into()),
                                )
                                .clicked()
                            {
                                if let Some(new_key) = model.item(i).map(|item| item.key()) {
                                    *key = match self.key_reverse_transform {
                                        Some(transform) => transform(&new_key),
                                        None => new_key.to_string(),
                                    };
                                    changed = true;
                                }
                            }
                        });
                    }
                }
            });
        let mut response = inner_response.response;
        if changed {
            response.mark_changed();
        }
        response
    }

    pub fn show_indexed<M, I, D>(
        self,
        ui: &mut Ui,
        model: &M,
        dependencies: &D,
        index: &mut Option<usize>,
    ) -> Response
    where
        M: ListModel<I>,
        I: ViewItem<Dependencies = D>,
    {
        let mut changed = false;
        let id = ui.auto_id_with("model_combo_box");
        let inner_response = ComboBox::from_id_source(id)
            .selected_text(
                index
                    .and_then(|index| model.item(index))
                    .map(|item| item.text(dependencies))
                    .unwrap_or_default(),
            )
            .width(ui.spacing().text_edit_width)
            .show_ui(ui, |ui| {
                if I::decorated(DecorationKind::DropDown) {
                    Grid::new(ui.auto_id_with("__model_combo_box_grid"))
                        .num_columns(2)
                        .show(ui, |ui| {
                            for i in 0..model.len() {
                                let response = drop_down_item_ui(
                                    ui,
                                    model,
                                    dependencies,
                                    i,
                                    Some(i) == *index,
                                );
                                ui.end_row();
                                if response.clicked() {
                                    *index = Some(i);
                                    changed = true;
                                }
                            }
                        });
                } else {
                    for i in 0..model.len() {
                        changed |= ui
                            .selectable_value(
                                index,
                                Some(i),
                                model
                                    .item(i)
                                    .map(|i| i.text(dependencies))
                                    .unwrap_or("{empty}".into()),
                            )
                            .changed();
                    }
                }
            });
        let mut response = inner_response.response;
        if changed {
            response.mark_changed();
        }
        response
    }
}
