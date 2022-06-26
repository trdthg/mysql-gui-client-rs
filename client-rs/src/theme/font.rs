use eframe::{
    egui::{FontData, FontDefinitions},
    epaint::FontFamily,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Font {
    pub inner: FontDefinitions,
    pub selected: i8,
    pub current: i8,
    pub lists: Vec<String>,
}

impl Default for Font {
    fn default() -> Self {
        let mut fonts = FontDefinitions::default();

        // 加载字体文件，支持 .ttf 和 .otf

        fonts.font_data.insert(
            "霞鹜文楷".to_owned(),
            FontData::from_static(include_bytes!("../../assets/font/LXGWWenKai-Regular.ttf")),
        );
        fonts.font_data.insert(
            "微软雅黑".to_owned(),
            FontData::from_static(include_bytes!("../../assets/font/微软雅黑.ttf")),
        );

        // 设定优先级
        fun_name(&mut fonts)
            .unwrap()
            .insert(0, "霞鹜文楷".to_owned());
        fun_name(&mut fonts)
            .unwrap()
            .insert(1, "微软雅黑".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("微软雅黑".to_owned());

        Self {
            selected: 0,
            current: -1,
            inner: fonts,
            lists: vec!["霞鹜文楷".to_owned(), "微软雅黑".to_owned()],
        }
    }
}

impl Font {
    pub fn update(&mut self) -> Option<FontDefinitions> {
        if self.current == -1 {
            self.current = 0;
            return Some(self.inner.clone());
        }
        if self.selected == self.current {
            return None;
        }
        // 恢复原来的顺序
        self.inner
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .swap(0, self.current as usize);
        // 交换
        self.inner
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .swap(0, self.selected as usize);
        tracing::debug!("{:?}", self.inner.families[&FontFamily::Proportional]);
        // 更新 current
        self.current = self.selected;
        return Some(self.inner.clone());
    }
}

fn fun_name(fonts: &mut FontDefinitions) -> Option<&mut Vec<String>> {
    fonts.families.get_mut(&FontFamily::Proportional)
}
