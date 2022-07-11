use sqlx::Pool;
use std::{collections::HashMap, sync::Arc, thread};
use tokio::sync::Mutex;

pub mod api;
pub mod article;
pub mod database;
pub mod entity;

use crate::{apps::database::Connection, service::api::fetch_articles};

use crate::service::database::{make_db_service, DatabaseClient};

use self::article::{make_article_service, ArticleClient};
pub struct Repo {
    pub article: ArticleClient,
    pub conn_manager: DatabaseClient,
}

pub struct Backend;

impl Backend {
    pub fn new() -> Repo {
        let (article_consumer, mut article_producer) = make_article_service();

        let (sql_sender, mut sql_executor) = make_db_service();
        let repo = Repo {
            article: article_consumer,
            conn_manager: sql_sender,
        };

        thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap();
            runtime.block_on(async move {
                let t1 = tokio::task::spawn(async move {
                    tracing::info!("机核网文章资讯查询器启动正常");
                    article_producer
                        .inner
                        .wait_produce(|| Box::pin(async { fetch_articles().await }))
                        .await;
                });

                let t3 = tokio::task::spawn(async move {
                    tracing::info!("SQL 执行器启动正常");
                    let conns: HashMap<String, Pool<sqlx::MySql>> = HashMap::new();
                    let conns = Arc::new(Mutex::new(conns));
                    let conns = Arc::clone(&conns);
                    loop {
                        match sql_executor.inner.try_recv().await {
                            None => {
                                tracing::error!("发送方已关闭");
                            }
                            Some((sql, save)) => {
                                let url = format!(
                                    "mysql://{}:{}@{}:{}/{}",
                                    &sql.username, &sql.password, &sql.ip, &sql.port, &sql.db
                                );
                                let key = sql.name.to_owned();
                                let mut res = Connection {
                                    config: sql,
                                    conn: None,
                                };
                                let pool = sqlx::MySqlPool::connect(&url).await;
                                if let Err(e) = pool {
                                    tracing::error!("目标数据库连接失败： {:?}", e);
                                    if let Err(e) = sql_executor.inner.send(res) {
                                        tracing::error!(
                                            "发送连接结果失败，GUI 可能停止工作：{}",
                                            e
                                        );
                                        continue;
                                    }
                                    tracing::debug!("回复成功");
                                    continue;
                                }
                                let pool = pool.unwrap();
                                if save == true {
                                    conns.lock().await.insert(key, pool);
                                    res.conn = Some(1);
                                }
                                tracing::info!("目标数据库连接成功");
                                if let Err(e) = sql_executor.inner.send(res) {
                                    tracing::error!("发送连接结果失败，GUI 可能停止工作：{}", e);
                                    continue;
                                }
                                tracing::debug!("回复成功");
                            }
                        }
                    }
                });

                let mut handlers = vec![];
                handlers.push(t1);
                handlers.push(t3);
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
