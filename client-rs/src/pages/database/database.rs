use eframe::{
    egui::{self, Context, RichText, ScrollArea},
    epaint::Color32,
    App,
};

use super::table::Table;

pub struct DataBase {
    items: Vec<String>,
    state: String,
    conns: Vec<String>,
    table: Table,
}

impl App for DataBase {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::panel::TopBottomPanel::top("数据库管理 top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.selectable_value(&mut self.state, "".to_string(), "数据管理");
                ui.selectable_value(&mut self.state, "".to_string(), "监控");
            });
        });

        egui::panel::TopBottomPanel::bottom("数据库管理 bottom").show(ctx, |ui| {
            //
            ui.label("状态栏：您当前正在观测的数据库是 XXX");
        });

        egui::SidePanel::left("数据库管理 sidebar").show(ctx, |ui| {
            ui.heading("数据库连接");
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    self.render_conn(ui);
                });
        });

        egui::panel::CentralPanel::default().show(ctx, |ui| {
            self.table.update(ctx, frame);
        });
    }
}

impl DataBase {
    fn render_conn(&self, ui: &mut egui::Ui) {
        for conn in self.conns.iter() {
            let res = ui.collapsing(RichText::new(conn), |ui| {
                ui.collapsing(RichText::new("dev"), |ui| {
                    ui.collapsing(RichText::new("tables"), |ui| {
                        ui.collapsing(RichText::new("student"), |ui| {
                            if ui.button("P: id").clicked() {}
                            if ui.button("N: name").clicked() {}
                            if ui.button("N: age").clicked() {}
                        });
                        ui.collapsing(RichText::new("class"), |ui| {
                            if ui.button("P: id").clicked() {}
                            if ui.button("N: name").clicked() {}
                            if ui.button("N: teacher").clicked() {}
                        });
                    });
                    ui.collapsing(RichText::new("views"), |ui| {
                        ui.collapsing(RichText::new("Student"), |ui| {
                            ui.label("P: id");
                            if ui.button("P: id").clicked() {}
                            if ui.button("N: name").clicked() {}
                            if ui.button("N: age").clicked() {}
                        });
                        ui.collapsing(RichText::new("Class"), |ui| {});
                    });
                    ui.collapsing(RichText::new("procedures"), |ui| {
                        ui.collapsing(RichText::new("Student"), |ui| {
                            if ui.label("P: id").clicked() {}
                            if ui.button("N: name").clicked() {}
                            if ui.button("N: age").clicked() {}
                        });
                    });
                    ui.collapsing(RichText::new("functions"), |ui| {
                        ui.collapsing(RichText::new("Student"), |ui| {
                            if ui.button("P: id").clicked() {}
                            if ui.button("N: name").clicked() {}
                            if ui.button("N: age").clicked() {}
                        });
                    });
                });
            });
            // if res.header_response.secondary_clicked() {
            res.header_response.context_menu(|ui| {
                egui::menu::bar(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.spacing();
                        if ui.button("查看详细信息").clicked() {};
                        ui.separator();
                        if ui.button("删除链接").clicked() {};
                        ui.separator();
                        if ui.button("测试文本长度度的点点滴滴的点点滴滴的点点滴滴的点点滴滴的点点滴滴单打独斗").clicked() {};
                    });
                });
            });
            // }
        }
    }
}

impl Default for DataBase {
    fn default() -> Self {
        Self {
            items: Default::default(),
            conns: vec![
                "Mysql example.com:3306".into(),
                "Mysql example2.com:3306".into(),
                "Mysql example3.com:3306".into(),
            ],
            state: "aaa".into(),
            table: Default::default(),
        }
    }
}
