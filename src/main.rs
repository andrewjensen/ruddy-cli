mod blender;
mod cli;

use blender::run;
use cli::parse_options;


pub struct RunnerOptions {
    input_file: String,
    output_dir: String,
    frame_start: u32,
    frame_end: u32,
}

fn main() {
    let options = parse_options();
    run(options);
}
