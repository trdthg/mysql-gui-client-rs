use eframe::{
    egui::{self, Layout, RichText, ScrollArea},
    emath::Vec2,
};

use crate::{
    server::{entity::NewsArticle, Repo},
    util::duplex_channel::DuplexConsumer,
};

pub struct Article {
    articles: Vec<NewsArticle>,
    fetcher: DuplexConsumer<(), Vec<NewsArticle>>,
}

impl eframe::App for Article {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("数据库管理 top").show(ctx, |ui| {
            self.render_header(ui);

            // egui::menu::bar(ui, |ui| {
            //     ui.selectable_value(&mut self.state, "".to_string(), "数据管理");
            //     ui.selectable_value(&mut self.state, "".to_string(), "监控");
            // });
        });

        egui::panel::TopBottomPanel::bottom("数据库管理 bottom").show(ctx, |ui| {
            // ui.label("状态栏：您当前正在观测的数据库是 XXX");
            self.render_footer(ui);
        });

        egui::SidePanel::left("数据库管理 sidebar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Article");
                if ui.button("↺").clicked() {
                    tracing::debug!("清空");
                    self.articles.clear();
                    if let Err(e) = self.fetcher.send(()) {
                        tracing::debug!("连接已关闭：{:?}", e);
                    }
                }
            });
            if let Ok(articles) = self.fetcher.try_recv() {
                self.articles = articles;
            }
        });

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            // self.table.update(ctx, frame);
            self.render_articles(ui);
        });
    }
}
impl Article {
    pub fn new(article_fetcher: DuplexConsumer<(), Vec<NewsArticle>>) -> Self {
        Self {
            articles: vec![],
            fetcher: article_fetcher,
        }
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("机核网 News");
        });
    }

    fn render_articles(&mut self, ui: &mut egui::Ui) {
        if self.articles.is_empty() {
            ui.centered_and_justified(|ui| ui.spinner());
            return;
        }

        let scroll_area = ScrollArea::vertical()
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
