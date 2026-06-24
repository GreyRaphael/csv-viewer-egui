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

                self.headers = rdr
                    .headers()
                    .map(|h| h.iter().map(|s| s.to_string()).collect())
                    .unwrap_or_default();

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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
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

        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Rows: {} | Columns: {}",
                    self.rows.len(),
                    self.headers.len()
                ));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
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

            for _ in &self.headers {
                table = table.column(Column::auto().at_least(80.0));
            }

            table
                .header(24.0, |mut header| {
                    for col_name in &self.headers {
                        header.col(|ui| {
                            ui.strong(col_name);
                        });
                    }
                })
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
        });
    }
}
