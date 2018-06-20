extern crate ansi_term;
use ansi_term::Colour::{RGB, White};
use ansi_term::ANSIGenericString;

fn get_block(v: u8) -> String {
    let c = RGB(v, v, v);
    let block = White.on(c).paint("   ");
    format!("{}", block)
}

fn main() {
    let a = get_block(0);
    let b = get_block(85);
    let c = get_block(170);
    let d = get_block(255);

    for i in 0..8 {
        for i in 0..8 {
            print!("{}", c);
        }
        println!("");
    }
}
