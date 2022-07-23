use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use sqlx::{prelude::*, Execute};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    RwLock,
};
pub mod datatype;
pub mod message;
pub mod sqls;

use crate::frontend::database::types::{ColumnKey, Field, TableRows, DB};

use self::{
    datatype::{DataCell, DataType},
    message::{ConnectionConfig, Request, Response},
};

use super::{make_chan, Channels, Client, Server};

#[derive(Clone)]
pub struct Conns {
    inner: Arc<RwLock<BTreeMap<String, Conn>>>,
}

// type DBCONN = Arc<RwLock<BTreeMap<String, sqlx::MySqlPool>>>;

struct Conn {
    conn: sqlx::MySqlPool,
    _dbs: Vec<String>,
}
impl Conn {
    pub fn new(conn: sqlx::Pool<sqlx::MySql>) -> Self {
        Self { conn, _dbs: vec![] }
    }
}

impl Conns {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
    pub async fn get_pool(&self, conn_name: &str) -> Option<sqlx::MySqlPool> {
        self.inner
            .read()
            .await
            .get(conn_name)
            .and_then(|conn| Some(conn.conn.clone()))
    }

    // pub async fn get_db_pool(&self, conn_name: &str, db_name: &str) -> Option<sqlx::MySqlPool> {
    //     if let Some(dbs) = self
    //         .inner
    //         .read()
    //         .await
    //         .get(conn_name)
    //         .and_then(|conn| Some(conn.dbs.as_ref()))
    //     {
    //         dbs.read()
    //             .await
    //             .get(db_name)
    //             .and_then(|db| Some(db.clone()))
    //     } else {
    //         None
    //     }
    // }

    pub async fn insert_conn(&self, conn_name: String, conn: sqlx::Pool<sqlx::MySql>) {
        self.inner.write().await.insert(conn_name, Conn::new(conn));
    }

    // pub async fn insert_db_conn(&self, conn_name: &str, db_name: String, conn: sqlx::MySqlPool) {
    //     if let Some(dbs) = self
    //         .inner
    //         .read()
    //         .await
    //         .get(conn_name)
    //         .and_then(|conn| Some(conn.dbs.as_ref()))
    //     {
    //         dbs.write().await.insert(db_name, conn);
    //     }
    // }
}

type S = Request;
type D = Response;

pub struct DatabaseServer {
    s: UnboundedSender<D>,
    r: UnboundedReceiver<S>,
    pub conns: Conns,
}

pub struct DatabaseClient {
    pub s: UnboundedSender<S>,
    pub r: UnboundedReceiver<D>,
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

