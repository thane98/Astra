use egui::{Response, Ui, Widget};

pub fn editable_list<'a, T: Default>(
    value: &'a mut Vec<T>,
    item_renderer: impl Fn(usize, &mut T, &mut Ui) -> Response + 'a,
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        let mut changed = false;
        let mut response = ui
            .vertical(|ui| {
                if ui.button("+").clicked() {
                    value.push(T::default());
                    changed = true;
                }
                let mut flagged_for_removal = None;
                for (i, item) in value.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.button("-").clicked() {
                            flagged_for_removal = Some(i);
                            changed = true;
                        }
                        changed |= item_renderer(i, item, ui).changed();
                    });
                }
                if let Some(index) = flagged_for_removal {
                    value.remove(index);
                }
            })
            .response;
        if changed {
            response.mark_changed()
        }
        response
    }
}
