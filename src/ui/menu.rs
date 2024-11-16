use crate::{load, ui::config::config, Picr};

use egui::{Context, TopBottomPanel};

pub fn menu(picr: &mut Picr, ctx: &Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        picr.files = load(path.clone()).unwrap();
                        picr.picked_path = Some(path);
                        ui.close_menu();
                    }
                }
                if ui.button("Options").clicked() {
                    picr.show_config = true;
                    ui.close_menu();
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

            config(picr, ctx);
            // Add other menus similarly
        });
    });
}
