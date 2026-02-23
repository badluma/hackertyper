use crate::Color;
use colored::Colorize;
use getch_rs::{Getch, Key::*};
use std::io;

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn scan_and_flush(chars: Vec<char>, color: Option<Color>, speed: u32) -> u8{
    let mut char_index = 0;

    loop {
        if char_index >= chars.len() {
            return 0;
        }

        let typed_char = Getch::new();

        match typed_char.getch() {
            Ok(Ctrl('c')) => return 1,
            _ => {}
        }

        for _i in 0..speed {
            if char_index < chars.len() {
                let ch = chars[char_index].to_string();

                match color.as_ref() {
                    Some(Color::Black) => print!("{}", ch.black()),
                    Some(Color::Red) => print!("{}", ch.red()),
                    Some(Color::Green) => print!("{}", ch.green()),
                    Some(Color::Yellow) => print!("{}", ch.yellow()),
                    Some(Color::Blue) => print!("{}", ch.blue()),
                    Some(Color::Magenta) => print!("{}", ch.magenta()),
                    Some(Color::Cyan) => print!("{}", ch.cyan()),
                    Some(Color::White) => print!("{}", ch.white()),
                    Some(Color::LightBlack) => print!("{}", ch.bright_black()),
                    Some(Color::LightRed) => print!("{}", ch.bright_red()),
                    Some(Color::LightGreen) => print!("{}", ch.bright_green()),
                    Some(Color::LightYellow) => print!("{}", ch.bright_yellow()),
                    Some(Color::LightBlue) => print!("{}", ch.bright_blue()),
                    Some(Color::LightMagenta) => print!("{}", ch.bright_magenta()),
                    Some(Color::LightCyan) => print!("{}", ch.bright_cyan()),
                    Some(Color::LightWhite) => print!("{}", ch.bright_white()),
                    Some(Color::Default) | None => print!("{}", ch),
                }

                char_index += 1;
            }
        }
        io::Write::flush(&mut io::stdout()).ok();
    }
}
