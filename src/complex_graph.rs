extern crate audio_adaptive;
extern crate portaudio;
extern crate rand;
extern crate time;

use audio_adaptive::effect::*;

use portaudio as pa;

use std::env;
use std::thread;

const NUM_SECONDS : u32 = 5;
const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

///Launch a audio graph with nb_oscillators
/// On my machine, 1500 - 1600 oscillators (1545...) seem to start entailing miss deadlines
fn run(nb_oscillators : u32) -> Result<(), pa::Error> {
    let pa = try!(pa::PortAudio::new());

    let settings = try!(pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));

    //Build the audiograph
    //let buffer_size = CHANNELS as usize * FRAMES_PER_BUFFER as usize;
    let mut audio_graph = AudioGraph::new(FRAMES_PER_BUFFER as usize, CHANNELS as u32);
    let mixer = audio_graph.add_node(DspNode::Mixer);
    // for i in 1..nb_oscillators {
    //     audio_graph.add_input(DspNode::Oscillator(i as f32, 350 + i*50, 0.9 / nb_oscillators as f32 ), mixer);
    // }
    let mut prev_mod = mixer;
    for i in 1..nb_oscillators {
        prev_mod = audio_graph.add_input(DspNode::Modulator(i as f32, 350 + i*50, 1.0 ), prev_mod);
    }
    audio_graph.add_input(DspNode::Oscillator(0., 135, 0.5 ), prev_mod);
    audio_graph.update_schedule().expect("Cycle detected");


    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, time, ..}| {

        //time members are in seconds. We need to convert it to microseconds
        let rel_deadline = (time.buffer_dac- time.current) * 1_000_000.; //microseconds
        assert!(time.buffer_dac- time.current < 1.0);
        audio_graph.process_adaptive2(buffer, SAMPLE_RATE as u32, CHANNELS as usize, rel_deadline);
        //audio_graph.process(buffer, SAMPLE_RATE as u32, CHANNELS as usize);

        pa::Continue
    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    thread::sleep_ms(NUM_SECONDS * 1000);

    try!(stream.stop());
    try!(stream.close());

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("Usage: basic_example nb_oscillators");
    }
    run(args[1].parse::<u32>().expect("Usage: basic_example nb_oscillators")).unwrap()
}
