use egui::Button;

use crate::config::Config;

#[derive(Default)]
pub struct Setting {}

impl Setting {
    pub fn ui(&mut self, ui: &mut egui::Ui, cfg: &mut Config) {
        ui.collapsing("Theme", |ui| {
            let dark = ui.add(Button::new("黑暗"));
            let light = ui.add(Button::new("明亮"));
            if dark.clicked() {
                cfg.theme.dark_mode = true;
            }
            if light.clicked() {
                cfg.theme.dark_mode = false;
            }
        });
        ui.separator();
        for (i, font) in cfg.theme.font.lists.iter().enumerate() {
            ui.radio_value(&mut cfg.theme.font.selected, i as i8, font);
        }

        // cfg.theme.font.update(1);
    }
}
