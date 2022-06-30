use eframe::{
    egui::{self, Context, RichText, ScrollArea},
    epaint::Color32,
    App,
};

pub struct DataBase {
    items: Vec<String>,
}

impl App for DataBase {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("数据库管理").show(ctx, |ui| {
            //
            ui.label("top");
        });

        egui::panel::TopBottomPanel::bottom("数据库管理 2").show(ctx, |ui| {
            //
            ui.label("bottom");
        });

        egui::SidePanel::left("数据库侧边栏").show(ctx, |ui| {
            ui.heading("侧边栏");
            ui.vertical_centered(|ui| {
                ui.collapsing(RichText::new("大标题 1"), |ui| {
                    if ui.button("子栏目 1").clicked() {}
                    if ui.button("子栏目 2").clicked() {}
                    if ui.button("子栏目 3").clicked() {}
                });
            });
        });

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            //
            ui.label("central");
            self.show_content(ui, ctx);
        });
    }
}

impl DataBase {
    pub fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let scroll_area = ScrollArea::vertical()
            .max_height(200.0)
            .auto_shrink([false; 2]);
        ui.separator();
        let (current_scroll, max_scroll) = scroll_area
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for item in 1..=50 {
                        ui.label(format!("This is item {}", item));
                    }
                });
                let margin = ui.visuals().clip_rect_margin;
                let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
                let max_scroll = ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
                (current_scroll, max_scroll)
            })
            .inner;
        ui.separator();
        ui.vertical_centered(|ui| {
            // egui::reset_button(ui, self);
            // ui.add(crate::egui_github_link_file!());
            ui.label(format!(
                "当前滚动条偏移量：{:.0}/{:.0} px",
                current_scroll, max_scroll
            ));
        });
    }
}

impl Default for DataBase {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}
