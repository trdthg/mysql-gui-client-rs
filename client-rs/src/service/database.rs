use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    RwLock,
};
pub mod datatype;
pub mod entity;
pub mod message;
pub mod sqls;

use self::{
    entity::ConnectionConfig,
    message::{Message, Response, SelectType},
};

use super::{make_chan, Channels, Client, Server};

type CONNS = Arc<RwLock<HashMap<String, sqlx::Pool<sqlx::MySql>>>>;

type S = Message;
type D = Response;

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

    fn send(&self, singal: Self::S) -> Result<(), tokio::sync::mpsc::error::SendError<Self::S>> {
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
    let conns = Arc::new(RwLock::new(conns));
    let conns = Arc::clone(&conns);

    let server = DatabaseServer {
        s: s_s,
        r: s_r,
        conns,
    };
    (client, Box::new(server))
}

impl DatabaseServer {}

#[async_trait]
impl Server for DatabaseServer {
    async fn block_on(&mut self) {
        tracing::info!("SQL 执行器启动正常");
        loop {
            if let Some(msg) = self.r.recv().await {
                let s = self.s.clone();
                let conns = self.conns.clone();
                match msg {
                    Message::Connect { config, save } => {
                        tokio::task::spawn(async move {
                            handle_connect(conns, s, config, save).await;
                        });
                    }
                    Message::Select {
                        sql,
                        key,
                        db,
                        table,
                        r#type,
                    } => {
                        handle_select(conns, s, key, db, table, r#type, sql).await;
                    }
                };
            } else {
                tracing::error!("发送方已关闭");
            }
        }
    }
}

async fn handle_connect(conns: CONNS, s: UnboundedSender<D>, config: ConnectionConfig, save: bool) {
    let url = config.get_url();
    let key = config.get_name();
    let mut result = None;
    let pool = sqlx::MySqlPool::connect(&url).await;
    match pool {
        Err(e) => {
            tracing::error!("目标数据库连接失败： {:?}", e);
        }
        Ok(p) => {
            if save == true {
                conns.write().await.insert(key, p);
                result = Some(1);
            }
            tracing::info!("目标数据库连接成功");
        }
    }
    if let Err(e) = s.send(Response::NewConn {
        config,
        save,
        result,
    }) {
        tracing::error!("发送连接结果失败，GUI 可能停止工作：{}", e);
    } else {
        tracing::debug!("回复成功");
    }
}

async fn handle_select(
    conns: CONNS,
    s: UnboundedSender<D>,
    key: String,
    db: Option<String>,
    table: Option<String>,
    r#type: SelectType,
    sql: String,
) {
    let mut conn = conns
        .read()
        .await
        .get(&key)
        .unwrap()
        .acquire()
        .await
        .unwrap();
    match r#type {
        SelectType::Databases => {
            let rows: Vec<sqlx::mysql::MySqlRow> =
                sqlx::query(&sql).fetch_all(&mut conn).await.unwrap();
            if let Err(e) = s.send(message::Response::Databases { key, data: rows }) {
                tracing::error!("返回数据失败：{}", e);
            }
        }
        SelectType::Tables => {
            let rows: Vec<sqlx::mysql::MySqlRow> =
                sqlx::query(&sql).fetch_all(&mut conn).await.unwrap();
            tracing::info!("查询数量 {}", rows.len());
            if let Err(e) = s.send(message::Response::Tables {
                key,
                db: db.unwrap(),
                data: rows,
            }) {
                tracing::error!("返回数据失败：{}", e);
            }
        }
        SelectType::Table => match sqlx::query(&sql).fetch_all(&mut conn).await {
            Ok(rows) => {
                if let Err(e) = s.send(message::Response::DataRows {
                    key,
                    table: table.unwrap(),
                    db: db.unwrap(),
                    data: rows,
                }) {
                    tracing::error!("返回数据失败：{}", e);
                }
            }
            Err(e) => {
                tracing::error!("查询失败：{}", e);
            }
        },
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn it_should_work() {
        let conn = sqlx::MySqlPool::connect("").await.unwrap();
        use sqlx::Arguments;
        let mut args = sqlx::mysql::MySqlArguments::default();
        args.add(5);
        args.add("foo");
        let sql = "insert into abc (a,b) values ($1, $2)";
        let query = sqlx::query(sql);
        let mut conn = conn.acquire().await.unwrap();
        let _: Vec<sqlx::mysql::MySqlRow> = query.fetch_all(&mut conn).await.unwrap();
    }
}
