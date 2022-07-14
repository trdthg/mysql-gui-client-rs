use crate::service::database::datatype::DataType;
use eframe::{
    egui::{self, Context, Label, RichText, ScrollArea, Sense},
    epaint::Color32,
};
use rust_decimal::Decimal;

use super::Field;

pub struct Table {
    state: State,
    fields: Option<Box<Vec<Field>>>,
    datas: Option<Box<Vec<sqlx::mysql::MySqlRow>>>,
    count: bool,
    input_cache: Box<Vec<String>>,
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
            datas: None,
            count: true,
            input_cache: Box::new(vec![]),
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
    pub fn update_content(
        &mut self,
        fields: Box<Vec<Field>>,
        datas: Box<Vec<sqlx::mysql::MySqlRow>>,
    ) {
        //
        let mut v = Vec::new();
        for _ in 0..fields.len() {
            v.push(String::new())
        }
        self.input_cache = Box::new(v);
        self.fields = Some(fields);
        self.datas = Some(datas);
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        use egui_extras::{Size, TableBuilder};
        tracing::info!("开始渲染表格...");
        if let (Some(fields), Some(datas)) = (&self.fields, &self.datas) {
            tracing::info!("字段数量：{}", fields.len(),);
            use sqlx::Row;
            let mut tb = TableBuilder::new(ui)
                .striped(true)
                .scroll(true)
                .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
                .resizable(true);

            tracing::info!("设置列数列宽...");
            if self.count {
                tb = tb.column(Size::Absolute {
                    initial: 20.,
                    range: (0., 40.),
                });
            }
            for (i, field) in fields.iter().enumerate() {
                let init_width = field.datatype.get_default_width();
                if i == fields.len() - 1 {
                    tb = tb.column(Size::Remainder {
                        range: (init_width * 2., f32::INFINITY),
                    });
                } else {
                    let size = Size::initial(init_width).at_least(50.).at_most(400.0);
                    tb = tb.column(size);
                }
            }

            tracing::info!("构造 header...");
            let tb = tb.header(20.0, |mut header| {
                if self.count {
                    header.col(|ui| {
                        ui.colored_label(Color32::DARK_GRAY, "");
                    });
                }
                for field in fields.iter() {
                    header.col(|ui| {
                        ui.heading(&field.details.column_name);
                    });
                }
            });
            tracing::info!("构造 body...");
            tb.body(|body| {
                let height = 18.0 * 2.;
                // 一次添加所有行 (相同高度)（效率最高）
                body.rows(height, datas.len(), |index, mut row| {
                    if self.count {
                        row.col(|ui| {
                            ui.label((index + 1).to_string());
                        });
                    }
                    for (i, meta) in datas[index].columns().iter().enumerate() {
                        row.col(|ui| {
                            let data_str = match fields[i].datatype {
                                DataType::TinyInt => {
                                    let data: i8 = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::SmallInt => {
                                    let data: i16 = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Integer => {
                                    let data: i32 = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::BigInt => {
                                    let data: i64 = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Varchar => {
                                    let data: String = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Char { .. } => {
                                    let data: String = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Boolean => {
                                    let data: bool = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Real => {
                                    let data: f32 = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Double => {
                                    let data: f64 = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::Decimal { .. } => {
                                    let data: Decimal = datas[index].try_get(i).unwrap_or_default();
                                    data.to_string()
                                }
                                DataType::DateTime => {
                                    let data: Result<
                                        sqlx::types::chrono::NaiveDateTime,
                                        sqlx::Error,
                                    > = datas[index].try_get(i);
                                    if let Ok(data) = data {
                                        let data = data.to_string();
                                        data.to_string()
                                    } else {
                                        "日期时间".to_string()
                                    }
                                }
                                DataType::Date => {
                                    let data: Result<sqlx::types::chrono::NaiveDate, sqlx::Error> =
                                        datas[index].try_get(i);
                                    if let Ok(data) = data {
                                        let data = data.to_string();
                                        data.to_string()
                                    } else {
                                        "日期".to_string()
                                    }
                                }
                                DataType::Time => {
                                    let data: Result<sqlx::types::chrono::NaiveTime, sqlx::Error> =
                                        datas[index].try_get(i);
                                    if let Ok(data) = data {
                                        let data = data.to_string();
                                        data.to_string()
                                    } else {
                                        "时间".to_string()
                                    }
                                }
                                DataType::TimeStamp => {
                                    let data: Result<chrono::DateTime<chrono::Utc>, sqlx::Error> =
                                        datas[index].try_get(i);
                                    if let Ok(data) = data {
                                        let data = data.to_string();
                                        data.to_string()
                                    } else {
                                        "时间戳".to_string()
                                    }
                                }
                            };

                            let label = Label::new(&data_str).sense(Sense::click());
                            let label = ui.add(label).on_hover_ui(|ui| {
                                ui.vertical(|ui| {
                                    ui.small("点击复制");
                                    ui.label(&data_str);
                                });
                            });
                            if label.clicked() {
                                ui.output().copied_text = data_str.to_owned();
                            }

                            if label.secondary_clicked() {
                                self.input_cache[i] = data_str.to_owned();
                            }

                            label.context_menu(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("编辑");
                                    ui.text_edit_singleline(&mut self.input_cache[i]);
                                });
                            });
                        });
                    }
                });

                // 动态高度（效率中等）
                // body.heterogeneous_rows(height_iter, |index, mut row| {

                // });

                // 每次添加一行（效率最低）
                // for data in datas.iter() {
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
