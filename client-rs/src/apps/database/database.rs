use eframe::{
    egui::{self, Context, Layout, RichText, ScrollArea},
    App,
};

use crate::{server::api::mysql::ConnectionConfig, util::duplex_channel::DuplexConsumer};

use super::table::Table;


pub struct DataBase {
    state: String,
    conns: Vec<Connection>,
    table: Table,
    tmp_config: ConnectionConfig,
    tmp_config_open: bool,
    conn_manager: DuplexConsumer<ConnectionConfig, Connection>,
}

#[derive(Clone)]
pub struct Connection {
    pub config: ConnectionConfig,
    pub conn: Option<usize>,
}

impl App for DataBase {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("数据库管理 top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.selectable_value(&mut self.state, "".to_string(), "数据管理");
                ui.selectable_value(&mut self.state, "".to_string(), "监控");
            });
        });

        egui::panel::TopBottomPanel::bottom("数据库管理 bottom").show(ctx, |ui| {
            //
            ui.label("状态栏：您当前正在观测的数据库是 XXX");
        });

        egui::SidePanel::left("数据库管理 sidebar").show(ctx, |ui| {
            ui.heading("数据库连接");
            if ui.button("新建连接").clicked() {
                self.tmp_config_open = true;
            };
            self.make_new_conn(ctx);
            if let Ok(v) = self.conn_manager.try_recv() {
                tracing::info!("连接成功！");
                self.conns.push(v);
            }
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    self.render_conn(ui);
                });
        });

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            self.table.update(ctx, frame);
        });
    }
}

impl DataBase {
    pub fn new(conn_manager: DuplexConsumer<ConnectionConfig, Connection>) -> Self {
        Self {
            conns: vec![],
            state: "aaa".into(),
            table: Default::default(),
            tmp_config: ConnectionConfig::default(),
            tmp_config_open: false,
            conn_manager: conn_manager,
        }
    }

    fn render_conn(&self, ui: &mut egui::Ui) {
        for conn in self.conns.iter() {
            if conn.conn.is_none() {
                ui.label(format!("{} {}", conn.config.name, "Failed"));
                continue;
            }
            let res = ui.collapsing(RichText::new(&conn.config.name), |ui| {
                ui.collapsing(RichText::new("dev"), |ui| {
                    ui.collapsing(RichText::new("tables"), |ui| {
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
            });
            // if res.header_response.secondary_clicked() {
            res.header_response.context_menu(|ui| {
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
                ui.label(RichText::new("请输入 数据库名称"));
                let input_5 = ui.text_edit_singleline(&mut self.tmp_config.db);

                // if input_5.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {}
                ui.allocate_ui_with_layout(ui.available_size(), Layout::right_to_left(), |ui| {
                    if ui.button("测试连接").clicked() {
                        if let Err(e) = self.conn_manager.send(self.tmp_config.clone()) {
                            tracing::error!("发送连接请求失败： {}", e);
                        }
                        tracing::info!("发送连接请求成功");
                        // self.config.api_key_setted = true;
                        // if let Err(e) = self.save_config(self.config.clone()) {
                        //     let err_msg = format!("配置保存失败：{:?}", e);
                        //     tracing::error!(err_msg);
                        //     ui.label(RichText::new(err_msg).color(self.config.theme.colors.error));
                        //     ui.ctx().memory().request_focus(input.id);
                        // } else {
                        //     tracing::info!("配置保存成功");
                        // }
                    };
                });
            });
    }
}
