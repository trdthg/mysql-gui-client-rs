use std::{
    sync::mpsc::{self, Receiver},
    thread,
};

use crate::pages::headline::NewsArticle;

pub mod article;

pub use article::fetch_articles;
pub struct Repo {
    pub article_channel: Receiver<Vec<NewsArticle>>,
}
impl Repo {
    pub(crate) fn new() -> Repo {
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
                    let articles = fetch_articles().await;
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

        repo
    }
}
