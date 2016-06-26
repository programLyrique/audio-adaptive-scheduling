//! Define an audio graph, an effect and so on
use petgraph::{Graph, EdgeDirection};
use petgraph::graph::{NodeIndex, EdgeIndex, Edges, WalkNeighbors};
use petgraph::algo::toposort;

use std::hash::{Hash,Hasher};

use samplerate::{Resampler, ConverterType};

use time::{PreciseTime};

use std::cell::{Cell};

use std::fmt;

#[derive(Debug)]
pub enum AudioGraphError {
    Cycle,
}

pub trait AudioEffect {
    //But several channels and several outputs? Later. Now, we rather mix the inputs in the buffer
    fn process(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize);

    /// How many differents effects there are
    fn nb_effects() -> usize;

    ///Id effect: should be in [|0, nb_effects - 1|]
    fn id(&self) -> usize;
}

pub struct Connection<'a> {
    buffer : Vec<f32>,
    resample : Cell<bool>,//Has to be resampled
    resampled : bool,//Was resampled during previous cycle
    resampler : Resampler<'a>
}

impl<'a> Connection<'a> {
    fn new(buffer : Vec<f32>, channels : u32) -> Connection<'a>  {
        Connection {buffer : buffer ,
                resample : Cell::new(false),
                resampled : false,
                resampler : Resampler::new(ConverterType::Linear, channels, 1.0)//We can change the number of channels
            }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TimeMonitor {
    pub quality : Quality,
    pub budget : i64,
    pub expected_remaining_time : u64,
    pub deadline : u64,
}

pub struct AudioGraph<'a, T : Copy + AudioEffect + Eq> {
    graph : Graph<T,Connection<'a>>,
    sink : Connection<'a>,
    schedule : Vec< NodeIndex<u32> >,
    schedule_expected_time : Vec<f64>,//Cumulated expected execution time for every node starting from the end
    //Use to calculate remaining expected time
    size : usize,//Default size of a connection buffer
    channels : u32,//Number of channels
    time_nodes : Vec<Stats>,//To keep mean execution time for every type of node
    //Why not a HashMap? Too slow! (100-150µs). We rather do our "own" hash table, which perfect
    //hashing as we know the number of different kinds of nodes (it is nb_effects)
    time_input : Stats, //Mean time to populate one input connection
    time_output : Stats,//Mean time to populate one output connection
    time_resampler : Stats,//Time to upsample/downsample
}

#[derive(Copy, Clone, Debug)]
pub enum Quality {
    Normal,
    Degraded
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Quality::Normal => "Normal",
            Quality::Degraded => "Degraded"
        };
        write!(f, "{}", name)
    }
}

impl<'a, T : AudioEffect + Eq + Hash + Copy> AudioGraph<'a, T> {
    /// Create a new AudioGraph
    /// `frames_per_buffer` and `channels`are used to compute the actual size of a buffer
    /// which is `frames_per_buffer * channels`
    pub fn new(frames_per_buffer : usize, channels : u32) -> AudioGraph<'a, T> {
        let size = frames_per_buffer * channels as usize;

