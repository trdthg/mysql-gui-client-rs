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
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct DataBase {
    state: String,
    conns: Conns,
    table: TableComponent,
    config_new_conn: config_new_conn::ConfigNewConnWindow,
    // conn_manager: DatabaseClient,
    s: UnboundedSender<message::Message>,
    r: UnboundedReceiver<message::Response>,
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

// #[derive(Debug, Clone)]
// pub struct Field {
//     pub datatype: DataType,
//     pub details: FieldMeta,
// }

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub r#type: DataType,
    pub column_type: String,
}
impl Field {
    pub fn default_width(&self) -> f32 {
        self.r#type.get_default_width()
    }
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
                    // ➕
                    self.config_new_conn.open();
                };
            });
            self.config_new_conn.run(&self.s, ctx);
            self.handle_sql();
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    self.render_conn(ui);
                });
        });

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            self.table.update(ui);
        });

        // egui::panel::TopBottomPanel::bottom("表管理 bottom").show(ctx, |ui| {
        //     ui.horizontal(|ui| {
        //         ui.horizontal(|ui| {
        //             if ui.button("奇妙的东西").clicked() {};
        //             if ui.button("奇妙的东西").clicked() {};
        //             if ui.button("奇妙的东西").clicked() {};
        //         });
        //     });
        // });
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
            table: TableComponent::new(conn_manager.s.clone()),
            // conn_manager,
            config_new_conn: config_new_conn::ConfigNewConnWindow::default(),
            s: conn_manager.s.clone(),
            r: conn_manager.r,
        }
    }

    fn render_conn(&mut self, ui: &mut egui::Ui) {
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
                                                        if ui.button(&field.name).clicked() {
                                                            //
                                                        }
                                                        ui.weak(RichText::new(&field.column_type));
                                                    });
                                                }
                                            });
                                        // if table_collapsing.header_response.double_clicked() {

                                        // }
                                        table_collapsing.header_response.context_menu(|ui| {
                                            if ui.button("刷新").clicked() {
                                                let fields = &self
                                                    .get_fields(conn_name, db_name, table_name)
                                                    .and_then(|x| Some(x.to_owned()));
                                                let fields =
                                                    Some(Box::new(fields.to_owned().unwrap()));
                                                if let Err(e) =
                                                    self.s.send(message::Message::Select {
                                                        conn: conn.config.get_name(),
                                                        db: Some(db_name.to_string()),
                                                        table: Some(table_name.to_string()),
                                                        r#type: message::SelectType::Table,
                                                        sql: sqls::select_by_page(
                                                            db_name, table_name, None, None,
                                                        ),
                                                        fields,
                                                    })
                                                {
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
                                            let sql = sqls::get_table_meta(&db.name);
                                            if let Err(e) = self.s.send(message::Message::Select {
                                                conn: conn.config.get_name(),
                                                db: Some(db_name.to_string()),
                                                table: None,
                                                r#type: message::SelectType::Tables,
                                                sql,
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
                            if let Err(e) = self.s.send(message::Message::Select {
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
        if let Ok(v) = self.r.try_recv() {
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
                    self.table.update_conns(self.conns.clone()); // 更新
                }
                message::Response::Databases { conn, data } => {
                    tracing::info!("查询所有数据库成功");
                    if let Some(conn) = self.get_conn_mut(&conn) {
                        conn.databases = Some(data)
                    }
                    self.table.update_conns(self.conns.clone()); // 更新
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
                    sql,
                } => {
                    tracing::info!("查询表数据成功！");
                    if let Some(fields) = self.get_fields(&conn, &db, &table) {
                        tracing::info!("列数：{}", fields.len());
                        let fields = Box::new(fields.to_owned());

                        let meta = Box::new(table::TableMeta {
                            conn_name: conn,
                            db_name: db,
                            table_name: table,
                            fields,
                            datas,
                        });
                        self.table.update_sql(&sql);
                        self.table.update_content(meta);
                    }
                }
                message::Response::Customed { fields, datas } => {
                    let meta = Box::new(table::TableMeta {
                        conn_name: "".to_string(),
                        db_name: "".to_string(),
                        table_name: "".to_string(),
                        fields,
                        datas,
                    });
                    self.table.update_content(meta);
                }
            }
        }
    }
}
