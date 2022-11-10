#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod backend;
mod config;
mod frontend;
mod theme;
mod util;

use config::Config;
use frontend::{App, State};
use tracing::Level;

pub fn new() -> (backend::Backend, App) {
    #[cfg(feature = "article")]
    let (article_consumer, article_producer) = backend::article::make_service();
    #[cfg(feature = "database")]
    let (sql_sender, sql_executor) = backend::database::make_service();
    let repo = backend::Repo {
        #[cfg(feature = "article")]
        article_client: article_consumer,
        #[cfg(feature = "database")]
        database_client: sql_sender,
    };
    let mut servers = vec![];
    #[cfg(feature = "article")]
    servers.push(article_producer);
    #[cfg(feature = "database")]
    servers.push(sql_executor);

    (
        backend::Backend { servers },
        App {
            state: State::new(repo),
            config: Config::new(),
        },
    )
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let (backend, frontend) = new();
    std::thread::spawn(move || {
        backend.run();
    });
    frontend.run();
}