        AudioGraph {graph : Graph::new(), schedule : Vec::new(),
            sink : Connection::new(vec![0.;size], channels),
            schedule_expected_time : Vec::new(),
            size : size,
            channels : channels,
            time_nodes : vec![Stats::new();T::nb_effects()],
            time_input : Stats::new(),
            time_output : Stats::new(),
            time_resampler : Stats::init(15.)}
    }

    pub fn add_node(&mut self, node : T) -> NodeIndex {
        self.graph.add_node(node)
    }

    pub fn add_input(&mut self, src : T, dest : NodeIndex) -> NodeIndex {
        let parent = self.graph.add_node(src);
        self.graph.add_edge(parent, dest, Connection::new(vec![0.;self.size], self.channels));
        parent
    }

    pub fn add_output(&mut self, src : NodeIndex, dest : T) -> NodeIndex {
        let child = self.graph.add_node(dest);
        self.graph.add_edge(src, child, Connection::new(vec![0.;self.size], self.channels));
        child
    }

    pub fn remove_connection(&mut self, src: NodeIndex, dest : NodeIndex) {
        if let Some(edge_index) = self.graph.find_edge(src, dest) {
            self.graph.remove_edge(edge_index).expect("Edge should exist");
        }
    }

    pub fn remove_edge(&mut self, edge : EdgeIndex) {
        self.graph.remove_edge(edge);
    }

    pub fn add_connection(&mut self, src: NodeIndex, dest : NodeIndex) -> EdgeIndex {
        self.graph.add_edge(src, dest, Connection::new(vec![0.;self.size], self.channels))
    }

    pub fn outputs(& self, src : NodeIndex) -> Edges<Connection, u32> {
        self.graph.edges_directed(src, EdgeDirection::Outgoing)
    }

    pub fn nb_outputs(& self, src : NodeIndex) -> u32 {
        self.outputs(src).count() as u32
    }

    pub fn outputs_mut(&self, src : NodeIndex) -> WalkNeighbors<u32> {
        self.graph.neighbors_directed(src, EdgeDirection::Outgoing).detach()
    }

    pub fn inputs(& self, dest : NodeIndex) -> Edges<Connection, u32> {
        self.graph.edges_directed(dest, EdgeDirection::Incoming)
    }

    pub fn inputs_mut(&self, src : NodeIndex) -> WalkNeighbors<u32> {
        self.graph.neighbors_directed(src, EdgeDirection::Incoming).detach()
    }

    pub fn nb_inputs(& self, dest : NodeIndex) -> u32 {
        self.inputs(dest).count() as u32
    }

    pub fn update_schedule(&mut self) -> Result<(), AudioGraphError> {
        self.schedule = toposort(&self.graph);
        self.schedule_expected_time.resize(self.schedule.len(), 0.);

        //we take this occasion to check if the graph is cyclic
        //For that, we just need to check if the schedule has less elements than the size of the graph
        if self.schedule.len() < self.graph.node_count() {
            Err(AudioGraphError::Cycle)
        }
        else {
            Ok(())
        }
    }


    /// Populate the vec `schedule_expected_time`
    /// `schedule_expected_time[i]` is the remaining time in the schedule `self.schedule` from node i included
    // to the last node.
    ///
    /// This should be invoked once at the beginning of every dsp cycle, or if the schedule changes
    fn update_remaining_times(& mut self) {
        let len = self.schedule.len();

        let mut expected_acc = 0.;

        //Iterating backward from the end
        for i in (0..len).rev() {
            let node_index = self.schedule[i];
            let node = self.graph.node_weight(node_index).unwrap();
            //Time to copy/mix the input, to to do the actual computations, and to copy
            //into the output buffers
            expected_acc += self.time_nodes[node.id()].mean +
                self.time_input.mean * self.nb_inputs(node_index) as f64 +
                self.time_output.mean * self.nb_outputs(node_index) as f64;
            self.schedule_expected_time[i] = expected_acc;
        }
    }

    ///Update the adaptive scheduling.
    /// `budget`is the remaining computing budget (in microseconds)
    /// `node` is the index in the schedule of the node that is going to be executed next
    ///
    /// If some resamplers are decided to be used, then
    fn update_adaptive(& self, budget : f64, node : usize) -> Quality {
        //Expected remaining time of computation after this node compared to budget?
        if budget >= self.schedule_expected_time[node] {
            //TODO: disable resampling
            //TODO: except if permanent overload
            return Quality::Normal;
        }
        //Otherwise, select the nodes to degrade, so where to insert downsampler and upsampler

        // let expected_time : f64 = self.schedule.iter().skip(node).map(|&node_index| {
        //     //Computation time of the node, but time to mix the input buffers to the input buffer of the node +
        //     // time to copy to the output buffers
        //     self.time_nodes[self.graph.node_weight(node_index).unwrap().id()].mean +
        //     self.time_input.mean +
        //     self.time_output.mean
        // }).fold(0., |acc, v| acc+v);
        let mut expected_time = 0.;
        let len = self.schedule.len();
        for i in node..len {
            let node_index = self.schedule[i];
            {//For lifetime and borrowing of self.graph
                let node = self.graph.node_weight(node_index).expect("Current node in schedule not found!");
                expected_time += self.time_nodes[node.id()].mean +
                    self.time_input.mean * self.nb_inputs(node_index) as f64 +
                    self.time_output.mean * self.nb_outputs(node_index) as f64;
            }
            //We are likely to have a missed deadline
            // So we are going to insert resamplers
            // We can do something if it's not the last node
            if expected_time > budget && i < len - 1 {
                //the further in a branch to the output, the better quality we have
                // So we backtrack from the node we attained, not exploring everything now
                //Why not from the last node?
                //Because anyway, we will have to degrade the non explored nodes after the current node,
                // because we are already missing the deadline

                //TODO: see if we have enough time to explore everything backward
                let mut current_node = node_index;
                let resampling_ratio = 2.;//Fixed now...
                let factor = (resampling_ratio - 1.) / resampling_ratio; // 1/2 here

                //Remaining time of all the remaining nodes in the schedule
                let expected_remaining_time = self.schedule_expected_time[i];
                //Calculate the overall time of the reamining nodes to execute
                //It is the expected time already calculated + the remaining time of the other
                // nodes which will all be degraded + the time to downsample and then upsample
                let mut degraded_time = expected_time + expected_remaining_time + 2. * self.time_resampler.mean;

                //We can only degrade nodes which have not been executed yet
                //Why iter().rev().any()? Because we are more likely to find nodes
                // to degrade at the end of the branch, near the outputs
                // Maybe the actual implementation of contains is faster...
                //TODO: we should have a more efficient lookup than this linear search
                while self.schedule[i+1..len].iter().rev().any(|&no| no == current_node) {
                    //Pick one input (and here the first one)
                    current_node = self.inputs(current_node).next().expect("No input node found").0;
                    {//For lifetime and borrowing of self.graph
                        let input_node = self.graph.node_weight(current_node).expect("Next remaining node not found");

                        /* We supose that the execution time of effects is at most linear in the number
                            of samples in their input buffer.
                            TODO we should measure also the mean execution time for degraded versions
                            instead of assuming linear decrease
                        */
                        degraded_time -= factor *  self.time_nodes[input_node.id()].mean;
                    }
                    if degraded_time <= budget {
                        //get the incoming edges going to this nodes
                        let mut edges = self.inputs_mut(current_node);
                        while let Some(edge_index) = edges.next_edge(&self.graph) {
                            //If we were not resampling before
                            let connection = self.graph.edge_weight(edge_index).expect("Connection not found!");
                            if !connection.resample.get() {
                                connection.resample.set(true);
                                //connection.resampler.reset();
                                //Change buffer sizes? Only in the children nodes

                            }
                        }
                        return Quality::Degraded;
                    }
                }
                //Even degrading is not enough but we decide to degrade nevertheless
                //get the incoming edges going to this nodes
                let mut edges = self.inputs_mut(current_node);
                while let Some(edge_index) = edges.next_edge(&self.graph) {
                    //If we were not resampling before
                    let connection = self.graph.edge_weight(edge_index).expect("Connection not found!");
                    if !connection.resample.get() {
                        connection.resample.set(true);
                        //connection.resampler.reset();
                        //Change buffer sizes? Only in the children nodes

                    }
                }
                return Quality::Degraded;

                //the last node must resample from the right input...
                //TODO: from node_index, find the path to the output... and change buffer sizes
                //Or rather, when buffer do not have the same size, automatically resample?


            }
        }
        return Quality::Normal;
    }


    /// Adaptive version of the process method for the audio graph
    /// rel_dealine must be in milliseconds (actually, we could even have a nanoseconds granularity)
    pub fn process_adaptive(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize, rel_deadline : f64) {
        let mut budget = rel_deadline as i64;

        let soundcard_size = buffer.len();

        let start = PreciseTime::now();

        self.update_remaining_times();//from 5-6 µs, to 36µs (300 elements), and 420µs for 30000 nodes

        budget -= start.to(PreciseTime::now()).num_microseconds().unwrap();

        let mut quality = Quality::Normal;

        let mut resample = false;

        for (i,index) in self.schedule.iter().enumerate() {

            //We won't perform another analysis on quality once we are in the degraded mode
            let time_update = PreciseTime::now();
            match quality {
                Quality::Normal =>  {
                    quality = self.update_adaptive(budget as f64, i);
                },
                Quality::Degraded => ()
            };
            budget -= time_update.to(PreciseTime::now()).num_microseconds().unwrap();//300 nodes, 100µs?!
            //println!("{} microseconds", rel_deadline as i64 - budget);

            //Duplication, but not possible to put it in a method, as rust will complain about
            // self borrowed as immutable and mutabel as the same time (as we need to modify some fields of self)
            match quality {
                Quality::Normal => {
                    let mut stats = self.time_input;
                    //Get input edges here, and the buffers on this connection, and mix them
                    for (_, connection) in self.inputs(*index) {
                        let input_time = PreciseTime::now();
                        for (s1,s2) in buffer.iter_mut().zip(&connection.buffer) {
                            *s1 += *s2
                        }
                        let time_now = PreciseTime::now();
                        let duration = input_time.to(time_now).num_microseconds().unwrap();
                        stats.update(duration as f64);
                        budget -= duration;
                    }
                    self.time_input = stats;

                    {
                        let node = self.graph.node_weight_mut(*index).unwrap();
                        let node_time = PreciseTime::now();

                        node.process(buffer, samplerate, channels);
                        let time_now = PreciseTime::now();
                        let duration = node_time.to(time_now).num_microseconds().unwrap();
                        self.time_nodes[node.id()].update(duration as f64);
                        budget -= duration;
                    }

                    //Write buffer in the output edges

                    let mut edges = self.outputs_mut(*index);
                    while let Some(edge) = edges.next_edge(&self.graph) {
                        let output_time = PreciseTime::now();
                        let connection = self.graph.edge_weight_mut(edge).unwrap();
                        connection.buffer.copy_from_slice(buffer);
                        connection.resampled = false;
                        let time_now = PreciseTime::now();
                        let duration = output_time.to(time_now).num_microseconds().unwrap();
                        self.time_output.update(duration as f64);
                        budget -= duration;
                    }
                },
                Quality::Degraded => {
                    let start_time = PreciseTime::now();

                    for (_, connection) in self.inputs(*index) {
                        //debug_assert_eq!(connection.buffer.len(), buffer.len());
                        for (s1,s2) in buffer.iter_mut().zip(&connection.buffer) {
                            *s1 += *s2
                        }
                    }

                    {
                        let node = self.graph.node_weight_mut(*index).unwrap();

                        let end = if resample {soundcard_size / 2} else {soundcard_size};
                        node.process(&mut buffer[0..end], samplerate, channels);
                    }

                    //Write buffer in the output edges
                    let mut edges = self.outputs_mut(*index);
                    while let Some(edge) = edges.next_edge(&self.graph) {

                        let connection = self.graph.edge_weight_mut(edge).unwrap();

                        if connection.resample.get() && !resample {//It's the beginning of a degrading chain
                        debug_assert_eq!(connection.buffer.len(), buffer.len());
                        println!("Starting degrading");
                            resample = true;
                            if connection.resampled {//Should take about 11 microseconds
                                connection.resampled = true;
                                connection.resampler.reset();
                            }

                            //downsample
                            connection.resampler.set_src_ratio_hard(0.5);
                            connection.buffer.truncate(soundcard_size/ 2);
                            connection.resampler.resample(buffer, connection.buffer.as_mut_slice()).expect("Downsampling failed.");
                            connection.resample.set(false);
                        }
                        else if !connection.resample.get() && resample {//The end of the chain
                            debug_assert_eq!(connection.buffer.len(), buffer.len()/2);
                            println!("Ending degrading");
                            resample = false;
                            if connection.resampled {
                                connection.resampled = true;
                                connection.resampler.reset();
                            }
                            //upsample
                            connection.resampler.set_src_ratio_hard(2.);
                            connection.buffer.resize(soundcard_size, 0.);
                            connection.resampler.resample(buffer, connection.buffer.as_mut_slice()).expect("Upsampling failed.");
                            connection.resample.set(false);
                        }
                        else {//Buffers have same size
                            debug_assert_eq!(connection.buffer.len(), buffer.len());
                            connection.resampled = false;
                            connection.buffer.copy_from_slice(buffer);
                        }

                    }

                    budget -= start_time.to(PreciseTime::now()).num_microseconds().unwrap();
                }
            }


            if  budget < 0 {
                //println!("Deadline missed with {} microseconds", -budget);
            }
        }
    }

    ///Same as process_adaptive but uses a less computationally costly strategy to find the nodes to degrade
    pub fn process_adaptive2(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize, rel_deadline : f64) -> TimeMonitor {
        let mut budget = rel_deadline as i64;

        let soundcard_size = buffer.len();
        let end = soundcard_size / 2;

        let start = PreciseTime::now();

        self.update_remaining_times();//from 5-6 µs, to 36µs (300 elements), and 420µs for 30000 nodes
        let mut expected_remaining_time = self.schedule_expected_time[0];

        let mut quality = Quality::Normal;

        budget -= start.to(PreciseTime::now()).num_microseconds().unwrap();

        for (i,index) in self.schedule.iter().enumerate() {
            let start_time = PreciseTime::now();
            //We won't perform another analysis on quality once we are in the degraded mode
            match quality {//300 nodes, 100µs?!
                Quality::Normal =>
                quality = if budget - self.schedule_expected_time[i] as i64 >= 0 {
                    Quality::Normal
                } else {
                    expected_remaining_time = start.to(PreciseTime::now()).num_microseconds().unwrap() as f64 + self.schedule_expected_time[i];
                    Quality::Degraded
                },
                Quality::Degraded => ()
            };
            // budget -= time_update.to(PreciseTime::now()).num_microseconds().unwrap();
            //println!("{} microseconds", rel_deadline as i64 - budget);

            //Duplication, but not possible to put it in a method, as rust will complain about
            // self borrowed as immutable and mutabel as the same time (as we need to modify some fields of self)
            match quality {
                Quality::Normal => {
                    let mut stats = self.time_input;
                    //Get input edges here, and the buffers on this connection, and mix them
                    for (_, connection) in self.inputs(*index) {
                        let input_time = PreciseTime::now();
                        for (s1,s2) in buffer.iter_mut().zip(&connection.buffer) {
                            *s1 += *s2
                        }
                        let time_now = PreciseTime::now();
                        let duration = input_time.to(time_now).num_microseconds().unwrap();
                        stats.update(duration as f64);
                    }
                    self.time_input = stats;

                    {
                        let node = self.graph.node_weight_mut(*index).unwrap();
                        let node_time = PreciseTime::now();

                        node.process(buffer, samplerate, channels);
                        let time_now = PreciseTime::now();
                        let duration = node_time.to(time_now).num_microseconds().unwrap();
                        self.time_nodes[node.id()].update(duration as f64);
                    }

                    //Write buffer in the output edges

                    let mut edges = self.outputs_mut(*index);
                    while let Some(edge) = edges.next_edge(&self.graph) {
                        let output_time = PreciseTime::now();
                        let connection = self.graph.edge_weight_mut(edge).unwrap();
                        connection.buffer.copy_from_slice(buffer);
                        connection.resampled = false;
                        let time_now = PreciseTime::now();
                        let duration = output_time.to(time_now).num_microseconds().unwrap();
                        self.time_output.update(duration as f64);
                    }
                },
                Quality::Degraded => {
                    //Incoming edges
                    let mut edges = self.inputs_mut(*index);
                    while let Some(edge) = edges.next_edge(&self.graph) {

                        let connection = self.graph.edge_weight_mut(edge).unwrap();
                        if connection.resample.get() {//Means that resampling has already been done previously on the chain
                            for (s1,s2) in buffer[0..end].iter_mut().zip(&connection.buffer[0..end]) {
                                *s1 += *s2
                            };
                            connection.resample.set(false);//Set to false for the next cycle
                        }
                        else {
                            if !connection.resampled {
                                connection.resampler.reset();
                                connection.resampler.set_src_ratio_hard(0.5);
                                connection.resampled = true;
                            }
                            for (s1,s2) in buffer[0..end].iter_mut().zip(&connection.buffer[0..end]) {
                                *s1 += *s2
                            }
                        }
                    }

                    //Node processing
                    {
                        let node = self.graph.node_weight_mut(*index).unwrap();
                        node.process(&mut buffer[0..end], samplerate, channels);
                    }

                    //Outcoming edges
                    let mut edges = self.outputs_mut(*index);
                    while let Some(edge) = edges.next_edge(&self.graph) {

                        let connection = self.graph.edge_weight_mut(edge).unwrap();
                        connection.buffer[0..end].copy_from_slice(&buffer[0..end]);
                        connection.resample.set(true);
                        //To indicate that we don't need to resample this connection for the next node
                    };

                }
            }
            budget -= start_time.to(PreciseTime::now()).num_microseconds().unwrap();
        }
        //If the quality has been degraded, in this version, we only upsample at the end, after the last node
        match quality {
            Quality::Degraded => {
                let start_time = PreciseTime::now();
                if !self.sink.resampled {
                    self.sink.resampler.reset();
                    self.sink.resampler.set_src_ratio_hard(2.0);
                    self.sink.resampled = true;
                }

                self.sink.resampler.resample(buffer, self.sink.buffer.as_mut_slice()).expect("Upsampling just before sink node failed");
                buffer.copy_from_slice(&self.sink.buffer);
                budget -= start_time.to(PreciseTime::now()).num_microseconds().unwrap();
            },
            Quality::Normal => (),
        }

        TimeMonitor {quality : quality, budget : budget,
                    deadline : rel_deadline as u64,
                    expected_remaining_time : expected_remaining_time as u64}
    }


}

