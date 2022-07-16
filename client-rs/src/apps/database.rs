mod config_new_conn;
mod table;
use crate::service::database::{entity::ConnectionConfig, DatabaseClient};
use crate::service::{
    database::{
        datatype::DataType,
        message,
        sqls::{self, FieldMeta},
    },
    Client,
};
use eframe::{
    egui::{self, RichText, ScrollArea},
    epaint::Color32,
};
use std::collections::BTreeMap;
use table::Table as TableComponent;

pub struct DataBase {
    state: String,
    conns: Conns,
    table: TableComponent,
    config_new_conn: config_new_conn::ConfigNewConnWindow,
    conn_manager: DatabaseClient,
}

#[derive(Clone, Debug)]
pub struct Conn {
    pub config: ConnectionConfig,
    pub conn: Option<usize>,
    pub databases: Option<Databases>,
}

pub type Conns = Box<BTreeMap<String, Conn>>;

#[derive(Clone, Debug)]
pub struct DB {
    pub name: String,
    pub tables: Option<Tables>,
}

pub type Databases = Box<BTreeMap<String, DB>>;

#[derive(Debug, Clone)]
pub struct Field {
    pub datatype: DataType,
    pub details: FieldMeta,
}
pub type Tables = Box<BTreeMap<String, Vec<Field>>>;

impl eframe::App for DataBase {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("数据库管理 top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.selectable_value(&mut self.state, "".to_string(), "数据查询");
                ui.selectable_value(&mut self.state, "".to_string(), "表结构");
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
        });

        egui::panel::TopBottomPanel::bottom("数据库管理 bottom").show(ctx, |ui| {
            //
            ui.label("状态栏：您当前正在观测的数据库是 XXX");
        });
    }
}

impl DataBase {
    pub fn get_conn_mut(&mut self, conn: &str) -> Option<&mut Conn> {
        self.conns.get_mut(conn)
    }

    pub fn get_db_mut(&mut self, conn: &str, db: &str) -> Option<&mut DB> {
        self.conns
            .get_mut(conn)
            .and_then(|conn| conn.databases.as_mut())
            .and_then(|database| database.get_mut(db))
    }

    pub fn get_db(&self, conn: &str, db: &str) -> Option<&DB> {
        self.conns
            .get(conn)
            .and_then(|conn| conn.databases.as_ref())
            .and_then(|database| database.get(db))
    }

    pub fn get_tables(&self, conn: &str, db: &str) -> Option<&Tables> {
        self.get_db(conn, db).and_then(|db| db.tables.as_ref())
    }

    pub fn get_fields(&self, conn: &str, db: &str, table: &str) -> Option<&Vec<Field>> {
        self.get_tables(conn, db)
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
            config_new_conn: config_new_conn::ConfigNewConnWindow::default(),
        }
    }

    fn render_conn(&self, ui: &mut egui::Ui) {
        for (conn_name, conn) in self.conns.iter() {
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
                                                    ui.horizontal(|ui| {
                                                        if ui
                                                            .button(&field.details.column_name)
                                                            .clicked()
                                                        {
                                                            //
                                                        }
                                                        ui.weak(RichText::new(
                                                            &field.details.column_type,
                                                        ));
                                                    });
                                                }
                                            });
                                        // if table_collapsing.header_response.double_clicked() {

                                        // }
                                        table_collapsing.header_response.context_menu(|ui| {
                                            if ui.button("刷新").clicked() {
                                                let fields = self
                                                    .get_fields(conn_name, db_name, table_name)
                                                    .and_then(|x| Some(x.to_owned()));
                                                let fields = Some(Box::new(fields.unwrap()));
                                                if let Err(e) = self.conn_manager.send(
                                                    message::Message::Select {
                                                        conn: conn.config.get_name(),
                                                        db: Some(db_name.to_string()),
                                                        table: Some(table_name.to_string()),
                                                        r#type: message::SelectType::Table,
                                                        sql: sqls::get_100_row(db_name, table_name),
                                                        fields,
                                                    },
                                                ) {
                                                    tracing::error!("查询数据表失败：{}", e);
                                                }
                                            }
                                        });
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

                            // 初始时，自动查询所有数据表（如果没查到，不应该继续）
                            // if init == true {

                            // }
                            db_collapsing.header_response.context_menu(|ui| {
                                egui::menu::bar(ui, |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.spacing();
                                        if ui.button("刷新").clicked() {
                                            if let Err(e) =
                                                self.conn_manager.send(message::Message::Select {
                                                    conn: conn.config.get_name(),
                                                    db: Some(db_name.to_string()),
                                                    table: None,
                                                    r#type: message::SelectType::Tables,
                                                    sql: sqls::get_table_meta(&db.name),
                                                    fields: None,
                                                })
                                            {
                                                tracing::error!("查询数据库失败：{}", e);
                                            }
                                        };
                                        ui.separator();
                                        if ui.button("删除").clicked() {};
                                    });
                                });
                            });
                        }
                    }
                },
            );

            // 数据库连接被点击时，触发查询所有连接
            // if conn_collapsing.header_response.clicked() && conn.databases.is_none() {}

            // if res.header_response.secondary_clicked() {
            conn_collapsing.header_response.context_menu(|ui| {
                egui::menu::bar(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.spacing();
                        if ui.button("刷新").clicked() {
                            if let Err(e) = self.conn_manager.send(message::Message::Select {
                                conn: conn.config.get_name(),
                                db: None,
                                table: None,
                                r#type: message::SelectType::Databases,
                                sql: sqls::get_databases(),
                                fields: None,
                            }) {
                                tracing::error!("查询数据库失败：{}", e);
                            }
                        };
                        ui.separator();
                        if ui.button("删除").clicked() {};
                    });
                });
            });
            // }
        }
    }

    fn handle_sql(&mut self) {
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
                    self.conns.insert(
                        config.get_name(),
                        Conn {
                            config,
                            conn: result,
                            databases: None,
                        },
                    );
                    self.config_new_conn.close();
                }
                message::Response::Databases { conn, data } => {
                    tracing::info!("查询所有数据库成功");
                    if let Some(conn) = self.get_conn_mut(&conn) {
                        conn.databases = Some(data)
                    }
                }
                message::Response::Tables { conn, db, data } => {
                    tracing::info!("查询数据表元数据成功！");
                    if let Some(db) = self.get_db_mut(&conn, &db) {
                        db.tables = Some(data);
                    }
                }
                message::Response::DataRows {
                    conn,
                    db,
                    table,
                    datas,
                } => {
                    tracing::info!("查询表数据成功！");
                    if let Some(fields) = self.get_fields(&conn, &db, &table) {
                        tracing::info!("列数：{}", fields.len());
                        let fields = Box::new(fields.to_owned());
                        self.table.update_content(fields, datas);
                    }
                }
            }
        }
    }
}
