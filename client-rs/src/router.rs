use crate::pages::{database::DataBase, headline::HeadLine, setting::Setting, talk::Talk};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
    pub article: HeadLine,
    pub database: DataBase,
    pub setting: Setting,
}

impl Default for Router {
    fn default() -> Self {
        // let client = Client::new(([127, 0, 0, 1], 1234)).unwrap();
        Self {
            page: Default::default(),
            article: HeadLine::default(),
            setting: Default::default(),
            database: Default::default(),
        }
    }
}
