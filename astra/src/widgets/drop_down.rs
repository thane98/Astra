use egui::{AboveOrBelow, Grid, Image, Response, ScrollArea, Sense, Ui, Widget};

use crate::{
    queue_transition, DecorationKind, KeyedListModel, KeyedViewItem, ListModel, Transition,
    ViewItem,
};

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

fn drop_down_item_ui<I, D>(
    ui: &mut Ui,
    dependencies: &D,
    item: Option<&I>,
    text: &str,
    selected: bool,
) -> Response
where
    I: ViewItem<Dependencies = D>,
{
    if let Some(item) = item {
        if let Some((decoration, scale)) = item.decoration(dependencies, DecorationKind::DropDown) {
            ui.add(Image::new(&decoration).max_size(decoration.size_vec2() * scale));
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

    fn show_impl<M, I, D>(
        ui: &mut Ui,
        model: &M,
        dependencies: &D,
        selected_index: Option<usize>,
    ) -> (Response, Option<usize>)
    where
        M: ListModel<I>,
        I: ViewItem<Dependencies = D>,
    {
        let id = ui.auto_id_with("model_combo_box");
        let prev_index_id = ui.auto_id_with("model_combo_box_prev_index");
        let mut selection = None;

        let display_text = selected_index
            .and_then(|index| model.item(index))
            .map(|item| item.text(dependencies))
            .unwrap_or_default();

        let prev_index = ui.memory_mut(|mem| {
            *mem.data
                .get_persisted_mut_or_default::<Option<usize>>(prev_index_id)
        });
        ui.memory_mut(|mem| mem.data.insert_persisted(prev_index_id, selected_index));

        let mut search = ui.memory_mut(|mem| {
            mem.data
                .get_persisted_mut_or_insert_with::<String>(id, || display_text.to_string())
                .to_owned()
        });

        let text_edit_response = ui.text_edit_singleline(&mut search);
        if text_edit_response.gained_focus() {
            ui.memory_mut(|mem| {
                mem.data.insert_persisted(id, String::new());
                mem.open_popup(id);
            })
        } else if text_edit_response.lost_focus() || prev_index != selected_index {
            ui.memory_mut(|mem| {
                mem.data.insert_persisted(id, display_text.to_string());
            });
        }

        // Copied from egui's ComboBox implementation.
        let above_or_below = if ui.next_widget_position().y + ui.spacing().interact_size.y + 200.0
            < ui.ctx().screen_rect().bottom()
        {
            AboveOrBelow::Below
        } else {
            AboveOrBelow::Above
        };

        egui::popup_above_or_below_widget(ui, id, &text_edit_response, above_or_below, |ui| {
            ScrollArea::vertical().max_height(200.).show(ui, |ui| {
                if I::decorated(DecorationKind::DropDown) {
                    Grid::new(ui.auto_id_with("__model_combo_box_grid"))
                        .num_columns(2)
                        .show(ui, |ui| {
                            for i in 0..model.len() {
                                let item = model.item(i);
                                let text =
                                    item.map(|item| item.text(dependencies)).unwrap_or_default();
                                let matches_search = item
                                    .map(|item| item.matches_filter(&search, &text))
                                    .unwrap_or_default();
                                if search.is_empty() || matches_search {
                                    let response = drop_down_item_ui(
                                        ui,
                                        dependencies,
                                        item,
                                        &text,
                                        Some(i) == selected_index,
                                    );
                                    ui.end_row();
                                    if response.clicked() {
                                        selection = Some(i);
                                    }
                                }
                            }
                        });
                } else {
                    for i in 0..model.len() {
                        let item = model.item(i);
                        let text = item.map(|item| item.text(dependencies)).unwrap_or_default();
                        let matches_search = item
                            .map(|item| item.matches_filter(&search, &text))
                            .unwrap_or_default();
                        if search.is_empty() || matches_search {
                            ui.vertical(|ui| {
                                if ui
                                    .selectable_label(
                                        Some(i) == selected_index,
                                        model
                                            .item(i)
                                            .map(|i| i.text(dependencies))
                                            .unwrap_or("{empty}".into()),
                                    )
                                    .clicked()
                                {
                                    selection = Some(i);
                                }
                            });
                        }
                    }
                }
            });
        });

        if text_edit_response.changed() {
            ui.memory_mut(|mem| {
                mem.data.insert_persisted(id, search);
            });
        }

        let mut response = ui.interact(
            text_edit_response.rect,
            id,
            Sense::focusable_noninteractive(),
        );
        if selection.is_some() {
            response.mark_changed();
        }
        (response, selection)
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
        let index = match self.key_transform {
            Some(transform) => model.index_of(&transform(key)),
            None => model.index_of(key),
        };

        ui.horizontal(|ui| {
            let (response, selection) = Self::show_impl(ui, model, dependencies, index);
            if let Some(i) = selection {
                if let Some(new_key) = model.item(i).map(|item| item.key()) {
                    *key = match self.key_reverse_transform {
                        Some(transform) => transform(&new_key),
                        None => new_key.to_string(),
                    };
                }
            }
            if let Some(index) = selection.or(index) {
                if let Some(screen) = I::screen() {
                    if ui.button("⮩ Go To").clicked() {
                        queue_transition(Transition::new(screen, index));
                    }
                }
            }
            response
        })
        .response
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
        let (response, selection) = Self::show_impl(ui, model, dependencies, *index);
        if let Some(i) = selection {
            *index = Some(i);
        }
        response
    }
}
