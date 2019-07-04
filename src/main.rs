mod blender;
mod cli;
mod status;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use blender::exec_blender;
use cli::parse_options;
use status::display_status;

pub struct RunnerOptions {
    input_file: String,
    output_dir: String,
    frame_start: u32,
    frame_end: u32,
}

pub enum StatusUpdate {
    Started,
    RenderedFrame {
        frame_number: u32,
        render_time: u32,
    },
    Finished
}

fn main() {
    let options = parse_options();

    let options_exec = Arc::new(options);
    let options_display = options_exec.clone();

    let (tx, rx) = mpsc::channel();

    let exec_thread = thread::spawn(move || {
        exec_blender(options_exec, tx);
    });

    let status_thread = thread::spawn(move || {
        display_status(options_display, rx);
    });

    exec_thread.join().unwrap();
    status_thread.join().unwrap();
}
