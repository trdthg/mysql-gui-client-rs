use eframe::{
    egui::{self, ScrollArea},
    epaint::Color32,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::server::database::message;

use super::types::{Conns, Field, TableRows};

pub struct Table {
    s: UnboundedSender<message::Request>,
    meta: Option<Box<TableMeta>>,
    code_editor: CodeEditor,
    tablectl: TableCtl,
    editctl: EditCtl,
}

pub struct TableCtl {
    page: String,
    size: String,
    filter: String,
    filter_indexs: Vec<usize>,
    orders: Box<Vec<Option<bool>>>,
}

impl TableCtl {
    pub fn new() -> Self {
        Self {
            page: String::from("0"),
            size: String::from("100"),
            filter: String::new(),
            filter_indexs: (0..100).map(|x| x).collect(),
            orders: Box::new(vec![]),
        }
    }
}

pub struct CodeEditor {
    input: String,
    chosed_conn: Option<String>,
    chosed_db: Option<String>,
    chosed_table: Option<String>,
    // avaliable_conns: Vec<String>,
    conns: Option<Conns>,
}
impl CodeEditor {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            chosed_conn: None,
            chosed_db: None,
            chosed_table: None,
            // avaliable_conns: vec![],
            conns: None,
        }
    }
}

pub struct TableMeta {
    pub conn_name: String,
    pub db_name: String,
    pub table_name: String,
    pub fields: Box<Vec<Field>>,
    pub datas: TableRows,
}

struct EditCtl {
    select_row: Option<usize>,
    selected_cell: Option<usize>,
    input_cache: String,
    input_caches: Box<Vec<Option<String>>>,
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
    pub fn update_sql(&mut self, sql: &str) {
        self.code_editor.input = sql.to_owned()
    }

    pub fn show_msg(&self, msg: String) {
        // self
    }

    pub fn update(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // SQL 编辑器
        self.render_sql_editor(ui);

        // 工具栏
        self.render_toolbar(ui);
        ui.separator();

        // 表格
        let scroll_area = ScrollArea::new([true, false]).show(ui, |ui| {
            self.render_table(ui);
        });
    }

    pub fn reset(&mut self) {
        self.editctl.select_row = None;
        self.editctl.adding_new_row = false;
    }

    pub fn refresh(&self) {
        if let Some(meta) = self.meta.as_ref() {
            let page = self.tablectl.page.parse::<usize>().and_then(|x| Ok(x)).ok();
            let size = self.tablectl.size.parse::<usize>().and_then(|x| Ok(x)).ok();
            if let Err(e) = self.s.send(message::Request::SelectTable {
                conn: meta.conn_name.to_owned(),
                db: meta.db_name.to_owned(),
                table: meta.table_name.to_owned(),
                fields: meta.fields.to_owned(),
                page: page.unwrap_or(0),
                size: size.unwrap_or(100),
                orders: Some(self.tablectl.orders.to_owned()),
            }) {
                tracing::error!("翻页请求失败：{}", e);
            }
        }
    }

    pub fn update_content_and_refresh(&mut self, meta: Box<TableMeta>) {
        self.editctl.input_caches = Box::new(vec![None; meta.fields.len()]);
        if let Some(old_meta) = &self.meta {
            if old_meta.conn_name != meta.conn_name
                || old_meta.db_name != meta.db_name
                || old_meta.table_name != meta.table_name
            {
                self.tablectl.orders = Box::new(vec![None; meta.fields.len()]);
            }
        } else {
            self.tablectl.orders = Box::new(vec![None; meta.fields.len()]);
        }
        self.tablectl.filter_indexs = (0..meta.datas.len()).map(|x| x).collect();
        self.meta = Some(meta);
    }

    pub fn update_conns(&mut self, conns: Conns) {
        self.code_editor.conns = Some(conns);
    }

    pub fn update_current_table(
        &mut self,
        conn: Option<String>,
        db: Option<String>,
        table: Option<String>,
    ) {
        self.code_editor.chosed_conn = conn;
        self.code_editor.chosed_db = db;
        self.code_editor.chosed_table = table;
    }
}

impl Table {
    pub fn new(s: UnboundedSender<message::Request>) -> Self {
        Self {
            editctl: EditCtl::new(),
            meta: None,
            code_editor: CodeEditor::new(),
            s,
            tablectl: TableCtl::new(),
        }
    }

