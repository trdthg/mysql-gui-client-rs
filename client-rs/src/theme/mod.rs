use self::{colors::Colors, font::Font};

pub mod colors;
pub mod font;
pub mod style;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Theme {
    pub dark_mode: bool,
    pub colors: Colors,
    pub font: Font,
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
}
