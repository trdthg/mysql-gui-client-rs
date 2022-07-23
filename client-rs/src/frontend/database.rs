mod config_new_conn;
mod sidebar;
mod table;
pub mod types;
use crate::backend::database::{
    message::{self, ConnectionConfig},
    DatabaseClient,
};
use eframe::egui::{self, RichText, ScrollArea};
use table::Table as TableComponent;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use self::{sidebar::SideBar, types::Conns};

pub struct DataBase {
    init: bool,
    state: String,
    conns: Conns,
    sidebar: SideBar,
    table: TableComponent,
    config_new_conn: config_new_conn::ConfigNewConnWindow,
    s: UnboundedSender<message::Request>,
    r: UnboundedReceiver<message::Response>,
}

impl eframe::App for DataBase {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.init(ctx, frame);
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
                self.handle_sql(frame);
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.sidebar.update(ui, frame);
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
            init: false,
            conns: Conns::default(),
            state: "aaa".into(),
            table: TableComponent::new(database_client.s.clone()),
            config_new_conn: config_new_conn::ConfigNewConnWindow::default(),
            s: database_client.s.clone(),
            r: database_client.r,
            sidebar: SideBar::new(database_client.s.clone()),
        }
    }

    fn init(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.init == true {
            return;
        }
        if let Some(configs) = frame.storage().and_then(|x| x.get_string("conns")) {
            if let Ok(configs) = serde_json::from_str::<Vec<ConnectionConfig>>(&configs) {
                for config in configs {
                    if let Err(e) = self
                        .s
                        .send(message::Request::Connect { config, save: true })
                    {
                        tracing::error!("后端未正常启动：{}", e);
                    }
                }
            }
        }

        self.init = true;
    }

    fn handle_sql(&mut self, frame: &mut eframe::Frame) {
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
                    // 持久化连接
                    if let Some(store) = frame.storage_mut() {
                        let conns = self.conns.borrow();
                        let configs: Vec<&ConnectionConfig> =
                            conns.values().map(|x| &x.config).collect();
                        if let Ok(s) = serde_json::to_string(&configs) {
                            store.set_string("conns", s);
                        }
                    }
                    self.config_new_conn.close();
                    self.sidebar.update_conns(self.conns.clone()); // 更新
                }
                message::Response::Databases { conn, data } => {
                    tracing::info!("查询所有数据库成功");
                    if let Some(conn) = self.conns.borrow_mut().get_conn_mut(&conn) {
                        conn.databases = Some(data);
                    }
                    // 更新侧边栏
                    self.sidebar.update_conns(self.conns.clone());
                    // 更新表格
                    self.table.update_conns(self.conns.clone());
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
                            table_name: table.to_owned(),
                            fields,
                            datas,
                        });
                        // 显示 SQl
                        self.table.update_sql(&sql);
                        // 更新表格数据
                        self.table.update_content_and_refresh(meta);
                        // 更新状态
                        self.table
                            .update_current_table(Some(conn), Some(db), Some(table));
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
