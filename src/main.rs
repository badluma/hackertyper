mod functions;

use clap::Parser;
use functions::*;

// Information for the --help argument
#[derive(Parser, Debug)]
#[command(
    author = "badluma",
    version = "1.0.0",
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

    /// Choose log level
    #[arg(short = 'c', long = "color", value_enum)]
    color_arg: Option<Color>,

    /// Do Loop
    #[arg(short = 'l', long = "loop")]
    loop_arg: bool,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Color {
    Black,
    White,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightBlack,
    LightWhite,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    Default
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

    let color: Color = if let Some(color) = args.color_arg {
        color
    } else {
        Color::Default
    };

    let loop_arg = args.loop_arg;

    let contents = std::fs::read_to_string(&path).expect("Failed to read file");
    let chars: Vec<char> = contents.chars().collect();

    loop {
        let return_code: u8 = scan_and_flush(chars.clone(), Some(color.clone()), speed);

        if return_code == 1 {
            break;
        }

        if !loop_arg {
            break;
        }
    }
}
