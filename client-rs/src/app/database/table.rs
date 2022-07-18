use eframe::{
    egui::{self, Context, ScrollArea},
    epaint::Color32,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::service::database::{message, sqls};

use super::{Conns, Field};

pub struct Table {
    s: UnboundedSender<message::Message>,
    count: bool,
    meta: Option<Box<TableMeta>>,
    code_editor: CodeEditor,
    tablectl: TableCtl,
    editctl: EditCtl,
}

impl Table {
    pub fn new(s: UnboundedSender<message::Message>) -> Self {
        Self {
            count: true,
            editctl: EditCtl::new(),
            meta: None,
            code_editor: CodeEditor::new(),
            s,
            tablectl: TableCtl::new(),
        }
    }

    pub fn update_sql(&mut self, sql: &str) {
        self.code_editor.input = sql.to_owned()
    }
}

pub struct TableCtl {
    page: String,
    size: String,
    filter: String,
}
impl TableCtl {
    pub fn new() -> Self {
        Self {
            page: String::from("1"),
            size: String::from("100"),
            filter: String::new(),
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
    pub fields: Box<Vec<Field>>,
    pub datas: Box<Vec<Vec<String>>>,
}

struct EditCtl {
    select_row: Option<usize>,
    selected_cell: Option<usize>,
    input_cache: String,
    input_caches: Box<Vec<String>>,
    adding_new_row: bool,
}

impl EditCtl {
    pub fn new() -> Self {
        Self {
            input_cache: String::new(),
            select_row: None,
            selected_cell: None,
            input_caches: Box::new(vec![]),
            adding_new_row: false,
        }
    }
}

impl Table {
    pub fn refresh(&self) {
        if let Some(meta) = self.meta.as_ref() {
            let page = self.tablectl.page.parse::<usize>().and_then(|x| Ok(x)).ok();
            let size = self.tablectl.size.parse::<usize>().and_then(|x| Ok(x)).ok();
            let sql =
                sqls::select_by_page(meta.db_name.as_str(), meta.table_name.as_str(), page, size);
            if let Err(e) = self.s.send(message::Message::Select {
                conn: meta.conn_name.to_owned(),
                db: Some(meta.db_name.to_owned()),
                table: Some(meta.table_name.to_owned()),
                fields: Some(meta.fields.to_owned()),
                r#type: message::SelectType::Table,
                sql,
            }) {
                tracing::error!("翻页请求失败：{}", e);
            }
        }
    }
    // https://juejin.cn/post/6920043290385448974#3__68
    pub fn update(&mut self, ui: &mut egui::Ui) {
        // 辅助信息
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
                    let chosed_conn = self.code_editor.chosed_conn.to_owned().unwrap_or_default();
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
            if ui.button("执行 ▶").clicked() {
                // ◀
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

        // sql 输入区
        let sql_editor_output = egui::TextEdit::multiline(&mut self.code_editor.input)
            .desired_width(f32::INFINITY)
            .desired_rows(1)
            .code_editor()
            .hint_text("自定义 SQL 语句")
            .show(ui);

        if let Some(text_cursor_range) = sql_editor_output.cursor_range {
            use egui::TextBuffer as _;
            let selected_chars = text_cursor_range.as_sorted_char_range();
            let selected_text = self.code_editor.input.char_range(selected_chars);
            ui.label("Selected text: ");
            ui.monospace(selected_text);
        }

        // 表格控制区
        egui::menu::bar(ui, |ui| {
            ui.horizontal(|ui| {
                egui::TextEdit::singleline(&mut self.tablectl.filter)
                    .hint_text("过滤")
                    .desired_width(100.)
                    .show(ui);

                if ui.button("新增").clicked() {
                    self.editctl.adding_new_row = true;
                };

                // TODO! 有主键或者唯一键才能操作
                // if meta.get_primary_key || unique {}
                if ui.button("删除").clicked() {
                    if let Some(selected_row) = self.editctl.select_row {
                        //
                    }
                };

                if ui.button("刷新").clicked() {
                    self.refresh();
                };

                if ui.button("⏪").clicked() {
                    self.tablectl.page = 0.to_string();
                    self.refresh();
                };
                if ui.button("⏴").clicked() {
                    if let Ok(n) = self.tablectl.page.parse::<usize>() {
                        self.tablectl.page = if n <= 0 {
                            0.to_string()
                        } else {
                            (n - 1).to_string()
                        };
                    }
                    self.refresh();
                }; // ◀
                let page_input = egui::TextEdit::singleline(&mut self.tablectl.page)
                    .hint_text("页")
                    .desired_width(30.)
                    .show(ui);
                if page_input.response.lost_focus() && page_input.response.changed() {
                    self.refresh();
                }
                if ui.button("⏵").clicked() {
                    if let Ok(n) = self.tablectl.page.parse::<usize>() {
                        self.tablectl.page = (n + 1).to_string();
                    }
                    self.refresh();
                }; // ▶
                if ui.button("⏩").clicked() {
                    if let Ok(n) = self.tablectl.page.parse::<usize>() {
                        self.tablectl.page = (n + 1).to_string();
                    }
                    self.refresh();
                };

                ui.label("数量");
                let size_input = egui::TextEdit::singleline(&mut self.tablectl.size)
                    .hint_text("")
                    .desired_width(30.)
                    .show(ui);
                if size_input.response.lost_focus() && size_input.response.changed() {
                    self.refresh();
                }

                ui.label(format!("当前选中行：",));
                if let Some(selected) = self.editctl.select_row {
                    ui.label(
                        egui::RichText::new(format!(" {} ", selected))
                            .color(Color32::WHITE)
                            .background_color(Color32::BLUE),
                    );
                } else {
                    ui.label("None".to_string());
                }
            });
        });
        ui.separator();

        // 表格显示区
        ScrollArea::horizontal().show(ui, |ui| {
            self.show_content(ui);
        });
    }
}

impl Table {
    pub fn update_content(&mut self, meta: Box<TableMeta>) {
        self.editctl.input_caches = Box::new(vec![String::new(); meta.fields.len()]);
        self.meta = Some(meta);
    }

    pub fn update_conns(&mut self, conns: Conns) {
        self.code_editor.tree = Some(conns);
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Size, TableBuilder};
        // tracing::info!("开始渲染表格...");
        if let Some(meta) = &self.meta {
            let fields = &meta.fields;
            let datas = &meta.datas;
            let row_len = datas.len();
            let col_len = fields.len();

            // tracing::info!("字段数量：{}", fields.len(),);
            let mut tb = TableBuilder::new(ui)
                .striped(true)
                .scroll(true)
                .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
                .resizable(true);

            // tracing::info!("设置列数列宽...");
            tb = tb.column(Size::Absolute {
                initial: 20.,
                range: (0., 400.),
            });
            for (i, field) in fields.iter().enumerate() {
                let init_width = field.default_width();
                if i == col_len - 1 {
                    tb = tb.column(Size::Remainder {
                        range: (init_width * 2., f32::INFINITY),
                    });
                } else {
                    let size = Size::initial(init_width).at_least(50.).at_most(400.0);
                    tb = tb.column(size);
                }
            }

            // tracing::info!("构造 header...");
            let tb = tb.header(40.0, |mut header| {
                header.col(|ui| {
                    // ui.colored_label(Color32::DARK_GRAY, "");
                    ui.centered_and_justified(|ui| if ui.button("⚐").clicked() {});
                });
                for field in fields.iter() {
                    header.col(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered_justified(|ui| {
                                ui.heading(&field.name);
                                ui.label(&field.column_type);
                            });
                        });
                    });
                }
            });
            // tracing::info!("构造 body...");

            // 过滤器
            let filtered_indexs = datas
                .iter()
                .enumerate()
                .filter(|(_, x)| {
                    for cell in x.iter() {
                        if cell.contains(self.tablectl.filter.as_str()) {
                            return true;
                        }
                    }
                    return false;
                })
                .map(|x| x.0)
                .collect::<Vec<usize>>();

            tb.body(|mut body| {
                let height = 18.0 * 2.;
                if self.editctl.adding_new_row && self.meta.is_some() {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            if ui.button("取消").clicked() {
                                self.editctl.input_caches.iter_mut().for_each(|x| x.clear());
                                self.editctl.adding_new_row = false;
                            };
                            // TODO!
                            if ui.button("保存").clicked() {
                                // if let Err(e) = self.s.send(message::Message::Insert {}) {
                                //     tracing::error!("后台挂了？ {}", e);
                                // }
                            };
                        });
                        for i in 0..col_len {
                            tracing::info!("{} ", self.editctl.input_caches.len());
                            row.col(|ui| {
                                let response =
                                    ui.text_edit_singleline(&mut self.editctl.input_caches[i]);
                                response.context_menu(|ui| {
                                    if ui.button("置空").clicked() {
                                        self.editctl.input_caches[i] = "".to_string();
                                    }
                                });
                            });
                        }
                    });
                }
                // 一次添加所有行 (相同高度)（效率最高）
                body.rows(height, filtered_indexs.len(), |row_index, mut row| {
                    // 单选框 + 索引
                    row.col(|ui| {
                        ui.radio_value(
                            &mut self.editctl.select_row,
                            Some(row_index),
                            egui::RichText::new(row_index.to_string()).color(Color32::BLUE),
                        );
                    });
                    // 数据行 (从过滤器中展示)
                    for (col_index, cell) in datas[filtered_indexs[row_index]].iter().enumerate() {
                        row.col(|ui| {
                            let data_str = cell.as_str();
                            let current_cell_id = row_index * col_len + col_index;
                            // focus 判断
                            if self.editctl.selected_cell == Some(current_cell_id) {
                                let response =
                                    ui.text_edit_singleline(&mut self.editctl.input_cache);
                                if response.lost_focus() {}
                                // 取消 focus
                                if response.clicked_elsewhere() {
                                    self.editctl.selected_cell = None;
                                    self.editctl.select_row = None;
                                }
                            } else {
                                let button = egui::Button::new(
                                    egui::RichText::new(data_str), // .font(self.font_id.clone()),
                                )
                                .frame(false);

                                let tooltip_ui = |ui: &mut egui::Ui| {
                                    ui.label(egui::RichText::new(data_str)); // .font(self.font_id.clone()));
                                    ui.label(format!("\nClick to copy"));
                                };

                                let response = ui.add(button).on_hover_ui(tooltip_ui);
                                if response.clicked() {
                                    ui.output().copied_text = data_str.to_string();
                                }
                                if response.double_clicked() {
                                    self.editctl.select_row = Some(row_index);
                                    self.editctl.selected_cell = Some(current_cell_id);
                                    self.editctl.input_cache = data_str.to_string();
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
