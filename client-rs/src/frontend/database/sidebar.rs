use eframe::{
    egui::{self, Id, RichText},
    epaint::Color32,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::backend::database::{message, sqls};

use super::Conns;

pub struct SideBar {
    pub open: bool,
    pub conns: Conns,
    pub s: UnboundedSender<message::Request>,
    pub hovered_widgt_id: Option<Id>,
}

impl SideBar {
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn new(s: UnboundedSender<message::Request>) -> SideBar {
        Self {
            open: true,
            conns: Conns::default(),
            s,
            hovered_widgt_id: None,
        }
    }

    pub fn update_conns(&mut self, conns: Conns) {
        self.conns = conns;
    }

    pub fn update(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        for (conn_name, conn) in self.conns.borrow().iter() {
            if conn.conn.is_none() {
                ui.label(format!("{}", conn.config.get_name()));
                ui.colored_label(Color32::RED, format!("{}", conn.config.get_name()));
                continue;
            }

            // 数据库连接
            let mut header_text = RichText::new(&conn.config.get_name()).color(Color32::GREEN);
            // let id = Id::new(header_text.text());
            // if let Some(stored_id) = self.hovered_widgt_id {
            //     tracing::info!("Hover 中 1 {:?} {:?}", id, stored_id);
            //     if id == stored_id {
            //         tracing::info!("Hover 中 2");
            //         header_text = header_text.background_color(Color32::GRAY);
            //     }
            // }
            let conn_collapsing = ui.collapsing(header_text.to_owned(), |ui| {
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
                                            if let Some(fields) = self
                                                .conns
                                                .borrow()
                                                .get_fields(conn_name, db_name, table_name)
                                                .and_then(|x| Some(x))
                                            {
                                                if let Err(e) =
                                                    self.s.send(message::Request::SelectTable {
                                                        conn: conn.config.get_name(),
                                                        db: db_name.to_string(),
                                                        table: table_name.to_string(),
                                                        page: 0,
                                                        size: 100,
                                                        fields: Box::new(fields.to_owned()),
                                                        orders: None,
                                                    })
                                                {
                                                    tracing::error!("查询数据表失败：{}", e);
                                                }
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
                                            self.s.send(message::Request::SelectTables {
                                                conn: conn.config.get_name(),
                                                db: db_name.to_string(),
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
            });

            // 数据库连接被点击时，触发查询所有连接
            // if conn_collapsing.header_response.clicked() && conn.databases.is_none() {}

            // if conn_collapsing.header_response.hovered() {
            //     let text = header_text.text().to_owned();
            //     self.hovered_widgt_id = Some(Id::new(text));
            // } else {
            //     self.hovered_widgt_id = None;
            // }

            conn_collapsing.header_response.context_menu(|ui| {
                egui::menu::bar(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.spacing();
                        if ui.button("刷新").clicked() {
                            if let Err(e) = self.s.send(message::Request::SelectDatabases {
                                conn: conn.config.get_name(),
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