    let conns = Conns::new();
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
                    Request::Connect { config, save } => {
                        tokio::task::spawn(async move {
                            handle_connect(conns, s, config, save).await;
                        });
                    }
                    Request::SelectDatabases { conn } => {
                        let mut query_builder = sqlx::QueryBuilder::new(format!("show databases"));
                        let query = query_builder.build();
                        let sql = query.sql();
                        let pool = conns.get_pool(conn.as_str()).await;
                        if pool.is_none() {
                            tracing::error!("获取数据库连接失败");
                            continue;
                        }
                        let pool = pool.unwrap();
                        match query.fetch_all(&pool).await {
                            Ok(rows) => {
                                let metas = rows
                                    .iter()
                                    .map(|x| {
                                        let name: String = x.get(0);
                                        (name.clone(), DB { name, tables: None })
                                    })
                                    .collect();
                                s.send(message::Response::Databases { conn, data: metas });
                            }
                            Err(e) => {
                                let msg = format!("查询失败：{}", e);
                                tracing::error!(msg);
                            }
                        }
                    }
                    Request::SelectTables { conn, db } => {
                        let sql = sqls::get_table_meta(&db);
                        let mut query_builder = sqlx::QueryBuilder::new(format!("{}", sql));
                        let query = query_builder.build();
                        let sql = query.sql();
                        let pool = conns.get_pool(conn.as_str()).await;
                        if pool.is_none() {
                            tracing::error!("获取数据库连接失败");
                            continue;
                        }
                        let pool = pool.unwrap();
                        match query.fetch_all(&pool).await {
                            Ok(rows) => {
                                tracing::info!("查询数量 {}", rows.len());
                                let data: Vec<sqls::FieldMeta> =
                                    rows.iter().map(|x| x.into()).collect();
                                tracing::info!("总字段数：{}", data.len());
                                let mut map: BTreeMap<String, Vec<Field>> = BTreeMap::new();
                                for row in data.into_iter() {
                                    let table_name = row.table_name.clone();
                                    let table_name = table_name.as_str();
                                    let field = Field {
                                        name: row.column_name.to_owned(),
                                        r#type: row.get_type(),
                                        column_type: row.column_type,
                                        column_key: match row.column_key.as_deref() {
                                            Some("PRI") => ColumnKey::Primary,
                                            Some(_) => ColumnKey::None,
                                            None => ColumnKey::None,
                                        },
                                        is_nullable: if row.is_nullable == "YES" {
                                            true
                                        } else {
                                            false
                                        }, // 是否可以为空
                                           // datatype: row.get_type(),
                                           // details: row,
                                    };
                                    if map.contains_key(table_name) {
                                        map.get_mut(table_name).unwrap().push(field);
                                    } else {
                                        map.insert(table_name.to_owned(), vec![field]);
                                    }
                                }
                                // for (db, fields) in map.iter() {
                                //     tracing::debug!("表名：{}  字段数量：{}", db, fields.len());
                                //     for field in fields {
                                //         tracing::trace!("名称： {}  类型：{}", field.name, field.column_type,);
                                //     }
                                // }
                                s.send(message::Response::Tables {
                                    conn,
                                    db,
                                    data: Box::new(map),
                                });
                            }
                            Err(e) => {
                                let msg = format!("查询失败：{}", e);
                                tracing::error!(msg);
                            }
                        }
                    }
                    Request::SelectTable {
                        conn,
                        db,
                        table,
                        fields,
                        page,
                        size,
                        orders,
                    } => {
                        let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM ");
                        query_builder
                            .push(db.as_str())
                            .push(".")
                            .push(table.as_str());

                        if let Some(orders) = orders {
                            let arr: Vec<usize> = orders
                                .iter()
                                .enumerate()
                                .filter(|(_, x)| x.is_some())
                                .map(|(i, _)| i)
                                .collect();
                            if arr.len() > 0 {
                                query_builder
                                    .push(" ORDER BY ")
                                    .push(fields[arr[0]].name.as_str())
                                    .push(if *orders[arr[0]].as_ref().unwrap() {
                                        " ASC "
                                    } else {
                                        " DESC "
                                    });

                                let arr = &arr[1..];
                                for i in arr.iter() {
                                    query_builder
                                        .push(" , ")
                                        .push(fields[*i].name.as_str())
                                        .push(if *orders[arr[0]].as_ref().unwrap() {
                                            " ASC "
                                        } else {
                                            " DESC "
                                        });
                                }
                            }
                        }

                        query_builder
                            .push(" LIMIT ")
                            .push((page * size).to_string())
                            .push(", ")
                            .push(size.to_string());

                        let query = query_builder.build();
                        let sql = query.sql();
                        let pool = conns.get_pool(conn.as_str()).await;
                        if pool.is_none() {
                            tracing::error!("获取数据库连接失败");
                            continue;
                        }
                        let pool = pool.unwrap();
                        match query.fetch_all(&pool).await {
                            Ok(rows) => {
                                let mut datas: TableRows =
                                    Box::new(vec![Vec::with_capacity(fields.len()); rows.len()]);
                                for col in 0..fields.len() {
                                    for (i, row) in rows.iter().enumerate() {
                                        let cell = DataCell::from_mysql_row(
                                            &row,
                                            col,
                                            &fields[col].r#type,
                                            fields[col].is_nullable,
                                        );
                                        datas[i].push(cell.to_string());
                                    }
                                }
                                s.send(message::Response::DataRows {
                                    conn,
                                    db: db,
                                    table: table,
                                    datas,
                                    sql: sql.to_owned(),
                                });
                            }
                            Err(e) => {
                                let msg = format!("查询失败：{}", e);
                                tracing::error!(msg);
                            }
                        }
                    }
                    Request::SelectCustomed { conn, sql } => {
                        let mut query_builder = sqlx::QueryBuilder::new(format!("{}", sql));
                        let query = query_builder.build();
                        let sql = query.sql();
                        let pool = conns.get_pool(conn.as_str()).await;
                        if pool.is_none() {
                            tracing::error!("获取数据库连接失败");
                            continue;
                        }
                        let pool = pool.unwrap();
                        match query.fetch_all(&pool).await {
                            Ok(rows) => {
                                use sqlx::{Column, TypeInfo};
                                let columns = rows[0].columns();
                                let mut datas: TableRows =
                                    Box::new(vec![Vec::with_capacity(columns.len()); rows.len()]);

                                let mut fields = Box::new(Vec::with_capacity(columns.len()));
                                for (col, field) in columns.iter().enumerate() {
                                    let field_name = field.name();
                                    let field_type =
                                        DataType::from_uppercase(field.type_info().name());
                                    for (i, row) in rows.iter().enumerate() {
                                        let cell =
                                            DataCell::from_mysql_row(&row, col, &field_type, true);
                                        datas[i].push(cell.to_string());
                                    }
                                    fields.push(Field {
                                        name: field_name.to_owned(),
                                        column_type: field_type.to_string(),
                                        column_key: ColumnKey::None,
                                        r#type: field_type,
                                        is_nullable: true,
                                    });
                                }
                                s.send(message::Response::Customed { fields, datas });
                            }
                            Err(e) => {
                                let msg = format!("查询失败：{}", e);
                                tracing::error!(msg);
                            }
                        }
                    }
                    Request::Delete {
                        conn,
                        db,
                        table,
                        fields,
                        datas,
                    } => {
                        // tracing::debug!("SQL 构建完毕");
                        let mut query_builder =
                            sqlx::QueryBuilder::new(format!("delete from {}.{} where ", db, table));
                        let mut arr = vec![];
                        for (i, field) in fields.iter().enumerate() {
                            if matches!(field.column_key, ColumnKey::Primary) {
                                arr.push(i)
                            }
                        }

                        if arr.len() > 0 {
                            let field = &fields[arr[0]];
                            query_builder.push(fields[arr[0]].name.as_str()).push(" = ");
                            if let Err(e) = datatype::query_push_bind(
                                &mut query_builder,
                                datas[arr[0]].as_deref().unwrap(),
                                &field.r#type,
                            ) {
                                if let Err(e) = s.send(message::Response::Delete {
                                    n: 0,
                                    msg: format!("构造 DELETE 语句失败：{}", e),
                                    sql: "".to_string(),
                                }) {
                                    tracing::error!("发送删除结果失败：{}", e);
                                }
                            }
                            let arr = &arr[1..];
                            for i in arr {
                                let field = &fields[i.to_owned()];
                                query_builder
                                    .push(" AND ")
                                    .push(field.name.as_str())
                                    .push(" = ");
                                if let Err(e) = datatype::query_push_bind(
                                    &mut query_builder,
                                    datas[*i].as_deref().unwrap(),
                                    &field.r#type,
                                ) {
                                    if let Err(e) = s.send(message::Response::Delete {
                                        n: 0,
                                        msg: format!("构造 DELETE 语句失败：{}", e),
                                        sql: "".to_string(),
                                    }) {
                                        tracing::error!("发送删除结果失败：{}", e);
                                    }
                                }
                            }
                        }
                        // tracing::debug!("SQL 构建完毕");
                        let query = query_builder.build();
                        use sqlx::Execute;
                        let sql = query.sql();
                        let pool = conns.get_pool(conn.as_str()).await;
                        if pool.is_none() {
                            tracing::error!("获取数据库连接失败");
                            return;
                        }
                        let pool = pool.unwrap();
                        if let Err(e) = match query.execute(&pool).await {
                            Ok(res) => {
                                let msg = format!("删除了 {} 行", res.rows_affected());
                                tracing::info!(msg);
                                s.send(message::Response::Delete {
                                    n: res.rows_affected(),
                                    msg: msg,
                                    sql: sql.to_string(),
                                })
                            }
                            Err(e) => {
                                let msg = format!("删除失败：{}", e);
                                tracing::error!(msg);
                                s.send(message::Response::Delete {
                                    n: 0,
                                    msg: msg,
                                    sql: sql.to_string(),
                                })
                            }
                        } {
                            tracing::error!("返回删除结果失败：{}", e);
                        }
                    }
                    Request::Insert {
                        conn,
                        db,
                        table,
                        fields,
                        datas,
                    } => {
                        let mut query_builder =
                            sqlx::QueryBuilder::new(format!("insert into {}.{} (", db, table));

                        // 找到不为空的参数的索引
                        let mut arr = vec![];
                        for i in 0..fields.len() {
                            if datas[i].is_some() {
                                arr.push(i)
                            }
                        }

                        for (i, col_index) in arr.iter().enumerate() {
                            let field = &fields[*col_index];
                            query_builder.push(field.name.as_str());
                            if i != arr.len() - 1 {
                                query_builder.push(", ");
                            }
                        }
                        query_builder.push(" ) VALUES ( ");
                        for (i, col_index) in arr.iter().enumerate() {
                            if let Err(e) = datatype::query_push_bind(
                                &mut query_builder,
                                datas[*col_index].to_owned().unwrap().as_str(),
                                &fields[*col_index].r#type,
                            ) {
                                let msg = format!("构建 Insert 语句失败：{}", e);
                                tracing::error!("{}", msg);
                                if let Err(e) = s.send(message::Response::Insert {
                                    n: 0,
                                    msg,
                                    sql: String::new(),
                                }) {
                                    tracing::error!("返回插入构建失败失败：{}", e);
                                }
                                continue;
                            } else {
                                if i != arr.len() - 1 {
                                    query_builder.push(", ");
                                }
                            }
                        }
                        query_builder.push(" )");
                        let query = query_builder.build();
                        tracing::info!("SQL 构建完毕");

                        let sql = query.sql();
                        // tracing::info!("{}", sql);
                        let pool = conns.get_pool(conn.as_str()).await;
                        if pool.is_none() {
                            tracing::error!("获取数据库连接失败");
                            return;
                        }
                        let pool = pool.unwrap();
                        if let Err(e) = match query.execute(&pool).await {
                            Ok(res) => {
                                let msg = format!("插入了 {} 行", res.rows_affected());
                                tracing::info!(msg);
                                s.send(message::Response::Insert {
                                    n: res.rows_affected(),
                                    msg,
                                    sql: sql.to_string(),
                                })
                            }
                            Err(e) => {
                                let msg = format!("插入失败：{}", e);
                                tracing::error!(msg);
                                s.send(message::Response::Insert {
                                    n: 0,
                                    msg,
                                    sql: sql.to_string(),
                                })
                            }
                        } {
                            tracing::error!("返回插入结果失败：{}", e);
                        }
                    }
                    Request::Update {
                        conn,
                        db,
                        table,
                        fields,
                        datas,
                        new_data_index,
                        new_data,
                    } => {
                        // tracing::debug!("SQL 构建完毕");
                        let mut query_builder =
                            sqlx::QueryBuilder::new(format!("UPDATE {}.{} SET ", db, table));
                        query_builder
                            .push(fields[new_data_index].name.as_str())
                            .push(" = ");
                        if let Some(new_data) = new_data.as_deref() {
                            datatype::query_push_bind(
                                &mut query_builder,
                                new_data,
                                &fields[new_data_index].r#type,
                            );
                        } else {
                            query_builder.push(" NULL ");
                        }
                        query_builder.push(" WHERE ");

                        let mut arr = vec![];
                        for (i, field) in fields.iter().enumerate() {
                            if matches!(field.column_key, ColumnKey::Primary) {
                                arr.push(i)
                            }
                        }

                        if arr.len() > 0 {
                            let field = &fields[arr[0]];
                            query_builder.push(fields[arr[0]].name.as_str()).push(" = ");
                            datatype::query_push_bind(
                                &mut query_builder,
                                datas[arr[0]].to_owned().unwrap().as_str(),
                                &field.r#type,
                            );
                            let arr = &arr[1..];
                            for i in arr {
                                let field = &fields[i.to_owned()];
                                query_builder
                                    .push(" AND ")
                                    .push(field.name.as_str())
                                    .push(" = ");
                                datatype::query_push_bind(
                                    &mut query_builder,
                                    datas[*i].to_owned().unwrap().as_str(),
                                    &field.r#type,
                                );
                            }
                        }
                        // tracing::debug!("SQL 构建完毕");
                        let query = query_builder.build();
                        use sqlx::Execute;
                        let sql = query.sql();
                        let pool = conns.get_pool(conn.as_str()).await;
                        if pool.is_none() {
                            tracing::error!("获取数据库连接失败");
                            return;
                        }
                        let pool = pool.unwrap();
                        if let Err(e) = match query.execute(&pool).await {
                            Ok(res) => {
                                let msg = format!("更新了 {} 行", res.rows_affected());
                                tracing::info!(msg);
                                s.send(message::Response::Delete {
                                    n: res.rows_affected(),
                                    msg,
                                    sql: sql.to_string(),
                                })
                            }
                            Err(e) => {
                                let msg = format!("更新失败：{}", e);
                                tracing::error!(msg);
                                s.send(message::Response::Update {
                                    n: 0,
                                    msg: msg,
                                    sql: sql.to_string(),
                                })
                            }
                        } {
                            tracing::error!("返回更新结果失败：{}", e);
                        }
                    }
                };
            } else {
                // tracing::error!("发送方已关闭");
            }
        }
    }
}

