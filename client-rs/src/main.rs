#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::App;
use eframe::emath::Vec2;
use tracing::Level;

mod app;
mod client;
mod config;
mod pages;
mod router;
mod theme;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let app = App::default();
    let mut options = eframe::NativeOptions::default();
    options.resizable = true;
    options.vsync = true;
    options.initial_window_size = Some(Vec2::new(480.0, 740.0));
    eframe::run_native("我的应用程序", options, Box::new(|_cc| Box::new(app)));
}