impl<'a, T : AudioEffect + Eq + Hash + Copy> AudioEffect for AudioGraph<'a, T> {
    ///A non adaptive version of the execution of the audio graph
    fn process(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize) {
        for index in self.schedule.iter() {
            //Get input edges here, and the buffers on this connection, and mix them
            for (_, connection) in self.inputs(*index) {
                //TODO: for later, case with connections that change of size
                for (s1,s2) in buffer.iter_mut().zip(&connection.buffer) {
                    *s1 += *s2
                }
            }
            self.graph.node_weight_mut(*index).unwrap().process(buffer, samplerate, channels);
            //Write buffer in the output edges

            let mut edges = self.outputs_mut(*index);
            while let Some(edge) = edges.next_edge(&self.graph) {
                //TODO: for later, case with connections that change of size
                self.graph.edge_weight_mut(edge).unwrap().buffer.copy_from_slice(buffer)
            }
        }
    }

    fn nb_effects() -> usize { 1}
    fn id(&self) -> usize {0}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DspNode {
    Oscillator(f32, u32, f32),
    Modulator(f32, u32, f32),
    Mixer,
}

impl Eq for DspNode {}

impl AudioEffect for DspNode {
    fn process(&mut self, buffer : &mut [f32], samplerate : u32, channels : usize) {
        match *self {
            DspNode::Mixer => (),
            DspNode::Modulator(ref mut phase, frequency, volume) => {
                for chunk in buffer.chunks_mut(channels) {
                    for channel in chunk.iter_mut() {
                        *channel *= sine_wave(*phase, volume);
                    }
                    *phase += frequency as f32 / samplerate as f32;
                }
            }
            DspNode::Oscillator(ref mut phase, frequency, volume) => {
                /*
                 * frame of size 3 with 3 channels. Nb samples is 9
                 * ||ch1|ch2|ch3||ch1|ch2|ch3||ch1|ch2|ch3||
                 */
                for chunk in buffer.chunks_mut(channels) {
                    for channel in chunk.iter_mut() {
                        *channel = sine_wave(*phase, volume);
                    }
                    *phase += frequency as f32 / samplerate as f32;
                }
            }
        }
    }

