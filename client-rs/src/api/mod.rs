use std::{collections::HashMap, sync::Mutex, thread};

pub mod article;
pub mod mysql;
pub use article::fetch_articles;
use sqlx::Pool;

use crate::{
    pages::database::database::Connection,
    util::duplex_channel::{self, DuplexConsumer},
};

use self::{article::NewsArticle, mysql::ConnectionConfig};
pub struct Repo {
    pub article: DuplexConsumer<(), Vec<NewsArticle>>,
    pub conn_manager: Option<DuplexConsumer<ConnectionConfig, Connection>>,
}

impl Repo {
    pub(crate) fn new() -> Repo {
        let (consumer, producer) = duplex_channel::channel::<(), Vec<NewsArticle>>();
        let (sql_sender, sql_executor) = duplex_channel::channel::<ConnectionConfig, Connection>();

        let repo = Repo {
            article: consumer,
            conn_manager: Some(sql_sender),
        };

        thread::spawn(move || {
            let mut producer = producer.take();
            let mut sql_executor = sql_executor;
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap();
            runtime.block_on(async move {
                let t1 = tokio::task::spawn(async move {
                    tracing::info!("机核网文章资讯查询器启动正常");
                    // 不断等待请求更新
                    producer
                        .wait_produce(|| Box::pin(async { fetch_articles().await }))
                        .await;
                });

                let t2 = tokio::task::spawn(async move {
                    // 不断等待请求更新
                    tracing::info!("SQL 执行器启动正常");
                    let conns: HashMap<usize, Pool<sqlx::MySql>> = HashMap::new();
                    let conns = Mutex::new(conns);
                    sql_executor
                        .wait_handle_produce(|sql| {
                            Box::pin(async {
                                // fetch_articles().await
                                let url = format!(
                                    "mysql://{}:{}@{}:{}/{}",
                                    &sql.username, &sql.password, &sql.ip, &sql.port, &sql.db
                                );
                                let mut res = Connection {
                                    config: sql,
                                    conn: None,
                                };
                                match sqlx::MySqlPool::connect(&url).await {
                                    Err(e) => tracing::error!("目标数据库连接失败： {}", e),
                                    Ok(p) => {
                                        conns.lock().unwrap().insert(1, p);
                                        res.conn = Some(1);
                                        tracing::info!("目标数据库连接成功");
                                    }
                                }
                                res
                            })
                        })
                        .await;
                });

                let mut handlers = vec![];
                handlers.push(t1);
                handlers.push(t2);
                for handler in handlers {
                    if let Err(e) = handler.await {
                        tracing::error!("tokio 任务执行失败：{}", e);
                    }
                }
            });
        });
        // producer;

        repo
    }
}
