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
use std::time as rust_time;//To be used for thread::sleep for instance
use std::process::exit;

use rand::Rng;

use std::io::prelude::*;
use std::fs::File;

const NUM_SECONDS : u64 = 5;
const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;

enum Mode {
    Exhaustive,
    Progressive,
}


///Launch a audio graph with nb_oscillators
/// On my machine, 1500 - 1600 oscillators (1545...) seem to start entailing miss deadlines
fn run(mode : Mode, nb_oscillators : u32) -> Result<(), pa::Error> {

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

    if nb_oscillators <= 1000 {
        println!("Matrix of random graph: {:?}", rand_gen);
        println!("Random graph: {}", audio_graph);
    }

    audio_graph.update_schedule().expect("Cycle detected");

    let settings = try!(pa.default_output_stream_settings(audio_graph.nb_channels() as i32,
    SAMPLE_RATE, audio_graph.frames_per_buffer()));

    //Thread to monitor the audio callback
    let (tx_monit, rx_monit) = mpsc::channel::<TimeMonitor>();

    thread::spawn(move || {

        let mut f = File::create(format!("complex_graph_{}_{}.csv",time::now().rfc3339(), nb_oscillators)).expect("Impossible to report execution times");

        f.write_all(b"Quality\tBudget\tExpectRemainingTime\tDeadline\tNbNodes\tExecutionTime\tChoosingDuration\tCallbackFlags\n").unwrap();
       for monitoring_infos in rx_monit.iter() {

            let seria = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\n", monitoring_infos.quality,
                                            monitoring_infos.budget,
                                            monitoring_infos.expected_remaining_time,
                                            monitoring_infos.deadline,
                                            monitoring_infos.nb_degraded,
                                            monitoring_infos.execution_time,
                                            monitoring_infos.choosing_duration,
                                            monitoring_infos.callback_flags);
            f.write_all(seria.as_bytes()).unwrap();
       }
    //println!("End monitoring execution times because {:?}", rx_monit.recv().unwrap_err().description());

    });

    //TODO: rather choose the closure (but have to use Box or impl trait)
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames : _frames , time, flags}| {
            //time members are in seconds. We need to convert it to microseconds
            let rel_deadline = (time.buffer_dac- time.current) * 1_000_000.; //microseconds
            assert!(time.buffer_dac- time.current < 1.0);
            let times = match mode {
                Mode::Exhaustive => audio_graph.process_adaptive_exhaustive(buffer, SAMPLE_RATE as u32, CHANNELS as usize, rel_deadline, CallbackFlags::from_callback_flags(flags)),
                Mode::Progressive => audio_graph.process_adaptive_progressive(buffer, SAMPLE_RATE as u32, CHANNELS as usize, rel_deadline, CallbackFlags::from_callback_flags(flags))
            };
            tx_monit.send(times).unwrap();

            pa::Continue

    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    let sleep_duration = rust_time::Duration::from_millis(NUM_SECONDS * 1000);
    thread::sleep(sleep_duration);

    try!(stream.stop());
    try!(stream.close());

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: basic_example [EX|PROG] nb_oscillators");
        exit(0);
    }
    let mode = match args[1].as_str() {
        "EX" => Mode::Exhaustive,
        "PROG" => Mode::Progressive,
         _ => {println!("Usage: basic_example [EX|PROG] nb_oscillators"); std::process::exit(1)}
    };
    let nb_oscillators = args[2].parse::<u32>().expect("Usage: basic_example [EX|PROG] nb_oscillators");
    run(mode, nb_oscillators).unwrap()
}
