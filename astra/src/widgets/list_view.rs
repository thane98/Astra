use egui::{Image, Response, ScrollArea, Sense, Ui, Widget};

use crate::{DecorationKind, ListModel, ViewItem, DOWN_ENTRY_SHORTCUT, UP_ENTRY_SHORTCUT};

fn item_number_ui(ui: &mut Ui, index: usize, max_indent: usize) {
    let label = (index + 1).to_string();
    ui.monospace(format!(
        "{}{}.",
        " ".repeat(max_indent.saturating_sub(label.len())),
        label
    ));
}

fn list_item_ui<M, I, D>(
    ui: &mut Ui,
    model: &M,
    dependencies: &D,
    index: usize,
    selected: bool,
    max_indent: usize,
) -> Response
where
    M: ListModel<I>,
    I: ViewItem<Dependencies = D>,
{
    if let Some(text) = model.item(index).map(|item| item.text(dependencies)) {
        item_number_ui(ui, model.row_to_index(index).unwrap_or(0), max_indent);
        if let Some((decoration, scale)) = model
            .item(index)
            .and_then(|item| item.decoration(dependencies, DecorationKind::List))
        {
            ui.add(Image::new(&decoration).max_size(decoration.size_vec2() * scale));
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

    let max_indent = (model.len() + 1).to_string().len();
    let mut changed = false;
    let output = ScrollArea::both().auto_shrink([false, false]).show_rows(
        ui,
        20.,
        model.len(),
        |ui, range| {
            for index in range {
                if I::decorated(DecorationKind::List) {
                    let response = ui
                        .horizontal(|ui| {
                            list_item_ui(
                                ui,
                                model,
                                dependencies,
                                index,
                                Some(index) == *selected_index,
                                max_indent,
                            )
                        })
                        .inner;
                    if response.clicked() {
                        *selected_index = Some(index);
                        changed = true;
                    }
                } else {
                    ui.horizontal(|ui| {
                        item_number_ui(ui, model.row_to_index(index).unwrap_or(0), max_indent);
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
            }
        },
    );

    if ui.input_mut(|input| input.consume_shortcut(&UP_ENTRY_SHORTCUT)) {
        if let Some(index) = *selected_index {
            if index > 0 {
                *selected_index = Some(index - 1);
            }
        }
        changed = true;
    }
    if ui.input_mut(|input| input.consume_shortcut(&DOWN_ENTRY_SHORTCUT)) {
        *selected_index = match *selected_index {
            Some(index) => (index < model.len() - 1)
                .then_some(index + 1)
                .or(Some(index)),
            None => (model.len() > 0).then_some(0),
        };
        changed = true;
    }

    let mut response = ui.interact(
        output.inner_rect,
        output.id,
        Sense::focusable_noninteractive(),
    );
    if changed {
        response.mark_changed();
    }
    response
}

pub fn list_view<'a, M, I, D>(
    model: &'a M,
    dependencies: &'a D,
    selected_index: &'a mut Option<usize>,
) -> impl Widget + 'a
where
    M: ListModel<I>,
    I: ViewItem<Dependencies = D>,
{
    move |ui: &mut Ui| list_view_ui(ui, model, dependencies, selected_index)
}
