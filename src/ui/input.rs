use crate::{load, Picr};

use egui::{Context, Key};

pub fn input(picr: &mut Picr, ctx: &Context) {
    ctx.input(|i| {
        if !i.raw.dropped_files.is_empty() {
            let path = i.raw.dropped_files[0].path.clone().unwrap();
            picr.files = load(path.clone()).unwrap();
        }
        if i.key_pressed(Key::ArrowDown) && picr.cursor < picr.files.len() - 1 {
            picr.cursor += 1;
        }
        if i.key_pressed(Key::ArrowUp) && picr.cursor > 0 {
            picr.cursor -= 1;
        }

        if i.key_pressed(Key::ArrowRight) || i.key_pressed(Key::ArrowLeft) {
            picr.files[picr.cursor].culled = !picr.files[picr.cursor].culled;
        }
    });
}
