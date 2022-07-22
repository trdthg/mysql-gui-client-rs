use crate::theme::Theme;

pub const CONFIG_PATH: &str = "test-client-config-rs";

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Config {
    #[serde(skip)]
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Default::default(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        tracing::info!("获取默认配置");
        let config: Config = confy::load(CONFIG_PATH).unwrap_or_default();
        tracing::info!("配置成功");
        config
    }

    pub fn update(&mut self, ctx: &eframe::egui::Context) {
        self.theme.font.update(ctx);
    }
}
