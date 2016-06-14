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

    /*
     * Event interaction with the console
     */
    // let rustbox = match RustBox::init(Default::default()) {
    //     Result::Ok(v) => v,
    //     Result::Err(e) => panic!("{}", e),
    // };
    //
    //
    // let nb_channels_str = &format!("Number of samples x number of channels = {}", nb_channels * nb_frames as usize);
    //
    // rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Simple test of sound nodes with tradeoff between quality and deadlines.");
    // rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, nb_channels_str);
    // rustbox.present();
    //
    // let mut ratio = UP_RATIO;
    // while stream.is_active().unwrap() {
    //     rustbox.clear();
    //     rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Simple test of sound nodes with tradeoff between quality and deadlines.");
    //     rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, nb_channels_str);
    //     rustbox.print(6, 9, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Ratio : {}", ratio));
    //
    //     // let cpu_load = stream.cpu_load();
    //     // let stream_infos = stream.info();
    //     // rustbox.print(1, 5, rustbox::RB_BOLD, Color::White, Color::Black,
    //     //     &format!("CPU load {} ; Input latency: {}s ; Output latency: {}", cpu_load, stream_infos.input_latency, stream_infos.output_latency));
    //
    //     match rustbox.poll_event(false) {
    //         Ok(rustbox::Event::KeyEvent(key)) => {
    //             match key {
    //                 Key::Up => {ratio += 1.; tx.send(ratio).unwrap();},
    //                 Key::Down => {ratio -= 1.; tx.send(ratio).unwrap();},
    //                 Key::Char('q') => {
    //                     stream.stop().unwrap();
    //                     break;},
    //                 _ => {}
    //             }
    //         },
    //         Err(e) => panic!("{}", e.description()),
    //         _ => {}
    //     };
    //
    //     rustbox.present();
    // }

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
