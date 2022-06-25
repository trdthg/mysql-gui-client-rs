use eframe::{
    egui::{
        self, Button, Context, Hyperlink, Label, Layout, RichText, ScrollArea, Separator, Style,
        TextStyle, TopBottomPanel,
    },
    epaint::{Color32, FontId},
};

pub struct HeadLine {
    articles: Vec<NewsArticle>,
}

impl Default for HeadLine {
    fn default() -> Self {
        let articles = (0..5)
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
        let seq = Separator::default().spacing(20.);
        ui.add(seq);
    }

    fn render_articles(&self, ui: &mut egui::Ui) {
        let scroll_area = ScrollArea::vertical()
            // .max_height(200.0)
            .always_show_scroll(true)
            .auto_shrink([false; 2]);
        scroll_area.show(ui, |ui| {
            ui.with_layout(Layout::top_down(eframe::emath::Align::Max), |ui| {
                for a in &self.articles {
                    ui.add_space(5.0);
                    ui.heading(RichText::new(&a.title).color(Color32::WHITE));

                    ui.add_space(5.0);
                    ui.label(
                        RichText::new(&a.desc).color(Color32::WHITE), // .font(FontId::proportional(40.0)),
                    );

                    ui.add_space(5.0);
                    ui.style_mut().visuals.hyperlink_color = Color32::LIGHT_BLUE;
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        ui.add(egui::Hyperlink::from_label_and_url("阅读原文 ⤴", &a.url));
                    });

                    ui.add_space(5.0);
                    ui.heading(RichText::new(&a.title).color(Color32::WHITE));
                    ui.separator();
                }
            });
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
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        //
        self.render_header(ui);

        self.render_articles(ui);
    }
}
