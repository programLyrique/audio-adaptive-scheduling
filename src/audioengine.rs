//! ALl the stuff related to audio, audio callback, portaudio, monitoring the callback

use portaudio as pa;
use std::sync::mpsc;
use std::error::Error;
use time::{PreciseTime, Duration};
use std::path::Path;
use std::thread;

use basic_example::sndfile::*;
use basic_example::samplerate::*;

use std::io::prelude::*;
use std::fs::File;

use std;

const FRAMES_PER_BUFFER : u32 = 64;
const UP_RATIO : f64 = 2.;


lazy_static! {
    static ref PORTAUDIO: pa::PortAudio = {
        let pa = pa::PortAudio::new()
            .expect("PortAudio construction shouldn't fail.");
        println!("PortAudio is initialized.");
        pa
    };
}


#[derive(Debug)]
struct TimeMonitoring {
    pub current_invocation : f64,//When the audio callback is invoked (in s)
    pub buffer_dac : f64, // when the first sample of the output buffer will be send to the DAC (in s)
    pub audio_processing : Duration,//duration between beginning of callback and when the audio processing has been finished in the audio callback
    pub ratio : f64,//Resampling ratio
}


pub struct AudioEngine<'a> {
    pub stream : pa::stream::Stream<'a, pa::stream::NonBlocking, pa::stream::Output<f32>>,
    pub control_sender : mpsc::Sender<f64>,
}


impl<'a> AudioEngine<'a> {
    pub fn new<T : AsRef<Path>+'a>(path: T) -> Result<AudioEngine<'a>, pa::Error> {

        /*
         * Load samples
         */
        let mut sndfile = SndFile::open(path).expect("Error while opening file");
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

             f.write_all(b"CurrentInvocation\tBufferDac\tAudioProcNS\tRatio\n").unwrap();
            for monitoring_infos in rx_monit_exec.iter() {
                 let duration : Duration = monitoring_infos.audio_processing;
                 let seria = format!("{}\t{}\t{}\t{}\n", monitoring_infos.current_invocation,
                    monitoring_infos.buffer_dac, duration.num_nanoseconds().unwrap(),
                    monitoring_infos.ratio);
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

         let mut upsampler = SmartResampler::new(ConverterType::Linear, nb_channels as u32, up_ratio, nb_samples_interm * 20);
         let mut downsampler = SmartResampler::new(ConverterType::Linear, nb_channels as u32, 1. / up_ratio, nb_samples_interm * 10);

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
                let gen1 = upsampler.resample(input_buffer, &mut interm_buffer[..]).unwrap();

                //Do some processing
                //buffer.clone_from_slice(&audiostream[chunk_it..std::cmp::min(chunk_it+ nb_samples, audiostream.len())]);
                // for  sample in buffer.iter_mut() {
                //     *sample = *sample;
                // }
                //

                //downsample
                let gen2 = downsampler.resample(&interm_buffer[..], &mut buffer[..]).unwrap();

                //Send monitoring infos
                let duration = start.to(PreciseTime::now());
                tx_monit_exec.send(TimeMonitoring {
                        current_invocation : time.current,
                        buffer_dac : time.buffer_dac,
                        audio_processing : duration,
                        ratio : up_ratio,
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

         //Show default API:
         println!("{:?}", PORTAUDIO.host_api_info(PORTAUDIO.default_host_api().unwrap()).unwrap());

         let settings = try!(PORTAUDIO.default_output_stream_settings(nb_channels as i32, samplerate, nb_frames));
         let mut stream = try!(PORTAUDIO.open_non_blocking_stream(settings, callback));

         try!(stream.start());
         Ok(AudioEngine {
             stream : stream,
             control_sender : tx,
         })
    }
}

impl<'a> Drop for AudioEngine<'a> {
    fn drop(&mut self) {
        use std::error::Error;
        if self.stream.is_active() == Ok(true) {
            if let Err(err) = self.stream.stop() {
                println!("PortAudio.stream.stop: {}", err.description());
            }
        }
        if let Err(err) = self.stream.close() {
            println!("PortAudio.stream.close: {}", err.description());
        }
    }
}
