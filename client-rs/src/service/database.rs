use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use async_trait::async_trait;
use sqlx::Row;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    RwLock,
};
pub mod datatype;
pub mod entity;
pub mod message;
pub mod sqls;

use crate::apps::database::{Field, DB};

use self::{
    datatype::{DataCell, DataType},
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
                        conn,
                        db,
                        table,
                        fields,
                        r#type,
                    } => {
                        handle_select(conns, s, conn, db, table, fields, r#type, sql).await;
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
    let conn = config.get_name();
    let mut result = None;
    let pool = sqlx::MySqlPool::connect(&url).await;
    match pool {
        Err(e) => {
            tracing::error!("目标数据库连接失败： {:?}", e);
        }
        Ok(p) => {
            if save == true {
                conns.write().await.insert(conn, p);
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
    conn: String,
    db: Option<String>,
    table: Option<String>,
    fields: Option<Box<Vec<Field>>>,
    r#type: SelectType,
    sql: String,
) {
    if let Some(pool) = conns.read().await.get(&conn) {
        if let Ok(rows) = sqlx::query(&sql).fetch_all(pool).await {
            if let Err(e) = match r#type {
                SelectType::Databases => {
                    let metas = rows
                        .iter()
                        .map(|x| {
                            let name: String = x.get(0);
                            (name.clone(), DB { name, tables: None })
                        })
                        .collect();
                    s.send(message::Response::Databases {
                        conn,
                        data: Box::new(metas),
                    })
                }
                SelectType::Tables => {
                    tracing::info!("查询数量 {}", rows.len());
                    let data: Vec<sqls::FieldMeta> = rows.iter().map(|x| x.into()).collect();
                    tracing::info!("总字段数：{}", data.len());
                    let mut map: BTreeMap<String, Vec<Field>> = BTreeMap::new();
                    for row in data.into_iter() {
                        let table_name = row.table_name.clone();
                        let table_name = table_name.as_str();
                        let field = Field {
                            datatype: row.get_type(),
                            details: row,
                        };
                        if map.contains_key(table_name) {
                            map.get_mut(table_name).unwrap().push(field);
                        } else {
                            map.insert(table_name.to_owned(), vec![field]);
                        }
                    }
                    for (db, fields) in map.iter() {
                        tracing::debug!("表名：{}  字段数量：{}", db, fields.len());
                        for field in fields {
                            tracing::trace!(
                                "名称： {}  类型：{}",
                                field.details.column_name,
                                field.details.column_type,
                            );
                        }
                    }
                    s.send(message::Response::Tables {
                        conn,
                        db: db.unwrap(),
                        data: Box::new(map),
                    })
                }
                SelectType::Table => {
                    if fields.is_none() || db.is_none() || table.is_none() {
                        return;
                    }
                    let fields = fields.unwrap();
                    let mut datas: Box<Vec<Vec<String>>> =
                        Box::new(vec![Vec::with_capacity(fields.len()); rows.len()]);
                    for col in 0..fields.len() {
                        for (i, row) in rows.iter().enumerate() {
                            let cell = DataCell::from_mysql_row(&row, col, &fields[col]);
                            datas[i].push(cell.to_string());
                        }
                    }
                    s.send(message::Response::DataRows {
                        conn,
                        db: db.unwrap(),
                        table: table.unwrap(),
                        datas,
                    })
                }
            } {
                tracing::error!("查询数据失败 {}", e);
            }
        } else {
            tracing::error!("查询数据失败");
        }
    } else {
        tracing::error!("获取数据库连接失败");
    }
}
#[cfg(test)]
mod test {

    #[tokio::test]
    async fn it_should_work() {
        let url = "mysql://tiangong2008:tiangong2008@www.91iedu.com:3391";
        let conn = sqlx::MySqlPool::connect(url).await.unwrap();

        let sql = "select * from tiangong2008.zqm_musics";

        use sqlx::mysql::MySqlTypeInfo;
        use sqlx::Column;
        use sqlx::Row;
        use sqlx::Type;
        use sqlx::TypeInfo;
        let res = sqlx::query(sql).fetch_one(&conn).await.unwrap();
        let columns = res.columns();
        for i in 0..columns.len() {
            let col = columns.get(i).unwrap();
            let type_info = col.type_info();
            println!("{:#?} {:#?}", col.name(), type_info.to_string());
        }
    }
}
