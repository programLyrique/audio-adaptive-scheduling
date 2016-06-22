extern crate audio_adaptive;
extern crate portaudio;
extern crate rand;
extern crate time;

use audio_adaptive::effect::*;

use portaudio as pa;


use std::thread;

const NUM_SECONDS : u32 = 5;
const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

fn run() -> Result<(), pa::Error> {
    let pa = try!(pa::PortAudio::new());

    let settings = try!(pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));

    //Build the audiograph
    let buffer_size = CHANNELS as usize * FRAMES_PER_BUFFER as usize;
    let mut audio_graph = AudioGraph::new(buffer_size);
    let mixer = audio_graph.add_node(DspNode::Mixer);
    for i in 1..11 {
        audio_graph.add_input(DspNode::Oscillator(i as f32, 350 + i*50, 0.8 ), mixer);
    }
    audio_graph.update_schedule().expect("There is a cycle here");


    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {

        audio_graph.process(buffer, SAMPLE_RATE as u32, CHANNELS as usize);

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
    run().unwrap()
}
