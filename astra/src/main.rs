#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod editors;
mod message_db;
mod model;
mod states;
mod texture_cache;
mod widgets;
mod util;

use astra_core::image;
use editors::*;
use message_db::*;
use model::*;
use states::*;
use texture_cache::*;
use widgets::*;

fn main() {
    // Attempt to truncate the log file.
    let _ = std::fs::write("astra.log", "");

    let (writer, _guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::never(".", "astra.log"));
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_ansi(false)
        .with_line_number(true)
        .init();

    let app_config = AppConfig::load().expect("failed to initialize application");

    let icon = image::load_from_memory(include_bytes!("../assets/astra.png")).unwrap();
    let icon = egui::IconData {
        width: icon.width(),
        height: icon.height(),
        rgba: icon.into_rgba8().into_raw(),
    };

    let viewport = egui::ViewportBuilder::default()
        .with_icon(icon)
        .with_inner_size([1310., 800.]);

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Astra",
        native_options,
        Box::new(|cc| Box::new(app::AstraApp::new(cc, app_config))),
    )
    .unwrap()
}
