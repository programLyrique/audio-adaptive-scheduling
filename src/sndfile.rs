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

#[repr(C)]
pub enum SndMajorFormat {
    Wav = 0x010000,
    Flac = 0x170000,
}


#[repr(C)]
pub enum SndTypeFormat {
    Pcm16 = 0x0002,
    Pcm24 = 0x0003,
    Pcm32 = 0x004,
    Float = 0x006,
    Double = 0x007,
}

fn file_format(major: SndMajorFormat, minor: SndTypeFormat) -> c_int {
    major as c_int | minor as c_int
}

#[link(name="sndfile")]
extern {
    fn sf_open(path : *const c_char, mode : SndOpen, sfinfo : *mut SF_INFO) -> *mut SNDFILE;
    //path should be LPCWSTR but according to doc, it is a char

    fn sf_strerror(sndfile : *const SNDFILE) -> *const c_char;

    fn sf_readf_float(sndfile : *mut SNDFILE, audio_stream: *mut c_float, frames: size_t) -> size_t;
    fn sf_read_float(sndfile : *mut SNDFILE, audio_stream: *mut c_float, items: size_t) -> size_t;
    fn sf_writef_float(sndfile : *mut SNDFILE, audio_stream : *const c_float, frames : size_t) -> size_t;
    fn sf_write_float(sndfile : *mut SNDFILE, audio_stream : *const c_float, items : size_t) -> size_t;

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
        let file = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let sndfile = unsafe { sf_open(file.as_ptr(), SndOpen::Read,  &mut sfinfo) };

        if sndfile.is_null() {
            println!("Error while opening file");
            let cstr = unsafe {CStr::from_ptr(sf_strerror(sndfile)) };
            Err(cstr.to_str().unwrap())
        }
        else {
            Ok(SndFile { sndfile: unsafe {&mut *sndfile }, sfinfo : sfinfo})
        }
    }

    //Open audio file to write-only
    pub fn open_write<T : AsRef<Path>+'a>(path: T, samplerate: u32, channels: u32) -> Result<SndFile<'a>, &'a str> {
        let mut sfinfo = SF_INFO { .. Default::default() };
        sfinfo.samplerate = samplerate as c_int;
        sfinfo.channels = channels as c_int;
        sfinfo.format = file_format(SndMajorFormat::Wav, SndTypeFormat::Float);

        let file = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let sndfile = unsafe { sf_open(file.as_ptr(), SndOpen::Write,  &mut sfinfo) };

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

    ///Only reads a buffer
    pub fn read_float(&mut self, samples: &mut [f32]) -> usize {
        let psamples = samples.as_mut_ptr();
        unsafe {
            sf_read_float(self.sndfile, psamples, samples.len()) as usize
        }
    }

    //write
    pub fn write_float(&mut self, buffer: &[f32]) {
        unsafe {
            let items_write = sf_write_float(self.sndfile, buffer.as_ptr(), buffer.len());
            assert!(items_write == buffer.len())
        }
    }

    pub fn nb_channels(&self) -> usize {
        self.sfinfo.channels as usize
    }

    pub fn samplerate(&self) -> f64 {
        self.sfinfo.samplerate as f64
    }

    pub fn nb_frames(&self) -> u32 {
        self.sfinfo.frames as u32
    }
}


impl<'a> Drop for SndFile<'a> {
    fn drop(&mut self) {
        unsafe {sf_close(self.sndfile)}
    }
}
