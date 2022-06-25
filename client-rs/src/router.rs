use eframe::egui::{self, ScrollArea};

use crate::pages::{headline::HeadLine, list::List, talk::Talk};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Page {
    Talk,
    Article,
    List,
    Other,
}

impl Default for Page {
    fn default() -> Self {
        Self::Article
    }
}
pub(crate) struct Router {
    page: Page,
    list: List,
    article: HeadLine,
    talk: Talk,
}
impl Default for Router {
    fn default() -> Self {
        Self {
            page: Default::default(),
            article: HeadLine::default(),
            list: Default::default(),
            talk: Default::default(),
        }
    }
}
impl Router {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.page, Page::Article, "文章");
            ui.selectable_value(&mut self.page, Page::List, "列表");
            ui.selectable_value(&mut self.page, Page::Talk, "聊天");
            ui.selectable_value(&mut self.page, Page::Other, "待续");
        });
        ui.separator();
        match self.page {
            Page::Article => {
                self.article.ui(ui);
            }
            Page::Talk => {
                self.talk.ui(ui);
            }
            Page::List => {
                self.list.ui(ui);
            }
            Page::Other => {
                ui.label("没有写呢！".to_owned());
            }
        }
    }
}
