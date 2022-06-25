#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::App;
use eframe::emath::Vec2;

mod app;
mod client;
mod font;
mod pages;
mod router;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let app = App::default();
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(480.0, 740.0));
    eframe::run_native("我的应用程序", options, Box::new(|_cc| Box::new(app)));
}
