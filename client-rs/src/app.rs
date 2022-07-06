use eframe::egui::{self, Button, Context, Layout, RichText, TopBottomPanel};

use crate::{
    apps::{database::DataBase, Article, Setting},
    config::Config,
    server::Repo,
};

pub struct State {
    article: Article,
    database: DataBase,
    setting: Setting,
    // #[cfg(feature = "http")]
    selected: String,
}
impl State {
    pub fn new(mut repo: Repo) -> Self {
        let database = DataBase::new(repo.conn_manager.take().unwrap());
        let article = Article::new(repo.article);
        let setting = Setting::default();
        Self {
            article,
            database,
            setting,
            selected: String::new(),
        }
    }
}

pub struct App {
    pub state: State,
    pub config: Config,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        tracing::trace!("更新配置");
        // ctx.set_debug_on_hover(true);
        self.config.update(ctx);
        // if !self.config.api_key_setted {
        //     self.render_config(ctx);
        //     return;
        // }

        if self.state.selected.is_empty() {
            let selected_anchor = self.apps_iter_mut().next().unwrap().0.to_owned();
            self.state.selected = selected_anchor;
        }

        tracing::trace!("渲染 Top");
        self.render_top_panel(ctx, frame);
        tracing::trace!("渲染 Side");
        self.render_side(ctx);
        tracing::trace!("渲染 Footer");
        self.render_footer(ctx);
        tracing::trace!("渲染 Content");
        self.render_content(ctx, frame);
    }
}

impl App {
    pub fn new(repo: Repo) -> Self {
        let state = State::new(repo);
        Self {
            state,
            config: Config::new(),
        }
    }

    fn save_config(&self, config: Config) -> Result<(), confy::ConfyError> {
        confy::store(crate::config::CONFIG_PATH, config)
    }

    fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                tracing::trace!("渲染头部 App 导航");
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.label(RichText::new("App").heading());
                    let mut selected_anchor = self.state.selected.clone();
                    for (name, anchor, _app) in self.apps_iter_mut() {
                        if ui
                            .selectable_label(selected_anchor == anchor, name)
                            .clicked()
                        {
                            selected_anchor = anchor.to_owned();
                            if frame.is_web() {
                                ui.output().open_url(format!("#{}", anchor));
                            }
                        }
                    }
                    self.state.selected = selected_anchor;
                });

                // 渲染右侧按钮
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new("✖")); // ✕
                    if close_btn.clicked() {
                        frame.quit();
                    }
                    if ctx.style().visuals.dark_mode {
                        let theme_btn = ui.add(Button::new("🌙"));
                        if theme_btn.clicked() {
                            ctx.set_visuals(egui::Visuals::light());
                        }
                    } else {
                        let theme_btn = ui.add(Button::new("🌞"));
                        if theme_btn.clicked() {
                            ctx.set_visuals(egui::Visuals::dark());
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let time = chrono::Local::now();
                        ui.label(RichText::new(format!(
                            "{} (timestamp)",
                            time.timestamp_millis().to_string()
                        )));
                    }
                });
                tracing::trace!("渲染 Top 结束！");
            });
        });
    }

    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        let vec = vec![
            (
                "✨ 数据库",
                "databae",
                &mut self.state.database as &mut dyn eframe::App,
            ),
            (
                "🖹 文章",
                "article",
                &mut self.state.article as &mut dyn eframe::App,
            ),
            (
                "🕑 设置",
                "setting",
                &mut self.state.setting as &mut dyn eframe::App,
            ),
        ];

        // #[cfg(feature = "http")]
        // "⬇ HTTP",
        // "🔺 3D painting",
        // "colors",
        // "🎨 Color test",
        // "custom3d",
        // &mut self.custom3d as &mut dyn eframe::App,

        vec.into_iter()
    }

    fn render_footer(&mut self, ctx: &Context) {
        // TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //     //
        // });
    }

    fn render_side(&mut self, ctx: &Context) {}

    fn render_content(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut found_anchor = false;
            let selected_anchor = self.state.selected.clone();
            for (_name, anchor, app) in self.apps_iter_mut() {
                if anchor == selected_anchor || ctx.memory().everything_is_visible() {
                    app.update(ctx, frame);
                    found_anchor = true;
                }
            }
            if !found_anchor {
                self.state.selected = "article".into();
            }
        });
    }

    fn render_config(&mut self, ctx: &Context) {
        // eframe::egui::Window::new("配置")
        //     .title_bar(false)
        //     .show(ctx, |ui| {
        //         ui.label(RichText::new("请输入 API_KEY"));
        //         let input = ui.text_edit_singleline(&mut self.config.api_key);
        //         if input.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
        //             if !self.config.api_key.is_empty() {
        //                 self.config.api_key_setted = true;
        //                 if let Err(e) = self.save_config(self.config.clone()) {
        //                     let err_msg = format!("配置保存失败：{:?}", e);
        //                     tracing::error!(err_msg);
        //                     ui.label(RichText::new(err_msg).color(self.config.theme.colors.error));
        //                     ui.ctx().memory().request_focus(input.id);
        //                 } else {
        //                     tracing::info!("配置保存成功");
        //                 }
        //             }
        //         }
        //         ui.label("如果您还没有注册，请访问下面的链接获取 API_KEY");
        //         ui.hyperlink("https://newsapi.org");
        //     });
    }
}
