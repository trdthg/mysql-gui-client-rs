use eframe::egui::{self, Context, ScrollArea};

pub struct Table {
    state: State,
    current_scroll: f32,
    max_scroll: f32,
}
#[derive(PartialEq)]
enum State {
    None,
    Watch,
    Insert,
    Refresh,
    Commit,
}
impl Default for Table {
    fn default() -> Self {
        Self {
            state: State::None,
            current_scroll: 0.,
            max_scroll: 0.,
        }
    }
}

impl eframe::App for Table {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("表管理 top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.selectable_value(&mut self.state, State::Insert, "新增");
                ui.selectable_value(&mut self.state, State::Refresh, "刷新");
                ui.selectable_value(&mut self.state, State::Commit, "提交");
            });
        });

        egui::panel::TopBottomPanel::bottom("表管理 bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // egui::reset_button(ui, self);
                // ui.add(crate::egui_github_link_file!());
                ui.label(format!("当前滚动条偏移量：px",));
                ui.horizontal(|ui| {
                    if ui.button("奇妙的东西").clicked() {};
                    if ui.button("奇妙的东西").clicked() {};
                    if ui.button("奇妙的东西").clicked() {};
                });
            });
        });

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            //
            self.show_content(ui, ctx);
            //             use egui_extras::{Size, StripBuilder};
            // StripBuilder::new(ui)
            //     .size(Size::remainder()) // for the table
            //     .size(Size::exact(10.)) // for the source code link
            //     .vertical(|mut strip| {
            //         strip.cell(|ui| {
            //             self.show_content(ui, ctx);
            //         });
            //         strip.cell(|ui| {
            //             ui.horizontal(|ui| {
            //                 if ui.button("奇妙的东西").clicked() {};
            //                 if ui.button("奇妙的东西").clicked() {};
            //                 if ui.button("奇妙的东西").clicked() {};
            //             });
            //         });
            //     });
        });
    }
}

impl Table {
    pub fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        use egui_extras::{Size, TableBuilder};
        TableBuilder::new(ui)
            .striped(true)
            .scroll(true)
            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
            .column(Size::initial(60.0).at_least(40.0))
            .column(Size::initial(60.0).at_least(40.0))
            .column(Size::initial(60.0).at_least(40.0))
            .column(Size::initial(60.0).at_least(40.0))
            .column(Size::initial(60.0).at_least(40.0))
            .column(Size::initial(90.0).at_least(90.0))
            .column(Size::remainder().at_least(60.0))
            // .resizable(self.resizable)
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Row");
                });
                header.col(|ui| {
                    ui.heading("Clock");
                });
                header.col(|ui| {
                    ui.heading("Content");
                });
                header.col(|ui| {
                    ui.heading("Extra 1");
                });
                header.col(|ui| {
                    ui.heading("Extra 2");
                });
                header.col(|ui| {
                    ui.heading("Extra 3");
                });
            })
            .body(|mut body| {
                for row_index in 0..100 {
                    let is_thick = row_index % 2 == 0;
                    let row_height = if is_thick { 30.0 } else { 18.0 };
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            ui.style_mut().wrap = Some(false);
                            if is_thick {
                                ui.heading("Extra thick row");
                            } else {
                                ui.label("Normal row");
                            }
                        });
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            let mut text = format!("{row_index}aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
                            ui.text_edit_singleline(&mut text);
                        });
                    });
                }
            });
    }
}
