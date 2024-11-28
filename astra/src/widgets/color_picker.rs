use egui::{Color32, DragValue, Sense, Ui, Widget};

pub fn rgb_color_picker<'a>(r: &'a mut u8, g: &'a mut u8, b: &'a mut u8) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        ui.horizontal_top(|ui| {
            let response = ui
                .add(DragValue::new(r))
                .union(ui.add(DragValue::new(g)))
                .union(ui.add(DragValue::new(b)));

            let color = Color32::from_rgb(*r, *g, *b);

            // Borrowing from egui's color_button implementation
            let size = ui.spacing().interact_size;
            let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
            if ui.is_rect_visible(rect) {
                let visuals = ui.style().noninteractive();
                let rect = rect.expand(visuals.expansion);
                ui.painter().rect_filled(rect, 0.0, color);
                let rounding = visuals.rounding.at_most(2.0);
                ui.painter()
                    .rect_stroke(rect, rounding, (2.0, visuals.bg_fill));
            }

            response
        })
        .inner
    }
}
