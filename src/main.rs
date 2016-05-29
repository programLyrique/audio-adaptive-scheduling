extern crate basic_example;
extern crate rustbox;
extern crate portaudio;
extern crate rand;

use std::env;
use basic_example::sndfile::*;

use std::sync::mpsc;
use std::sync::Arc;
use std::cell::RefCell;

use std::error::Error;

use rustbox::Key;
use rustbox::{Color, RustBox};

use portaudio as pa;
//TODO : cpu_load function of portaudio?
// And other metric such as time when we got a buffer, and time when the buffer should be used

use rand::Rng;


fn main() {

    let args: Vec<String> = env::args().collect();

    let mut sndfile = SndFile::open(&args[1]).expect("Error while opening file");
    let nb_channels = sndfile.nb_channels();
    let samplerate = sndfile.samplerate();
    let nb_frames = sndfile.nb_frames();

    let audiostream = sndfile.readf_float_all();

    println!("Simple test of sound nodes with tradeoff between quality and deadlines.");
    println!("Number of samples x number of channels = {}", audiostream.len());

    /*
     * Playback with portaudio
     */

    //  let mut prev_time = None;
    //  let mut timer: f64 = 10.0;
     let mut rng = rand::weak_rng();

     //let chunk_audio_stream : Vec<&[f32]>= audiostream.chunks(nb_frames as usize * nb_channels).collect();
     //let nb_chunks = chunk_audio_stream.len();
     //let mut chunk_number = 0;

     let mut chunk_it = 0;

     let mut volume = 0.5;

     let (tx, rx) = mpsc::channel();

     let callback = move |pa::OutputStreamCallbackArgs {buffer, frames, time, flags}| {
         volume = rx.try_recv().unwrap_or(volume);//Update volume if new value

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

        if chunk_it  < audiostream.len() {
            //buffer.clone_from_slice(chunk_audio_stream[chunk_number]);
            buffer.clone_from_slice(&audiostream[chunk_it..std::cmp::min(chunk_it+ nb_samples, audiostream.len())]);
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

    let mut vol = 0.5;
    loop {
        rustbox.clear();
        rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Simple test of sound nodes with tradeoff between quality and deadlines.");
        rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, nb_channels_str);
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Up => {vol += 0.1; rustbox.print(6, 6, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Up : {}", vol));},
                    Key::Down => {vol -= 0.1; rustbox.print(6, 9, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Down : {}", vol));},
                    Key::Char('q') => {
                        stream.stop();
                        break;},
                    _ => {}
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => {}
        };
        tx.send(vol);
        rustbox.present();
    }

    stream.close();
}
