//! Execute an audiograph file and report stats about it.

extern crate audio_adaptive;
extern crate portaudio;
extern crate rand;
extern crate time;
extern crate crossbeam_channel;

use portaudio as pa;
use crossbeam_channel::unbounded;
use std::env;
use std::thread;
use std::time as rust_time;//To be used for thread::sleep for instance
use std::process::exit;

use time::{PreciseTime, Duration};

use std::io::prelude::*;
use std::fs::File;


use audio_adaptive::audiograph::*;
use audio_adaptive::audiograph_parser::*;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: u32 = 44_100;
const NB_CYCLES : u32 = 600;

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


//Launch a audio graph in real time
fn real_time_run(mut audio_graph: AudioGraph, graph_name: String) -> Result<(), pa::Error> {

    let pa = try!(pa::PortAudio::new());

    audio_graph.update_schedule().expect("Cycle detected");

    let nb_nodes = audio_graph.nb_active_nodes();
    let nb_edges = audio_graph.nb_edges();

    let buffer_size = audio_graph.frames_per_buffer() * audio_graph.nb_channels();

    let settings = try!(pa.default_output_stream_settings(audio_graph.nb_channels() as i32,
    SAMPLE_RATE as f64, buffer_size));

    //Thread to monitor the audio callback
    let (tx_monit, rx_monit) = unbounded::<TimeMonitor>();
    let mut nb_cycles = 0;

    thread::spawn(move || {
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

            let times = TimeMonitor {
                deadline: rel_deadline as u64,
                execution_time,
                budget: rel_deadline as i64 - execution_time,
                callback_flags: audio_adaptive::effect::CallbackFlags::from_callback_flags(flags),
            };
            tx_monit.send(times).unwrap();
            if nb_cycles >= NB_CYCLES {
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

fn main() {

}
