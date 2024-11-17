use std::{
    fs::{create_dir, rename},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
};

use egui::Context;

use crate::ui::progress::progress;

#[derive(Clone)]
enum Status {
    Pass,
    // Fail(String),
    Processing(f32),
    NotStarted,
}

#[derive(Default)]
pub struct Process {
    thread: Option<JoinHandle<Status>>,
    progress: Arc<Mutex<f32>>,
}

impl Process {
    pub fn process(&mut self, files: Vec<(PathBuf, bool)>) {
        if files.is_empty() {
            return;
        }
        let progress = self.progress.clone();
        self.thread = Some(spawn(move || -> Status {
            process_files(&files, progress);
            Status::Pass
        }));
    }

    fn status(&mut self) -> Status {
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

fn process_files(files: &[(PathBuf, bool)], progress: Arc<Mutex<f32>>) {
    let total = files.len();
    let root = {
        let path = Path::new(&files[0].0);
        path.parent().expect("Couldn't get parent dir")
    };
    let mut culled_path = root.to_path_buf();
    culled_path.push("culled/");
    let culled_dir = Path::new(&culled_path);
    if !culled_dir.exists() {
        create_dir(culled_dir).expect("Couldn't create culled directory");
    }

    for (i, file) in files.iter().enumerate() {
        let mut progress = progress.lock().unwrap();
        *progress = i as f32 / total as f32;

        let path = Path::new(&file.0);
        let file_name = path.file_name().expect("Couldn't get file name");

        if file.1 {
            let mut dst_path = culled_path.clone();
            dst_path.push(file_name);
            let destination = Path::new(&dst_path);
            println!("Moving culled file: {:?}", file.0);
            rename(path, destination).expect("Failed to move file");
        }
    }
}
