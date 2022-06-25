use eframe::{
    egui::{FontData, FontDefinitions},
    epaint::FontFamily,
};

pub fn init_font() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "my_font".to_owned(),
        FontData::from_static(include_bytes!("../assets/微软雅黑.ttf")),
    ); // .ttf and .otf supported

    // Put my font first (highest priority):
    fun_name(&mut fonts)
        .unwrap()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("my_font".to_owned());

    fonts
}

fn fun_name(fonts: &mut FontDefinitions) -> Option<&mut Vec<String>> {
    fonts.families.get_mut(&FontFamily::Proportional)
}
