use eframe::{
    egui::{self, Layout, RichText, ScrollArea},
    emath::Vec2,
};

use crate::{client::api::Repo, config::Config};

pub struct HeadLine {
    articles: Vec<NewsArticle>,
}

impl Default for HeadLine {
    fn default() -> Self {
        let articles = (0..0)
            .map(|i| NewsArticle {
                id: i.to_string(),
                title: format!("title: {}", i),
                desc: format!(
                    "desc: {} urally this is a very long sentence and you can't change it",
                    i
                ),
                url: format!("url: http://localhsot:/{}", i),
                author: "default".to_string(),
                time: "00:00".to_string(),
            })
            .collect();
        Self { articles }
    }
}

impl HeadLine {
    pub fn new() -> Self {
        let articles = vec![];
        Self { articles }
    }

    pub fn update_articles(&mut self) {

        // let a = "
        // https://www.gcores.com/gapi/v1/articles?
        // page[limit]=${limit}
        // &page[offset]=${offset}
        // &sort=-published-at
        // &include=category,user
        // &filter[is-news]=${isNews}
        // &fields[articles]=title,desc,is-published,thumb,app-cover,cover,comments-count,likes-count,bookmarks-count,is-verified,published-at,option-is-official,option-is-focus-showcase,duration,category,user";
    }

    fn render_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| ui.heading("机核网 News"));
        ui.add_space(5.);
        // let seq = Separator::default().spacing(20.);
        ui.separator();
        ui.add_space(5.);
    }

    fn render_articles(&mut self, ui: &mut egui::Ui, cfg: &Config, repo: &mut Repo) {
        if self.articles.is_empty() {
            if let Ok(articles) = repo.article_channel.try_recv() {
                self.articles = articles;
            }
        }
        let scroll_area = ScrollArea::vertical()
            // .max_height(200.0)
            .always_show_scroll(true)
            .auto_shrink([false; 2]);

        scroll_area.show(ui, |ui| {
            for (i, a) in self.articles.iter().enumerate() {
                ui.add_space(5.0);
                ui.heading(
                    RichText::new(format!("#{} {}", i + 1, &a.title))
                        .strong()
                        .heading(), // .size(22.), // .color(cfg.theme.colors.h2),
                );

                ui.add_space(5.0);
                ui.label(
                    RichText::new(&a.desc), // .color(cfg.theme.colors.body), // .font(FontId::proportional(40.0)),
                );

                ui.add_space(5.0);
                ui.label(
                    RichText::new(format!("{} 发布于 {}", a.author, a.time)), // .color(cfg.theme.colors.body), // .font(FontId::proportional(40.0)),
                );
                // ui.style_mut().visuals.hyperlink_color = cfg.theme.colors.link;
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), 0.),
                    Layout::right_to_left(),
                    |ui| {
                        ui.add(egui::Hyperlink::from_label_and_url("阅读原文 ⤴", &a.url));
                    },
                );

                ui.add_space(5.0);
                ui.separator();
            }
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewsArticle {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub url: String,
    pub author: String,
    pub time: String,
}

impl Default for NewsArticle {
    fn default() -> Self {
        Self {
            id: Default::default(),
            title: "文章标题".to_owned(),
            desc: Default::default(),
            url: Default::default(),
            author: Default::default(),
            time: Default::default(),
        }
    }
}

impl HeadLine {
    pub fn ui(&mut self, ui: &mut egui::Ui, cfg: &Config, repo: &mut Repo) {
        //
        self.render_header(ui);

        self.render_articles(ui, cfg, repo);
    }
}
