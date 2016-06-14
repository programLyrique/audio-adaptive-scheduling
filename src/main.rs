extern crate basic_example;
extern crate rustbox;
extern crate portaudio;
extern crate rand;
extern crate time;
#[macro_use]
extern crate lazy_static;


use std::env;
use basic_example::sndfile::*;
use basic_example::samplerate::*;

use std::sync::mpsc;
use std::thread;

use std::error::Error;

use rustbox::Key;
use rustbox::{Color, RustBox};

use portaudio as pa;

use std::io::prelude::*;
use std::fs::File;

use time::{PreciseTime, Duration};

mod audioengine;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("Usage: basic_example Audio_File");
    }

    let mut audioengine = audioengine::AudioEngine::new(&args[1]).unwrap();

    let mut ratio = 1.;
    while audioengine.stream.is_active().unwrap() {
        thread::sleep_ms(1000);
        ratio = ((ratio * 10. ) as u32 % 100) as f64 / 10. + 1.;
        println!("At {}s, resampling ratio is now: {}", time::precise_time_s(), ratio);
        audioengine.control_sender.send(ratio).unwrap();
    }

    println!("End of playback");

    audioengine.stream.close().unwrap();
}
