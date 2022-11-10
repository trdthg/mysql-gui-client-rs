use eframe::{
    egui::{Context, Layout, RichText},
    emath::Vec2,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{backend::database::message, frontend::database};

#[derive(Default)]
pub struct ConfigNewConnWindow {
    tmp_config: message::ConnectionConfig,
    tmp_config_open: bool,
}

impl ConfigNewConnWindow {
    pub fn run(&mut self, s: &UnboundedSender<message::Request>, ctx: &Context) {
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
                    Layout::right_to_left(eframe::emath::Align::Center),
                    |ui| {
                        let mut conn_btn = ui.button("连接并保存");
                        conn_btn = conn_btn.on_hover_text("新建一个新的数据库连接，并添加至侧边栏");
                        let mut test_btn = ui.button("测试连接");
                        test_btn = test_btn.on_hover_text("仅测试，不添加到侧边栏");
                        if conn_btn.clicked() {
                            if let Err(e) = s.send(database::message::Request::Connect {
                                config: self.tmp_config.clone(),
                                save: true,
                            }) {
                                tracing::error!("发送连接请求失败： {}", e);
                            }
                            tracing::info!("发送连接请求成功");
                        }
                        if test_btn.clicked() {
                            if let Err(e) = s.send(database::message::Request::Connect {
                                config: self.tmp_config.clone(),
                                save: false,
                            }) {
                                tracing::error!("发送连接请求失败： {}", e);
                            }
                            tracing::info!("发送连接请求成功");
                        }
                    },
                );
            });
    }

    pub fn close(&mut self) {
        self.tmp_config_open = false;
    }
    pub fn open(&mut self) {
        self.tmp_config_open = true;
    }
}
