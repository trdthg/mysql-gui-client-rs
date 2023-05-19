#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct NewsArticle {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub url: String,
    pub author: String,
    pub time: String,
}

impl Default for NewsArticle {
    fn default() -> Self {
        Self {
            id: Default::default(),
            title: "文章标题".to_owned(),
            desc: Default::default(),
            url: Default::default(),
            author: Default::default(),
            time: Default::default(),
        }
    }
}
