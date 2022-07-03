use crate::{
    api::{mysql::ConnectionConfig, Repo},
    pages::{
        article::Article,
        database::database::{Connection, DataBase},
        setting::Setting,
        talk::Talk,
    },
    util::duplex_channel::DuplexConsumer,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Page {
    Article,
    DataBase,
    Setting,
}

impl Default for Page {
    fn default() -> Self {
        Self::Article
    }
}
pub struct Router {
    pub page: Page,
    pub article: Article,
    pub database: DataBase,
    pub setting: Setting,
}

impl Router {
    pub fn new(conn_manager: DuplexConsumer<ConnectionConfig, Connection>) -> Self {
        // let client = Client::new(([127, 0, 0, 1], 1234)).unwrap();
        Self {
            page: Default::default(),
            article: Article::default(),
            setting: Default::default(),
            database: DataBase::new(conn_manager),
        }
    }
}
