use crate::service::database::datatype::DataType;
use chrono::DateTime;
use eframe::egui::{self, Context, RichText, ScrollArea};
use rust_decimal::Decimal;

use super::Field;

pub struct Table {
    state: State,
    fields: Option<Vec<Field>>,
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
            ScrollArea::horizontal().show(ui, |ui| {
                self.show_content(ui, ctx);
            });
        });
    }
}

impl Table {
    pub fn update_content(&mut self, fields: Vec<Field>, datas: Vec<sqlx::mysql::MySqlRow>) {
        //
        self.fields = Some(fields);
        self.datas = datas;
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        use egui_extras::{Size, TableBuilder};
        tracing::info!("开始渲染表格...");
        if let Some(fields) = &self.fields {
            tracing::info!("字段数量：{}", fields.len(),);
            use sqlx::Row;
            let mut tb = TableBuilder::new(ui)
                .striped(true)
                .scroll(true)
                .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
                .resizable(true);
            tracing::info!("设置列数列宽...");
            for (i, field) in fields.iter().enumerate() {
                let init_width = match field.datatype {
                    DataType::TinyInt => 50.,
                    DataType::SmallInt => 50.,
                    DataType::Integer => 60.,
                    DataType::BigInt => 80.,
                    DataType::Varchar => 100.,
                    DataType::Char { width } => 10. * width as f32,
                    DataType::Boolean => 50.,
                    DataType::Real => 50.,
                    DataType::Double => 60.,
                    DataType::Decimal { scale, precision } => (scale + precision) as f32 * 10.,
                    DataType::Date => 80.,
                    DataType::Time => 80.,
                    DataType::DateTime => 120.,
                    DataType::TimeStamp => 50.,
                };
                let size = Size::initial(init_width).at_least(50.).at_most(400.0);
                if i == fields.len() {
                    tb = tb.column(Size::remainder());
                } else {
                    tb = tb.column(size);
                }
            }
            tracing::info!("构造 header...");
            let tb = tb.header(20.0, |mut header| {
                for field in fields.iter() {
                    header.col(|ui| {
                        ui.vertical(|ui| {
                            ui.heading(&field.details.column_name);
                            ui.heading(&field.details.data_type);
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
                            let data_str = match fields[i].datatype {
                                DataType::TinyInt => {
                                    let data: i8 = self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::SmallInt => {
                                    let data: i16 =
                                        self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Integer => {
                                    let data: i32 =
                                        self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::BigInt => {
                                    let data: i64 =
                                        self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Varchar => {
                                    let data: String =
                                        self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Char { .. } => {
                                    let data: String =
                                        self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Boolean => {
                                    let data: bool =
                                        self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Real => {
                                    let data: f32 =
                                        self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Double => {
                                    let data: f64 =
                                        self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Decimal { .. } => {
                                    let data: Decimal =
                                        self.datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::DateTime => {
                                    let data: Result<
                                        sqlx::types::chrono::NaiveDateTime,
                                        sqlx::Error,
                                    > = self.datas[index].try_get(i);
                                    if let Ok(data) = data {
                                        let data = data.to_string();
                                        data.to_string()
                                    } else {
                                        "日期时间".to_string()
                                    }
                                }
                                DataType::Date => {
                                    let data: Result<sqlx::types::chrono::NaiveDate, sqlx::Error> =
                                        self.datas[index].try_get(i);
                                    if let Ok(data) = data {
                                        let data = data.to_string();
                                        data.to_string()
                                    } else {
                                        "日期".to_string()
                                    }
                                }
                                DataType::Time => {
                                    let data: Result<sqlx::types::chrono::NaiveTime, sqlx::Error> =
                                        self.datas[index].try_get(i);
                                    if let Ok(data) = data {
                                        let data = data.to_string();
                                        data.to_string()
                                    } else {
                                        "时间".to_string()
                                    }
                                }
                                DataType::TimeStamp => {
                                    let data: Result<chrono::DateTime<chrono::Utc>, sqlx::Error> =
                                        self.datas[index].try_get(i);
                                    if let Ok(data) = data {
                                        let data = data.to_string();
                                        data.to_string()
                                    } else {
                                        "时间戳".to_string()
                                    }
                                }
                            };
                            let label = ui.label(&data_str);
                            label.on_hover_text(RichText::new(&data_str));
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
