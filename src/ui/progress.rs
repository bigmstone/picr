// use crate::Picr;

use egui::{Area, Color32, Context, Id, LayerId, Order, ProgressBar, RichText, TextStyle, Vec2};

pub fn progress(ctx: &Context, title: &str, progress: f32) {
    let painter = ctx.layer_painter(LayerId::new(Order::Background, Id::new("progress_overlay")));
    let screen_rect = ctx.screen_rect();

    painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));

    let progress_bar_width = screen_rect.width() * 0.7;
    let content_size = Vec2::new(progress_bar_width, 60.0);
    let content_position = screen_rect.center() - content_size / 2.0;

    Area::new(Id::new("progress_area"))
        .fixed_pos(content_position)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(title)
                        .color(Color32::WHITE)
                        .text_style(TextStyle::Heading),
                );

                ui.add_sized(
                    [progress_bar_width, 24.0],
                    ProgressBar::new(progress).show_percentage(),
                );
            });
        });
}
