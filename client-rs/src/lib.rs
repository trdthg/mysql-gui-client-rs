#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{sync::mpsc, thread};

use api::Repo;
use app::App;
use eframe::emath::Vec2;
use pages::headline::NewsArticle;
use tracing::Level;

mod api;
mod app;
mod client;
mod config;
mod pages;
mod router;
mod theme;

pub fn headers(headers: &[(&str, &str)]) -> std::collections::BTreeMap<String, String> {
    headers
        .iter()
        .map(|e| (e.0.to_owned(), e.1.to_owned()))
        .collect()
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    tracing_wasm::set_as_global_default();
    tracing::info!("打印信息");

    tracing::info!("初始化");
    let (sender, receiver) = mpsc::channel::<Vec<NewsArticle>>();

    let repo = Repo {
        article_channel: receiver,
    };

    #[cfg(target_arch = "wasm32")]
    {
        let sender2 = sender.clone();
        gloo_timers::callback::Timeout::new(10, move || {
            wasm_bindgen_futures::spawn_local(async move {
                let articles = api::fetch_articles().await;
                if let Err(e) = sender2.send(articles) {
                    tracing::error!("Channel 发送数据失败：{:?}", e);
                }
            });
        })
        .forget();
    }

    tracing::info!("New App");
    let app = App::new(repo);
    tracing::info!("启动");
    eframe::start_web(canvas_id, Box::new(|cc| Box::new(app)))
}
