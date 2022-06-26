use eframe::{
    egui::{self, Layout, RichText, ScrollArea},
    emath::Vec2,
};

use crate::config::Config;

pub struct HeadLine {
    articles: Vec<NewsArticle>,
}

impl Default for HeadLine {
    fn default() -> Self {
        let articles = (0..50)
            .map(|i| NewsArticle {
                id: i as usize,
                title: format!("title: {}", i),
                desc: format!(
                    "desc: {} urally this is a very long sentence and you can't change it",
                    i
                ),
                url: format!("url: http://localhsot:/{}", i),
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

    fn render_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| ui.heading("标题"));
        ui.add_space(5.);
        // let seq = Separator::default().spacing(20.);
        ui.separator();
        ui.add_space(5.);
        // ui.collapsing("Theme", |ui| {
        //     let dark = ui.add(Button::new("黑暗"));
        //     let light = ui.add(Button::new("明亮"));
        //     if dark.clicked() {
        //         self.config.dark_mode = true;
        //     }
        //     if light.clicked() {
        //         self.config.dark_mode = false;
        //     }
        // });    ui.add(seq);
    }

    fn render_articles(&self, ui: &mut egui::Ui, cfg: &Config) {
        let scroll_area = ScrollArea::vertical()
            // .max_height(200.0)
            .always_show_scroll(true)
            .auto_shrink([false; 2]);
        scroll_area.show(ui, |ui| {
            for a in &self.articles {
                ui.add_space(5.0);
                ui.heading(RichText::new(&a.title).color(cfg.theme.colors.h2));

                ui.add_space(5.0);
                ui.label(
                    RichText::new(&a.desc).color(cfg.theme.colors.body), // .font(FontId::proportional(40.0)),
                );

                ui.add_space(5.0);
                ui.style_mut().visuals.hyperlink_color = cfg.theme.colors.link;
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), 1.),
                    Layout::right_to_left(),
                    |ui| {
                        ui.add(egui::Hyperlink::from_label_and_url("阅读原文 ⤴", &a.url));
                    },
                );
                // ui.with_layout(Layout::right_to_left(), |ui| {
                // });

                ui.add_space(5.0);
                ui.separator();
            }
        });
    }
}
pub struct NewsArticle {
    id: usize,
    title: String,
    desc: String,
    url: String,
}

impl HeadLine {
    pub fn ui(&mut self, ui: &mut egui::Ui, cfg: &Config) {
        //
        self.render_header(ui);

        self.render_articles(ui, cfg);
    }
}
