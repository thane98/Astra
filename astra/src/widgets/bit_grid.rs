use egui::{Grid, Ui};

macro_rules! bitgrid {
    ($name:ident, $target:ty) => {
        pub fn $name<'a>(
            bits: &'a [&str],
            num_columns: usize,
            value: &'a mut Option<$target>,
        ) -> impl egui::Widget + 'a {
            move |ui: &mut Ui| {
                let mut changed = false;
                let mut response = Grid::new(ui.auto_id_with("bitgrid"))
                    .show(ui, |ui| {
                        let original_value = value.unwrap_or_default();
                        let mut new_value = original_value;
                        for i in (0..bits.len()).step_by(num_columns) {
                            for j in i..(i + num_columns) {
                                if j >= bits.len() {
                                    break;
                                }
                                let original_bit = original_value & (1 << j) != 0;
                                let mut new_bit = original_bit;
                                ui.checkbox(&mut new_bit, bits[j]);
                                if original_bit != new_bit {
                                    new_value ^= 1 << j;
                                }
                            }
                            ui.end_row();
                        }
                        if new_value != original_value {
                            *value = Some(new_value);
                            changed = true;
                        }
                    })
                    .response;
                if changed {
                    response.mark_changed();
                }
                response
            }
        }
    };
}

bitgrid!(bitgrid_u8, u8);
// bitgrid!(bitgrid_i8, i8);
bitgrid!(bitgrid_u16, u16);
// bitgrid!(bitgrid_i16, i16);
// bitgrid!(bitgrid_u32, u32);
bitgrid!(bitgrid_i32, i32);
bitgrid!(bitgrid_u64, u64);
// bitgrid!(bitgrid_i64, i64);
