use crate::Picr;

use egui::{Context, Window};
#[derive(PartialEq)]
enum Enum {
    First,
}

pub fn config(picr: &mut Picr, ctx: &Context) {
    if !picr.show_config {
        return;
    }

    Window::new("Options").show(ctx, |ui| {
        let mut my_enum = Enum::First;

        ui.label(
            "I'll level with you. This isn't anything right now. It'll turn into
            something in the future.",
        );

        ui.selectable_value(&mut my_enum, Enum::First, "First");

        if ui.button("Save").clicked() {
            picr.show_config = false;
        }
        if ui.button("Close").clicked() {
            picr.show_config = false;
        }
    });
}
