use std::{sync::mpsc, thread};

use eframe::{
    egui::{self, Button, Context, Hyperlink, Layout, RichText, TopBottomPanel},
    epaint::Color32,
};

use crate::{
    api::{self, Repo},
    config::Config,
    pages::headline::NewsArticle,
    router::Router,
};

pub struct App {
    pub router: Router,
    pub config: Config,
    pub repo: Repo,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        tracing::info!("更新配置");
        // ctx.set_debug_on_hover(true);
        self.config.update(ctx);
        // if !self.config.api_key_setted {
        //     self.render_config(ctx);
        //     return;
        // }

        tracing::info!("渲染 Top");
        self.render_top_panel(ctx, frame);
        tracing::info!("渲染 Side");
        self.render_side(ctx);
        tracing::info!("渲染 Footer");
        self.render_footer(ctx);
        tracing::info!("渲染 Content");
        self.render_content(ctx, frame);
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::default(),
            config: Config::new(),
            repo: Repo::new(),
        }
    }

    fn save_config(&self, config: Config) -> Result<(), confy::ConfyError> {
        confy::store(crate::config::CONFIG_PATH, config)
    }

    fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // let f = egui::Frame::none()
        //     .inner_margin(0.)
        //     .outer_margin(0.)
        //     .fill(Color32::RED);
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                tracing::debug!("开始渲染 Top！");
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.label(RichText::new("软件").heading());
                });
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new("关闭"));
                    if close_btn.clicked() {
                        frame.quit();
                    }
                    let refresh_btn = ui.add(Button::new("刷新"));
                    if refresh_btn.clicked() {
                        // frame.ref
                    }
                    let theme_btn = ui.add(Button::new("主题"));
                    if theme_btn.clicked() {
                        self.config.theme.toogle_dark_mode();
                    }
                    // time
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let time = chrono::Local::now();
                        ui.label(RichText::new(format!(
                            "{} (时间戳)",
                            time.timestamp_millis().to_string()
                        )));
                    }
                });
                tracing::debug!("渲染 Top 结束！");
            });
        });
    }
    fn render_footer(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(RichText::new("Api Source: xxx.com").monospace());
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("Made with egui").monospace(),
                    "https://github.com/emilk/egui",
                ));
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("Github").monospace(),
                    "https://github.com/creativcoder/headlines",
                ));
            });
        });
    }

    fn render_side(&mut self, ctx: &Context) {}

    fn render_content(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let f = eframe::egui::Frame::none()
            .inner_margin(0.)
            .outer_margin(0.)
            .fill(Color32::WHITE);
        egui::CentralPanel::default().frame(f).show(ctx, |ui| {
            self.router.ui(ui, ctx, &mut self.config, &mut self.repo);
        });
    }

    fn render_config(&mut self, ctx: &Context) {
        eframe::egui::Window::new("配置")
            .title_bar(false)
            .show(ctx, |ui| {
                ui.label(RichText::new("请输入 API_KEY"));
                let input = ui.text_edit_singleline(&mut self.config.api_key);
                if input.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    if !self.config.api_key.is_empty() {
                        self.config.api_key_setted = true;
                        if let Err(e) = self.save_config(self.config.clone()) {
                            let err_msg = format!("配置保存失败：{:?}", e);
                            tracing::error!(err_msg);
                            ui.label(RichText::new(err_msg).color(self.config.theme.colors.error));
                            ui.ctx().memory().request_focus(input.id);
                        } else {
                            tracing::info!("配置保存成功");
                        }
                    }
                }
                ui.label("如果您还没有注册，请访问下面的链接获取 API_KEY");
                ui.hyperlink("https://newsapi.org");
            });
    }
}