    fn nb_effects() -> usize {3}

    fn id(&self) -> usize {
        match *self {
            DspNode::Mixer => 0,
            DspNode::Oscillator(_,_,_) => 1,
            DspNode::Modulator(_,_,_) => 2,
        }
    }
}
impl Hash for DspNode {
    fn hash<H>(&self, state : &mut H) where H: Hasher {
        state.write_u32 (match *self {
            DspNode::Mixer => 1,
            DspNode::Oscillator(_, _, _) => 2,
            DspNode::Modulator(_, _, _) => 3
        })
    }
}

fn sine_wave(phase : f32, volume : f32) -> f32 {
    use std::f64::consts::PI;
    (phase * PI as f32 * 2.0).sin() as f32 * volume
}

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    mean : f64,
    var : f64,//We may use it later, but we don't compute it so far
    n : u64,
}

/// Compute an online mean: mean_{n+1} = f(mean_n, x)
/// TODO: make it also possible to use an exponential moving average
impl Stats {
    fn new() -> Stats {
        Stats {mean : 0., var : 0., n : 0}
    }

    fn init(m : f64) -> Stats {
        Stats {mean : m, var : 0., n : 1}
    }
    //Better to make it generic on Num types?
    //TODO: rather calculate a moving average
    #[inline(always)]
    fn update(&mut self, x : f64) -> f64 {
        self.n += 1;
        let delta = x - self.mean;
        self.mean += delta / self.n as f64;
        self.mean
    }

