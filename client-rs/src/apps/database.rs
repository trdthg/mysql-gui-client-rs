mod component;
pub mod table;
mod tabwindow;
use std::{collections::BTreeMap, time::Duration};

use eframe::{
    egui::{self, RichText, ScrollArea},
    epaint::Color32,
};

use crate::service::{
    database::{
        datatype::DataType,
        message,
        sqls::{self, TableMeta},
    },
    Client,
};

use table::Table as TableComponent;

use crate::service::database::{entity::ConnectionConfig, DatabaseClient};

use self::tabwindow::TabWindow;

pub struct DataBase {
    state: String,
    conns: Conns,
    table: TableComponent,
    tabs: TabWindow,
    config_new_conn: component::ConfigNewConnWindow,
    conn_manager: DatabaseClient,
}

#[derive(Clone, Debug, Default)]
pub struct Conns {
    pub inner: BTreeMap<String, Conn>,
}

#[derive(Clone, Debug)]
pub struct Conn {
    pub config: ConnectionConfig,
    pub conn: Option<usize>,
    pub databases: Option<BTreeMap<String, DB>>,
}

type Tables = BTreeMap<String, Vec<Field>>;

#[derive(Clone, Debug)]
pub struct DB {
    name: String,
    tables: Option<Tables>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub datatype: DataType,
    pub details: TableMeta,
}

impl eframe::App for DataBase {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("数据库管理 top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.selectable_value(&mut self.state, "".to_string(), "数据管理");
                ui.selectable_value(&mut self.state, "".to_string(), "监控");
            });
        });

        egui::SidePanel::left("数据库管理 sidebar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("DATABASE");
                if ui.button("+").clicked() {
                    self.config_new_conn.open();
                };
            });
            self.config_new_conn.run(&self.conn_manager, ctx);
            self.handle_sql();
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    self.render_conn(ui);
                });
        });

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            self.table.update(ctx, frame);
            // self.tabs.run(ctx);
        });

        egui::panel::TopBottomPanel::bottom("数据库管理 bottom").show(ctx, |ui| {
            //
            ui.label("状态栏：您当前正在观测的数据库是 XXX");
        });
    }
}

impl DataBase {
    pub fn get_conn_mut(&mut self, key: &str) -> Option<&mut Conn> {
        self.conns.inner.get_mut(key)
    }

    pub fn get_db_mut(&mut self, key: &str, db: &str) -> Option<&mut DB> {
        self.conns
            .inner
            .get_mut(key)
            .and_then(|conn| conn.databases.as_mut())
            .and_then(|database| database.get_mut(db))
    }

    pub fn get_db(&self, key: &str, db: &str) -> Option<&DB> {
        self.conns
            .inner
            .get(key)
            .and_then(|conn| conn.databases.as_ref())
            .and_then(|database| database.get(db))
    }

    pub fn get_tables(&self, key: &str, db: &str) -> Option<&Tables> {
        self.get_db(key, db).and_then(|db| db.tables.as_ref())
    }

    pub fn get_fields(&self, key: &str, db: &str, table: &str) -> Option<&Vec<Field>> {
        self.get_tables(key, db)
            .and_then(|tables| tables.get(table))
    }
}

impl DataBase {
    pub fn new(conn_manager: DatabaseClient) -> Self {
        Self {
            conns: Conns::default(),
            state: "aaa".into(),
            table: Default::default(),
            conn_manager,
            config_new_conn: component::ConfigNewConnWindow::default(),
            tabs: TabWindow::default(),
        }
    }

