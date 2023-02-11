#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::Arc;
use eframe::egui::{Align, Layout};
use eframe::{egui, Theme};
use parking_lot::RwLock;

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
    data: Arc<RwLock<Vec<String>>>,
}

impl Default for MyApp {
    fn default() -> Self {
        let data = Arc::new(RwLock::new((0..100).enumerate().into_iter().map(|(idx, _)| format!("Some text: {}", idx)).collect()));
        Self {
            cnt: 100,
            data,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.set_debug_on_hover(true);

        let c = ctx.clone();
        let data = self.data.clone();
        std::thread::spawn(move || {
            loop {
                let idx = {
                    let a: &Vec<String> = &data.read();
                    a.len()
                };
                for item in (idx..idx + 100).enumerate().map(|(_, idx)| format!("Some text: {}", idx)) {
                    let data: &mut Vec<String> = &mut data.write();
                    data.push(item);
                }
                c.request_repaint();
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        });

        egui::SidePanel::left("test")
            .default_width(150.)
            .resizable(true)
            .show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.cnt, 1..=200));
            if ui.button("Add long text").clicked() {
                // for i in 0..self.cnt {
                //     self.data.push(format!("Some long text will expand the grid and ignore the max_width and at_most option: {}", i));
                // }
            }

            if ui.button("Add short text").clicked() {
                for i in 0..self.cnt {
                    // self.data.push(format!("Some text: {}", i));
                }
            }

            if ui.button("Clear").clicked() {
                // self.data.clear();
            }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                TableBuilder::new(ui)
                    .stick_to_bottom(true)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .column(Column::initial(800.).clip(true))
                    .column(Column::auto().clip(true))
                    // .column(Size::initial(800.))
                    // .column(Size::remainder())
                    // .max_scroll_height(f32::MAX)
                    // .auto_shrink([false;2])
                    // .striped(true)
                    .body(|body| {
                        let data = self.data.read();
                        body.rows(20., data.len(), |idx, mut row| {
                            let item = data.get(idx).expect("Invalid idx");
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
