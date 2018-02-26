extern crate audio_adaptive;
extern crate portaudio;
extern crate rand;
extern crate time;

use audio_adaptive::effect::*;

use audio_adaptive::experiments::{RandomGenerator, GraphGenerator, NodeClass};

use portaudio as pa;


use std::sync::mpsc;
use std::env;
use std::thread;
use std::process::exit;

use rand::Rng;

use std::io::prelude::*;
use std::fs::File;

const NUM_SECONDS : u32 = 5;
const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;


///Launch a audio graph with nb_oscillators
/// On my machine, 1500 - 1600 oscillators (1545...) seem to start entailing miss deadlines
fn run(nb_oscillators : u32) -> Result<(), pa::Error> {

    let pa = try!(pa::PortAudio::new());

    //Build the audiograph
    // let buffer_size = CHANNELS as usize * FRAMES_PER_BUFFER as usize;
    //
    //
    // let mut audio_graph = AudioGraph::new(FRAMES_PER_BUFFER as usize, CHANNELS as u32);
    // let mixer = audio_graph.add_node(DspNode::Mixer);
    // for i in 1..nb_oscillators {
    //     audio_graph.add_input(DspNode::Oscillator(i as f32, 350 + i*50, 0.9 / nb_oscillators as f32 ), mixer);
    // }
    // let mut prev_mod = mixer;
    // for i in 1..nb_oscillators {
    //     prev_mod = audio_graph.add_input(DspNode::Modulator(i as f32, 350 + i*50, 1.0 ), prev_mod);
    // }
    // audio_graph.add_input(DspNode::Oscillator(0., 135, 0.5 ), prev_mod);

    println!("==== Generation of random graph ====", );

    let mut rand_gen = RandomGenerator::new(nb_oscillators as usize);

    let mut audio_graph = rand_gen.generate(& |c, rng|
        {
            let generators = vec![DspNode::Modulator(5., 500 + rng.gen_range::<u32>(0, 400), 1.0),
                DspNode::LowPass([5.,6.,7.,8.],200. + rng.gen_range::<f32>(0., 400.),0.8)];
            match c  {
                NodeClass::Input => DspNode::Oscillator(6., 500 + rng.gen_range::<u32>(0, 400), 1.0),
                NodeClass::Transformer | NodeClass::Output => *rng.choose(&generators).unwrap()
            }
        });

    println!("Matrix of random graph: {:?}", rand_gen);
    println!("Random graph: {}", audio_graph);

    audio_graph.update_schedule().expect("Cycle detected");

    let settings = try!(pa.default_output_stream_settings(audio_graph.nb_channels() as i32,
    SAMPLE_RATE, audio_graph.frames_per_buffer()));

    //Thread to monitor the audio callback
    let (tx_monit, rx_monit) = mpsc::channel::<TimeMonitor>();

    thread::spawn(move || {

        let mut f = File::create(format!("complex_graph_{}_{}.csv",time::now().rfc3339(), nb_oscillators)).expect("Impossible to report execution times");

        f.write_all(b"Quality\tBudget\tExpectRemainingTime\tDeadline\tNbNodes\n").unwrap();
       for monitoring_infos in rx_monit.iter() {

            let seria = format!("{}\t{}\t{}\t{}\t{}\n", monitoring_infos.quality,
                                            monitoring_infos.budget,
                                            monitoring_infos.expected_remaining_time,
                                            monitoring_infos.deadline,
                                            monitoring_infos.nb_degraded);
            f.write_all(seria.as_bytes()).unwrap();
       }
    //println!("End monitoring execution times because {:?}", rx_monit.recv().unwrap_err().description());

    });

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, time, ..}| {

        //time members are in seconds. We need to convert it to microseconds
        let rel_deadline = (time.buffer_dac- time.current) * 1_000_000.; //microseconds
        assert!(time.buffer_dac- time.current < 1.0);
        let times = audio_graph.process_adaptive_progressive(buffer, SAMPLE_RATE as u32, CHANNELS as usize, rel_deadline);
        //audio_graph.process(buffer, SAMPLE_RATE as u32, CHANNELS as usize);
        tx_monit.send(times).unwrap();

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

    if args.len() < 2 {
        println!("Usage: basic_example nb_oscillators");
        exit(0);
    }
    run(args[1].parse::<u32>().expect("Usage: basic_example nb_oscillators")).unwrap()
}
