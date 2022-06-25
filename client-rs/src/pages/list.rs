use eframe::egui::{self, ScrollArea};

pub struct List {
    items: Vec<String>,
}
impl Default for List {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}

impl List {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
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
