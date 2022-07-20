#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod backend;
mod config;
mod frontend;
mod theme;
mod util;

use backend::{article, database, Backend, Repo};
use config::Config;
use frontend::{App, State};
use tracing::Level;

pub fn new() -> (Backend, App) {
    let (article_consumer, article_producer) = article::make_service();
    let (sql_sender, sql_executor) = database::make_service();
    let repo = Repo {
        article_client: article_consumer,
        database_client: sql_sender,
    };
    let servers = vec![article_producer, sql_executor];

    (
        Backend { servers },
        App {
            state: State::new(repo),
            config: Config::new(),
        },
    )
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let (backend, frontend) = new();
    std::thread::spawn(move || {
        backend.run();
    });
    frontend.run()
}
