pub mod article;
pub mod database;
pub mod talk;

use article::ArticleClient;
use async_trait::async_trait;
use database::DatabaseClient;
use std::thread;

#[async_trait]
pub trait Server: Send + Sync + 'static {
    async fn block_on(&mut self);
}
pub struct Repo {
    pub article: ArticleClient,
    pub conn_manager: DatabaseClient,
}

pub struct Backend {
    servers: Vec<Box<dyn Server>>,
}

impl Backend {
    pub fn new() -> (Self, Repo) {
        let (article_consumer, article_producer) = article::make_service();
        let (sql_sender, sql_executor) = database::make_service();
        let repo = Repo {
            article: article_consumer,
            conn_manager: sql_sender,
        };
        let servers = vec![article_producer, sql_executor];

        (Self { servers }, repo)
    }

    pub fn run(self) {
        thread::spawn(|| {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap();
            runtime.block_on(async move {
                let mut handlers = vec![];
                for mut producer in self.servers {
                    let t = tokio::task::spawn(async move {
                        producer.block_on().await;
                    });
                    handlers.push(t);
                }
                for handler in handlers {
                    if let Err(e) = handler.await {
                        tracing::error!("tokio 任务执行失败：{}", e);
                    }
                }
            });
        });
    }
}
