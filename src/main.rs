use egui_extras::{Column, TableBuilder};
use std::path::PathBuf;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSV Viewer (egui)",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

struct App {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    file_path: Option<PathBuf>,
    striped: bool,
    resizable: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
            file_path: None,
            striped: true,
            resizable: true,
        }
    }
}

impl App {
    fn load_csv(&mut self, path: &PathBuf) {
        match std::fs::File::open(path) {
            Ok(file) => {
                let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .from_reader(file);

                // 读取表头
                self.headers = rdr
                    .headers()
                    .map(|h| h.iter().map(|s| s.to_string()).collect())
                    .unwrap_or_default();

                // 读取所有行
                self.rows = rdr
                    .records()
                    .filter_map(|r| r.ok())
                    .map(|r| r.iter().map(|s| s.to_string()).collect())
                    .collect();

                self.file_path = Some(path.clone());
            }
            Err(e) => {
                eprintln!("Failed to open CSV: {e}");
            }
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 顶部工具栏
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("📂 Open CSV").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("CSV", &["csv", "tsv"])
                        .pick_file()
                    {
                        self.load_csv(&path);
                    }
                }

                if let Some(path) = &self.file_path {
                    ui.label(format!("📄 {}", path.display()));
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.checkbox(&mut self.resizable, "Resizable");
                    ui.checkbox(&mut self.striped, "Striped");
                });
            });
        });

        // 底部状态栏
        egui::Panel::bottom("statusbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Rows: {} | Columns: {}",
                    self.rows.len(),
                    self.headers.len()
                ));
            });
        });

        // 主区域：表格（直接在 ui 上绘制，无需再套 CentralPanel）
        if self.headers.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);
                ui.heading("No CSV loaded");
                ui.label("Click \"Open CSV\" to get started");
            });
            return;
        }

        let text_height = egui::TextStyle::Body.resolve(ui.style()).size;

        let mut table = TableBuilder::new(ui)
            .striped(self.striped)
            .resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .sense(egui::Sense::click());

        // 动态列：每列 Column::auto()，让 egui 自动计算宽度
        for _ in &self.headers {
            table = table.column(Column::auto().at_least(80.0));
        }

        // 表头
        table
            .header(24.0, |mut header| {
                for col_name in &self.headers {
                    header.col(|ui| {
                        ui.strong(col_name);
                    });
                }
            })
            // 表体：虚拟滚动
            .body(|body| {
                body.rows(text_height, self.rows.len(), |mut row| {
                    let idx = row.index();
                    for cell in &self.rows[idx] {
                        row.col(|ui| {
                            ui.label(cell);
                        });
                    }
                });
            });
    }
}
