#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod server;
mod config;
mod ui;
mod theme;
mod util;

use config::Config;
use ui::{App, State};
use tracing::Level;

pub fn new() -> (server::Backend, App) {
    #[cfg(feature = "article")]
    let (article_consumer, article_producer) = server::article::make_service();
    #[cfg(feature = "database")]
    let (sql_sender, sql_executor) = server::database::make_service();
    let repo = server::Repo {
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
        server::Backend { servers },
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
