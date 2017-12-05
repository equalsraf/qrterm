extern crate qrcode;
use qrcode::types::Color;
use qrcode::types::Color::*;

extern crate term;

use std::env;

fn print_char(tl: Color, tr: Color, bl: Color, br: Color) {
    match (tl, tr, bl, br) {
        (Light, Light, Light, Light) => print!("  "),
        (Dark, Dark, Dark, Dark) => print!("██"),
        (Dark, Light, Light, Light) => print!("▀ "),
        (Light, Dark, Light, Light) => print!(" ▀"),
        (Dark, Dark, Light, Light) => print!("▀▀"),
        (Light, Light, Dark, Light) => print!("▄ "),
        (Light, Light, Light, Dark) => print!(" ▄"),
        (Light, Light, Dark, Dark) => print!("▄▄"),
        (Dark, Dark, Dark, Light) => print!("█▀"),
        (Dark, Dark, Light, Dark) => print!("▀█"),
        (Dark, Light, Dark, Dark) => print!("█▄"),
        (Light, Dark, Dark, Dark) => print!("▄█"),
        (Dark, Light, Light, Dark) => print!("▀▄"),
        (Light, Dark, Dark, Light) => print!("▄▀"),
        (Dark, Light, Dark, Light) => print!("█ "),
        (Light, Dark, Light, Dark) => print!(" █"),
    }
}

fn print_line(data: &[Color]) {
    // The indent adds a white margin at the side, that helps the camera detect the edges
    print!("    ");
    for pair in data.chunks(2) {
        print_char(pair.get(0).map(|v| *v).unwrap_or(Light),
                    pair.get(1).map(|v| *v).unwrap_or(Light),
                    Light, Light);
    }
    println!("    ");
}

fn print_two_lines(line0: &[Color], line1: &[Color]) {
    // The indent adds a white margin at the side, that helps the camera detect the edges
    print!("    ");
    for (top, bot) in line0.chunks(2).zip(line1.chunks(2)) {
        print_char(top.get(0).map(|v| *v).unwrap_or(Light),
                    top.get(1).map(|v| *v).unwrap_or(Light),
                    bot.get(0).map(|v| *v).unwrap_or(Light),
                    bot.get(1).map(|v| *v).unwrap_or(Light));
    }
    
    println!("    ");
}

pub fn setup_term() -> Option<Box<term::StdoutTerminal>> {
    match term::stdout() {
        Some(mut term) => {
            // Not all QR readers can deal with other color combinations e.g. white on black,
            // force black on white if possible
            if term.supports_color() {
                term.fg(term::color::BLACK).unwrap();
                term.bg(term::color::BRIGHT_WHITE).unwrap();
                Some(term)
            } else {
                None
            }
        }
        None => None,
    }
}

fn main() {

    let input = env::args()
        .nth(1)
        .expect("Usage: qrterm <arg>");

    let code = qrcode::QrCode::new(input)
                    .expect("Failed to encode QR code");

    let t = setup_term();
    if t.is_some() {
        // Add a couple blank lines at the top
        println!("");
        println!("");
    }

    let v = &code.to_colors();

    // Process two lines at a time
    for pixels in v.chunks(code.width()*2) {
        if pixels.len() > code.width() {
            let (line0, line1) = pixels.split_at(code.width());
            print_two_lines(line0, line1);
        } else {
            print_line(pixels);
        }
    }

    if let Some(mut term) = t {
        let _ = term.reset().unwrap();
        println!("");
    }
}

