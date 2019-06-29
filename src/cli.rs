use std::env;
use getopts::Options;

use super::RunnerOptions;

pub fn parse_options() -> RunnerOptions {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt("i", "input", "input file name", "FILENAME");
    opts.reqopt("o", "output", "output directory name", "DIRNAME");
    opts.reqopt("s", "frame-start", "first frame to render", "NUMBER");
    opts.reqopt("e", "frame-end", "last frame to render", "NUMBER");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(error_message) => {
            println!("Error: {}", error_message);
            println!();
            print_usage(&program, opts);
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    let input_file = matches.opt_str("i").unwrap();
    let output_dir = matches.opt_str("o").unwrap();
    let frame_start = matches.opt_str("s").unwrap();
    let frame_end = matches.opt_str("e").unwrap();

    RunnerOptions {
        input_file,
        output_dir,
        frame_start: frame_start.parse::<u32>().unwrap(),
        frame_end: frame_end.parse::<u32>().unwrap()
    }
}

pub fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
