use eframe::egui::{self, Button, Context, Visuals};

use crate::config::Config;

pub struct Setting {
    config: Config,
}
impl Setting {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn init(&mut self, ctx: &Context) {
        ctx.set_visuals(Visuals::dark());
        self.config.theme.font.update(ctx);
    }
}

impl Setting {
    pub fn ui(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context) {
        #[cfg(feature = "test")]
        ctx.settings_ui(ui);
        self.config.update(ctx);
        ui.collapsing("Theme", |ui| {
            ui.horizontal(|ui| {
                let dark = ui.add(Button::new("黑暗"));
                let light = ui.add(Button::new("明亮"));
                if dark.clicked() {
                    // cfg.theme.dark_mode = true;
                    ctx.set_visuals(Visuals::dark());
                }
                if light.clicked() {
                    // cfg.theme.dark_mode = false;
                    ctx.set_visuals(Visuals::light());
                }
            });

            self.config.theme.show(ui, ctx);
        });
    }
}

impl eframe::App for Setting {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| self.ui(ui, ctx));
    }
}
