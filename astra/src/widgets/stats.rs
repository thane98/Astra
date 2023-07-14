use egui::Ui;

pub fn standard_stat_column_headers(ui: &mut Ui) {
    ui.label("");
    ui.label("HP");
    ui.label("Str");
    ui.label("Def");
    ui.label("Skl");
    ui.label("Spd");
    ui.label("Lck");
    ui.label("Mag");
    ui.label("Res");
    ui.label("Con");
    ui.label("Mov");
    ui.label("Sight");
    ui.end_row();
}

#[macro_export]
macro_rules! standard_stats_row {
    ($ui:ident, $target:ident, $prefix:ident, $changed:ident) => {
        paste::paste! {
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _hp>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _str>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _def>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _tech>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _quick>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _luck>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _magic>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _mdef>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _phys>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _move>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _sight>], $ui).changed() {
                $changed = true;
            }
        }
        $ui.end_row();
    };
}

pub fn stat_column_headers_no_sight(ui: &mut Ui) {
    ui.label("");
    ui.label("HP");
    ui.label("Str");
    ui.label("Def");
    ui.label("Skl");
    ui.label("Spd");
    ui.label("Lck");
    ui.label("Mag");
    ui.label("Res");
    ui.label("Con");
    ui.label("Mov");
    ui.end_row();
}

#[macro_export]
macro_rules! stats_row_no_sight {
    ($ui:ident, $target:ident, $prefix:ident, $changed:ident) => {
        paste::paste! {
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _hp>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _str>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _def>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _tech>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _quick>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _luck>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _magic>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _mdef>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _phys>], $ui).changed() {
                $changed = true;
            }
            if $crate::DefaultWidget::default_widget(&mut $target.[<$prefix _move>], $ui).changed() {
                $changed = true;
            }
        }
        $ui.end_row();
    };
}
