use async_trait::async_trait;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
pub mod api;
pub mod entity;

use self::entity::NewsArticle;

use super::{make_chan, Channels, Client, Server};

type S = ();
type D = Vec<NewsArticle>;
pub struct ArticleServer {
    s: UnboundedSender<D>,
    r: UnboundedReceiver<S>,
}
pub struct ArticleClient {
    s: UnboundedSender<S>,
    r: UnboundedReceiver<D>,
}

impl Client for ArticleClient {
    type S = S;

    type D = D;

    fn send(&self, singal: Self::S) -> Result<(), tokio::sync::mpsc::error::SendError<Self::S>> {
        self.s.send(singal)
    }

    fn try_recv(&mut self) -> Result<Self::D, tokio::sync::mpsc::error::TryRecvError> {
        self.r.try_recv()
    }
}

pub fn make_service() -> (ArticleClient, Box<dyn Server>) {
    let Channels { c_s, c_r, s_s, s_r } = make_chan::<S, D>();
    let client = ArticleClient { s: c_s, r: c_r };
    let server = ArticleServer { s: s_s, r: s_r };
    (client, Box::new(server))
}

#[async_trait]
impl Server for ArticleServer {
    async fn block_on(&mut self) {
        tracing::info!("机核网文章资讯查询器启动正常");
        loop {
            match self.r.recv().await {
                None => {
                    // tracing::error!("发送方已关闭");
                }
                Some(_) => {
                    let res = api::fetch_articles().await;
                    if let Err(e) = self.s.send(res) {
                        tracing::error!("发送连接结果失败，GUI 可能停止工作：{}", e);
                        continue;
                    }
                    tracing::debug!("发送请求结果成功");
                }
            }
        }
    }
}
