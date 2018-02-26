extern crate audio_adaptive;
extern crate rustbox;
extern crate portaudio;
extern crate rand;
extern crate time;
#[macro_use]
extern crate lazy_static;


use std::env;


use std::error::Error;

use rustbox::Key;
use rustbox::{Color, RustBox};
use std::process::exit;


mod audioengine;

const UP_RATIO : f64 = 1.;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: basic_example Audio_File");
        exit(0);
    }

    let mut audioengine = audioengine::AudioEngine::new(&args[1]).unwrap();

    /*
     * Event interaction with the console
     */
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Simple test of sound nodes with tradeoff between quality and deadlines.");
    rustbox.present();

    let mut ratio = UP_RATIO;
    while audioengine.stream.is_active().unwrap() {
        rustbox.clear();

        // let cpu_load = stream.cpu_load();
        // let stream_infos = stream.info();
        // rustbox.print(1, 5, rustbox::RB_BOLD, Color::White, Color::Black,
        //     &format!("CPU load {} ; Input latency: {}s ; Output latency: {}", cpu_load, stream_infos.input_latency, stream_infos.output_latency));

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Up => {ratio += 1.; audioengine.control_sender.send(ratio).unwrap();},
                    Key::Down if ratio > 1. => {ratio -= 1.; audioengine.control_sender.send(ratio).unwrap();},
                    Key::Char('q') => {
                        audioengine.stream.stop().unwrap();
                        break;},
                    _ => {}
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => {}
        };
        rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Simple test of sound nodes with tradeoff between quality and deadlines.");
        rustbox.print(6, 9, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Ratio : {}", ratio));

        rustbox.present();
    }

    println!("End of playback");

}