    fn render_sql_editor(&mut self, ui: &mut egui::Ui) {
        // 辅助信息
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "标签", |ui| {
                ui.menu_button("title", |ui| {});
            });
            let current_conn = ui.label("连接");
            egui::ComboBox::from_id_source(current_conn.id)
                .selected_text(match &self.code_editor.chosed_conn {
                    Some(conn) => conn.as_str(),
                    None => "None",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.code_editor.chosed_conn, None, "None");
                    if let Some(conns) = &self.code_editor.conns {
                        for conn in conns.borrow().keys() {
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
                    Some(db) => db.as_str(),
                    None => "None",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.code_editor.chosed_db, None, "None");

                    if let CodeEditor {
                        chosed_conn: Some(chosed_conn),
                        chosed_db: Some(chosed_db),
                        ..
                    } = &self.code_editor
                    {
                        if let Some(conns) = self.code_editor.conns.as_ref() {
                            if let Some(dbs) = conns
                                .borrow()
                                .get(chosed_conn)
                                .and_then(|x| x.databases.as_ref())
                            {
                                for db in dbs.keys() {
                                    ui.selectable_value(
                                        &mut self.code_editor.chosed_db,
                                        Some(db.to_string()),
                                        db,
                                    );
                                }
                            }
                        }
                    }
                });

