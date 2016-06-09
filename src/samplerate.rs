//! # Simple binding to libsamplerate
//!
use libc::{c_int, c_float, c_long, c_double, c_char};
use std::ffi::CStr;
use std::ptr;
use ringbuffer as rb;


#[derive(Debug)]
#[repr(C)]
struct src_data {
    data_in : *const c_float,
    data_out : *mut c_float,
    input_frames : c_long,
    output_frames : c_long,
    input_frames_used : c_long,
    output_frames_gen : c_long,
    end_of_input: c_int,
    src_ratio: c_double,
}

#[derive(Debug)]
#[repr(C)]
pub enum ConverterType {
    SincBestQuality,
    SincMediumQuality,
    SincFastest,
    ZeroOrderHold,
    Linear,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum SRC_STATE {}

// #[derive(Debug)]
// pub enum ResamplingError {
//     SRC_ERR_MALLOC_FAILED,
// 	SRC_ERR_BAD_STATE,
// 	SRC_ERR_BAD_DATA,
// 	SRC_ERR_BAD_DATA_PTR,
// 	SRC_ERR_NO_PRIVATE,
// 	SRC_ERR_BAD_SRC_RATIO,
// 	SRC_ERR_BAD_PROC_PTR,
// 	SRC_ERR_SHIFT_BITS,
// 	SRC_ERR_FILTER_LEN,
// 	SRC_ERR_BAD_CONVERTER,
// 	SRC_ERR_BAD_CHANNEL_COUNT,
// 	SRC_ERR_SINC_BAD_BUFFER_LEN,
// 	SRC_ERR_SIZE_INCOMPATIBILITY,
// 	SRC_ERR_BAD_PRIV_PTR,
// 	SRC_ERR_BAD_SINC_STATE,
// 	SRC_ERR_DATA_OVERLAP,
// 	SRC_ERR_BAD_CALLBACK,
// 	SRC_ERR_BAD_MODE,
// 	SRC_ERR_NULL_CALLBACK,
// 	SRC_ERR_NO_VARIABLE_RATIO,
// 	SRC_ERR_SINC_PREPARE_DATA_BAD_LEN,
// 	SRC_ERR_BAD_INTERNAL_STATE,
// }

#[link(name ="samplerate")]
extern {
    fn src_simple(src_data : *mut src_data, converter_type : ConverterType, channels : c_int) -> c_int;

    //Full API
    fn src_new(converter_type : ConverterType, channels : c_int, error : *mut c_int) -> *mut SRC_STATE;
    fn src_delete(src_state : *mut SRC_STATE) -> *const SRC_STATE;//Return is supposed to be NULL
    fn src_process(src_state : *mut SRC_STATE, data : *mut src_data) -> c_int;
    fn src_reset(src_state : *mut SRC_STATE) -> c_int;
    //If we didn't want smooth transition when changing the resampling ratio
    fn src_set_ratio(src_state : *mut SRC_STATE, new_ratio : c_double) -> c_int;

    fn src_strerror(error : c_int) -> *const c_char;
}

#[derive(Debug)]
struct SimpleResampler<'a> {
    data_in : &'a[f32],
    data_out : &'a mut[f32],
    src_ratio : f64
}

impl<'a> SimpleResampler<'a> {
    pub fn resample_simple(&mut self, converter_type : ConverterType, channels : u32) -> Result<u64, &str> {
        let mut src_data = src_data {
            data_in : self.data_in.as_ptr(),
            data_out : self.data_out.as_mut_ptr(),
            src_ratio : self.src_ratio,
            input_frames : self.data_in.len() as c_long / channels as c_long,
            output_frames : self.data_out.len() as c_long / channels as c_long,
            input_frames_used : 0,
            output_frames_gen : 0,
            end_of_input : 0,
        };

        let result = unsafe { src_simple(&mut src_data as *mut src_data, converter_type, channels as c_int) };

        if 0 == result  {
            return Ok(src_data.output_frames_gen as u64);
        }
        else {
            let str_error = unsafe { CStr::from_ptr(src_strerror(result)) };
            return Err(str_error.to_str().unwrap())
        }
    }
    pub fn new(data_in : &'a[f32], data_out : &'a mut [f32], src_ratio : f64) -> SimpleResampler<'a> {
        SimpleResampler {data_in : data_in, data_out : data_out, src_ratio : src_ratio}
    }
}

