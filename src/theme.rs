use self::{colors::Colors, font::Font};

pub mod colors;
pub mod font;
pub mod style;
pub mod text;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Theme {
    pub dark_mode: bool,
    #[serde(skip)]
    pub colors: Colors,
    #[serde(skip)]
    pub font: Font,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            dark_mode: false,
            colors: Default::default(),
            font: Default::default(),
        }
    }
}

impl Theme {
    pub fn toogle_dark_mode(&mut self) {
        self.dark_mode = !self.dark_mode;
    }

    pub fn to_dark(&mut self) {
        self.colors = Colors::dark();
    }

    pub fn to_light(&mut self) {
        self.colors = Colors::light();
    }

    pub fn show(&mut self, ui: &mut eframe::egui::Ui, ctx: &eframe::egui::Context) {
        self.font.show(ui, ctx);
    }
}
