use colored::Colorize;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use termios::{cfmakeraw, tcgetattr, tcsetattr, Termios, TCSADRAIN};

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
pub fn getch(char_width: usize) -> anyhow::Result<String> {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();

    let mut old_term = Termios::from_fd(fd)?;
    tcgetattr(fd, &mut old_term)?;

    let mut raw_term = old_term.clone();
    cfmakeraw(&mut raw_term);
    tcsetattr(fd, TCSADRAIN, &raw_term)?;

    let result = (|| -> anyhow::Result<String> {
        let mut buf = vec![0u8; char_width];
        stdin.lock().read_exact(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    })();

    tcsetattr(fd, TCSADRAIN, &old_term)?;

    result
}

pub fn scan_and_flush(chars: Vec<char>, color: Option<String>, speed: u32) {
    let mut char_index = 0;

    while char_index < chars.len() {
        let typed_char = getch(1).ok();

        if typed_char == Some("\u{3}".to_string()) {
            break;
        }

        for _i in 0..speed {
            if char_index < chars.len() {
                let ch = chars[char_index].to_string();

                match color.as_deref() {
                    Some("black") => print!("{}", ch.black()),
                    Some("red") => print!("{}", ch.red()),
                    Some("green") => print!("{}", ch.green()),
                    Some("yellow") => print!("{}", ch.yellow()),
                    Some("blue") => print!("{}", ch.blue()),
                    Some("magenta") => print!("{}", ch.magenta()),
                    Some("cyan") => print!("{}", ch.cyan()),
                    Some("white") => print!("{}", ch.white()),
                    Some("bright_black") => print!("{}", ch.bright_black()),
                    Some("bright_red") => print!("{}", ch.bright_red()),
                    Some("bright_green") => print!("{}", ch.bright_green()),
                    Some("bright_yellow") => print!("{}", ch.bright_yellow()),
                    Some("bright_blue") => print!("{}", ch.bright_blue()),
                    Some("bright_magenta") => print!("{}", ch.bright_magenta()),
                    Some("bright_cyan") => print!("{}", ch.bright_cyan()),
                    Some("bright_white") => print!("{}", ch.bright_white()),
                    Some("default") => print!("{}", ch),
                    None => print!("{}", ch),
                    _ => {
                        println!("Invalid color: {:?}", color);
                        println!(
                            "Available colors:

                    - black
                    - red
                    - green
                    - yellow
                    - blue
                    - magenta
                    - cyan
                    - white

                    - bright_black
                    - bright_red
                    - bright_green
                    - bright_yellow
                    - bright_blue
                    - bright_magenta
                    - bright_cyan
                    - bright_white",
                        );
                    }
                }

                char_index += 1;
            }
        }
        io::Write::flush(&mut std::io::stdout()).ok();
    }
}
