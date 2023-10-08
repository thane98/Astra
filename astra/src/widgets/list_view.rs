use egui::{Grid, Response, ScrollArea, Ui, Widget};

use crate::{DecorationKind, ListModel, ViewItem};

fn list_item_ui<M, I, D>(
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
        let true_index = model.row_to_index(index).unwrap_or(0);
        ui.label(format!("{}.", true_index + 1));
        if let Some((decoration, scale)) = model
            .item(index)
            .and_then(|item| item.decoration(dependencies, DecorationKind::List))
        {
            ui.image(decoration.id(), decoration.size_vec2() * scale);
        } else {
            ui.label("");
        }
        ui.selectable_label(selected, text)
    } else {
        // Out of bounds - fill with empty space.
        ui.label("");
        ui.label("");
        ui.label("")
    }
}

fn list_view_ui<M, I, D>(
    ui: &mut Ui,
    row_height: f32,
    model: &M,
    dependencies: &D,
    selected_index: &mut Option<usize>,
) -> Response
where
    M: ListModel<I>,
    I: ViewItem<Dependencies = D>,
{
    if model.len() == 0 && selected_index.is_some() {
        *selected_index = None;
    }

    let mut changed = false;
    let output = ScrollArea::both().auto_shrink([false, false]).show_rows(
        ui,
        row_height,
        model.len(),
        |ui, range| {
            if I::decorated(DecorationKind::List) {
                Grid::new(ui.auto_id_with("astra_list_view_grid"))
                    .num_columns(3)
                    .show(ui, |ui| {
                        for index in range {
                            let response = list_item_ui(
                                ui,
                                model,
                                dependencies,
                                index,
                                Some(index) == *selected_index,
                            );
                            ui.end_row();
                            if response.clicked() {
                                *selected_index = Some(index);
                                changed = true;
                            }
                        }
                    })
            } else {
                ui.vertical(|ui| {
                    for index in range {
                        ui.horizontal(|ui| {
                            let true_index = model.row_to_index(index).unwrap_or(0);
                            ui.label(format!("{}.", true_index + 1));
                            if ui
                                .selectable_label(
                                    Some(index) == *selected_index,
                                    model
                                        .item(index)
                                        .map(|item| item.text(dependencies))
                                        .unwrap_or_default(),
                                )
                                .clicked()
                            {
                                *selected_index = Some(index);
                                changed = true;
                            }
                        });
                    }
                })
            }
        },
    );
    let mut response = output.inner.response;
    if changed {
        response.mark_changed();
    }
    response
}

pub fn list_view<'a, M, I, D>(
    row_height: f32,
    model: &'a M,
    dependencies: &'a D,
    selected_index: &'a mut Option<usize>,
) -> impl Widget + 'a
where
    M: ListModel<I>,
    I: ViewItem<Dependencies = D>,
{
    move |ui: &mut Ui| list_view_ui(ui, row_height, model, dependencies, selected_index)
}
