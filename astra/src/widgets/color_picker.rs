use egui::{Color32, Sense, Ui, Widget};

use crate::u8_drag;

pub fn rgb_color_picker<'a>(
    r: &'a mut Option<u8>,
    g: &'a mut Option<u8>,
    b: &'a mut Option<u8>,
) -> impl Widget + 'a {
    move |ui: &mut Ui| {
        ui.horizontal_top(|ui| {
            let response = ui
                .add(u8_drag(r))
                .union(ui.add(u8_drag(g)))
                .union(ui.add(u8_drag(b)));

            let color = Color32::from_rgb(
                r.unwrap_or_default(),
                g.unwrap_or_default(),
                b.unwrap_or_default(),
            );

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
