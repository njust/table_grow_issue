#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{Align, Layout};
use eframe::{egui, Theme};

// use egui_extras::{Size, TableBuilder};
use egui_extras::{Column, TableBuilder};
fn main() {
    let options = eframe::NativeOptions {
        default_theme: Theme::Dark,
        initial_window_size: Some([800., 400.].into()),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    cnt: u32,
    data: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            cnt: 100,
            data: (0..100).enumerate().into_iter().map(|(idx, _)| format!("Some text: {}", idx)).collect(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.set_debug_on_hover(true);
        egui::SidePanel::left("test")
            .default_width(150.)
            .resizable(true)
            .show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.cnt, 1..=200));
            if ui.button("Add long text").clicked() {
                for i in 0..self.cnt {
                    self.data.push(format!("Some long text will expand the grid and ignore the max_width and at_most option: {}", i));
                }
            }

            if ui.button("Add short text").clicked() {
                for i in 0..self.cnt {
                    self.data.push(format!("Some text: {}", i));
                }
            }

            if ui.button("Clear").clicked() {
                self.data.clear();
            }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                TableBuilder::new(ui)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .column(Column::initial(800.).clip(true))
                    .column(Column::auto().clip(true))
                    .max_scroll_height(f32::MAX)
                    .auto_shrink([false;2])
                    .striped(true)
                    .body(|body| {
                        body.rows(20., self.data.len(), |idx, mut row| {
                            let item = self.data.get(idx).expect("Invalid idx");
                            row.col(|col| {
                                col.label(item);
                            });

                            row.col(|col| {
                                col.label("item");
                            });
                        })
                    });
            });
        });
    }
}
