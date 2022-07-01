use eframe::egui::{self, Context, ScrollArea};

pub struct Table {
    state: State,
    current_scroll: f32,
    max_scroll: f32,
}
#[derive(PartialEq)]
enum State {
    None,
    Watch,
    Insert,
    Refresh,
    Commit,
}
impl Default for Table {
    fn default() -> Self {
        Self {
            state: State::None,
            current_scroll: 0.,
            max_scroll: 0.,
        }
    }
}

impl eframe::App for Table {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("表管理 top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.selectable_value(&mut self.state, State::Insert, "新增");
                ui.selectable_value(&mut self.state, State::Refresh, "刷新");
                ui.selectable_value(&mut self.state, State::Commit, "提交");
            });
        });

        egui::panel::TopBottomPanel::bottom("表管理 bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // egui::reset_button(ui, self);
                // ui.add(crate::egui_github_link_file!());
                ui.label(format!(
                    "当前滚动条偏移量：{:.0}/{:.0} px",
                    self.current_scroll, self.max_scroll
                ));
            });
        });

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            //
            self.show_content(ui, ctx);
        });
    }
}

impl Table {
    pub fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.vertical(|ui| {
            let scroll_area = ScrollArea::vertical().auto_shrink([false; 2]);
            let (current_scroll, max_scroll) = scroll_area
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for item in 1..=50 {
                            ui.label(format!("This is item {}", item));
                        }
                    });

                    let margin = ui.visuals().clip_rect_margin;
                    let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
                    let max_scroll =
                        ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
                    (current_scroll, max_scroll)
                })
                .inner;
            self.current_scroll = current_scroll;
            self.max_scroll = max_scroll;
        });
    }
}
