use eframe::{
    egui::{Style, TextStyle},
    epaint::{FontFamily::Proportional, FontId},
};

pub fn init_style() -> Style {
    let mut style: Style = Style::default();
    // style.visuals.dark_mode = false;
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (
            TextStyle::Name("H2".into()),
            FontId::new(25.0, Proportional),
        ),
        (
            TextStyle::Name("Context".into()),
            FontId::new(23.0, Proportional),
        ),
        (TextStyle::Body, FontId::new(18.0, Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, Proportional)),
        (TextStyle::Button, FontId::new(14.0, Proportional)),
        (TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    // style.spacing.item_spacing = egui::vec2(10.0, 20.0);
    style
}
