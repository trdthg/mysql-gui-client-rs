use std::collections::HashMap;

use eframe::{
    egui::{self, Context, FontData, FontDefinitions, Style, TextStyle},
    epaint::FontFamily,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Font {
    inner: FontDefinitions,
    selected: HashMap<FontFamily, SelectHistory>,
    avalizble_mono: Vec<String>,
    avalizble_prop: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct SelectHistory {
    current: i8,
    selected: i8,
}

impl Default for SelectHistory {
    fn default() -> Self {
        Self {
            current: -1,
            selected: 0,
        }
    }
}

impl Default for Font {
    fn default() -> Self {
        let mut fonts = FontDefinitions::default();

        // 加载字体文件，支持 .ttf 和 .otf
        let fonts_static: Vec<(&str, &[u8])> = vec![
            (
                "微软雅黑",
                &include_bytes!("../../assets/fonts/微软雅黑.ttf")[..],
            ),
            (
                "霞鹜文楷",
                &include_bytes!("../../assets/fonts/LXGWWenKai-Regular.ttf")[..],
            ),
        ];
        for font in fonts_static.iter() {
            fonts
                .font_data
                .insert(font.0.to_owned(), FontData::from_static(font.1));
        }

        let lists = (0..fonts_static.len()).collect::<Vec<usize>>();

        // 设定优先级
        if let Some(fonts) = fonts.families.get_mut(&FontFamily::Proportional) {
            for i in lists.iter() {
                fonts.push(fonts_static[*i].0.to_owned());
            }
        }

        // Put my font as last fallback for monospace:
        if let Some(fonts) = fonts.families.get_mut(&FontFamily::Monospace) {
            for i in lists {
                fonts.push(fonts_static[i].0.to_owned());
            }
        }

        Self {
            selected: HashMap::from([
                (FontFamily::Monospace, SelectHistory::default()),
                (FontFamily::Proportional, SelectHistory::default()),
            ]),
            avalizble_mono: fonts
                .families
                .get(&FontFamily::Monospace)
                .and_then(|x| Some(x.to_owned()))
                .unwrap_or(vec![]),
            avalizble_prop: fonts
                .families
                .get(&FontFamily::Proportional)
                .and_then(|x| Some(x.to_owned()))
                .unwrap_or(vec![]),
            inner: fonts,
        }
    }
}

impl Font {
    fn get_avaliable_fonts(&self, family: &FontFamily) -> Option<&Vec<String>> {
        self.inner.families.get(family)
    }

    fn get_avaliable_fonts_mut(&mut self, family: &FontFamily) -> Option<&mut Vec<String>> {
        self.inner.families.get_mut(family)
    }

    pub fn init(&mut self, ctx: &Context) {}

    pub fn update(&mut self, ctx: &Context) {
        if let Some(font) = self.update_font_family(FontFamily::Monospace) {
            ctx.set_fonts(font);
        }
        if let Some(font) = self.update_font_family(FontFamily::Proportional) {
            ctx.set_fonts(font);
        }
    }

    fn update_font_family(&mut self, family: FontFamily) -> Option<FontDefinitions> {
        if let Some(select_status) = self.selected.get_mut(&family) {
            if select_status.current == -1 {
                select_status.current = 0;
                return Some(self.inner.clone());
            }
            if select_status.selected == select_status.current {
                return None;
            }
            if let Some(fonts) = self.inner.families.get_mut(&family) {
                // 恢复原来的顺序
                fonts.swap(0, select_status.current as usize);
                // 交换
                fonts.swap(0, select_status.selected as usize);
                // 更新 current
                select_status.current = select_status.selected;
                return Some(self.inner.clone());
            }
        }
        return None;
    }

    pub fn show(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context) {
        ui.heading("Monospace");
        if let Some(select_status) = self.selected.get_mut(&FontFamily::Monospace) {
            for (i, font) in self.avalizble_mono.iter().enumerate() {
                ui.radio_value(&mut select_status.selected, i as i8, font);
            }
        }

        ui.heading("Proportional");
        if let Some(select_status) = self.selected.get_mut(&FontFamily::Proportional) {
            for (i, font) in self.avalizble_prop.iter().enumerate() {
                ui.radio_value(&mut select_status.selected, i as i8, font);
            }
        }
    }
}
