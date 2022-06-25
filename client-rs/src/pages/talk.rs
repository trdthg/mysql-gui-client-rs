use eframe::{
    egui::{self, RichText, ScrollArea},
    epaint::Color32,
};

use crate::client::{message::Message, Client};

pub struct Talk {
    count: usize,
    inner_count: usize,
    client: Client,
    state: State,
    text: String,
    msg_buf: Vec<Message>,
}

#[derive(PartialEq)]
pub enum State {
    First,
    Second,
}
impl Default for State {
    fn default() -> Self {
        Self::First
    }
}

impl Default for Talk {
    fn default() -> Self {
        let client = Client::new(([127, 0, 0, 1], 1234)).unwrap();
        Self {
            count: Default::default(),
            inner_count: Default::default(),
            client,
            state: Default::default(),
            text: Default::default(),
            msg_buf: Default::default(),
        }
    }
}

impl Talk {
    pub fn new(client: Client) -> Self {
        Self {
            count: 0,
            inner_count: 0,
            client,
            state: State::First,
            text: String::new(),
            msg_buf: vec![],
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // table.ui(ui);self.msg_buf.borrow()
        ui.add(egui::DragValue::new(&mut self.count));
        self.count += 1;
        self.inner_count += 1;
        ui.heading("Q Talk 轻聊");

        ui.separator();
        ui.label(format!("聊天展示区"));

        ui.vertical(|ui| {
            let scroll_area = ScrollArea::vertical()
                .max_height(300.0)
                .auto_shrink([false; 2]);
            scroll_area.show(ui, |ui| {
                for msg in self.msg_buf.iter() {
                    if let Message::Normal { msg, user } = msg {
                        let user = {
                            if user.is_empty() {
                                "default"
                            } else {
                                &user
                            }
                        };
                        ui.label(RichText::new(user));
                        let text = format!("[{}] {}", user.clone(), msg.clone());
                        ui.label(
                            RichText::new(text)
                                .background_color(Color32::LIGHT_YELLOW)
                                .monospace(),
                        );
                    }
                }
            });
        });

        // ui.image(, [640.0, 480.0]);
        ui.separator();
        let output = egui::TextEdit::multiline(&mut self.text)
            .hint_text("请输入消息")
            .show(ui);
        let anything_selected = output
            .cursor_range
            .map_or(false, |cursor| !cursor.is_empty());
        ui.add_enabled(
            anything_selected,
            egui::Label::new("Press ctrl+T to toggle the case of selected text (cmd+T on Mac)"),
        );

        if ui
            .input_mut()
            .consume_key(egui::Modifiers::CTRL, egui::Key::ArrowLeft)
        {
            if let Some(text_cursor_range) = output.cursor_range {
                use egui::TextBuffer as _;
                let selected_chars = text_cursor_range.as_sorted_char_range();
                let selected_text = self.text.char_range(selected_chars.clone());
                let upper_case = selected_text.to_uppercase();
                let new_text = if selected_text == upper_case {
                    selected_text.to_lowercase()
                } else {
                    upper_case
                };
                self.text.delete_char_range(selected_chars.clone());
                self.text.insert_text(&new_text, selected_chars.start);
            }
        }

        if ui
            .input_mut()
            .consume_key(egui::Modifiers::NONE, egui::Key::Enter)
        {
            loop {
                let msg = Message::Normal {
                    user: "".to_string(),
                    msg: self.text.clone(),
                };
                match self.client.send_msg(msg.clone()) {
                    Ok(_) => {
                        self.msg_buf.push(msg);
                        self.text.clear();
                        self.count += 1;
                        ui.ctx().memory().request_focus(output.response.id);
                        break;
                    }
                    Err(_) => self.client.connect().expect("重连失败"),
                }
            }
        }
        ui.horizontal(|ui| {
            ui.label("模式：");
            ui.radio_value(&mut self.state, State::First, "发送全体");
            ui.radio_value(&mut self.state, State::Second, "私聊模式");
            if ui.button("发送").clicked() {}
        });
        ui.horizontal(|ui| {
            ui.label(format!("更新次数：{}", self.count));
            ui.label(format!("UI 更新次数：{}", self.inner_count));
            ui.collapsing("查看使用说明！", |ui| {
                ui.label("Not much, as it turns out");
            });
        });
    }
}
