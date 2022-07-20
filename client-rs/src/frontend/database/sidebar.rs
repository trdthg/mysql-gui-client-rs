use eframe::{
    egui::{self, RichText},
    epaint::Color32,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::backend::database::{message, sqls};

use super::Conns;

pub struct SideBar {
    pub open: bool,
    pub conns: Conns,
    pub s: UnboundedSender<message::Message>,
}

impl SideBar {
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn new(s: UnboundedSender<message::Message>) -> SideBar {
        Self {
            open: true,
            conns: Conns::default(),
            s,
        }
    }

    pub fn update_conns(&mut self, conns: Conns) {
        self.conns = conns;
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        for (conn_name, conn) in self.conns.borrow().iter() {
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
                                                    .conns
                                                    .borrow()
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
}
