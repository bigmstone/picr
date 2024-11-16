use std::fmt::Write;

use crate::Picr;

use egui::{
    Align, Align2, CentralPanel, Color32, Context, Id, LayerId, Layout, Order, RichText,
    ScrollArea, SidePanel, TextStyle,
};

pub fn files(picr: &mut Picr, ctx: &Context) {
    let window_width = ctx.available_rect().width();
    let sidebar_width = window_width * 0.15;
    SidePanel::left("menu_panel")
        .min_width(sidebar_width)
        .show(ctx, |ui| {
            let mut cursor_change = false;
            let prev_cursor = picr.cursor;

            ScrollArea::both().auto_shrink(true).show(ui, |ui| {
                ui.with_layout(Layout::top_down(Align::LEFT).with_main_wrap(false), |ui| {
                    for (i, file) in picr.files.iter().enumerate() {
                        ui.horizontal(|ui| {
                            let path = file.path.file_name().unwrap().to_str().unwrap();
                            let indicator = if file.culled {
                                RichText::new(format!("{}. ❌ {}", i + 1, path)).color(Color32::RED)
                            } else {
                                RichText::new(format!("{}. ✔ {}", i + 1, path))
                                    .color(Color32::GREEN)
                            };
                            let indicator = if i == picr.cursor {
                                ui.scroll_to_cursor(Some(Align::Center));
                                indicator.underline()
                            } else {
                                indicator
                            };
                            if ui.link(indicator).clicked() {
                                cursor_change = true;
                                picr.cursor = i;
                            }
                        });
                    }
                });
            });
            if cursor_change {
                picr.files[prev_cursor].texture.take();
            }
        });

    CentralPanel::default().show(ctx, |ui| {
        if !picr.files.is_empty() {
            let file = picr.files.get_mut(picr.cursor).unwrap();
            file.build_texture(ctx).expect("Couldn't create texture");
            if let Some(texture) = file.texture.as_ref() {
                ui.image((texture.id(), texture.size_vec2()));
            }
        }
    });
}

pub fn preview_files_being_dropped(ctx: &Context) {
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