async fn handle_connect(conns: Conns, s: UnboundedSender<D>, config: ConnectionConfig, save: bool) {
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
                conns.insert_conn(conn, p).await;
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

#[allow(unused)]
#[cfg(test)]
mod test {

    #[tokio::test]
    async fn it_should_work() {
        let url = "mysql://tiangong2008:tiangong2008@www.91iedu.com:3391";
        let conn = sqlx::MySqlPool::connect(url).await.unwrap();

        let sql = "select * from tiangong2008.zqm_musics";

        use sqlx::Column;
        use sqlx::Row;
        let res = sqlx::query(sql).fetch_one(&conn).await.unwrap();
        let columns = res.columns();
        for i in 0..columns.len() {
            let col = columns.get(i).unwrap();
            let type_info = col.type_info();
            println!("{:#?} {:#?}", col.name(), type_info.to_string());
        }
    }

    #[tokio::test]
    async fn use_db() {
        use sqlx::prelude::*;
        let url = "mysql://tiangong2008:tiangong2008@www.91iedu.com:3391";
        let conn = sqlx::MySqlPool::connect(url).await.unwrap();

        // let sql = "select * from tiangong2008.zqm_musics";
        // let res = sqlx::query(sql).fetch_one(&conn).await.unwrap();
        // conn.begin().await.unwrap();
        // conn.prepare(sqlx::query("use tiangong2008")).await.unwrap();
        conn.execute(sqlx::query("DELETE FROM ymz_movie_top250"))
            .await
            .unwrap();
        // 不支持 use 语句
    }

    #[tokio::test]
    async fn join_type() {
        let url = "mysql://tiangong2008:tiangong2008@www.91iedu.com:3391";
        let conn = sqlx::MySqlPool::connect(url).await.unwrap();

        let sql = r#"
            SELECT *
                FROM ymz_dd_info , ymz_dd_tags
            WHERE tiangong2008.ymz_dd_info.id = tiangong2008.ymz_dd_tags.id
                AND ymz_dd_info.id = 1231
            LIMIT 0,100
        "#;

        // let res = sqlx::query(sql).fetch(&conn);
        // let columns = res.columns();
    }

    #[tokio::test]
    async fn group_type() {
        let url = "mysql://tiangong2008:tiangong2008@www.91iedu.com:3391/tiangong2008";
        let conn = sqlx::MySqlPool::connect(url).await.unwrap();
        // 不支持 use 语句
        // sqlx::query("use tiangong2008; ")
        //     .execute(&conn)
        //     .await
        //     .unwrap();
        let sql = "select * from zqm_musics;";

        use sqlx::Column;
        use sqlx::Row;
        use sqlx::Type;
        use sqlx::TypeInfo;
        let res = sqlx::query(sql).fetch_one(&conn).await.unwrap();
        let columns = res.columns();
        for i in 0..columns.len() {
            let col = columns.get(i).unwrap();
            let type_info = col.type_info();
            println!("{:#?} {:#?}", col.name(), type_info.name());
        }
    }

    #[tokio::test]
    async fn show_databases_type() {
        let url = "mysql://tiangong2008:tiangong2008@www.91iedu.com:3391/tiangong2008";
        let conn = sqlx::MySqlPool::connect(url).await.unwrap();
        let sql = "show databases;";

        use sqlx::Column;
        use sqlx::Row;
        use sqlx::Type;
        use sqlx::TypeInfo;
        let res = sqlx::query(sql).fetch_one(&conn).await.unwrap();
        let columns = res.columns();
        for i in 0..columns.len() {
            let col = columns.get(i).unwrap();
            let type_info = col.type_info();
            println!("{:#?} {:#?}", col.name(), type_info.name());
        }
    }
}
