extern crate basic_example;

use std::env;
use basic_example::sndfile::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut sndfile = SndFile::open(&args[1]).expect("Error while opening file");

    let audiostream = sndfile.readf_float_all();

    println!("Simple test of sound nodes with tradeoff between quality and deadlines.");
    println!("Number of samples x number of channels = {}", audiostream.len());
}
