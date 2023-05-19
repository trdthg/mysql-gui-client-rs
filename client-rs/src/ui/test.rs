use chrono::{DateTime, Duration, Local};
use eframe::{
    egui::{self, Id, Layout},
    emath::Pos2,
    epaint::Color32,
};

pub struct Test {
    //
    time: DateTime<Local>,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            time: chrono::Local::now() + Duration::seconds(3),
        }
    }
}

impl eframe::App for Test {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            let text = "这是一条警告信息";
            ui.with_layout(Layout::right_to_left(), |ui| {
                if chrono::Local::now() <= self.time {
                    egui::popup::show_tooltip_at(
                        ctx,
                        Id::new("msgbox"),
                        Some(Pos2::new(ui.available_width() - 100., 10.)),
                        |ui| {
                            //
                            ui.colored_label(Color32::RED, "Error");
                            ui.label(text);
                        },
                    );
                }
            });

            let response = ui.button("Open popup");
            if response.clicked() {
                self.time = chrono::Local::now() + Duration::seconds(3);
            }
            let popup_id = ui.make_persistent_id("my_unique_id");
            if response.clicked() {
                ui.memory().toggle_popup(popup_id);
            }
            egui::popup::popup_below_widget(ui, popup_id, &response, |ui| {
                ui.label("Some more info, or things you can select:");
                ui.label("…");
            });
        });
    }
}
