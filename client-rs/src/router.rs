use crate::pages::{article::Article, database::database::DataBase, setting::Setting, talk::Talk};

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

impl Default for Router {
    fn default() -> Self {
        // let client = Client::new(([127, 0, 0, 1], 1234)).unwrap();
        Self {
            page: Default::default(),
            article: Article::default(),
            setting: Default::default(),
            database: Default::default(),
        }
    }
}
