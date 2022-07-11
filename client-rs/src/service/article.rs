use crate::util::duplex_channel::{self, DuplexConsumer, DuplexProducer};

use super::entity::NewsArticle;

type S = ();
type D = Vec<NewsArticle>;
pub struct ArticleServer {
    pub inner: DuplexProducer<S, D>,
}
pub struct ArticleClient {
    pub inner: DuplexConsumer<S, D>,
}

pub fn make_article_service() -> (ArticleClient, ArticleServer) {
    let (consumer, producer) = duplex_channel::channel::<S, D>();
    let client = ArticleClient { inner: consumer };
    let server = ArticleServer { inner: producer };
    (client, server)
}
