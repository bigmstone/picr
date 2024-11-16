use std::{
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
};

use egui::Context;

use crate::ui::progress::progress;

#[derive(Clone)]
enum Status {
    Pass,
    Fail(String),
    Processing(f32),
    NotStarted,
}

pub struct Process {
    thread: Option<JoinHandle<Status>>,
    progress: Arc<Mutex<f32>>,
}

impl Process {
    pub fn process(&mut self) {
        let progress = self.progress.clone();
        self.thread = Some(spawn(move || -> Status {
            let mut prog = progress.lock().unwrap();
            *prog = 0.3;
            Status::Pass
        }));
    }

    pub fn status(&mut self) -> Status {
        if self.thread.is_some() {
            if let Some(thread) = self.thread.as_ref() {
                if !thread.is_finished() {
                    return Status::Processing(*self.progress.lock().unwrap());
                }
            }
        } else {
            return Status::NotStarted;
        }

        self.thread.take().unwrap().join().unwrap()
    }

    pub fn draw(&mut self, ctx: &Context) {
        if let Status::Processing(i) = self.status() {
            progress(ctx, "Processing files", i);
        }
    }
}
