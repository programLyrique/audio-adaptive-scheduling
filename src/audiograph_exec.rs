//! Execute an audiograph file and report stats about it.

extern crate audio_adaptive;
extern crate portaudio;
extern crate rand;
extern crate time;
extern crate crossbeam_channel;
extern crate clap;

use portaudio as pa;
use crossbeam_channel::unbounded;
use std::env;
use std::thread;
use std::time as rust_time;//To be used for thread::sleep for instance
use std::process::exit;

use time::{PreciseTime, Duration};

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

use clap::{Arg, App, ArgGroup};

use audio_adaptive::audiograph::*;
use audio_adaptive::audiograph_parser::*;
use audio_adaptive::sndfile;

const CHANNELS: i32 = 1;
const SAMPLE_RATE: u32 = 44_100;
const NB_CYCLES : u32 = 6000;
const FRAMES_PER_BUFFER : usize = 64;

#[derive(Clone, Copy, Debug)]
pub struct TimeMonitor {
    /// Time budget remaining at the end (if negative, deadline exceeded)
    pub budget : i64,
    /// Deadline as given by portaudio
    pub deadline : u64,
    /// Execution time for one cycle
    pub execution_time : i64,
    pub callback_flags : audio_adaptive::effect::CallbackFlags,
}

impl Default for TimeMonitor {
    fn default() -> Self { TimeMonitor {budget:0, deadline:0, execution_time:0,
        callback_flags: audio_adaptive::effect::CallbackFlags::NO_FLAG} }
}


//Launch a audio graph in real time
fn real_time_run(mut audio_graph: AudioGraph, graph_name: String, cycles: u32, monitor: bool) -> Result<(), pa::Error> {

    let pa = try!(pa::PortAudio::new());

    //audio_graph.update_schedule().expect("Cycle detected");//Already done when parsing

    let nb_nodes = audio_graph.nb_active_nodes();
    let nb_edges = audio_graph.nb_edges();

    let buffer_size = audio_graph.frames_per_buffer() * audio_graph.nb_channels();

    let settings = try!(pa.default_output_stream_settings(audio_graph.nb_channels() as i32,
    SAMPLE_RATE as f64, buffer_size));

    let mut nb_cycles = 0;

    //Thread to monitor the audio callback
    let (tx_monit, rx_monit) = unbounded::<TimeMonitor>();

    thread::spawn(move || {
        if monitor {
            let mut f = File::create(format!("{}_{}-rt.csv",time::now().rfc3339(),graph_name)).expect("Impossible to report execution times");
            f.write_all(format!("{} {}\n", nb_nodes, nb_edges).as_bytes()).unwrap();
            f.write_all(b"Budget\tDeadline\tExecutionTime\tCallbackFlags\n").unwrap();
            for monitoring_infos in rx_monit.try_iter() {
                let seria = format!("{}\t{}\t{}\t{:?}\n",
                                                monitoring_infos.budget,
                                                monitoring_infos.deadline,
                                                monitoring_infos.execution_time,
                                                monitoring_infos.callback_flags);
                f.write_all(seria.as_bytes()).unwrap();
            }

            //println!("End monitoring execution times because {:?}", rx_monit.recv().unwrap_err().description());
        }
    });

    let mut buf_in = vec![DspEdge::new(1, 1, buffer_size as usize);1];
    let mut buf_out = vec![DspEdge::new(1, 1, buffer_size as usize);1];

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames , time, flags}| {
            debug_assert!(frames == buf_in[0].buffer().len());
            debug_assert!(frames == buf_out[0].buffer().len());
            //time members are in seconds. We need to convert it to microseconds
            let rel_deadline = (time.buffer_dac- time.current) * 1_000_000.; //microseconds
            nb_cycles += 1;
            let start = PreciseTime::now();
            //assert!(time.buffer_dac- time.current < 1.0);
            buf_in[0].buffer_mut().copy_from_slice(buffer);
            audio_graph.process(&buf_in, &mut buf_out, SAMPLE_RATE);
            buffer.copy_from_slice(buf_out[0].buffer());

            let execution_time = start.to(PreciseTime::now()).num_microseconds().unwrap();

            if monitor {
                let times = TimeMonitor {
                    deadline: rel_deadline as u64,
                    execution_time,
                    budget: rel_deadline as i64 - execution_time,
                    callback_flags: audio_adaptive::effect::CallbackFlags::from_callback_flags(flags),
                };
                tx_monit.send(times).unwrap();
            }

            if nb_cycles >= cycles {
                pa::Complete
            }
            else {
                pa::Continue
            }
    };


    println!("Opening non blocking stream");
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    println!("Starting stream");
    try!(stream.start());

    let sleep_duration = rust_time::Duration::from_millis(500);

    while try!(stream.is_active()) {
        thread::sleep(sleep_duration);
    }

    try!(stream.stop());
    try!(stream.close());

    Ok(())
}