            let current_db = ui.label("表");
            egui::ComboBox::from_id_source(current_db.id)
                .selected_text(match &self.code_editor.chosed_table {
                    Some(db) => db.as_str(),
                    None => "None",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.code_editor.chosed_table, None, "None");

                    if let CodeEditor {
                        chosed_conn: Some(chosed_conn),
                        chosed_db: Some(chosed_db),
                        ..
                    } = &self.code_editor
                    {
                        if let Some(conns) = self.code_editor.conns.as_ref() {
                            if let Some(tables) = conns
                                .borrow()
                                .get(chosed_conn)
                                .and_then(|x| x.databases.as_ref())
                                .and_then(|x| x.get(chosed_db))
                                .and_then(|x| x.tables.as_ref())
                            {
                                for table in tables.keys() {
                                    ui.selectable_value(
                                        &mut self.code_editor.chosed_table,
                                        Some(table.to_string()),
                                        table,
                                    );
                                }
                            }
                        }
                    }
                });

            if ui.button("执行 ▶").clicked() {
                // ◀
                if let Some(conn) = &self.code_editor.chosed_conn {
                    if let Err(e) = self.s.send(message::Request::SelectCustomed {
                        conn: conn.to_owned(),
                        sql: self.code_editor.input.to_owned(),
                    }) {
                        tracing::error!("自定义查询请求失败，后台服务连接断开：{}", e);
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
            .interactive(false)
            .show(ui);

        sql_editor_output.response.on_hover_ui(|ui| {
            ui.label("只能用于查询，暂时不支持增删改等操作");
        });

        // if let Some(text_cursor_range) = sql_editor_output.cursor_range {
        //     use egui::TextBuffer as _;
        //     let selected_chars = text_cursor_range.as_sorted_char_range();
        //     let selected_text = self.code_editor.input.char_range(selected_chars);
        //     ui.label("Selected text: ");
        //     ui.monospace(selected_text);
        // }
    }

    fn render_toolbar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.horizontal(|ui| {
                // 过滤器
                let filter_output = egui::TextEdit::singleline(&mut self.tablectl.filter)
                    .hint_text("过滤")
                    .desired_width(100.)
                    .show(ui);
                if filter_output.response.changed() {
                    if let Some(meta) = &self.meta {
                        self.tablectl.filter_indexs = meta
                            .datas
                            .iter()
                            .enumerate()
                            .filter(|(_, x)| {
                                for cell in x.iter() {
                                    if let Some(cell) = cell {
                                        if cell.contains(self.tablectl.filter.as_str()) {
                                            return true;
                                        }
                                    }
                                }
                                return false;
                            })
                            .map(|x| x.0)
                            .collect::<Vec<usize>>();
                    }
                }

                if ui.button("新增").clicked() {
                    // 当前处于编辑状态就置空
                    // 初始化
                    if let Some(meta) = &self.meta {
                        if self.editctl.adding_new_row == true {
                            self.editctl
                                .input_caches
                                .iter_mut()
                                .enumerate()
                                .for_each(|(i, x)| {
                                    if meta.fields[i].is_nullable == false {
                                        *x = Some(String::new());
                                    } else {
                                        *x = None;
                                    }
                                });
                        }
                    }
                    // toggle
                    self.editctl.adding_new_row = !self.editctl.adding_new_row;
                };

                // TODO! 有主键或者唯一键才能操作
                // if meta.get_primary_key || unique {}
                if ui.button("删除").clicked() {
                    if let Some(selected_row) = self.editctl.select_row {
                        //
                        if let Some(meta) = &self.meta {
                            if let Err(e) = self.s.send(message::Request::Delete {
                                conn: meta.conn_name.to_owned(),
                                db: meta.db_name.to_owned(),
                                table: meta.table_name.to_owned(),
                                fields: meta.fields.clone(),
                                datas: Box::new(meta.datas[selected_row].clone()),
                            }) {
                                tracing::error!("删除请求发送失败 {}", e);
                            }
                        }
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
                if page_input.response.changed() {
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
    }

    fn render_table(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Size, TableBuilder};
        // tracing::info!("开始渲染表格...");
        let meta = self.meta.as_ref();
        if meta.is_none() {
            ui.centered_and_justified(|ui| ui.heading("Loading..."));
            return;
        }
        let meta = meta.unwrap();

        let fields = &meta.fields;
        let datas = &meta.datas;
        let row_len = datas.len();
        let col_len = fields.len();

        // 构造表格
        // tracing::info!("字段数量：{}", fields.len(),);
        let mut tb = TableBuilder::new(ui)
            .striped(true)
            .scroll(true)
            .cell_layout(
                egui::Layout::left_to_right(eframe::emath::Align::Min)
                    .with_cross_align(egui::Align::Center),
            )
            .resizable(true);

        // 设置列数，列宽
        tb = tb.column(Size::Absolute {
            initial: 50.,
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

        // 设置表头
        // tracing::info!("构造 header...");
        let tb = tb.header(40.0, |mut header| {
            header.col(|ui| {
                // ui.colored_label(Color32::DARK_GRAY, "");
                ui.centered_and_justified(|ui| if ui.button("⚐").clicked() {});
            });
            for (i, field) in fields.iter().enumerate() {
                header.col(|ui| {
                    // ui.centered_and_justified(|ui| {
                    if ui
                        .button(format!("{}\n{}", &field.name, &field.column_type))
                        .clicked()
                    {
                        if let Some(order) = self.tablectl.orders.get_mut(i) {
                            tracing::info!("{:?}", order);
                            *order = {
                                match order {
                                    Some(true) => Some(false),
                                    Some(false) => None,
                                    None => Some(true),
                                }
                            };
                        }
                        self.s.send(message::Request::SelectTable {
                            conn: meta.conn_name.to_owned(),
                            db: meta.db_name.to_owned(),
                            table: meta.table_name.to_owned(),
                            fields: meta.fields.to_owned(),
                            orders: Some(self.tablectl.orders.to_owned()),
                            page: 0,
                            size: 100,
                        });
                    }
                    // ui.vertical_centered_justified(|ui| {
                    //     ui.heading(&field.name);
                    //     ui.label(&field.column_type);
                    // });
                    // });
                });
            }
        });

        // 渲染表格数据
        tb.body(|mut body| {
            let height = 18.0 * 2.;

            // 添加新行
            if self.editctl.adding_new_row {
                body.row(height, |mut row| {
                    row.col(|ui| {
                        // TODO!
                        if ui.button("保存").clicked() {
                            if let Err(e) = self.s.send(message::Request::Insert {
                                conn: meta.conn_name.to_owned(),
                                db: meta.db_name.to_owned(),
                                table: meta.table_name.to_owned(),
                                fields: meta.fields.to_owned(),
                                datas: self.editctl.input_caches.clone(),
                            }) {
                                tracing::error!("新增请求发送失败，后台挂了？ {}", e);
                            }
                        };
                    });
                    for i in 0..col_len {
                        // tracing::info!("{} ", self.editctl.input_caches.len());
                        row.col(|ui| {
                            if let Some(s) = self.editctl.input_caches[i].as_mut() {
                                // let response = ui.text_edit_singleline(s);
                                let response = egui::TextEdit::singleline(s)
                                    .desired_width(f32::INFINITY)
                                    .show(ui);
                                response.response.context_menu(|ui| {
                                    // 新增是置空
                                    if meta.fields[i].is_nullable == true {
                                        if ui.button("置空").clicked() {
                                            self.editctl.input_caches[i].take();
                                            ui.close_menu();
                                        }
                                    } else {
                                        if ui.button("该列不能置空").clicked() {};
                                    }
                                });
                            } else {
                                if ui.button("编辑").clicked() {
                                    self.editctl.input_caches[i] = Some(String::new());
                                }
                            }
                        });
                    }
                });
            }

            // 一次添加所有行 (相同高度)（效率最高）
            body.rows(
                height,
                self.tablectl.filter_indexs.len(),
                |row_index, mut row| {
                    // 单选框 + 索引
                    row.col(|ui| {
                        ui.radio_value(
                            &mut self.editctl.select_row,
                            Some(row_index),
                            egui::RichText::new(row_index.to_string()).color(Color32::BLUE),
                        );
                    });

                    // 数据行 (从过滤器中展示)
                    for (col_index, cell) in datas[self.tablectl.filter_indexs[row_index]]
                        .iter()
                        .enumerate()
                    {
                        row.col(|ui| {
                            let current_cell_id = row_index * col_len + col_index;
                            // 是否是编辑模式
                            if self.editctl.selected_cell == Some(current_cell_id) {
                                // 编辑模式
                                let response =
                                    ui.text_edit_singleline(&mut self.editctl.input_cache);

                                // 处理保存事件
                                if response.has_focus() {
                                    if let Some(selected_row) = self.editctl.select_row {
                                        if ui
                                            .input_mut()
                                            .consume_key(egui::Modifiers::CTRL, egui::Key::S)
                                        {
                                            tracing::info!("尝试提交编辑 row_id {}", selected_row);
                                            if let Err(e) = self.s.send(message::Request::Update {
                                                conn: meta.conn_name.to_owned(),
                                                db: meta.db_name.to_owned(),
                                                table: meta.table_name.to_owned(),
                                                fields: meta.fields.to_owned(),
                                                datas: Box::new(meta.datas[selected_row].clone()),
                                                new_data_index: col_index,
                                                new_data: Box::new(Some(
                                                    self.editctl.input_cache.to_owned(),
                                                )),
                                            }) {
                                                tracing::error!("发送编辑请求失败：{}", e);
                                            }
                                        }
                                    }
                                }
                                // 处理失去焦点时间
                                if response.clicked_elsewhere() {
                                    self.editctl.selected_cell = None;
                                    self.editctl.input_cache.clear();
                                    self.editctl.select_row = None;
                                }
                            } else {
                                // 显示模式

                                // 初始化样式
                                // 拿到应该显示的内容荣
                                let data_str = cell
                                    .as_ref()
                                    .and_then(|x| Some(x.as_str()))
                                    .unwrap_or("(NULL)");
                                let mut label = egui::RichText::new(data_str);
                                // 给予显示的颜色
                                if cell.is_none() {
                                    label = label.color(Color32::GRAY);
                                }
                                // 防置标签
                                let button = egui::Button::new(
                                    label.clone(), // .font(self.font_id.clone()),
                                )
                                .frame(false);

                                // 处理事件
                                // hover 是显示详细内容
                                let response = ui.add(button).on_hover_ui(|ui| {
                                    ui.label(label); // .font(self.font_id.clone()));
                                    ui.label(format!("\nClick to copy"));
                                });

                                // 单击复制
                                if response.clicked() {
                                    ui.output().copied_text = data_str.to_string();
                                }
                                // 双击切换到输入模式
                                if response.double_clicked() {
                                    self.editctl.select_row = Some(row_index);
                                    self.editctl.selected_cell = Some(current_cell_id);
                                    self.editctl.input_cache = if cell.is_none() {
                                        String::new()
                                    } else {
                                        data_str.to_string()
                                    };
                                }
                                // 右键菜单功能
                                response.context_menu(|ui| {
                                    // 发送置空请求
                                    if ui.button("置为空 (NULL)").clicked() {
                                        tracing::info!("尝试提交编辑 row_id {}", row_index);
                                        if let Err(e) = self.s.send(message::Request::Update {
                                            conn: meta.conn_name.to_owned(),
                                            db: meta.db_name.to_owned(),
                                            table: meta.table_name.to_owned(),
                                            fields: meta.fields.to_owned(),
                                            datas: Box::new(meta.datas[row_index].clone()),
                                            new_data_index: col_index,
                                            new_data: Box::new(None),
                                        }) {
                                            tracing::error!("发送编辑 (置空) 请求失败：{}", e);
                                        }
                                    }
                                });
                            }
                        });
                    }
                },
            );

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
    }
}
