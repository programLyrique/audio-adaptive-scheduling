//! # Simple binding to libsamplerate
//!
use libc::{c_int, c_float, c_long, c_double};


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


#[link(name ="samplerate")]
extern {
    fn src_simple(src_data : *mut src_data, converter_type : ConverterType, channels : c_int) -> c_int;
}


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

#[derive(Debug)]
struct Resampler<'a> {
    data_in : &'a[f32],
    data_out : &'a mut[f32],
    src_ratio : f64
}

impl<'a> Resampler<'a> {
    pub fn resample_simple(&mut self, converter_type : ConverterType, channels : u32) -> Option<u64> {
        let mut src_data = src_data {
            data_in : self.data_in.as_ptr(),
            data_out : self.data_out.as_mut_ptr(),
            src_ratio : self.src_ratio,
            input_frames : self.data_in.len() as c_long,
            output_frames : self.data_out.len() as c_long,
            input_frames_used : 0,
            output_frames_gen : 0,
            end_of_input : 0,
        };

        let result = unsafe { src_simple(&mut src_data as *mut src_data, converter_type, channels as c_int) };

        if 0 == result  {
            return Some(src_data.output_frames_gen as u64);
        }
        else {
            return None
        }
    }
}