fn bounce_run<'a>(mut audio_graph: AudioGraph, graph_name: String, audio_input: Option<&'a str>, cycles: u32, monitor: bool) -> Result<(), &'a str> {
    let nb_frames = FRAMES_PER_BUFFER;

    //audio_graph.update_schedule().expect("Cycle detected");Already done when parsing

    let nb_nodes = audio_graph.nb_active_nodes();
    let nb_edges = audio_graph.nb_edges();

    let mut f = None;

    //For reporting
    if monitor {
        let mut file  = File::create(format!("{}_{}-rt.csv",time::now().rfc3339(),graph_name)).expect("Impossible to report execution times");
        file.write_all(format!("{} {}\n", nb_nodes, nb_edges).as_bytes()).unwrap();
        file.write_all(b"Execution time\n").unwrap();
        f = Some(file);
    }

    let mut nb_channels = CHANNELS as usize;
    let mut samplerate = SAMPLE_RATE;
    let mut nb_cycles = 0;

    let mut advance : Box<dyn FnMut(&mut [f32]) -> u32> = if let Some(audio_input_name) = audio_input {
        let mut input_file = sndfile::SndFile::open(audio_input_name)?;
        nb_channels = input_file.nb_channels();
        samplerate = input_file.samplerate() as u32;
        Box::new(move |buf| {input_file.read_float(buf) as u32})
    } else {
        Box::new(|_buf| { nb_cycles += 1; cycles - nb_cycles })
    };

    let buffer_size = nb_frames * nb_channels;
    let mut buf_in = vec![DspEdge::new(1, 1, buffer_size as usize);1];
    let mut buf_out = vec![DspEdge::new(1, 1, buffer_size as usize);1];

    let mut output_file = sndfile::SndFile::open_write(graph_name + ".wav", samplerate, nb_channels as u32)?;

    while  advance(buf_in[0].buffer_mut()) != 0  {
        let start = PreciseTime::now();
        audio_graph.process(&buf_in, &mut buf_out, samplerate);
        output_file.write_float(buf_out[0].buffer());

        //Reporting
        let execution_time = start.to(PreciseTime::now()).num_microseconds().unwrap();
        if monitor {
            let seria = format!("{}\n", execution_time);
            f.as_mut().unwrap().write_all(seria.as_bytes()).unwrap();
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("Audiograph")
        .version("0.1.0")//use env! macro to get it from Cargo.toml
        .author("Pierre Donat-Bouillud")
        .about("Execute an audio graph .ag in real time or in bounce mode and get timing information about it.")
        .arg(Arg::with_name("INPUT")
             .help("Sets the audiograph to use.")
             .required(true)
             .index(1))
        .arg(Arg::with_name("real-time")
             .short("r")
             .long("real-time")
             .help("Execute in real-time"))
         .arg(Arg::with_name("bounce")
              .short("b")
              .long("bounce")
              .help("Execute the graph offline (bounce), as fast as possible."))
        .arg(Arg::with_name("audio_input")
              .short("a")
              .long("audio-input")
              .help("Audio input used as source when bouncing")
              .requires("bounce"))
        .arg(Arg::with_name("cycles")
            .short("c")
            .long("cycles")
            .value_name("NbCycles")
            .takes_value(true)
            .conflicts_with("audio_input")
            .help("Number of cycles to execute the audio graph"))
        .arg(Arg::with_name("monitor")
              .short("m")
              .long("monitor")
              .help("Monitor execution and save it as a csv file."))
        .group(ArgGroup::with_name("execution-mode")
                .args(&["real-time", "bounce"])
                .required(true))
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    //We cannot get both at the same time thanks to the ArgGroup
    let real_time = matches.is_present("real-time");
    let bounce = matches.is_present("bounce");
    let nb_cycles : u32 = matches.value_of("cycles").map_or(NB_CYCLES, |v| v.parse().unwrap_or(NB_CYCLES));
    let monitor = matches.is_present("monitor");

    let mut audiograph = parse_audiograph_from_file(filename, FRAMES_PER_BUFFER, 1).unwrap();
    audiograph.update_schedule().expect(&format!("Audio graph in {} is cyclic!!", filename));

    let basename = Path::new(filename).file_stem().and_then(OsStr::to_str).unwrap();

    println!("Starting processing");
    let start = PreciseTime::now();
    if real_time {
        real_time_run(audiograph, basename.to_string(), nb_cycles, monitor).unwrap();
    }
    else if bounce {
        let audio_input = matches.value_of("audio_input");
        bounce_run(audiograph, basename.to_string(), audio_input, nb_cycles, monitor).unwrap();
    }
    let execution_time = start.to(PreciseTime::now()).num_microseconds().unwrap();
    println!("End processing in {}s", execution_time as f64 / 1_000_000.0);
}
