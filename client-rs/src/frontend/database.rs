mod config_new_conn;
mod sidebar;
mod table;
pub mod types;
use crate::backend::database::{message, DatabaseClient};
use eframe::egui::{self, RichText, ScrollArea};
use table::Table as TableComponent;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use self::{sidebar::SideBar, types::Conns};

pub struct DataBase {
    state: String,
    conns: Conns,
    sidebar: SideBar,
    table: TableComponent,
    config_new_conn: config_new_conn::ConfigNewConnWindow,
    s: UnboundedSender<message::Message>,
    r: UnboundedReceiver<message::Response>,
}

impl eframe::App for DataBase {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("数据库管理 top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("💻 侧边栏").clicked() {
                    self.sidebar.toggle();
                }
                ui.selectable_value(&mut self.state, "".to_string(), "数据查询");
                ui.selectable_value(&mut self.state, "".to_string(), "表结构");
            });
        });

        egui::panel::TopBottomPanel::bottom("数据库管理 bottom").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("奇妙的东西").clicked() {};
                if ui.button("奇妙的东西").clicked() {};
                if ui.button("奇妙的东西").clicked() {};
            });
        });
        if self.sidebar.open {
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
                        self.sidebar.update(ui);
                    });
            });
        }

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            self.table.update(ui, ctx);
        });
    }
}

impl DataBase {
    pub fn new(database_client: DatabaseClient) -> Self {
        Self {
            conns: Conns::default(),
            state: "aaa".into(),
            table: TableComponent::new(database_client.s.clone()),
            config_new_conn: config_new_conn::ConfigNewConnWindow::default(),
            s: database_client.s.clone(),
            r: database_client.r,
            sidebar: SideBar::new(database_client.s.clone()),
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
                    self.conns.borrow_mut().insert(
                        config.get_name(),
                        types::Conn {
                            config,
                            conn: result,
                            databases: None,
                        },
                    );
                    self.config_new_conn.close();
                    self.sidebar.update_conns(self.conns.clone()); // 更新
                }
                message::Response::Databases { conn, data } => {
                    tracing::info!("查询所有数据库成功");
                    if let Some(conn) = self.conns.borrow_mut().get_conn_mut(&conn) {
                        conn.databases = Some(data);
                    }
                    // 更新 table 可选的连接
                    self.table.update_avaliable_conns(
                        self.conns.borrow().keys().map(|x| x.to_owned()).collect(),
                    );
                    // 更新侧边栏
                    self.sidebar.update_conns(self.conns.clone());
                }
                message::Response::Tables { conn, db, data } => {
                    tracing::info!("查询数据表元数据成功！");
                    if let Some(db) = self.conns.borrow_mut().get_db_mut(&conn, &db) {
                        // 更新自己
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
                    if let Some(fields) = self.conns.borrow().get_fields(&conn, &db, &table) {
                        tracing::info!("列数：{}", fields.len());
                        let fields = Box::new(fields.to_owned());

                        let meta = Box::new(table::TableMeta {
                            conn_name: conn.to_owned(),
                            db_name: db.to_owned(),
                            table_name: table,
                            fields,
                            datas,
                        });
                        // 显示 SQl
                        self.table.update_sql(&sql);
                        // 更新表格数据
                        self.table.update_content_and_refresh(meta);
                        // 更新状态
                        self.table.update_current_conn(Some(conn));
                        self.table.update_current_db(Some(db));
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
                    self.table.update_content_and_refresh(meta);
                }
                message::Response::Delete { n, msg, sql } => {
                    if n == 0 {
                        self.table.show_msg(msg);
                    } else {
                        self.table.refresh();
                        self.table.update_sql(&sql);
                    }
                }
                message::Response::Insert { n, msg, sql } => {
                    if n == 0 {
                        self.table.show_msg(msg);
                    } else {
                        self.table.refresh();
                        self.table.update_sql(&sql);
                    }
                }
                message::Response::Update { n, msg, sql } => {
                    if n == 0 {
                        self.table.show_msg(msg);
                    } else {
                        self.table.refresh();
                        self.table.update_sql(&sql);
                    }
                }
            }
        }
    }
}
