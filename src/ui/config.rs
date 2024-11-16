use crate::Picr;

use egui::{Context, ScrollArea, Window};
#[derive(PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}

pub fn config(picr: &mut Picr, ctx: &Context) {
    if !picr.show_config {
        return;
    }

    Window::new("Options").show(ctx, |ui| {
        let mut my_enum = Enum::First;

        ui.selectable_value(&mut my_enum, Enum::First, "First");
        if ui
            .add(egui::SelectableLabel::new(my_enum == Enum::First, "First"))
            .clicked()
        {
            my_enum = Enum::First
        }

        if ui.button("Save").clicked() {
            picr.show_config = false;
        }
        if ui.button("Close").clicked() {
            picr.show_config = false;
        }
    });
}