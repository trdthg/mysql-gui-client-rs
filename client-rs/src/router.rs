use eframe::{
    egui::{self, style::Margin, Context, Frame},
    epaint::Color32,
};

use crate::{
    api::Repo,
    client::Client,
    config::Config,
    pages::{database::DataBase, headline::HeadLine, setting::Setting, talk::Talk},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Page {
    Article,
    DataBase,
    Setting,
}

impl Default for Page {
    fn default() -> Self {
        Self::Article
    }
}
pub struct Router {
    page: Page,
    article: HeadLine,
    database: DataBase,
    setting: Setting,
}

impl Default for Router {
    fn default() -> Self {
        // let client = Client::new(([127, 0, 0, 1], 1234)).unwrap();

        Self {
            page: Default::default(),
            article: HeadLine::default(),
            setting: Default::default(),
            database: Default::default(),
        }
    }
}

impl Router {
    pub fn ui(&mut self, ui: &mut egui::Ui, ctx: &Context, cfg: &mut Config, repo: &mut Repo) {
        let f = Frame::none()
            .inner_margin(5.)
            .outer_margin(0.)
            .fill(Color32::LIGHT_BLUE);

        // ui.spacing_mut().window_margin.bottom = 0.;
        // ui.spacing_mut().window_margin.top = 0.;
        // ui.spacing_mut().item_spacing.y = 0.;

        egui::TopBottomPanel::top("section_baraaa")
            .frame(f)
            .show_inside(ui, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.selectable_value(&mut self.page, Page::Article, "文章");
                    ui.selectable_value(&mut self.page, Page::DataBase, "数据库");
                    ui.selectable_value(&mut self.page, Page::Setting, "设置");
                });
            });
        let f = Frame::none()
            .inner_margin(0.)
            .outer_margin(0.)
            .fill(Color32::WHITE);
        egui::CentralPanel::default()
            .frame(f)
            .show_inside(ui, |ui| {
                // ui.separator();
                ui.add_space(4.);
                match self.page {
                    Page::Article => {
                        self.article.ui(ui, cfg, repo);
                    }
                    Page::Setting => {
                        self.setting.ui(ui, ctx, cfg);
                    }
                    Page::DataBase => self.database.ui(ui, ctx),
                }
            });
    }
}
