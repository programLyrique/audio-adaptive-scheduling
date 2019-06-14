extern crate audio_adaptive;
extern crate portaudio;
extern crate rand;
extern crate time;

use audio_adaptive::effect::*;

use audio_adaptive::experiments::{GraphGenerator, NodeClass, RandomGenerator};

use portaudio as pa;

use std::env;
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time as rust_time; //To be used for thread::sleep for instance

use rand::prelude::*;
use rand::seq::SliceRandom;

use std::fs::File;
use std::io::prelude::*;

const NUM_SECONDS: u64 = 5;
const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const NB_CYCLES: u32 = 500;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    Baseline,
    Exhaustive,
    Progressive,
}

///Launch a audio graph with nb_oscillators
/// On my machine, 1500 - 1600 oscillators (1545...) seem to start entailing miss deadlines
fn run(mode: Mode, nb_oscillators: u32, proba_edge: f64) -> Result<(), pa::Error> {
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

    println!("==== Generation of random graph ====",);

    let mut rand_gen = RandomGenerator::new(nb_oscillators as usize, proba_edge);

    let mut audio_graph = rand_gen.generate(&|c, rng| {
        let generators = vec![
            DspNode::Modulator(5., 500 + rng.gen_range(0, 400), 1.0),
            DspNode::LowPass([5., 6., 7., 8.], 200. + rng.gen_range(0., 400.), 0.8),
        ];
        match c {
            NodeClass::Input => DspNode::Oscillator(6., 500 + rng.gen_range(0, 400), 1.0),
            NodeClass::Transformer | NodeClass::Output => *generators.choose(rng).unwrap(),
        }
    });
    //TODO: don't generate cyclic graphs!!

    println!("Random graph has been generated.");
    if nb_oscillators <= 100 {
        println!("Matrix of random graph: {:?}", rand_gen);
        println!("Random graph: {}", audio_graph);
    }

    audio_graph.update_schedule().expect("Cycle detected");

    let nb_nodes = audio_graph.nb_active_nodes();
    let nb_edges = audio_graph.nb_edges();

    let settings = try!(pa.default_output_stream_settings(
        audio_graph.nb_channels() as i32,
        SAMPLE_RATE,
        audio_graph.frames_per_buffer()
    ));

    //Thread to monitor the audio callback
    let (tx_monit, rx_monit) = mpsc::channel::<TimeMonitor>();
    let mut nb_cycles = 0;

    thread::spawn(move || {
        let mut f = File::create(format!(
            "complex_graph_{}_{}_{}_{}.csv",
            time::now().rfc3339(),
            match mode {
                Mode::Exhaustive => "ex",
                Mode::Progressive => "prog",
                Mode::Baseline => "base",
            },
            nb_oscillators,
            proba_edge
        ))
        .expect("Impossible to report execution times");
        f.write_all(format!("{} {}\n", nb_nodes, nb_edges).as_bytes())
            .unwrap();
        f.write_all(b"Quality\tBudget\tExpectRemainingTime\tDeadline\tNbDegradedNodes\tNbResamplers\tExecutionTime\tChoosingDuration\tCallbackFlags\n").unwrap();
        for monitoring_infos in rx_monit.iter() {
            let seria = format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\n",
                monitoring_infos.quality,
                monitoring_infos.budget,
                monitoring_infos.expected_remaining_time,
                monitoring_infos.deadline,
                monitoring_infos.nb_degraded,
                monitoring_infos.nb_resamplers,
                monitoring_infos.execution_time,
                monitoring_infos.choosing_duration,
                monitoring_infos.callback_flags
            );
            f.write_all(seria.as_bytes()).unwrap();
        }
        //println!("End monitoring execution times because {:?}", rx_monit.recv().unwrap_err().description());
    });

    let callback = move |pa::OutputStreamCallbackArgs {
                             buffer,
                             frames: _frames,
                             time,
                             flags,
                         }| {
        //time members are in seconds. We need to convert it to microseconds
        let rel_deadline = (time.buffer_dac - time.current) * 1_000_000.; //microseconds
        nb_cycles += 1;
        assert!(time.buffer_dac - time.current < 1.0);
        let times = match mode {
            Mode::Baseline => audio_graph.process_baseline(
                buffer,
                SAMPLE_RATE as u32,
                CHANNELS as usize,
                rel_deadline,
                CallbackFlags::from_callback_flags(flags),
            ),
            Mode::Exhaustive => audio_graph.process_adaptive_exhaustive(
                buffer,
                SAMPLE_RATE as u32,
                CHANNELS as usize,
                rel_deadline,
                CallbackFlags::from_callback_flags(flags),
            ),
            Mode::Progressive => audio_graph.process_adaptive_progressive(
                buffer,
                SAMPLE_RATE as u32,
                CHANNELS as usize,
                rel_deadline,
                CallbackFlags::from_callback_flags(flags),
            ),
        };
        tx_monit.send(times).unwrap();
        if nb_cycles >= NB_CYCLES {
            pa::Complete
        } else {
            pa::Continue
        }
    };

    println!("Opening non blocking stream");
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    println!("Starting stream");
    try!(stream.start());

    let sleep_duration = rust_time::Duration::from_millis(500);
    /*thread::sleep(sleep_duration);

    try!(stream.stop());
    try!(stream.close());
    thread::sleep(sleep_duration / NUM_SECONDS as u32);//To give time to the monitoring infos to be written*/
    while try!(stream.is_active()) {
        thread::sleep(sleep_duration);
    }

    try!(stream.stop());
    try!(stream.close());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: basic_example [BASE|EX|PROG] nb_oscillators [proba_edge]");
        exit(0);
    }
    let mode = match args[1].as_str() {
        "BASE" => Mode::Baseline,
        "EX" => Mode::Exhaustive,
        "PROG" => Mode::Progressive,
        _ => {
            println!("Usage: basic_example [BASE|EX|PROG] nb_oscillators [proba_edge]");
            std::process::exit(1)
        }
    };
    let nb_oscillators = args[2]
        .parse::<u32>()
        .expect("Usage: basic_example [BASE|EX|PROG] nb_oscillators [proba_edge]");

    let proba_edge = if args.len() == 4 {
        let res = args[3]
            .parse::<f64>()
            .expect("proba_edge must a floating point number");
        if 0. <= res && res <= 1. {
            res
        } else {
            eprintln!("proba_edge must be in [0,1]");
            exit(1)
        }
    } else {
        0.5
    };

    run(mode, nb_oscillators, proba_edge).unwrap()
}