#[derive(Debug)]
pub struct Resampler<'a> {
    channels : u32,
    src_ratio :  f64,
    end_of_input : bool,
    src_state : &'a mut SRC_STATE,
}

impl<'a> Resampler<'a> {
    pub fn new(converter_type : ConverterType, channels : u32, src_ratio: f64) -> Resampler<'a> {
        let error : *mut c_int = ptr::null_mut();
        unsafe {
            let state = src_new(converter_type, channels as c_int, error);
            Resampler {src_ratio : src_ratio, end_of_input : false, src_state : &mut *state, channels : channels}
        }
    }

    pub fn reset(&mut self) {
        unsafe {src_reset(self.src_state)};
    }

    pub fn next_buffer_last(&mut self) {
        self.end_of_input = true;
    }

    pub fn set_src_ratio(&mut self, src_ratio : f64) {
        self.src_ratio = src_ratio;
    }

    pub fn set_src_ratio_hard(&mut self, src_ratio : f64) {
        self.src_ratio = src_ratio;
        unsafe {src_set_ratio(self.src_state, src_ratio)};
    }

    pub fn resample(&mut self, data_in : &[f32], data_out : &mut [f32])  -> Result<(u64, u64), &str> {
        let mut src_data = src_data {
            data_in : data_in.as_ptr(),
            data_out : data_out.as_mut_ptr(),
            src_ratio : self.src_ratio,
            input_frames : data_in.len() as c_long / self.channels as c_long,
            output_frames : data_out.len() as c_long / self.channels as c_long,
            input_frames_used : 0,
            output_frames_gen : 0,
            end_of_input : self.end_of_input as c_int,
        };

        let result = unsafe { src_process(self.src_state, &mut src_data as *mut src_data) };

        if 0 == result  {
            return Ok((src_data.input_frames_used as u64, src_data.output_frames_gen as u64));
        }
        else {
            let str_error = unsafe { CStr::from_ptr(src_strerror(result)) };
            return Err(str_error.to_str().unwrap())
        }
    }
}

impl<'a> Drop for Resampler<'a> {
    fn drop(&mut self) {
        unsafe {src_delete(self.src_state)};
    }
}

/// For several resampling algorithms, libresample yields less samples than requested because of a delay.
/// SmartResampler uses a ring buffer to output the same number of samples as requested.
/// It makes it easier to change the resampling ratio in real time. We also aim at making easier at changing
/// the resampling algorithm, in real time.
pub struct SmartResampler<'a> {
    resampler: Resampler<'a>,
    input_ring : rb::RingBuffer<f32>,
    output_ring : rb::RingBuffer<f32>,
    interm_buffer : Vec<f32>,
    count : u64,
}

// TODO: we don't output the last half buffer now... maybe use the next_buffer_last flag?
impl<'a> SmartResampler<'a> {
    /// `max_buffer_size` must the maximum size an input buffer can be. This is typically
    /// `nb_channels * frames_per_buffer * max_up_ratio`.
    pub fn new(converter_type : ConverterType, channels : u32, src_ratio: f64, max_buffer_size: usize) -> SmartResampler<'a> {
        let resampler = Resampler::new(converter_type, channels, src_ratio);

        let input_buffer  = rb::RingBuffer::new(2 * max_buffer_size);
        let output_buffer  = rb::RingBuffer::new(2 * max_buffer_size);
        let interm_buffer = Vec::with_capacity(max_buffer_size);