    #[inline(always)]
    fn update_time(&mut self, prev_time : PreciseTime) -> f64 {
        let time_now = PreciseTime::now();
        let duration = prev_time.to(time_now).num_microseconds().unwrap();
        self.update(duration as f64)
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;
    use std::hash::{Hash, SipHasher, Hasher};

    #[test]
    fn test_audio_graph() {
        let mut audio_graph = AudioGraph::new(64, 2);
        let mut buffer = vec![0.;64 * 2];

        let mixer = audio_graph.add_node(DspNode::Mixer);

        let nb_oscillators = 300;

        for i in 1..nb_oscillators {
            audio_graph.add_input(DspNode::Oscillator(i as f32, 350 + i*50, 0.9 / nb_oscillators as f32), mixer);
        }

        audio_graph.update_schedule().expect("There is a cycle here");

        for _ in 0..10000 {
            audio_graph.process_adaptive(buffer.as_mut_slice(), 44100, 2, 500.)
        };
        assert!(buffer.iter().any(|x| (*x).abs() > EPSILON))
    }

    #[test]
    fn test_chain() {
        let nb_frames = 128;
        let mut audio_graph = AudioGraph::new(nb_frames, 2);
        let mut buffer = vec![0.;nb_frames * 2];

        let mixer = audio_graph.add_node(DspNode::Mixer);
        let nb_modulators = 300;
        let mut prev_mod = mixer;
        for i in 1..nb_modulators {
            prev_mod = audio_graph.add_input(DspNode::Modulator(i as f32, 350 + i*50, 1. ), prev_mod);
        }
        audio_graph.add_input(DspNode::Oscillator(0., 135, 0.7 ), prev_mod);
        audio_graph.update_schedule().expect("Cycle detected");

        for _ in 0..1000 {
            let times = audio_graph.process_adaptive2(buffer.as_mut_slice(), 44100, 2, 3000.);
            if times.budget < 0 {
                println!("Missed deadline!");
            }
        };
        assert!(buffer.iter().any(|x| (*x).abs() > EPSILON))
    }

    #[test]
    fn test_dsp_node() {
        let n1 = DspNode::Oscillator(2., 44100, 0.5);
        let n2 = DspNode::Oscillator(3., 44100, 0.6);

        let mut s = SipHasher::new();
        let h1 = n1.hash(&mut s);
        let h2 = n2.hash(&mut s);
        s.finish();

        assert_eq!(h1, h2);

    }
}
