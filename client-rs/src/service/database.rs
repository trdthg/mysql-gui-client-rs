use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;
pub mod entity;
use crate::apps::Connection;
use crate::util::duplex_channel;
use crate::util::duplex_channel::{DuplexConsumer, DuplexProducer};
use entity::ConnectionConfig;

use super::Server;

type CONNS = Arc<Mutex<HashMap<String, sqlx::Pool<sqlx::MySql>>>>;
pub struct DatabaseServer {
    pub inner: DuplexProducer<(ConnectionConfig, bool), Connection>,
    pub conns: CONNS,
}

pub struct DatabaseClient {
    pub inner: DuplexConsumer<(ConnectionConfig, bool), Connection>,
}

pub fn make_service<'a>() -> (DatabaseClient, Box<dyn Server>) {
    let (sql_sender, sql_executor) =
        duplex_channel::channel::<(ConnectionConfig, bool), Connection>();
    let client = DatabaseClient { inner: sql_sender };

    let conns = HashMap::new();
    let conns = Arc::new(Mutex::new(conns));
    let conns = Arc::clone(&conns);

    let server = DatabaseServer {
        inner: sql_executor,
        conns,
    };
    (client, Box::new(server))
}

#[async_trait]
impl Server for DatabaseServer {
    async fn block_on(&mut self) {
        tracing::info!("SQL 执行器启动正常");
        loop {
            match self.inner.try_recv().await {
                None => {
                    tracing::error!("发送方已关闭");
                }
                Some((sql, save)) => {
                    let url = sql.to_url();
                    let key = sql.name.to_owned();
                    let mut res = Connection {
                        config: sql,
                        conn: None,
                    };
                    let pool = sqlx::MySqlPool::connect(&url).await;
                    match pool {
                        Err(e) => {
                            tracing::error!("目标数据库连接失败： {:?}", e);
                        }
                        Ok(p) => {
                            if save == true {
                                self.conns.lock().await.insert(key, p);
                                res.conn = Some(1);
                            }
                            tracing::info!("目标数据库连接成功");
                        }
                    }
                    if let Err(e) = self.inner.send(res) {
                        tracing::error!("发送连接结果失败，GUI 可能停止工作：{}", e);
                        continue;
                    }
                    tracing::debug!("回复成功");
                }
            }
        }
    }
}
