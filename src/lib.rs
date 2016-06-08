#![allow(dead_code)]
extern crate libc;

#[cfg(test)]
extern crate rand;
#[cfg(bench)]
#[cfg(test)]
extern crate test;

pub mod samplerate;
pub mod sndfile;
pub mod ringbuffer;