    fn render_conn(&self, ui: &mut egui::Ui) {
        for (key, conn) in self.conns.inner.iter() {
            if conn.conn.is_none() {
                ui.label(format!("{}", conn.config.get_name()));
                ui.colored_label(Color32::RED, format!("{}", conn.config.get_name()));
                continue;
            }

            // 数据库连接
            let conn_collapsing = ui.collapsing(
                RichText::new(&conn.config.get_name()).color(Color32::GREEN),
                |ui| {
                    if let Some(databases) = &conn.databases {
                        for (db_name, db) in databases.iter() {
                            // 数据库
                            let db_collapsing = ui.collapsing(RichText::new(db_name), |ui| {
                                if let Some(tables) = &db.tables {
                                    for (table_name, table) in tables.iter() {
                                        // 数据表
                                        let table_collapsing =
                                            ui.collapsing(RichText::new(table_name), |ui| {
                                                // 各字段
                                                for field in table.iter() {
                                                    if ui
                                                        .button(&field.details.column_name)
                                                        .clicked()
                                                    {
                                                    }
                                                }
                                            });
                                        if table_collapsing.header_response.double_clicked() {
                                            if let Err(e) =
                                                self.conn_manager.send(message::Message::Select {
                                                    key: conn.config.get_name(),
                                                    db: Some(db_name.to_string()),
                                                    table: Some(table_name.to_string()),
                                                    r#type: message::SelectType::Table,
                                                    sql: sqls::get_100_row(db_name, table_name),
                                                })
                                            {
                                                tracing::error!("查询数据库失败：{}", e);
                                            }
                                        }
                                    }
                                }
                                ui.collapsing("其他", |ui| {
                                    ui.collapsing(RichText::new("views"), |ui| {
                                        ui.collapsing(RichText::new("Student"), |ui| {
                                            ui.label("P: id");
                                            if ui.button("P: id").clicked() {}
                                            if ui.button("N: name").clicked() {}
                                            if ui.button("N: age").clicked() {}
                                        });
                                    });
                                });
                            });

                            // 数据库被点击时，触发查询所有数据表
                            if db_collapsing.header_response.clicked() && db.tables.is_none() {
                                if let Err(e) = self.conn_manager.send(message::Message::Select {
                                    key: conn.config.get_name(),
                                    db: Some(db_name.to_string()),
                                    table: None,
                                    r#type: message::SelectType::Tables,
                                    sql: sqls::get_table_meta(&db.name),
                                }) {
                                    tracing::error!("查询数据库失败：{}", e);
                                }
                            }
                        }
                    }
                },
            );

            // 数据库连接被点击时，触发查询所有连接
            if conn_collapsing.header_response.clicked() && conn.databases.is_none() {
                if let Err(e) = self.conn_manager.send(message::Message::Select {
                    key: conn.config.get_name(),
                    db: None,
                    table: None,
                    r#type: message::SelectType::Databases,
                    sql: sqls::get_databases(),
                }) {
                    tracing::error!("查询数据库失败：{}", e);
                }
            }

            // if res.header_response.secondary_clicked() {
            conn_collapsing.header_response.context_menu(|ui| {
                egui::menu::bar(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.spacing();
                        if ui.button("查看详细信息").clicked() {};
                        ui.separator();
                        if ui.button("删除链接").clicked() {};
                        ui.separator();
                        if ui.button("测试文本长度度的点点滴滴的点点滴滴的点点滴滴的点点滴滴的点点滴滴单打独斗").clicked() {};
                    });
                });
            });
            // }
        }
    }

    fn handle_sql(&mut self) {
        use sqlx::Row;
        if let Ok(v) = self.conn_manager.try_recv() {
            match v {
                message::Response::NewConn {
                    config,
                    save,
                    result,
                } => {
                    tracing::info!("连接数据库成功！");
                    if save == false {
                        return;
                    }
                    self.conns.inner.insert(
                        config.get_name(),
                        Conn {
                            config,
                            conn: result,
                            databases: None,
                        },
                    );
                    self.config_new_conn.close();
                }
                message::Response::Databases { key, data } => {
                    tracing::info!("查询所有数据库成功");
                    let metas = data
                        .iter()
                        .map(|x| {
                            let name: String = x.get(0);
                            (name.clone(), DB { name, tables: None })
                        })
                        .collect();
                    if let Some(conn) = self.get_conn_mut(&key) {
                        conn.databases = Some(metas)
                    }
                }
                message::Response::Tables { key, db, data } => {
                    tracing::info!("查询数据表元数据成功！");
                    let data: Vec<TableMeta> = data.iter().map(|x| x.into()).collect();
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
                    if let Some(db) = self.get_db_mut(&key, &db) {
                        db.tables = Some(map);
                    }
                }
                message::Response::DataRows {
                    key,
                    db,
                    table,
                    data,
                } => {
                    tracing::info!("查询表数据成功！");
                    if let Some(fields) = self.get_fields(&key, &db, &table) {
                        for field in fields.iter() {
                            println!(
                                "{} {}",
                                field.details.column_name, field.details.column_type
                            );
                        }
                        tracing::info!("列数：{}", fields.len());
                        self.table.update_content(fields.to_owned(), data);
                    }
                }
            }
        }
    }
}
