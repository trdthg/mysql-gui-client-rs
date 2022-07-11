use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex,
};
pub mod entity;
use crate::apps::Connection;
use entity::ConnectionConfig;

use super::{make_chan, Channels, Client, Server};

type CONNS = Arc<Mutex<HashMap<String, sqlx::Pool<sqlx::MySql>>>>;

type S = (ConnectionConfig, bool);
type D = Connection;

pub struct DatabaseServer {
    s: UnboundedSender<D>,
    r: UnboundedReceiver<S>,
    pub conns: CONNS,
}

pub struct DatabaseClient {
    s: UnboundedSender<S>,
    r: UnboundedReceiver<D>,
}

impl Client for DatabaseClient {
    type S = S;

    type D = D;

    fn send(
        &self,
        singal: Self::S,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<(ConnectionConfig, bool)>> {
        self.s.send(singal)
    }

    fn try_recv(&mut self) -> Result<Self::D, tokio::sync::mpsc::error::TryRecvError> {
        self.r.try_recv()
    }
}

pub fn make_service() -> (DatabaseClient, Box<dyn Server>) {
    let Channels { c_s, c_r, s_s, s_r } = make_chan::<S, D>();
    let client = DatabaseClient { s: c_s, r: c_r };

    let conns = HashMap::new();
    let conns = Arc::new(Mutex::new(conns));
    let conns = Arc::clone(&conns);

    let server = DatabaseServer {
        s: s_s,
        r: s_r,
        conns,
    };
    (client, Box::new(server))
}

#[async_trait]
impl Server for DatabaseServer {
    async fn block_on(&mut self) {
        tracing::info!("SQL 执行器启动正常");
        loop {
            match self.r.recv().await {
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
                    if let Err(e) = self.s.send(res) {
                        tracing::error!("发送连接结果失败，GUI 可能停止工作：{}", e);
                        continue;
                    }
                    tracing::debug!("回复成功");
                }
            }
        }
    }
}
