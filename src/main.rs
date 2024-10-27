#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod file;
mod imgproc;

use {
    eframe::egui,
    egui::{Align, Color32, Layout, RichText, TopBottomPanel},
};

use file::{load, File};

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native("Picr", options, Box::new(|_cc| Ok(Box::<Picr>::default())))
}

#[derive(Default)]
struct Picr {
    files: Vec<File>,
    picked_path: Option<std::path::PathBuf>,
    cursor: usize,
}

impl eframe::App for Picr {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open folder…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.files = load(path.clone()).unwrap();
                            self.picked_path = Some(path);
                        }
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        // Handle "Undo" action
                    }
                    if ui.button("Redo").clicked() {
                        // Handle "Redo" action
                    }
                    if ui.button("Preferences").clicked() {
                        // Handle "Preferences" action
                    }
                });

                // Add other menus similarly
            });
        });
        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::SidePanel::left("menu_panel").show(ctx, |ui| {
                let mut cursor_change = false;
                let prev_cursor = self.cursor;

                egui::ScrollArea::both().auto_shrink(true).show(ui, |ui| {
                    ui.with_layout(Layout::top_down(Align::LEFT).with_main_wrap(false), |ui| {
                        for (i, file) in self.files.iter().enumerate() {
                            ui.horizontal(|ui| {
                                let path = file.path.file_name().unwrap().to_str().unwrap();
                                let indicator = if file.culled {
                                    RichText::new(format!("❌ {}", path)).color(Color32::RED)
                                } else {
                                    RichText::new(format!("✔️{}", path)).color(Color32::GREEN)
                                };
                                let indicator = if i == self.cursor {
                                    ui.scroll_to_cursor(Some(Align::Center));
                                    indicator.underline()
                                } else {
                                    indicator
                                };
                                if ui.link(indicator).clicked() {
                                    cursor_change = true;
                                    self.cursor = i;
                                }
                            });
                        }
                    });
                });
                if cursor_change {
                    self.files[prev_cursor].texture.take();
                }
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                if !self.files.is_empty() {
                    let file = self.files.get_mut(self.cursor).unwrap();
                    file.build_texture(ctx).expect("Couldn't create texture");
                    let texture = file.texture.clone().unwrap();
                    ui.label("Image:");
                    ui.image((texture.id(), texture.size_vec2()));
                }
            });
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let path = i.raw.dropped_files[0].path.clone().unwrap();
                self.files = load(path.clone()).unwrap();
            }
            if i.key_pressed(egui::Key::ArrowRight) && self.cursor < self.files.len() - 1 {
                self.cursor += 1;
            }
            if i.key_pressed(egui::Key::ArrowLeft) && self.cursor > 0 {
                self.cursor -= 1;
            }

            if i.key_pressed(egui::Key::ArrowUp) {
                self.files[self.cursor].culled = !self.files[self.cursor].culled;
            }
        });
    }
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
