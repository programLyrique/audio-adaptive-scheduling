extern crate basic_example;
extern crate rustbox;
extern crate portaudio;
extern crate rand;
extern crate time;

use std::env;
use basic_example::sndfile::*;
use basic_example::samplerate::*;

use std::sync::mpsc;
use std::thread;

use std::error::Error;

use rustbox::Key;
use rustbox::{Color, RustBox};

use portaudio as pa;

use std::io::prelude::*;
use std::fs::File;


const FRAMES_PER_BUFFER : u32 = 64;
const UP_RATIO : f64 = 2.;

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
     let mut up_ratio = UP_RATIO;
     let mut chunk_it = 0;

     //Resampling apparatus...
     let mut nb_samples_interm = nb_channels as usize * FRAMES_PER_BUFFER as usize * up_ratio as usize;

     let mut upsampler = SmartResampler::new(ConverterType::Linear, nb_channels as u32, up_ratio, nb_samples_interm * 10);
     let mut downsampler = Resampler::new(ConverterType::Linear, nb_channels as u32, 1. / up_ratio);

     let mut interm_buffer = vec![0.;nb_samples_interm];
     interm_buffer.reserve(nb_channels as usize * FRAMES_PER_BUFFER as usize * std::cmp::max(20, UP_RATIO as usize));

     let callback = move |pa::OutputStreamCallbackArgs {buffer, frames, time, ..}| {
         let start = PreciseTime::now();

         while let Ok(val) = rx.try_recv() {
             up_ratio = val;
         }
         //New ratio so resize buffer and change resampling ratios
         nb_samples_interm = (nb_channels as f64 * FRAMES_PER_BUFFER as f64 * up_ratio).ceil() as usize;
         //println!("New interm buffer size: {}", nb_samples_interm);
         interm_buffer.resize(nb_samples_interm, 0.);//It shoudn't reallocate memory as we have reserved enough before starting the audio thread
         upsampler.set_src_ratio(up_ratio);
         downsampler.set_src_ratio_hard(1. / up_ratio);
         //The problem with set_src_ratio is that it is going to try to transition smoothly to the
         // new ratio, not yielding the righ number of samples.

         let nb_samples = frames * nb_channels;

         /*
          * frame of size 3 with 3 channels. Nb samples is 9
          * ||ch1|ch2|ch3||ch1|ch2|ch3||ch1|ch2|ch3||
          */


        if chunk_it  < audiostream.len()  {

            let input_buffer = &audiostream[chunk_it..std::cmp::min(chunk_it+ nb_samples, audiostream.len())];
            //chunk_it+ nb_samples = audiostream.len() at the end, normally (TODO: to check)

            //upsample
            let gen1 = upsampler.resample(input_buffer, &mut interm_buffer[..]);
            //assert_eq!(gen1.unwrap().1, (FRAMES_PER_BUFFER as f64 * up_ratio).ceil() as u64);//number of frames, not sample
            //Will panic at the end...

            //Do some processing
            //buffer.clone_from_slice(&audiostream[chunk_it..std::cmp::min(chunk_it+ nb_samples, audiostream.len())]);
            // for  sample in buffer.iter_mut() {
            //     *sample = *sample;
            // }
            //

            //downsample
            let gen2 = downsampler.resample(&interm_buffer[..], &mut buffer[..]);
            assert_eq!(gen2.unwrap().1, FRAMES_PER_BUFFER as u64);

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
            upsampler.next_buffer_last();
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
    // let rustbox = match RustBox::init(Default::default()) {
    //     Result::Ok(v) => v,
    //     Result::Err(e) => panic!("{}", e),
    // };
    //
    //
    // let nb_channels_str = &format!("Number of samples x number of channels = {}", nb_channels * nb_frames as usize);
    //
    // rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Simple test of sound nodes with tradeoff between quality and deadlines.");
    // rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, nb_channels_str);
    // rustbox.present();
    //
    // let mut ratio = UP_RATIO;
    // while stream.is_active().unwrap() {
    //     rustbox.clear();
    //     rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Simple test of sound nodes with tradeoff between quality and deadlines.");
    //     rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, nb_channels_str);
    //     rustbox.print(6, 9, rustbox::RB_BOLD, Color::White, Color::Black, &format!("Ratio : {}", ratio));
    //
    //     // let cpu_load = stream.cpu_load();
    //     // let stream_infos = stream.info();
    //     // rustbox.print(1, 5, rustbox::RB_BOLD, Color::White, Color::Black,
    //     //     &format!("CPU load {} ; Input latency: {}s ; Output latency: {}", cpu_load, stream_infos.input_latency, stream_infos.output_latency));
    //
    //     match rustbox.poll_event(false) {
    //         Ok(rustbox::Event::KeyEvent(key)) => {
    //             match key {
    //                 Key::Up => {ratio += 1.; tx.send(ratio).unwrap();},
    //                 Key::Down => {ratio -= 1.; tx.send(ratio).unwrap();},
    //                 Key::Char('q') => {
    //                     stream.stop().unwrap();
    //                     break;},
    //                 _ => {}
    //             }
    //         },
    //         Err(e) => panic!("{}", e.description()),
    //         _ => {}
    //     };
    //
    //     rustbox.present();
    // }

    let mut ratio = 1.;
    while stream.is_active().unwrap() {
        pa.sleep(1000);
        ratio = ((ratio * 10. ) as u32 % 150) as f64 / 10. + 1.;
        println!("At {}s, resampling ratio is now: {}", time::precise_time_s(), ratio);
        tx.send(ratio).unwrap();
    }

    println!("End of playback");

    stream.close().unwrap();
}
