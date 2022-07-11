use async_trait::async_trait;
pub mod api;
pub mod entity;
use crate::util::duplex_channel::{self, DuplexConsumer, DuplexProducer};

use self::entity::NewsArticle;

use super::Server;

type S = ();
type D = Vec<NewsArticle>;
pub struct ArticleServer {
    pub inner: DuplexProducer<S, D>,
}
pub struct ArticleClient {
    pub inner: DuplexConsumer<S, D>,
}

pub fn make_service() -> (ArticleClient, Box<dyn Server>) {
    let (consumer, producer) = duplex_channel::channel::<S, D>();
    let client = ArticleClient { inner: consumer };
    let server = ArticleServer { inner: producer };
    (client, Box::new(server))
}

#[async_trait]
impl Server for ArticleServer {
    async fn block_on(&mut self) {
        tracing::info!("机核网文章资讯查询器启动正常");
        self.inner
            .wait_produce(|| Box::pin(async { api::fetch_articles().await }))
            .await;
    }
}
