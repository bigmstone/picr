use crate::Picr;

use egui::{CentralPanel, Context, ScrollArea, TextureHandle, Window};

pub fn image(picr: &mut Picr, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        if !picr.files.is_empty() {
            let file = picr.files.get_mut(picr.cursor).unwrap();
            file.build_texture(ctx).expect("Couldn't create texture");
            let texture = file.texture.clone().unwrap();
            scaled_image(ui, &texture);
            Window::new("Image Properties").show(ctx, |ui| {
                if file.culled && ui.button("Include Image").clicked() {
                    file.culled = false;
                } else if !file.culled && ui.button("Cull Image").clicked() {
                    file.culled = true;
                }
                ScrollArea::both().auto_shrink(true).show(ui, |ui| {
                    for (k, v) in file.metadata.iter() {
                        ui.label(format!("{}: {}", k, v));
                    }
                });
            });
        }
    });
}

fn scaled_image(ui: &mut egui::Ui, texture: &TextureHandle) {
    let available_size = ui.available_size();

    let texture_size = texture.size_vec2();
    let aspect_ratio = texture_size.x / texture_size.y;

    let mut image_size = available_size;
    if available_size.x / available_size.y > aspect_ratio {
        image_size.x = available_size.y * aspect_ratio;
    } else {
        image_size.y = available_size.x / aspect_ratio;
    }

    ui.image((texture.id(), image_size));
}
