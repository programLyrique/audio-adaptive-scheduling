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

#[link(name ="samplerate")]
extern {
    fn src_simple(src_data : *mut src_data, converter_type : c_int, channels : c_int) -> c_int;

}

pub enum ConverterType {
    SincBestQuality,
    SincMediumQuality,
    SincFastest,
    ZeroOrderHold,
    Linear,
}

//TODO: add first argument, but not necessarily with the same layout as src_data. Maybe rather use a method?
pub fn resample_simple(converter_type : ConverterType, channels : u32) {
    unimplemented!();
}
