#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod config;
mod service;
mod theme;
mod util;

use app::App;
use eframe::emath::Vec2;
use service::Backend;
use tracing::Level;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let (server, repo) = Backend::new();
    server.run();
    let app = App::new(repo);
    let mut options = eframe::NativeOptions::default();
    options.resizable = true;
    options.vsync = true;
    options.initial_window_size = Some(Vec2::new(480.0, 740.0));
    eframe::run_native("My App", options, Box::new(|_cc| Box::new(app)));
}
