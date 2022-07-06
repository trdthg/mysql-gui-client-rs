use eframe::egui::{self, Button, Context, Visuals};

#[derive(Default)]
pub struct Setting {}

impl Setting {
    pub fn ui(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context) {
        ctx.settings_ui(ui);
        ui.collapsing("Theme", |ui| {
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
        // ui.separator();
        // let fonts = ctx.fonts();
        // ctx.set_fonts()
        // for (i, font) in cfg.theme.font.lists.iter().enumerate() {
        //     ui.radio_value(&mut cfg.theme.font.selected, i as i8, font);
        // }
    }
}

impl eframe::App for Setting {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| self.ui(ui, ctx));
    }
}
