extern crate audio_adaptive;
extern crate portaudio;
extern crate rand;
extern crate time;
#[macro_use]
extern crate lazy_static;


use std::env;
use std::thread;
use std::time as rust_time;
use std::process::exit;

mod audioengine;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: basic_example Audio_File");
        exit(0);
    }

    let mut audioengine = audioengine::AudioEngine::new(&args[1]).unwrap();

    let sleep_duration = rust_time::Duration::from_millis(1000);

    let mut ratio = 1.;
    while audioengine.stream.is_active().unwrap() {
        thread::sleep(sleep_duration);
        ratio = ((ratio * 10. ) as u32 % 100) as f64 / 10. + 1.;
        println!("At {}s, resampling ratio is now: {}", time::precise_time_s(), ratio);
        audioengine.control_sender.send(ratio).unwrap();
    }

    println!("End of playback");

    audioengine.stream.close().unwrap();
}