        SmartResampler {resampler : resampler, input_ring : input_buffer, output_ring : output_buffer,
              interm_buffer : interm_buffer, count : 0}
    }

    pub fn resample(&mut self, data_in : &[f32], data_out : &mut [f32])  -> Result<(), &str> {
        let nb_channels = self.resampler.channels as u64;

        self.count += 1;

        //Push new samples to the input ring buffer

        //If first, we push some silence at the beginning, creating a delay
        //It's due to the sync resampler (and even linear one?) that have a delay
        if self.count == 1 {
            self.input_ring.fill(data_in.len() / 2, 0.).unwrap();//We may have to resize the ringbuffer in some cases...
            //TODO: resize ringbuffer
        }

        // debug_assert!(self.input_ring.slots_free() <= self.input_ring.capacity());
        self.input_ring.write(data_in).unwrap();

        //Read the right samples. We need to copy them in another buffer because the ringbuffer wraps around, so
        //the memory might not been contiguous
        if self.interm_buffer.len() != data_in.len() {
            self.interm_buffer.resize(data_in.len(), 0.);
        }
        self.input_ring.get(self.interm_buffer.as_mut_slice()).unwrap();


        let (frames_used, frames_gen) = try!(self.resampler.resample(self.interm_buffer.as_slice(), data_out));
        //Advance the input ring buffer by the number of samples used:
        // This number is often less than the size of the provided input buffer (interm_buffer)
        // especially if the resampler is a "sync" resampler

        let gen_size = frames_gen * nb_channels;
        let used_size = frames_used * nb_channels;

        self.input_ring.skip(used_size as usize).unwrap();

        // //Prepare the output ring buffer
        // if self.count == 1 {
        //     self.output_ring.fill(data_out.len() / 2, 0.).unwrap();
        // }
        //
        //
        // self.output_ring.write(&data_out[0..gen_size as usize]).unwrap();//Fill the buffer with the gen samples
        // self.output_ring.read(data_out).unwrap();//Get back the right number of samples

        //It should not be the case as we provided more samples in input
        // if frames_gen * nb_channels < (data_out.len() as u64) {
        //     panic!("Not enough samples to output: got {}, expected {}", frames_gen * nb_channels, data_out.len());
        //     //TODO: also delay output by some samples
        //     //TODO: better. Find out the latency in samples, given buffer sizes
        // }
        Ok(())
    }

    pub fn set_src_ratio(&mut self, src_ratio : f64) {
        self.resampler.src_ratio = src_ratio;
    }

    pub fn set_src_ratio_hard(&mut self, src_ratio : f64) {
        self.resampler.set_src_ratio_hard(src_ratio);
    }

    pub fn next_buffer_last(&mut self) {
        self.resampler.end_of_input = true;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::{Rng, SeedableRng, StdRng};
    use std::f64;
    #[cfg(bench)]
    use test::Bencher;

    #[test]
    pub fn identity_test() {
        let seed : &[_] = &[1, 21, 37, 4];
        let mut rng : StdRng = SeedableRng::from_seed(seed);
        let  input_buffer = (0..300).map(|_| rng.gen::<f32>()).collect::<Vec<_>>();
        // a bit more than 256, because of the delay line of best_sync
        //If we did subsequent calls to usampler and downsampler, we would get the desired numbers...
        //Wouldn't be the case with Linear or ZeroOrderHold

        let mut interm_buffer  = vec![0.;512];

        let mut output_buffer  = vec![0.;256];


        let mut upsampler = Resampler::new(ConverterType::SincBestQuality, 1, 2.);
        upsampler.next_buffer_last();

        let mut downsampler = Resampler::new(ConverterType::SincBestQuality, 1, 0.5);
        downsampler.next_buffer_last();

        let (_,gen1) = upsampler.resample(&input_buffer[..], &mut interm_buffer[..]).unwrap();
        assert_eq!(gen1, 512);

        let (_,gen2) = downsampler.resample(&interm_buffer[..], &mut output_buffer[..]).unwrap();
        assert_eq!(gen2, 255);

        output_buffer[255] = input_buffer[255];//WHy libsamplerate doesn't write the last sample?
        //because of the delay line of best_sync

        //Calculate SRE^2
        let sre = input_buffer.iter().zip(output_buffer.iter())
            .fold(0. as f64, |ac, (&x1, &x2)| {
                let v = (x1 - x2) as f64;
                ac + v * v
            }).sqrt();

        println!("Gen1: {:?} ; gen2: {:?}, sre: {} ", gen1, gen2, sre);

        assert!(sre.abs() <= 0.5);
    }

    /*
    #[bench]
    fn bench_resample_best_sync(b : &mut Bencher) {
        let seed : &[_] = &[1, 21, 37, 4];
        let mut rng : StdRng = SeedableRng::from_seed(seed);
        let  input_buffer = (0..1024).map(|_| rng.gen::<f32>()).collect::<Vec<_>>();
        let mut output_buffer  = vec![0.;2048];


        let mut upsampler = Resampler::new(ConverterType::SincBestQuality, 1, 2.);
        //upsampler.next_buffer_last();

        b.iter( || {
            upsampler.reset();
            upsampler.next_buffer_last();
            upsampler.resample(&input_buffer[..], &mut output_buffer[..]);}
            );
    */
}
