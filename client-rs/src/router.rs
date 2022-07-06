use crate::apps::{article::Article, database::DataBase, setting::Setting};

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
