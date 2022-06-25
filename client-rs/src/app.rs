use eframe::{
    egui::{self, Button, Context, Hyperlink, Layout, RichText, Style, TextStyle, TopBottomPanel},
    epaint::FontFamily::Proportional,
    epaint::FontId,
};

use crate::router::Router;

pub struct App {
    router: Router,
}
impl App {
    fn render_top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.label(RichText::new("Bok").heading());
                });
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new("Close"));
                    let refresh_btn = ui.add(Button::new("Refresh"));
                    let theme_btn = ui.add(Button::new("Theme"));
                    if close_btn.clicked() {}
                    if refresh_btn.clicked() {}
                });
            });
        });
    }
    fn render_footer(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            // ui.vertical_centered_justified(|ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(10.);
                ui.label(RichText::new("Api Source: xxx.com").monospace());
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("Made with egui").monospace(),
                    "https://github.com/emilk/egui",
                ));
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("Github").monospace(),
                    "https://github.com/creativcoder/headlines",
                ));
                ui.add_space(10.);
            });
            // });
        });
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            router: Router::default(),
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let font = crate::font::init_font();
        ctx.set_fonts(font);
        let style = init_style();
        ctx.set_style(style);
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.render_top_panel(ctx);
            self.router.ui(ui);
            self.render_footer(ctx);
        });
    }
}

fn init_style() -> Style {
    let mut style: Style = Style::default();
    // style.visuals.dark_mode = false;
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (
            TextStyle::Name("Heading2".into()),
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
