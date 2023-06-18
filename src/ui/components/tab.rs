use eframe::egui;

pub trait TabInner: Send + Sync {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct Tab {
    pub icon: char,
    pub title: String,
    pub inner: Box<dyn TabInner>,
}

impl ToString for Tab {
    fn to_string(&self) -> String {
        format!("{}  {}", self.icon, self.title)
    }
}

impl Tab {
    pub fn new(icon: char, title: impl Into<String>, inner: impl TabInner + 'static) -> Self {
        Self {
            icon,
            title: title.into(),
            inner: Box::new(inner),
        }
    }
}
