use eframe::egui::Visuals;

use crate::theme::Theme;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub theme: Theme,
}

impl Config {
    pub fn new() -> Self {
        let config: Config = confy::load("config").unwrap_or_default();
        config
    }

    pub fn update(&mut self, ctx: &eframe::egui::Context) {
        if let Some(new_font) = self.theme.font.update() {
            ctx.set_fonts(new_font);
        }
        let style = crate::theme::style::init_style();
        ctx.set_style(style);

        if self.theme.dark_mode {
            ctx.set_visuals(Visuals::dark());
            self.theme.to_dark();
        } else {
            ctx.set_visuals(Visuals::light());
            self.theme.to_light();
        }
    }
}
