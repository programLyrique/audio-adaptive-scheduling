//! Define an audio graph, an effect and so on
use petgraph::{Graph, EdgeDirection, Directed};
use petgraph::graph::{NodeIndex, EdgeIndex, Edges, WalkNeighbors};
use petgraph::algo::toposort;
use petgraph::dot::{Dot};
use petgraph::visit::EdgeRef;
use petgraph;

use std::hash::{Hash,Hasher};

use samplerate::{Resampler, ConverterType};

use time::{PreciseTime, Duration};
use portaudio as pa;

use std::cell::{Cell};

use std::fmt;

use stats::*;

#[derive(Debug)]
pub enum AudioGraphError {
    Cycle,
}

impl From<petgraph::algo::Cycle<NodeIndex>> for AudioGraphError {
    fn from(_e : petgraph::algo::Cycle<NodeIndex>) -> AudioGraphError {
        AudioGraphError::Cycle
    }
}

pub trait AudioEffect {
    //But several channels and several outputs? Later. Now, we rather mix the inputs in the buffer
    fn process(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize);

    /// How many differents effects there are
    fn nb_effects() -> usize;

    ///Id effect: should be in [|0, nb_effects - 1|]
    fn id(&self) -> usize;
}


pub struct Connection {
    buffer : Vec<f32>,
    resample : Cell<bool>,//Has to be resampled
    resampled : bool,//Was resampled during previous cycle. Used to know whether we need to change the state of the resampler or if it has already been put in the right state on the previous cycles
    resampler : Resampler
}

impl Connection {
    fn new(buffer : Vec<f32>, channels : u32) -> Connection  {
        Connection {buffer : buffer ,
                resample : Cell::new(false),
                resampled : false,
                resampler : Resampler::new(ConverterType::Linear, channels, 1.0)//We can change the number of channels
            }
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.buffer.len())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CallbackFlags {
    NO_FLAG,
    /// In a stream opened with paFramesPerBufferUnspecified, indicates that input data is
    /// all silence (zeros) because no real data is available. In a stream opened without
    /// `FramesPerBufferUnspecified`, it indicates that one or more zero samples have been
    /// inserted into the input buffer to compensate for an input underflow.
    INPUT_UNDERFLOW,
    /// In a stream opened with paFramesPerBufferUnspecified, indicates that data prior to
    /// the first sample of the input buffer was discarded due to an overflow, possibly
    /// because the stream callback is using too much CPU time. Otherwise indicates that
    /// data prior to one or more samples in the input buffer was discarded.
    INPUT_OVERFLOW,
    /// Indicates that output data (or a gap) was inserted, possibly because the stream
    /// callback is using too much CPU time.
    OUTPUT_UNDERFLOW,
    /// Indicates that output data will be discarded because no room is available.
    OUTPUT_OVERFLOW,
    /// Some of all of the output data will be used to prime the stream, input data may be
    /// zero.
    PRIMING_OUTPUT,
}

impl CallbackFlags {
    pub fn from_callback_flags(flags : pa::stream::callback_flags::CallbackFlags) -> CallbackFlags {
        match flags  {
            pa::stream::callback_flags::NO_FLAG => CallbackFlags::NO_FLAG,
            pa::stream::callback_flags::INPUT_UNDERFLOW => CallbackFlags::INPUT_UNDERFLOW,
            pa::stream::callback_flags::INPUT_OVERFLOW => CallbackFlags::INPUT_OVERFLOW,
            pa::stream::callback_flags::OUTPUT_UNDERFLOW => CallbackFlags::OUTPUT_UNDERFLOW,
            pa::stream::callback_flags::OUTPUT_OVERFLOW => CallbackFlags::OUTPUT_OVERFLOW,
            pa::stream::callback_flags::PRIMING_OUTPUT => CallbackFlags::PRIMING_OUTPUT,
            _ => CallbackFlags::NO_FLAG,
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct TimeMonitor {
    /// Quality chosen
    pub quality : Quality,
    /// Time budget remaining at the end (if negative, deadline exceeded)
    pub budget : i64,
    /// Expected remaining time when we decided to start degraded (or at the end)
    pub expected_remaining_time : u64,
    /// Deadline as given by portaudio
    pub deadline : u64,
    /// Execution time for one cycle
    pub execution_time : u64,
    ///Number of degraded effects
    pub nb_degraded : u64,
    pub nb_resamplers : u64,//Number of inserted resamplers
    pub callback_flags : CallbackFlags,
    ///Duration taken to compute the degradation
    pub choosing_duration : u64,
}

pub struct AudioGraph<T : Copy + AudioEffect + fmt::Display + Eq> {
    graph : Graph<T,Connection>,
    sink : Connection,
    schedule : Vec< NodeIndex<u32> >,
    schedule_expected_time : Vec<f64>,//Cumulated expected execution time for every node starting from the end
    //Use to calculate remaining expected time
    size : usize,//Default size of a connection buffer
    channels : u32,//Number of channels,
    frames_per_buffer : u32,
    time_nodes : Vec<Stats>,//To keep mean execution time for every type of node
    //Why not a HashMap? Too slow! (100-150µs). We rather do our "own" hash table, which perfect
    //hashing as we know the number of different kinds of nodes (it is nb_effects)
    time_input : Stats, //Mean time to populate one input connection
    time_output : Stats,//Mean time to populate one output connection
    time_resampler : Stats,//Time to upsample/downsample
    temp_buffer: Vec<f32>,//Used for mixing in the exhaustive strategy
}

#[derive(Copy, Clone, Debug)]
pub enum Quality {
    Normal,
    Degraded
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Quality::Normal => 1,
            Quality::Degraded => 0
        };
        write!(f, "{}", name)
    }
}

impl<T : fmt::Display + AudioEffect + Eq + Hash + Copy> fmt::Display for AudioGraph<T> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        /*let config = vec![Config::EdgeNoLabel];
        let dot_fmt = Dot::with_config(&self.graph, &config);*/
        let dot_fmt = Dot::new(&self.graph);
        write!(f, "Default size: {}\n", self.size)?;
        write!(f, "Channels: {}\n", self.channels)?;
        dot_fmt.fmt(f)
    }
}

