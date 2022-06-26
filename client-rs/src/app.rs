use eframe::egui::{self, Button, Context, Hyperlink, Layout, RichText, TopBottomPanel};

use crate::{config::Config, router::Router};

pub struct App {
    pub router: Router,
    pub config: Config,
    pub api_key: String,
}

impl App {
    fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.label(RichText::new("Bok").heading());
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
                    let time = chrono::Local::now();

                    ui.label(RichText::new(time.timestamp_millis().to_string()));
                });
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

    fn render_content(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.router.ui(ui, &mut self.config);
        });
    }

    fn render_config(&mut self, ctx: &Context) {
        eframe::egui::Window::new("配置").show(ctx, |ui| {
            ui.label(RichText::new("请输入 API_KEY"));
            ui.text_edit_singleline(&mut self.api_key);
        });
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            router: Router::default(),
            config: Config::default(),
            api_key: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.config.update(ctx);
        self.render_config(ctx);
        self.render_top_panel(ctx, frame);
        self.render_content(ctx);
        self.render_footer(ctx);
    }
}
