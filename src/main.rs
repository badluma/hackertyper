mod functions;

use clap::Parser;
use functions::*;

// Information for the --help argument
#[derive(Parser, Debug)]
#[command(
    name = "hackertyper",
    about = "A local, customizable CLI alternative for hackertyper.net"
)]

// Define the different Arguments
struct Args {
    /// Speed value
    #[arg(short = 's', long = "speed")]
    speed_arg: Option<u32>,

    /// Path to file
    #[arg(short = 'p', long = "path")]
    path_arg: String,

    /// Text Color
    #[arg(short = 'c', long = "color")]
    color_arg: Option<String>,

    // Do Loop
    #[arg(short = 'l', long = "loop")]
    loop_arg: bool,
}

fn main() {
    clear();

    let args = Args::parse();

    let speed = if let Some(speed) = args.speed_arg {
        speed
    } else {
        4
    };

    let path = args.path_arg;

    let color: String = if let Some(color) = args.color_arg {
        color
    } else {
        "default".to_string()
    };

    let loop_arg = args.loop_arg;

    let contents = std::fs::read_to_string(&path).expect("Failed to read file");
    let chars: Vec<char> = contents.chars().collect();

    if loop_arg {
        loop {
            scan_and_flush(chars.clone(), Some(color.clone()), speed)
        }
    }
}
