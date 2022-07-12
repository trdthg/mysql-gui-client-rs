use eframe::{
    egui::{self, Context, Layout, RichText, ScrollArea},
    emath::Vec2,
    epaint::Color32,
};

use crate::service::{
    database::{self, message, sqls},
    Client,
};
pub mod table;
use table::Table;

use crate::service::database::{entity::ConnectionConfig, DatabaseClient};

pub struct DataBase {
    state: String,
    conns: Conns,
    table: Table,
    tmp_config: ConnectionConfig,
    tmp_config_open: bool,
    conn_manager: DatabaseClient,
}

#[derive(Clone, Debug, Default)]
struct Conns {
    pub inner: Vec<Conn>,
}

#[derive(Clone, Debug)]
struct Conn {
    pub config: ConnectionConfig,
    pub conn: Option<usize>,
    pub databases: Vec<DB>,
}

#[derive(Clone, Debug)]
struct DB {
    name: String,
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
                    self.tmp_config_open = true;
                };
            });
            self.make_new_conn(ctx);
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
    pub fn new(conn_manager: DatabaseClient) -> Self {
        Self {
            conns: Conns::default(),
            state: "aaa".into(),
            table: Default::default(),
            tmp_config: ConnectionConfig::default(),
            tmp_config_open: false,
            conn_manager,
        }
    }

    fn render_conn(&self, ui: &mut egui::Ui) {
        for conn in self.conns.inner.iter() {
            if conn.conn.is_none() {
                ui.label(format!("{}", conn.config.get_name()));
                ui.colored_label(Color32::RED, format!("{}", conn.config.get_name()));
                continue;
            }
            // 各数据库连接
            let conn_name = ui.collapsing(
                RichText::new(&conn.config.get_name()).color(Color32::GREEN),
                |ui| {
                    // 各数据库
                    for db in conn.databases.iter() {
                        let db_name = RichText::new(&db.name);
                        ui.collapsing(db_name, |ui| {
                            // 各数据表
                            let table_name = RichText::new("tables");
                            ui.collapsing(table_name, |ui| {
                                // 各字段
                                ui.collapsing(RichText::new("student"), |ui| {
                                    if ui.button("P: id").clicked() {}
                                    if ui.button("N: name").clicked() {}
                                    if ui.button("N: age").clicked() {}
                                });
                                ui.collapsing(RichText::new("class"), |ui| {
                                    if ui.button("P: id").clicked() {}
                                    if ui.button("N: name").clicked() {}
                                    if ui.button("N: teacher").clicked() {}
                                });
                            });
                            ui.collapsing(RichText::new("views"), |ui| {
                                ui.collapsing(RichText::new("Student"), |ui| {
                                    ui.label("P: id");
                                    if ui.button("P: id").clicked() {}
                                    if ui.button("N: name").clicked() {}
                                    if ui.button("N: age").clicked() {}
                                });
                                ui.collapsing(RichText::new("Class"), |ui| {});
                            });
                            ui.collapsing(RichText::new("procedures"), |ui| {
                                ui.collapsing(RichText::new("Student"), |ui| {
                                    if ui.label("P: id").clicked() {}
                                    if ui.button("N: name").clicked() {}
                                    if ui.button("N: age").clicked() {}
                                });
                            });
                            ui.collapsing(RichText::new("functions"), |ui| {
                                ui.collapsing(RichText::new("Student"), |ui| {
                                    if ui.button("P: id").clicked() {}
                                    if ui.button("N: name").clicked() {}
                                    if ui.button("N: age").clicked() {}
                                });
                            });
                        });
                    }
                },
            );
            // 数据库连接被点击时，触发查询所有连接
            if conn_name.header_response.clicked() {
                if let Err(e) = self.conn_manager.send(message::Message::Select {
                    key: conn.config.get_name(),
                    db: None,
                    r#type: message::SelectType::Database,
                    sql: sqls::get_databases(),
                }) {
                    tracing::error!("查询数据库失败：{}", e);
                }
            }
            // if res.header_response.secondary_clicked() {
            conn_name.header_response.context_menu(|ui| {
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

    fn make_new_conn(&mut self, ctx: &Context) {
        eframe::egui::Window::new("配置新的连接")
            .open(&mut self.tmp_config_open)
            .show(ctx, |ui| {
                ui.label(RichText::new("请输入 IP"));
                let input_1 = ui.text_edit_singleline(&mut self.tmp_config.ip);
                ui.label(RichText::new("请输入 Port"));
                let input_2 = ui.text_edit_singleline(&mut self.tmp_config.port);
                ui.label(RichText::new("请输入 用户名"));
                let input_3 = ui.text_edit_singleline(&mut self.tmp_config.username);
                ui.label(RichText::new("请输入 密码"));
                let input_4 = ui.text_edit_singleline(&mut self.tmp_config.password);
                // ui.label(RichText::new("请输入 数据库名称"));
                // let input_5 = ui.text_edit_singleline(&mut self.tmp_config.db);

                // if input_5.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {}
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), 0.),
                    Layout::right_to_left(),
                    |ui| {
                        let mut conn_btn = ui.button("连接并保存");
                        conn_btn = conn_btn.on_hover_text("新建一个新的数据库连接，并添加至侧边栏");
                        let mut test_btn = ui.button("测试连接");
                        test_btn = test_btn.on_hover_text("仅测试，不添加到侧边栏");
                        if conn_btn.clicked() {
                            if let Err(e) =
                                self.conn_manager.send(database::message::Message::Connect {
                                    config: self.tmp_config.clone(),
                                    save: true,
                                })
                            {
                                tracing::error!("发送连接请求失败： {}", e);
                            }
                            tracing::info!("发送连接请求成功");
                        }
                        if test_btn.clicked() {
                            if let Err(e) =
                                self.conn_manager.send(database::message::Message::Connect {
                                    config: self.tmp_config.clone(),
                                    save: false,
                                })
                            {
                                tracing::error!("发送连接请求失败： {}", e);
                            }
                            tracing::info!("发送连接请求成功");
                        }
                    },
                );
            });
    }

    fn handle_sql(&mut self) {
        if let Ok(v) = self.conn_manager.try_recv() {
            match v {
                message::Response::NewConn {
                    config,
                    save,
                    result,
                } => {
                    tracing::info!("连接成功！");
                    if save == false {
                        return;
                    }
                    self.conns.inner.push(Conn {
                        config,
                        conn: result,
                        databases: vec![],
                    });
                    self.tmp_config_open = false;
                }
                message::Response::Databases { data } => {
                    tracing::info!("{:?}", data);
                    tracing::info!("查询数据库成功");
                    // self.conns.inner.get(1).unwrap().databases = data.iter().map(|x| {

                    // })
                }
            }
        }
    }
}
