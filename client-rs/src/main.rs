#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{sync::mpsc, thread};

use app::App;
use client::api::{self, Repo};
use eframe::emath::Vec2;
use pages::headline::NewsArticle;
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

    let (sender, receiver) = mpsc::channel::<Vec<NewsArticle>>();

    let repo = Repo {
        article_channel: receiver,
    };

    thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();
        runtime.block_on(async move {
            if let Err(e) = tokio::spawn(async move {
                let articles = api::fetch_articles().await;
                if let Err(e) = sender.send(articles) {
                    tracing::error!("Channel 发送数据失败：{:?}", e);
                }
            })
            .await
            {
                tracing::error!("任务执行异常：{:?}", e);
            }
        });
    });

    let app = App::new(repo);
    let mut options = eframe::NativeOptions::default();
    options.resizable = true;
    options.vsync = true;
    options.initial_window_size = Some(Vec2::new(480.0, 740.0));
    eframe::run_native("我的应用程序", options, Box::new(|_cc| Box::new(app)));
}

async fn fetch() -> anyhow::Result<()> {
    Ok(())
}
