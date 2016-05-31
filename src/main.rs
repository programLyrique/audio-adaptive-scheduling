extern crate basic_example;
extern crate rustbox;
extern crate portaudio;
extern crate rand;
extern crate time;

use std::env;
use basic_example::sndfile::*;

use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;

use std::error::Error;

use rustbox::Key;
use rustbox::{Color, RustBox};

use portaudio as pa;

use std::io::prelude::*;
use std::fs::File;


const FRAMES_PER_BUFFER : u32 = 64;

use time::{PreciseTime, Duration};

#[derive(Debug)]
struct TimeMonitoring {
    pub current_invocation : f64,//When the audio callback is invoked
    pub buffer_dac : f64, // when the first sample of the output buffer will be send to the DAC
    pub audio_processing : Duration,//duration between beginning of callback and when the audio processing has been finished in the audio callback
}


fn main() {

    let args: Vec<String> = env::args().collect();

    let mut sndfile = SndFile::open(&args[1]).expect("Error while opening file");
    let nb_channels = sndfile.nb_channels();
    let samplerate = sndfile.samplerate();
    let nb_frames = FRAMES_PER_BUFFER;

    let audiostream = sndfile.readf_float_all();

    println!("Simple test of sound nodes with tradeoff between quality and deadlines.");
    println!("Number of samples x number of channels = {}", audiostream.len());

    /*
     * Playback with portaudio
     */

     //Thread to monitor the audio callback
     let (tx_monit_exec, rx_monit_exec) = mpsc::channel::<TimeMonitoring>();

     thread::spawn(move || {
         let mut f = File::create("execution_audio").expect("Impossible to report execution times");

         f.write_all(b"CurrentInvocation\tBufferDac\tAudioProcNS\n").unwrap();
        for monitoring_infos in rx_monit_exec.iter() {
             let duration : Duration = monitoring_infos.audio_processing;
             let seria = format!("{}\t{}\t{}\n", monitoring_infos.current_invocation, monitoring_infos.buffer_dac, duration.num_nanoseconds().unwrap());
             f.write_all(seria.as_bytes()).unwrap();
        }
         println!("End monitoring execution times because {:?}", rx_monit_exec.recv().unwrap_err().description());

     });

     //Audio callback and audio callback communication
     let ( tx,  rx) = mpsc::channel();
     let mut volume = 5;
     let mut chunk_it = 0;

     let callback = move |pa::OutputStreamCallbackArgs {buffer, frames, time, ..}| {
         let start = PreciseTime::now();

         //volume = rx.try_recv().and_then(|v| {tx_monit_vol.send(v).unwrap(); Ok(v)}).unwrap_or(volume);
         while let Ok(val) = rx.try_recv() {
             volume = val;
         }

         let vol = volume as f32 / 10.;
         let nb_samples = frames * nb_channels;

         /*
          * frame of size 3 with 3 channels. Nb samples is 9
          * ||ch1|ch2|ch3||ch1|ch2|ch3||ch1|ch2|ch3||
          */

        //  for frame in buffer.chunks_mut(nb_channels) {
            //  let val = rng.gen::<f32>() * volume;
            //  for sample in frame.iter_mut() {
                //  *sample = val;
            //  }
        //  }
        if frames != nb_frames as usize {//Should never happen as we crate the output audio stream with the number of frames of the input ones
            panic!("Not equal number of frames in output and input.")
        }

        if chunk_it  < audiostream.len() - 1 {//Don't play the last chunk as it is smaller than the normal one
            //The best would be too copy zeroes at the end...
            buffer.clone_from_slice(&audiostream[chunk_it..std::cmp::min(chunk_it+ nb_samples, audiostream.len())]);
            for  sample in buffer.iter_mut() {
                *sample = *sample * vol;
            }
            //Send monitoring infos
            let duration = start.to(PreciseTime::now());
            tx_monit_exec.send(TimeMonitoring {
                    current_invocation : time.current,
                    buffer_dac : time.buffer_dac,
                    audio_processing : duration
                }).unwrap();

            chunk_it += nb_samples;
            pa::Continue
        }
        else {
            pa::Complete
        }
     };

     //Init portaudio and stream
     let pa = pa::PortAudio::new().expect("Bad initialization of Portaudio");

     //Show default API:
     println!("{:?}", pa.host_api_info(pa.default_host_api().unwrap()).unwrap());

     let settings = pa.default_output_stream_settings(nb_channels as i32, samplerate, nb_frames).unwrap();
     let mut stream = pa.open_non_blocking_stream(settings, callback).unwrap();

     stream.start().expect("Impossible to start playing");

    /*
     * Event interaction with the console
     */
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };


    let nb_channels_str = &format!("Number of samples x number of channels = {}", 6);

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Simple test of sound nodes with tradeoff between quality and deadlines.");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, nb_channels_str);
    rustbox.present();

    let mut vol = 5;
    while stream.is_active().unwrap() {
        rustbox.clear();
        rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Simple test of sound nodes with tradeoff between quality and deadlines.");
        rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, nb_channels_str);


        // let cpu_load = stream.cpu_load();
        // let stream_infos = stream.info();
        // rustbox.print(1, 5, rustbox::RB_BOLD, Color::White, Color::Black,
        //     &format!("CPU load {} ; Input latency: {}s ; Output latency: {}", cpu_load, stream_infos.input_latency, stream_infos.output_latency));

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Up => {vol += 1; tx.send(vol).unwrap();rustbox.print(6, 9, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Up : {}", vol));},
                    Key::Down => {vol -= 1; tx.send(vol).unwrap(); rustbox.print(6, 9, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Down : {}", vol));},
                    Key::Char('q') => {
                        stream.stop().unwrap();
                        break;},
                    _ => {}
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => {}
        };

        rustbox.present();
    }

    // let mut vol = 5;
    // while stream.is_active().unwrap() {
    //     pa.sleep(1000);
    //     vol = (vol+1) % 10;
    //     println!("At {}s, volume is now: {}", time::precise_time_s(), vol);
    //     tx.send(vol).unwrap();
    // }
    // tx.send(vol).unwrap();

    println!("End of playback");

    stream.close().unwrap();
}
