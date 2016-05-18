//!
//! Simple binding to libsndfile
//!

use libc::{c_int, c_char, size_t, c_float};
use std::ffi::{CString, CStr};
use std::path::Path;
use std::default::Default;

#[repr(C)]
#[derive(Default)]
#[derive(Debug)]
struct SF_INFO {
     frames : size_t,
     samplerate : c_int,
     channels : c_int,
     format : c_int,
     sections : c_int,
     seekable : c_int
 }


///Used by sndfile. Inside internally used
enum SNDFILE {}

#[repr(C)]
pub enum SndOpen {
    Read = 0x10,
    Write = 0x20,
    ReadWrite = 0x30
}


#[link(name="sndfile")]
extern {
    fn sf_open(path : *const c_char, mode : SndOpen, sfinfo : *mut SF_INFO) -> *mut SNDFILE;
    //path should be LPCWSTR but according to doc, it is a char

    fn sf_strerror(sndfile : *const SNDFILE) -> *const c_char;

    fn sf_readf_float(sndfile : *mut SNDFILE, audio_stream: *mut c_float, frames: size_t) -> size_t;
    fn sf_writef_float(sndfile : *mut SNDFILE, audio_stream : *const c_float, frames : size_t) -> size_t;

    fn sf_close(sndfile : *mut SNDFILE);
}

pub struct SndFile<'a> {
    sndfile: &'a mut SNDFILE,
    sfinfo: SF_INFO,
}

impl<'a> SndFile<'a> {
    #[allow(unused_variables)]
    pub fn new(path : &Path, mode : SndOpen) -> SndFile {
        unimplemented!()
    }

    ///Open audio file as read-only
    pub fn open<T : AsRef<Path>+'a>(path: T) -> Result<SndFile<'a>, &'a str> {
        let mut sfinfo = SF_INFO { .. Default::default() };
        let input_file = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let sndfile = unsafe { sf_open(input_file.as_ptr(), SndOpen::Read,  &mut sfinfo) };

        if sndfile.is_null() {
            println!("Error while opening file");
            let cstr = unsafe {CStr::from_ptr(sf_strerror(sndfile)) };
            Err(cstr.to_str().unwrap())
        }
        else {
            Ok(SndFile { sndfile: unsafe {&mut *sndfile }, sfinfo : sfinfo})
        }


    }

    /// Read all the audio stream
    pub fn readf_float_all(&mut self) -> Vec<f32> {
        let size = self.sfinfo.frames * self.sfinfo.channels as usize;
        let mut samples : Vec<f32> = Vec::with_capacity(size);
        let psamples = samples.as_mut_ptr();
        unsafe {
            let frames_read = sf_readf_float(self.sndfile, psamples, self.sfinfo.frames);
            samples.set_len(frames_read * self.sfinfo.channels as usize)
        };
        samples
    }
}


impl<'a> Drop for SndFile<'a> {
    fn drop(&mut self) {
        unsafe {sf_close(self.sndfile)}
    }
}
