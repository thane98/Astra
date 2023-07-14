#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod editors;
mod message_db;
mod model;
mod states;
mod texture_cache;
mod widgets;

use editors::*;
use message_db::*;
use model::*;
use states::*;
use texture_cache::*;
use widgets::*;

use eframe::IconData;

fn main() {
    tracing_subscriber::fmt::init();

    let app_config = AppConfig::load().expect("failed to initialize application");
    let mut native_options = eframe::NativeOptions {
        icon_data: Some(
            IconData::try_from_png_bytes(include_bytes!("../assets/astra.png")).unwrap(),
        ),
        ..Default::default()
    };
    native_options.initial_window_size = Some([1310., 800.].into());

    eframe::run_native(
        "Astra",
        native_options,
        Box::new(|cc| Box::new(app::AstraApp::new(cc, app_config))),
    )
    .unwrap()
}
