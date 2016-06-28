Audio adaptive scheduling
=========================

[![Build Status](https://travis-ci.org/programLyrique/audio-adaptive-scheduling.svg?branch=master)](https://travis-ci.org/programLyrique/audio-adaptive-scheduling)


It aims at experimenting a tradeoff between quality and execution time for audio processing.

- Automatic resampling
- Substitution of processing nodes by worse quality versions


### Non Rust dependencies

- libsamplerate
- libsndfile

## Running

### Complex audio graph benchmarks

`cargo run  --release --bin complex_graph nb_nodes`

The results are saved in a file `complex_graph_{date}_{nb_nodes}.csv` with the following columns:

| Quality | Budget | ExpectRemainingTime | Deadline | NbNodes |
| ------- | ------ | ------------------- | -------- | --------|
|         | remaining time after the graph has been executed | Expected remaining time at the beginning, or when the graph starts to be degraded |  |  | |
