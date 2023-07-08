macro_rules! numeric_get_set {
    ($get_set_name:ident, $widget_name:ident, $target:ty) => {
        pub fn $get_set_name(value: &mut Option<$target>) -> impl FnMut(Option<f64>) -> f64 + '_ {
            move |new_value: Option<f64>| {
                if let Some(new_value) = new_value {
                    *value = Some(new_value as $target);
                }
                value.unwrap_or_default() as f64
            }
        }

        pub fn $widget_name(value: &mut Option<$target>) -> egui::DragValue<'_> {
            egui::DragValue::from_get_set($get_set_name(value))
        }
    };
}

numeric_get_set!(u8_numeric_get_set, u8_drag, u8);
numeric_get_set!(u16_numeric_get_set, u16_drag, u16);
numeric_get_set!(u32_numeric_get_set, u32_drag, u32);
numeric_get_set!(i8_numeric_get_set, i8_drag, i8);
numeric_get_set!(i16_numeric_get_set, i16_drag, i16);
numeric_get_set!(i32_numeric_get_set, i32_drag, i32);
numeric_get_set!(f32_numeric_get_set, f32_drag, f32);
