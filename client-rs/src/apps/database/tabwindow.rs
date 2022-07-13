use eframe::{
    egui::{Context, Frame},
    epaint::Color32,
};

#[derive(Default)]
pub struct TabWindow {
    open: bool,
}

impl TabWindow {
    pub fn run(&mut self, ctx: &Context) {
        let f = Frame::none();
        let f = f.fill(Color32::RED);
        eframe::egui::Window::new("测试窗口")
            .frame(f)
            .show(ctx, |ui| {
                //
                ui.label("一行文本");
            });
    }
}
