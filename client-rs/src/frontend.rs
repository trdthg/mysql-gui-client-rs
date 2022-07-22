use eframe::{
    egui::{self, Button, Context, Layout, RichText, TopBottomPanel},
    emath::Vec2,
};
mod article;
mod component;
pub mod database;
mod setting;
mod talk;
mod test;

use crate::{backend::Repo, config::Config};

use self::{article::Article, database::DataBase, setting::Setting, talk::Talk, test::Test};

pub struct State {
    article: Article,
    database: DataBase,
    talk: Talk,
    setting: Setting,
    test: Test,
    // #[cfg(feature = "http")]
    selected: String,
}
impl State {
    pub fn new(repo: Repo) -> Self {
        let database = DataBase::new(repo.database_client);
        let article = Article::new(repo.article_client);
        let setting = Setting::new(Config::new());
        let test = Default::default();
        let talk = Talk::new();
        Self {
            article,
            database,
            setting,
            talk,
            test,
            selected: String::new(),
        }
    }
}

pub struct App {
    pub state: State,
    pub config: Config,
}

impl eframe::App for App {
    fn warm_up_enabled(&self) -> bool {
        println!("绘制行吗？？？");
        false
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // _storage.flush()
        // println!("原来的： {:?}", _storage.get_string("key"));
        // _storage.set_string("key", "1231221".to_owned());
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        tracing::trace!("更新配置");
        // 初始化作用
        self.state.setting.init(ctx);
        // ctx.set_debug_on_hover(true);

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
    pub fn run(self) -> ! {
        let mut options = eframe::NativeOptions::default();
        options.resizable = true;
        options.vsync = true;
        options.initial_window_size = Some(Vec2::new(480.0, 740.0));
        eframe::run_native("My App", options, Box::new(|_cc| Box::new(self)));
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
                    let close_btn = ui.add(Button::new("✖")); // ✕ ❌ ✖ ❎ ✅ ✔ ➕+
                    if close_btn.clicked() {
                        frame.quit();
                    }
                    if ctx.style().visuals.dark_mode {
                        let theme_btn = ui.add(Button::new("🌙")); // 🌛 🌙 ⛭
                        if theme_btn.clicked() {
                            ctx.set_visuals(egui::Visuals::light());
                        }
                    } else {
                        let theme_btn = ui.add(Button::new("🔆")); // ⟳ 🔆 🔅 🌞
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
            // (
            //     "✨ Redis",
            //     "databae",
            //     &mut self.state.redis as &mut dyn eframe::App,
            // ),
            (
                "📓 文章",
                "article",
                &mut self.state.article as &mut dyn eframe::App,
            ),
            (
                "⛭ 聊天", // 齿轮 🔨 🔧
                "talk",
                &mut self.state.talk as &mut dyn eframe::App,
            ),
            (
                "⛭ 设置", // 齿轮 🔨 🔧
                "setting",
                &mut self.state.setting as &mut dyn eframe::App,
            ),
            (
                "🎮 测试",
                "test",
                &mut self.state.test as &mut dyn eframe::App,
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
        // egui::panel::TopBottomPanel::bottom("表管理 bottom").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //         if ui.button("奇妙的东西").clicked() {};
        //         if ui.button("奇妙的东西").clicked() {};
        //         if ui.button("奇妙的东西").clicked() {};
        //     });
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
