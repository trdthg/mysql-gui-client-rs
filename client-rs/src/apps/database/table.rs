use eframe::egui::{self, Context, ScrollArea};

use crate::service::database::sqls::TableMeta;

pub struct Table {
    state: State,
    fields: Option<Vec<TableMeta>>,
    datas: Vec<sqlx::mysql::MySqlRow>,
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
            fields: None,
            datas: vec![],
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
            self.show_content(ui, ctx);
        });
    }
}

impl Table {
    pub fn update_content(&mut self, fields: Vec<TableMeta>, datas: Vec<sqlx::mysql::MySqlRow>) {
        //
        self.fields = Some(fields);
        self.datas = datas;
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        use egui_extras::{Size, TableBuilder};
        tracing::info!("开始渲染表格...");
        if let Some(fields) = &self.fields {
            tracing::info!(
                "字段数量：{} 数据列数：{} {}",
                fields.len(),
                self.datas[0].len(),
                self.datas[0].columns().len(),
            );
            use sqlx::Row;
            let mut tb = TableBuilder::new(ui)
                .striped(true)
                .scroll(true)
                .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
                .resizable(true);
            tracing::info!("设置列数列宽...");
            // for _ in self.fields.iter() {
            //     tb = tb.column(Size::initial(60.0).at_least(40.0));
            // }
            tb = tb.columns(
                Size::Absolute {
                    initial: 50.,
                    range: (10., 200.),
                },
                fields.len(),
            );
            tracing::info!("构造 header...");
            let tb = tb.header(20.0, |mut header| {
                for field in fields.iter() {
                    header.col(|ui| {
                        ui.vertical(|ui| {
                            ui.heading(&field.column_name);
                            ui.heading(&field.data_type);
                            ui.separator();
                        });
                    });
                }
            });
            tracing::info!("构造 body...");
            tb.body(|body| {
                let height = 18.0;
                // 一次添加所有行 (相同高度)（效率最高）
                body.rows(height, self.datas.len(), |index, mut row| {
                    for (i, meta) in self.datas[index].columns().iter().enumerate() {
                        row.col(|ui| {
                            let data_str: String = self.datas[index]
                                .try_get(i)
                                .unwrap_or("default".to_string());
                            ui.label(data_str);
                        });
                    }
                });

                // 动态高度（效率中等）
                // body.heterogeneous_rows(height_iter, |index, mut row| {

                // });

                // 每次添加一行（效率最低）
                // for data in self.datas.iter() {
                //     body.row(height, |mut row| {
                //         for (i, meta) in data.columns().iter().enumerate() {
                //             row.col(|ui| {
                //                 let data_str: String =
                //                     data.try_get(i).unwrap_or("default".to_string());
                //                 ui.label(data_str);
                //             });
                //         }
                //     })
                // }
            });
        } else {
            ui.centered_and_justified(|ui| ui.spinner());
        }
    }
}
