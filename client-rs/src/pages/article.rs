use eframe::{
    egui::{self, Layout, RichText, ScrollArea},
    emath::Vec2,
};

use crate::{
    api::{article::NewsArticle, Repo},
    config::Config,
};

pub struct Article {
    articles: Vec<NewsArticle>,
}

impl Default for Article {
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
        Self {
            articles,
        }
    }
}

impl Article {

    pub fn ui(&mut self, ui: &mut egui::Ui, cfg: &Config, repo: &mut Repo) {
        //
        self.render_header(ui, repo);

        self.render_articles(ui, repo);

        self.render_footer(ui);
    }

    pub fn update_articles(&mut self) {}

    fn render_header(&mut self, ui: &mut egui::Ui, repo: &mut Repo) {
        ui.vertical_centered(|ui| {
            ui.heading("机核网 News");
        });
        ui.add_space(5.);
        ui.separator();
    }

    fn render_articles(&mut self, ui: &mut egui::Ui, repo: &mut Repo) {
        if ui.button("↺").clicked() {
            tracing::debug!("清空");
            self.articles.clear();
            if let Err(e) = repo.article.send() {
                tracing::debug!("连接已关闭：{:?}", e);
            }
        }
        if let Ok(articles) = repo.article.try_recv() {
            self.articles = articles;
        }
        if self.articles.is_empty() {
            ui.centered_and_justified(|ui| ui.spinner());
        }

        let scroll_area = ScrollArea::vertical()
            // .max_height(200.0)
            .always_show_scroll(false)
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

    fn render_footer(&self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.label(RichText::new("Api Source: xxx.com").monospace());
            ui.add(egui::Hyperlink::from_label_and_url(
                RichText::new("Made with egui").monospace(),
                "https://github.com/emilk/egui",
            ));
            ui.add(egui::Hyperlink::from_label_and_url(
                RichText::new("Github").monospace(),
                "https://github.com/creativcoder/headlines",
            ));
        });
    }
}
