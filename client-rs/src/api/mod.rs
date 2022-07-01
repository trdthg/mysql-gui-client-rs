use std::thread;

pub mod article;

pub use article::fetch_articles;

use crate::util::duplex_channel::{self, DuplexConsumer};

use self::article::NewsArticle;
pub struct Repo {
    pub article: DuplexConsumer<Vec<NewsArticle>>,
}

impl Repo {
    pub(crate) fn new() -> Repo {
        let (consumer, mut producer) = duplex_channel::channel::<Vec<NewsArticle>>();

        let repo = Repo { article: consumer };

        thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap();
            runtime.block_on(async move {
                if let Err(e) = tokio::spawn(async move {
                    // 不断等待请求更新
                    producer
                        .wait_for_produce(|| Box::pin(async { fetch_articles().await }))
                        .await;
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
