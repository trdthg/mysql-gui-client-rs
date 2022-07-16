use eframe::{
    egui::{self, Context, ScrollArea},
    epaint::Color32,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::service::database::{message, DatabaseClient};

use super::{Conns, FieldType};

pub struct Table {
    s: UnboundedSender<message::Message>,
    count: bool,
    edit: EditCtl,
    meta: Option<Box<TableMeta>>,
    code_editor: CodeEditor,
}

impl Table {
    pub fn new(s: UnboundedSender<message::Message>) -> Self {
        Self {
            count: true,
            edit: EditCtl::new(),
            meta: None,
            code_editor: CodeEditor::new(),
            s,
        }
    }
}

pub struct CodeEditor {
    input: String,
    chosed_conn: Option<String>,
    chosed_db: Option<String>,
    tree: Option<Conns>,
}
impl CodeEditor {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            chosed_conn: None,
            chosed_db: None,
            tree: None,
        }
    }
}

pub struct TableMeta {
    pub conn_name: String,
    pub db_name: String,
    pub table_name: String,
    pub fields: Box<Vec<FieldType>>,
    pub datas: Box<Vec<Vec<String>>>,
}

struct EditCtl {
    input_caches: Box<Vec<String>>,
    selected_cell: Option<usize>,
    input_cache: String,
}

impl EditCtl {
    pub fn new() -> Self {
        Self {
            input_caches: Box::new(vec![]),
            selected_cell: None,
            input_cache: String::new(),
        }
    }
}

impl eframe::App for Table {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("表管理 top")
            .resizable(true)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    let current_conn = ui.label("连接");

                    egui::ComboBox::from_id_source(current_conn.id)
                        .selected_text(match &self.code_editor.chosed_conn {
                            Some(conn) => conn.as_str(),
                            None => "None",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.code_editor.chosed_conn, None, "None");
                            if let Some(tree) = &self.code_editor.tree {
                                for conn in tree.keys() {
                                    ui.selectable_value(
                                        &mut self.code_editor.chosed_conn,
                                        Some(conn.to_string()),
                                        conn,
                                    );
                                }
                            }
                        });

                    let current_db = ui.label("数据库");
                    egui::ComboBox::from_id_source(current_db.id)
                        .selected_text(match &self.code_editor.chosed_db {
                            Some(db_name) => db_name.as_str(),
                            None => "None",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.code_editor.chosed_db, None, "None");
                            let chosed_conn =
                                self.code_editor.chosed_conn.to_owned().unwrap_or_default();
                            if let Some(dbs) = self
                                .code_editor
                                .tree
                                .as_ref()
                                .and_then(|tree| tree.get(&chosed_conn))
                                .and_then(|conn| conn.databases.as_ref())
                            {
                                for db in dbs.keys() {
                                    ui.selectable_value(
                                        &mut self.code_editor.chosed_db,
                                        Some(db.to_string()),
                                        db,
                                    );
                                }
                            }
                        });
                    if ui.button("执行").clicked() {
                        //
                        if let Some(conn) = &self.code_editor.chosed_conn {
                            if let Err(e) = self.s.send(message::Message::Select {
                                conn: conn.to_owned(),
                                db: self.code_editor.chosed_db.to_owned(),
                                table: None,
                                fields: None,
                                r#type: message::SelectType::Customed,
                                sql: self.code_editor.input.to_owned(),
                            }) {
                                tracing::error!("后台服务连接断开：{}", e);
                            }
                        }
                    }
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.code_editor.input)
                            .desired_width(f32::INFINITY)
                            .code_editor(),
                    )
                });
            });

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("新增").clicked() {};
                if ui.button("删除").clicked() {};
                if ui.button("刷新").clicked() {};
            });
            ui.separator();
            ScrollArea::horizontal().show(ui, |ui| {
                self.show_content(ui, ctx);
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
    }
}

impl Table {
    pub fn update_content(&mut self, meta: Box<TableMeta>) {
        self.meta = Some(meta);
    }

    pub fn update_conns(&mut self, conns: Conns) {
        self.code_editor.tree = Some(conns);
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        use egui_extras::{Size, TableBuilder};
        tracing::info!("开始渲染表格...");
        if let Some(meta) = &self.meta {
            let fields = &meta.fields;
            let datas = &meta.datas;
            let row_n = datas.len();
            let col_n = fields.len();

            tracing::info!("字段数量：{}", fields.len(),);
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
                let init_width = field.default_width();
                if i == col_n - 1 {
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
                        ui.colored_label(Color32::DARK_GRAY, "多选");
                    });
                }
                for field in fields.iter() {
                    header.col(|ui| {
                        ui.heading(&field.name);
                    });
                }
            });
            tracing::info!("构造 body...");
            tb.body(|body| {
                let height = 18.0 * 2.;
                // 一次添加所有行 (相同高度)（效率最高）
                body.rows(height, row_n, |index, mut row| {
                    if self.count {
                        row.col(|ui| {
                            ui.label(
                                egui::RichText::new((index + 1).to_string())
                                    .color(Color32::DARK_GREEN),
                            );
                        });
                    }
                    for (i, cell) in datas[index].iter().enumerate() {
                        row.col(|ui| {
                            let data_str = cell.as_str();
                            let current_cell_id = index * col_n + i;
                            if self.edit.selected_cell == Some(current_cell_id) {
                                let response = ui.text_edit_singleline(&mut self.edit.input_cache);
                                if response.lost_focus() {
                                    // self.edit.selected_cell = None;
                                }
                                if response.clicked_elsewhere() {
                                    self.edit.selected_cell = None;
                                }
                            } else {
                                let button = egui::Button::new(
                                    egui::RichText::new(data_str), // .font(self.font_id.clone()),
                                )
                                .frame(false);

                                let tooltip_ui = |ui: &mut egui::Ui| {
                                    ui.label(egui::RichText::new(data_str)); // .font(self.font_id.clone()));
                                    ui.label(format!("\n\nClick to copy"));
                                };

                                let response = ui.add(button).on_hover_ui(tooltip_ui);
                                if response.clicked() {
                                    ui.output().copied_text = data_str.to_string();
                                }
                                if response.double_clicked() {
                                    self.edit.selected_cell = Some(current_cell_id);
                                    self.edit.input_cache = data_str.to_string();
                                }
                            }
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
            ui.centered_and_justified(|ui| ui.label("Loading..."));
        }
    }
}