impl<T : fmt::Display + AudioEffect + Eq + Hash + Copy> AudioGraph<T> {
    /// Create a new AudioGraph
    /// `frames_per_buffer` and `channels`are used to compute the actual size of a buffer
    /// which is `frames_per_buffer * channels`
    pub fn new(frames_per_buffer : u32, channels : u32) -> AudioGraph<T> {
        let size = frames_per_buffer as usize * channels as usize;

        AudioGraph {graph : Graph::new(), schedule : Vec::new(),
            sink : Connection::new(vec![0.;size], channels),
            schedule_expected_time : Vec::new(),
            size : size,
            frames_per_buffer : frames_per_buffer,
            channels : channels,
            time_nodes : vec![Stats::new();T::nb_effects()],
            time_input : Stats::new(),
            time_output : Stats::new(),
            time_resampler : Stats::init(15.),
            temp_buffer : Vec::with_capacity(size)
        }
    }

    pub fn nb_nodes(&self) -> usize {
        //Nb of active nodes
        self.graph.node_count()
    }

    pub fn nb_edges(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn nb_channels(&self) -> u32 {
        self.channels
    }

    pub fn default_buffer_size(&self) -> usize  {
        self.size
    }

    pub fn frames_per_buffer(&self) -> u32 {
        self.frames_per_buffer
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

    pub fn outputs(& self, src : NodeIndex) -> Edges<Connection, Directed> {
        self.graph.edges_directed(src, EdgeDirection::Outgoing)
    }

    pub fn nb_outputs(& self, src : NodeIndex) -> u32 {
        self.outputs(src).count() as u32
    }

    pub fn outputs_mut(&self, src : NodeIndex) -> WalkNeighbors<u32> {
        self.graph.neighbors_directed(src, EdgeDirection::Outgoing).detach()
    }

    pub fn inputs(& self, dest : NodeIndex) -> Edges<Connection, Directed> {
        self.graph.edges_directed(dest, EdgeDirection::Incoming)
    }

    pub fn inputs_mut(&self, src : NodeIndex) -> WalkNeighbors<u32> {
        self.graph.neighbors_directed(src, EdgeDirection::Incoming).detach()
    }

    pub fn nb_inputs(& self, dest : NodeIndex) -> u32 {
        self.inputs(dest).count() as u32
    }

    pub fn update_schedule(&mut self) -> Result<(), AudioGraphError> {
        self.schedule = toposort(&self.graph, None)?;//If Cycle, returns an AudioGraphError::Cycle
        self.schedule_expected_time.resize(self.schedule.len(), 0.);

        if self.schedule.len() <= 100
        {
            print!("The schedule is: ", );
            for node_index in self.schedule.iter() {
                let node = self.graph.node_weight(*node_index).unwrap();
                print!("{} -> ", node);
            }
            println!(" Sink");
        }


        Ok(())
    }

    pub fn nb_active_nodes(&self) -> usize {
        self.schedule.len()
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
                    current_node = self.inputs(current_node).next().expect("No input node found").source();
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

    /// Process without degrading. Baseline to compare
    pub fn process_baseline(&mut self, buffer: &mut [f32], samplerate : u32, channels : usize, rel_deadline : f64, flags : CallbackFlags) -> TimeMonitor {
        let mut budget = rel_deadline as i64;

        let start = PreciseTime::now();

        let quality = Quality::Normal;

        self.update_remaining_times();
        let mut expected_remaining_time = self.schedule_expected_time[0];

        for (i, index) in self.schedule.iter().enumerate() {
            let start_time  = PreciseTime::now();

            if (budget as f64) < self.schedule_expected_time[i] {
                expected_remaining_time = start.to(PreciseTime::now()).num_microseconds().unwrap() as f64 + self.schedule_expected_time[i]
            }

            let mut stats = self.time_input;
            //Get input edges here, and the buffers on this connection, and mix them
            for connection in self.inputs(*index) {
                let duration = Duration::span(|| {
                    mixer(buffer, &connection.weight().buffer)
                }).num_microseconds().unwrap();
                stats.update(duration as f64);
            }
            self.time_input = stats;

            {
                let node = self.graph.node_weight_mut(*index).unwrap();

                let duration = Duration::span(|| {
                    node.process(buffer, samplerate, channels)
                }).num_microseconds().unwrap();

                self.time_nodes[node.id()].update(duration as f64);
            }

            //Write buffer in the output edges

            let mut edges = self.outputs_mut(*index);
            while let Some(edge) = edges.next_edge(&self.graph) {
                let connection = self.graph.edge_weight_mut(edge).unwrap();
                let duration = Duration::span(|| {
                    debug_assert_eq!(buffer.len() , connection.buffer.len());
                    connection.buffer.copy_from_slice(buffer);
                }).num_microseconds().unwrap();

                self.time_output.update(duration as f64);
            }
            budget -= start_time.to(PreciseTime::now()).num_microseconds().unwrap();
        }

        budget = start.to(PreciseTime::now()).num_microseconds().unwrap();

        TimeMonitor {quality, budget,
                    deadline : rel_deadline as u64,
                    expected_remaining_time : expected_remaining_time as u64,
                    execution_time : start.to(PreciseTime::now()).num_microseconds().unwrap() as u64,
                    callback_flags : flags,
                    nb_resamplers : 0,
                    choosing_duration : 0,
                    nb_degraded : 0
                }
    }

    /// Adaptive version of the process method for the audio graph
    /// rel_dealine must be in milliseconds (actually, we could even have a nanoseconds granularity)
    pub fn process_adaptive_progressive(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize, rel_deadline : f64, flags : CallbackFlags) -> TimeMonitor {
        let mut budget = rel_deadline as i64;

        let soundcard_size = buffer.len();

        let start = PreciseTime::now();
        let mut choosing_duration = Duration::seconds(0);
        let mut nb_resamplers=0;//Nb of inserted resamplers

        self.update_remaining_times();//from 5-6 µs, to 36µs (300 elements), and 420µs for 30000 nodes

        let mut expected_remaining_time = self.schedule_expected_time[0];
        let mut first_degraded_node : Option<u64> = None;

        budget -= start.to(PreciseTime::now()).num_microseconds().unwrap();

        let mut quality = Quality::Normal;

        let mut resample = false;

        for (i,index) in self.schedule.iter().enumerate() {
            #[cfg(debuger_Assertions)]
            println!("Dealing with node {}", self.graph.node_weight(*index).unwrap());

            let start_time = PreciseTime::now();

            //We won't perform another analysis on quality once we are in the degraded mode
            let time_update = PreciseTime::now();
            match quality {
                Quality::Normal =>  {
                    choosing_duration = Duration::span(|| {quality = self.update_adaptive(budget as f64, i);});
                    match quality  {
                        Quality::Degraded => {expected_remaining_time = start.to(PreciseTime::now()).num_microseconds().unwrap() as f64 + self.schedule_expected_time[i];
                        first_degraded_node = Some(i as u64);},
                        _ => ()
                    }
                },
                Quality::Degraded => ()
            };
            budget -= time_update.to(PreciseTime::now()).num_microseconds().unwrap();//300 nodes, 100µs?!
            //println!("{} microseconds", rel_deadline as i64 - budget);

            let end = if resample {soundcard_size / 2} else {soundcard_size};
            //if resample {println!("Resample: {} so end = {}", resample, end)};

            //Duplication, but not possible to put it in a method, as rust will complain about
            // self borrowed as immutable and mutable as the same time (as we need to modify some fields of self)
            match quality {
                Quality::Normal => {
                    let mut stats = self.time_input;
                    //Get input edges here, and the buffers on this connection, and mix them
                    for connection in self.inputs(*index) {
                        let duration = Duration::span(|| {
                            mixer(buffer, &connection.weight().buffer)
                        }).num_microseconds().unwrap();
                        stats.update(duration as f64);
                    }
                    self.time_input = stats;

                    {
                        let node = self.graph.node_weight_mut(*index).unwrap();

                        let duration = Duration::span(|| {
                            node.process(buffer, samplerate, channels)
                        }).num_microseconds().unwrap();

                        self.time_nodes[node.id()].update(duration as f64);
                    }

                    //Write buffer in the output edges

                    let mut edges = self.outputs_mut(*index);
                    while let Some(edge) = edges.next_edge(&self.graph) {
                        let connection = self.graph.edge_weight_mut(edge).unwrap();
                        if connection.buffer.len() != buffer.len() {
                            //We are in normal mode so
                            // Resize to normal size
                            connection.buffer.resize(soundcard_size, 0.0);//should be soundcard_size
                         }
                        let duration = Duration::span(|| {
                            debug_assert_eq!(buffer.len() , connection.buffer.len());
                            connection.buffer.copy_from_slice(buffer);//TODO: panic here because of destination and source slice with not same size
                            connection.resampled = false;
                        }).num_microseconds().unwrap();

                        self.time_output.update(duration as f64);
                    }
                },
                Quality::Degraded => {


                    let mut input_edges = self.inputs_mut(*index);
                    while let Some(edge) = input_edges.next_edge(&self.graph) {
                        // crashed here before. Of course: buffer can be 2x connection.buffer
                        // We need to check if it's inside a resampled branch and then
                        // mix to buffer[0..end]

                        let connection = self.graph.edge_weight_mut(edge).unwrap();
                        if resample {
                            connection.buffer.truncate(end);
                        }
                        else {
                            connection.buffer.resize(soundcard_size, 0.);
                        }
                        debug_assert_eq!(connection.buffer.len(), end);
                        mixer(&mut buffer[0..end], &connection.buffer);
                    }

                    {
                        let node = self.graph.node_weight_mut(*index).unwrap();

                        node.process(&mut buffer[0..end], samplerate, channels);
                    }

                    //Write buffer in the output edges
                    let mut edges = self.outputs_mut(*index);
                    let mut to_resample_next_cycle = false;
                    while let Some(edge) = edges.next_edge(&self.graph) {

                        let connection = self.graph.edge_weight_mut(edge).unwrap();

                        if connection.resample.get() && !resample {//It's the beginning of a degrading chain
                            #[cfg(debuger_Assertions)]
                            println!("Starting degrading");
                            to_resample_next_cycle = true;
                            if !connection.resampled {//Should take about 11 microseconds
                                //debug_assert_eq!(connection.buffer.len(), buffer.len());
                                connection.resampled = true;
                                connection.resampler.reset();
                                connection.resampler.set_src_ratio_hard(0.5);
                            }
                            connection.buffer.truncate(soundcard_size/ 2);
                            debug_assert_eq!(connection.buffer.len(), buffer.len() / 2);
                            //downsample
                            let duration  = Duration::span(|| {
                                connection.resampler.resample(buffer, connection.buffer.as_mut_slice()).expect("Downsampling failed.");
                             }).num_microseconds().unwrap();
                            self.time_resampler.update(duration as f64);

                            nb_resamplers += 1;
                            connection.resample.set(false);
                        }
                        else if !connection.resample.get() && resample {//The end of the chain
                            #[cfg(debuger_Assertions)]
                            println!("Ending degrading");
                            to_resample_next_cycle = false;
                            if !connection.resampled {// before, it could have been a dowsampled connection!
                                //debug_assert_eq!(connection.buffer.len(), buffer.len());
                                connection.resampled = true;
                                connection.resampler.reset();
                                connection.resampler.set_src_ratio_hard(2.);
                            }
                            connection.buffer.resize(soundcard_size, 0.);
                            debug_assert_eq!(connection.buffer.len(), 2*end);
                            //upsample
                            let duration  = Duration::span(|| {
                                connection.resampler.resample(buffer, connection.buffer.as_mut_slice()).expect("Upsampling failed.");
                             }).num_microseconds().unwrap();
                            self.time_resampler.update(duration as f64);
                            nb_resamplers += 1;
                            connection.resample.set(false);
                        }
                        else {
                            if resample {
                                connection.buffer.truncate(end);
                            }
                            else {
                                connection.buffer.resize(soundcard_size, 0.);
                            }
                            debug_assert_eq!(connection.buffer.len(), end);
                            connection.resampled = false;//There is no resampler inserted here, juste maybe an already resampled signal
                            connection.buffer.copy_from_slice(&buffer[0..end]);
                        }

                    }

                    resample = to_resample_next_cycle;

                }
            }
            budget -= start_time.to(PreciseTime::now()).num_microseconds().unwrap();
        }

        TimeMonitor {quality, budget,
                    deadline : rel_deadline as u64,
                    expected_remaining_time : expected_remaining_time as u64,
                    execution_time : start.to(PreciseTime::now()).num_microseconds().unwrap() as u64,
                    callback_flags : flags,
                    nb_resamplers,
                    choosing_duration : choosing_duration.num_microseconds().unwrap() as u64,
                    nb_degraded : first_degraded_node.map_or(0, |n| self.schedule.len() as u64 - n)
                }
    }

    ///Same as process_adaptive but uses a less computationally costly strategy to find the nodes to degrade
    pub fn process_adaptive_exhaustive(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize, rel_deadline : f64, flags : CallbackFlags) -> TimeMonitor {
        let mut budget = rel_deadline as i64;

        let soundcard_size = buffer.len();
        let end = soundcard_size / 2;

        let start = PreciseTime::now();

        self.update_remaining_times();//from 5-6 µs, to 36µs (300 elements), and 420µs for 30000 nodes
        let mut expected_remaining_time = self.schedule_expected_time[0];
        let mut first_degraded_node : Option<u64> = None;

        let  choosing_duration = Duration::seconds(0);
        let mut nb_resamplers = 0;
        let mut quality = Quality::Normal;

        budget -= start.to(PreciseTime::now()).num_microseconds().unwrap();

        for (i,index) in self.schedule.iter().enumerate() {
            #[cfg(debuger_Assertions)]
            println!("Dealing with node {}", self.graph.node_weight(*index).unwrap());
            let start_time = PreciseTime::now();
            //We won't perform another analysis on quality once we are in the degraded mode
            match quality {//300 nodes, 100µs?!
                Quality::Normal =>
                quality = if budget - self.schedule_expected_time[i] as i64 >= 0 {
                    Quality::Normal
                } else {
                    expected_remaining_time = start.to(PreciseTime::now()).num_microseconds().unwrap() as f64 + self.schedule_expected_time[i];
                    first_degraded_node = Some(i as u64);
                    #[cfg(debuger_Assertions)]
                    println!("Start degrading");
                    Quality::Degraded
                },
                Quality::Degraded => () //TODO: Go back to normal if there is enough time budget
            };


            //Duplication, but not possible to put it in a method, as rust will complain about
            // self borrowed as immutable and mutable as the same time (as we need to modify some fields of self)
            match quality {
                Quality::Normal => {
                    let mut stats = self.time_input;
                    //Get input edges here, and the buffers on this connection, and mix them
                    for connection in self.inputs(*index) {
                        let duration = Duration::span(|| {
                            assert_eq!(buffer.len(), connection.weight().buffer.len());//Strange that it does not fail
                            //We certainly need to resize buffer size here //TODO
                            mixer(buffer, &connection.weight().buffer)
                        }).num_microseconds().unwrap();
                        stats.update(duration as f64);
                    }
                    self.time_input = stats;

                    {
                        let node = self.graph.node_weight_mut(*index).unwrap();

                        let duration = Duration::span(|| {
                            node.process(buffer, samplerate, channels);
                        }).num_microseconds().unwrap();
                        self.time_nodes[node.id()].update(duration as f64);
                    }

                    //Write buffer in the output edges

                    let mut edges = self.outputs_mut(*index);
                    while let Some(edge) = edges.next_edge(&self.graph) {
                        let connection = self.graph.edge_weight_mut(edge).unwrap();
                        let duration = Duration::span(|| {
                            //If it was resampled during the previous cycles, reset bufffer sizes
                            if connection.resampled {
                                connection.buffer.resize(buffer.len(), 0.);
                                //Should we also reset the resampler, put ratio to 1, and feed the resample function?
                            }
                            connection.buffer.copy_from_slice(buffer);
                            connection.resampled = false;
                        }).num_microseconds().unwrap();

                        self.time_output.update(duration as f64);
                    }
                },
                Quality::Degraded => {
                    //Incoming edges
                    if self.nb_inputs(*index) > 0 { //We resample the connections in that case
                        //TODO Otherwise, we will simple resample the output edges.
                        let mut edges = self.inputs_mut(*index);
                        //prepare temp buffer. Not fill function in rust...
                        self.temp_buffer.clear();//Does not touch the allocated memo
                        self.temp_buffer.resize(end, 0.0);
                        while let Some(edge) = edges.next_edge(&self.graph) {

                            let connection = self.graph.edge_weight_mut(edge).unwrap();
                            if connection.resample.get() {//Means that resampling has already been done previously on the chain
                                mixer(&mut buffer[0..end], &connection.buffer[0..end]);
                                connection.resample.set(false);//Set to false for the next cycle
                            }
                            else {
                                #[cfg(debuger_Assertions)]
                                println!("Starting to degrade quality");

                                if !connection.resampled {
                                    connection.resampler.reset();
                                    connection.resampler.set_src_ratio_hard(0.5);
                                    connection.buffer.truncate(end);
                                }
                                connection.resampled = true;
                                let time_here = PreciseTime::now();//Not with Duration::span, otherwise problems with borrowing
                                connection.resampler.resample(connection.buffer.as_slice(), &mut self.temp_buffer[0..end]).expect("Downsampling failed.");
                                self.time_resampler.update_time(time_here);

                                nb_resamplers +=1;
                                //TODO: rather mix all the ones that have not been resampled yet. Resample them. Will be more efficient
                                mixer(&mut buffer[0..end], &self.temp_buffer[0..end]);
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
                        if !connection.resampled {
                            connection.buffer.truncate(end);
                        }
                        connection.resampled = true;
                        connection.buffer[0..end].copy_from_slice(&buffer[0..end]);
                        connection.resample.set(true);
                        //To indicate that we don't need to resample this connection for the next node
                    };

                }
            }
            budget -= start_time.to(PreciseTime::now()).num_microseconds().unwrap();
        }
        //If the quality has been degraded, in this version, we only upsample at the end, after the last node
        if nb_resamplers > 0 { //we do not need to oversample if we did not downsample before!
            match quality {
                Quality::Degraded => {
                    #[cfg(debuger_Assertions)]
                    println!("Ending degradation");
                    let start_time = PreciseTime::now();
                    if !self.sink.resampled {
                        self.sink.resampler.reset();
                        self.sink.resampler.set_src_ratio_hard(2.0);
                        self.sink.resampled = true;
                    }
                    let duration  = Duration::span(|| {
                        self.sink.resampler.resample(buffer, self.sink.buffer.as_mut_slice()).expect("Upsampling just before sink node failed");
                     }).num_microseconds().unwrap();
                    self.time_resampler.update(duration as f64);

                    nb_resamplers += 1;
                    buffer.copy_from_slice(&self.sink.buffer);
                    budget -= start_time.to(PreciseTime::now()).num_microseconds().unwrap();
                },
                Quality::Normal => (),
            }
        }

        TimeMonitor {quality, budget,
                    deadline : rel_deadline as u64,
                    expected_remaining_time : expected_remaining_time as u64,
                    execution_time : start.to(PreciseTime::now()).num_microseconds().unwrap() as u64,
                    callback_flags : flags,
                    nb_resamplers,
                    choosing_duration : choosing_duration.num_microseconds().unwrap() as u64,
                    nb_degraded : first_degraded_node.map_or(0, |n| self.schedule.len() as u64 - n)
                }
    }


}

impl<'a, T : fmt::Display + AudioEffect + Eq + Hash + Copy> AudioEffect for AudioGraph<T> {
    ///A non adaptive version of the execution of the audio graph
    fn process(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize) {
        for index in self.schedule.iter() {
            //Get input edges here, and the buffers on this connection, and mix them
            for connection in self.inputs(*index) {
                //TODO: for later, case with connections that change of size
                for (s1,s2) in buffer.iter_mut().zip(&connection.weight().buffer) {
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
    //The first arg is used to compute the delays the IIR filter requires
    LowPass([f32;4], f32,f32),
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
            },
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
            },
            DspNode::LowPass(ref mut prev_values, cutoff, quality) => {
                let mut x_n = prev_values[0];//n
                let mut x_nM = prev_values[1];// n - 1
                let mut y_nM = prev_values[2];// y - 1
                let mut y_nMM = prev_values[3];// y - 2
                // Or n + 1, n, y, and y - 1...

                use std::f64::consts::PI;
                let w = 2. * PI as f32  * cutoff / 44100.;//Make it possible to change the sampling rate frequency
                let d = 1. / quality;
                let beta = ( (1. - (d/2.) * w.sin()) / ( 1. + (d/2.)* w.sin())) / 2.;
                let gamma = (0.5 + beta) * w.cos();

                let a0 = (0.5 + beta - gamma) / 2.;
                let a1 = 0.5 + beta - gamma;
                let a2 = a0;
                let b1 = -2. * gamma;
                let b2 = 2. * beta;

                for chunk in buffer.chunks_mut(channels) {
                    for channel in chunk.iter_mut() {
                        let cur_channel = *channel;
                        *channel = a0 * *channel + a1 * x_n + a2 * x_nM - b1 * y_nM - b2 * y_nMM;

                        //Update delays
                        x_nM = x_n;
                        x_n = cur_channel;
                        y_nMM = y_nM;
                        y_nM = *channel;
                    }

                }
            }
        }
    }

    fn nb_effects() -> usize {4}

    fn id(&self) -> usize {
        match *self {
            DspNode::Mixer => 0,
            DspNode::Oscillator(_,_,_) => 1,
            DspNode::Modulator(_,_,_) => 2,
            DspNode::LowPass(_,_,_) => 3,
        }
    }
}

fn mixer(buffer: &mut [f32], input_buffer: & [f32]) {
    for (s1,s2) in buffer.iter_mut().zip(input_buffer) {
        *s1 += *s2
    }
}

impl Hash for DspNode {
    fn hash<H>(&self, state : &mut H) where H: Hasher {
        state.write_u32 (match *self {
            DspNode::Mixer => 1,
            DspNode::Oscillator(_, _, _) => 2,
            DspNode::Modulator(_, _, _) => 3,
            DspNode::LowPass(_,_,_) => 4,
        })
    }
}

impl fmt::Display for DspNode {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

fn sine_wave(phase : f32, volume : f32) -> f32 {
    use std::f64::consts::PI;
    (phase * PI as f32 * 2.0).sin() as f32 * volume
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

        let mut missed_deadlines = 0;

        for _ in 0..1000 {
            let times = audio_graph.process_adaptive_progressive(buffer.as_mut_slice(), 44100, 2, 500., CallbackFlags::NO_FLAG);
            if times.budget < 0 {
                missed_deadlines +=1;
            }
        };
        println!("Missed deadlines: {}", missed_deadlines);
        assert!(buffer.iter().any(|x| (*x).abs() > EPSILON))
    }

    #[test]
    fn test_chain_total() {
        let nb_frames = 128;
        let mut audio_graph = AudioGraph::new(nb_frames, 2);
        let mut buffer = vec![0.;nb_frames as usize * 2];

        let mixer = audio_graph.add_node(DspNode::Mixer);
        let nb_modulators = 300;
        let mut prev_mod = mixer;
        for i in 1..nb_modulators {
            prev_mod = audio_graph.add_input(DspNode::Modulator(i as f32, 350 + i*50, 1. ), prev_mod);
        }
        audio_graph.add_input(DspNode::Oscillator(0., 135, 0.7 ), prev_mod);
        audio_graph.update_schedule().expect("Cycle detected");

        let mut missed_deadlines = 0;

        for _ in 0..1000 {
            let times = audio_graph.process_adaptive_exhaustive(buffer.as_mut_slice(), 44100, 2, 3000., CallbackFlags::NO_FLAG);
            if times.budget < 0 {
                missed_deadlines+=1;
            }
        };
        println!("Missed deadlines: {}", missed_deadlines);
        assert!(buffer.iter().any(|x| (*x).abs() > EPSILON))
    }

    #[test]
    fn test_chain_progressive() {
        let nb_frames = 128;
        let mut audio_graph = AudioGraph::new(nb_frames, 2);
        let mut buffer = vec![0.;nb_frames as usize * 2];

        let mixer = audio_graph.add_node(DspNode::Mixer);
        let nb_modulators = 300;
        let mut prev_mod = mixer;
        for i in 1..nb_modulators {
            prev_mod = audio_graph.add_input(DspNode::Modulator(i as f32, 350 + i*50, 1. ), prev_mod);
        }
        audio_graph.add_input(DspNode::Oscillator(0., 135, 0.7 ), prev_mod);
        audio_graph.update_schedule().expect("Cycle detected");

        let mut missed_deadlines = 0;

        for _ in 0..1000 {
            let times = audio_graph.process_adaptive_progressive(buffer.as_mut_slice(), 44100, 2, 3000., CallbackFlags::NO_FLAG);
            if times.budget < 0 {
                missed_deadlines += 1;
            }
        };
        println!("Missed deadlines: {}", missed_deadlines);
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
