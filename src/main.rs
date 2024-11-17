#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod file;
mod imgproc;
mod process;
mod ui;

use file::{load, File};

use {
    eframe::egui,
    egui::{CentralPanel, Context, ViewportBuilder},
};

#[derive(Default)]
struct Picr {
    files: Vec<File>,
    picked_path: Option<std::path::PathBuf>,
    cursor: usize,
    pub show_config: bool,
    process: process::Process,
}

impl eframe::App for Picr {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ui::menu::menu(self, ctx);
        CentralPanel::default().show(ctx, |_ui| {
            ui::files::files(self, ctx);
            ui::image::image(self, ctx);
        });
        ui::files::preview_files_being_dropped(ctx);
        ui::input::input(self, ctx);
        self.process.draw(ctx);
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native("Picr", options, Box::new(|_cc| Ok(Box::<Picr>::default())))
}
