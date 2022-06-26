use eframe::egui;

use crate::{
    config::Config,
    pages::{headline::HeadLine, list::List, setting::Setting, talk::Talk, test::Test},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Page {
    Talk,
    Article,
    List,
    Other,
    Setting,
}

impl Default for Page {
    fn default() -> Self {
        Self::Article
    }
}
pub struct Router {
    page: Page,
    list: List,
    article: HeadLine,
    talk: Talk,
    test: Test,
    setting: Setting,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            page: Default::default(),
            article: HeadLine::default(),
            list: Default::default(),
            talk: Default::default(),
            test: Default::default(),
            setting: Default::default(),
        }
    }
}
impl Router {
    pub fn ui(&mut self, ui: &mut egui::Ui, cfg: &mut Config) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.page, Page::Article, "文章");
            ui.selectable_value(&mut self.page, Page::List, "列表");
            ui.selectable_value(&mut self.page, Page::Talk, "聊天");
            ui.selectable_value(&mut self.page, Page::Other, "待续");
            ui.selectable_value(&mut self.page, Page::Setting, "设置");
        });
        ui.separator();
        match self.page {
            Page::Article => {
                self.article.ui(ui, cfg);
            }
            Page::Talk => {
                self.talk.ui(ui);
            }
            Page::List => {
                self.list.ui(ui);
            }
            Page::Other => {
                self.test.ui(ui);
            }
            Page::Setting => {
                self.setting.ui(ui, cfg);
            }
        }
    }
}
