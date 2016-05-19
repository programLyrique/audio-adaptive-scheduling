extern crate basic_example;

extern crate rustbox;

use std::env;
use basic_example::sndfile::*;

use std::error::Error;

use rustbox::Key;
use rustbox::{Color, RustBox};

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut sndfile = SndFile::open(&args[1]).expect("Error while opening file");

    let audiostream = sndfile.readf_float_all();

    println!("Simple test of sound nodes with tradeoff between quality and deadlines.");
    println!("Number of samples x number of channels = {}", audiostream.len());

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    loop {
        rustbox.clear();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Up => {rustbox.print(6, 6, rustbox::RB_BOLD, Color::White, Color::Black, "Up");},
                    Key::Down => {rustbox.print(6, 9, rustbox::RB_BOLD, Color::White, Color::Black, "Down");},
                    Key::Char('q') => {break;},
                    _ => {}
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => {}
        };
        rustbox.present();
    }
}
