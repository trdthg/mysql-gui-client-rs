use eframe::epaint::Color32;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Colors {
    pub h1: Color32,
    pub h2: Color32,
    pub h3: Color32,
    pub h4: Color32,
    pub h5: Color32,
    pub h6: Color32,
    pub body: Color32,
    pub small: Color32,
    pub link: Color32,
}

impl Colors {
    pub fn light() -> Self {
        Self {
            h1: Color32::BLACK,
            h2: Color32::BLACK,
            h3: Color32::BLACK,
            h4: Color32::BLACK,
            h5: Color32::BLACK,
            h6: Color32::BLACK,
            body: Color32::BLACK,
            small: Color32::GREEN,
            link: Color32::BLUE,
        }
    }
    pub fn dark() -> Self {
        Self {
            h1: Color32::WHITE,
            h2: Color32::WHITE,
            h3: Color32::WHITE,
            h4: Color32::WHITE,
            h5: Color32::WHITE,
            h6: Color32::WHITE,
            body: Color32::WHITE,
            small: Color32::WHITE,
            link: Color32::RED,
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::light()
    }
}
