//! Define an audio graph, an effect and so on
use petgraph::{Graph, EdgeDirection};
use petgraph::graph::{NodeIndex, EdgeIndex, Edges, WalkNeighbors};
use petgraph::algo::toposort;

use std::hash::{Hash,Hasher};

use samplerate::{Resampler, ConverterType};

use time::{PreciseTime, Duration};

#[derive(Debug)]
pub enum AudioGraphError {
    Cycle,
}

pub trait AudioEffect {
    //But several channels and several outputs? Later. Now, we rather mix the inputs in the buffer
    fn process(&mut self, buffer: &mut [f32], samplerate : u32, channels : usize);

    /// How many differents effects there are
    fn nb_effects() -> usize;

    ///Id effect: should be in [|0, nb_effects - 1|]
    fn id(&self) -> usize;
}

pub struct Connection<'a> {
    buffer : Vec<f32>,
    resample : bool,
    resampler : Resampler<'a>
}

impl<'a> Connection<'a> {
    fn new(buffer : Vec<f32>, channels : u32) -> Connection<'a>  {
        Connection {buffer : buffer ,
                resample : false,
                resampler : Resampler::new(ConverterType::Linear, channels, 1.0)//We can change the number of channels
            }
    }
}

pub struct AudioGraph<'a, T : Copy + AudioEffect + Eq> {
    graph : Graph<T,Connection<'a> >,
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
}

enum Quality {
    Normal,
    Degraded
}


impl<'a, T : AudioEffect + Eq + Hash + Copy> AudioGraph<'a, T> {
    /// Create a new AudioGraph
    /// `frames_per_buffer` and `channels`are used to compute the actual size of a buffer
    /// which is `frames_per_buffer * channels`
    pub fn new(frames_per_buffer : usize, channels : u32) -> AudioGraph<'a, T> {
        AudioGraph {graph : Graph::new(), schedule : Vec::new(),
            schedule_expected_time : Vec::new(),
            size : frames_per_buffer * channels as usize,
            time_nodes : vec![Stats::new();T::nb_effects()], time_input : Stats::new(),
            time_output : Stats::new(), channels : channels}
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

    pub fn outputs(&self, src : NodeIndex) -> Edges<Connection, u32> {
        self.graph.edges_directed(src, EdgeDirection::Outgoing)
    }

    pub fn nb_outputs(&self, src : NodeIndex) -> u32 {
        self.outputs(src).count() as u32
    }

    pub fn outputs_mut(&self, src : NodeIndex) -> WalkNeighbors<u32> {
        self.graph.neighbors_directed(src, EdgeDirection::Outgoing).detach()
    }

    pub fn inputs(&self, dest : NodeIndex) -> Edges<Connection, u32> {
        self.graph.edges_directed(dest, EdgeDirection::Incoming)
    }

    pub fn inputs_mut(&self, src : NodeIndex) -> WalkNeighbors<u32> {
        self.graph.neighbors_directed(src, EdgeDirection::Incoming).detach()
    }

    pub fn nb_inputs(&self, dest : NodeIndex) -> u32 {
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
    fn update_remaining_times(&mut self) {
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
    fn update_adaptive(&mut self, budget : f64, node : usize) -> Quality {
        //Expected remaining time of computation after this node compared to budget?
        if budget >= self.schedule_expected_time[node] {
            //TODO: disable resampling
            //TODO: except if permanent overload
            return Quality::Normal;
        }
        //Otherwise, select the nodes to degrade, so wheer to insert downsampler and upsampler

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
                let node = self.graph.node_weight(node_index).unwrap();
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
                let expected_remaining_time = self.schedule.iter().skip(i).map(|&node_index| {
                    self.time_nodes[self.graph.node_weight(node_index).unwrap().id()].mean +
                    self.time_input.mean +
                    self.time_output.mean
                }).fold(0., |acc, v| acc+v);
                //Calculate the overall time of the reamining nodes to execute
                //It is the expected time already calculated + the remaining time of the other
                // nodes which will all be degraded
                let mut degraded_time = expected_time + expected_remaining_time;

                //We can only degrade nodes which have not been executed yet
                //Why iter().rev().any()? Because we are more likely to find nodes
                // to degrade at the end of the branch, near the outputs
                // Maybe the actual implementation of contains is faster...
                //TODO: we should have a more efficient lookup than this linear search
                while self.schedule[i..len].iter().rev().any(|&no| no == current_node) {
                    //Pick one input (and here the first one)
                    let input_node_index = self.inputs(current_node).next().unwrap().0;
                    {//For lifetime and borrowing of self.graph
                        let input_node = self.graph.node_weight(input_node_index).unwrap();

                        /* We supose that the execution time of effects is at most linear in the number
                            of samples in their input buffer.
                            TODO we should measure also the mean execution time for degraded versions
                            instead of assuming linear decrease
                        */
                        degraded_time -= factor *  self.time_nodes[input_node.id()].mean;
                    }
                    if degraded_time <= budget {
                        //get the incoming edges going this nodes
                        let mut edges = self.inputs_mut(input_node_index);
                        while let Some(edge_index) = edges.next_edge(&self.graph) {
                            //If we were not resampling before
                            let connection = self.graph.edge_weight_mut(edge_index).unwrap();
                            if !connection.resample {
                                connection.resample = true;
                                connection.resampler.reset();
                                //Change buffer sizes? Only in the children nodes

                            }
                        }
                    }
                }
                //the last node must resample from the right input...
                //TODO: from node_index, find the path to the output... and change buffer sizes
                //Or rather, when buffer do not have the same size, automatically resample?

                break;
            }
        }
        return Quality::Normal;
    }

    /// Adaptive version of the process method for the audio graph
    /// rel_dealine must be in milliseconds (actually, we could even have a nanoseconds granularity)
    pub fn process_adaptive(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize, rel_deadline : f64) {
        let start = PreciseTime::now();

        self.update_remaining_times();

        for (i,index) in self.schedule.iter().enumerate() {
            //Get input edges here, and the buffers on this connection, and mix them
            let mut stats = self.time_input;
            for (_, connection) in self.inputs(*index) {
                //TODO: for later, case with connections that change of size
                let input_time = PreciseTime::now();
                for (s1,s2) in buffer.iter_mut().zip(&connection.buffer) {
                    *s1 += *s2
                }
                stats.update_time(input_time);
            }
            self.time_input = stats;

            {
                let node = self.graph.node_weight_mut(*index).unwrap();
                let node_time = PreciseTime::now();

                node.process(buffer, samplerate, channels);
                self.time_nodes[node.id()].update_time(node_time);
            }

            //Write buffer in the output edges

            // for (_,buf) in self.outputs(*index) {
            //     // for (s1,s2) in buf.iter_mut().zip(buffer) {
            //     //     *s1 = *s2
            //     // }
            //     buf.as_mut_slice().copy_from_slice(buffer);
            // }

            let mut edges = self.outputs_mut(*index);
            while let Some(edge) = edges.next_edge(&self.graph) {
                let output_time = PreciseTime::now();
                //TODO: for later, case with connections that change of size
                self.graph.edge_weight_mut(edge).unwrap().buffer.copy_from_slice(buffer);
                self.time_output.update_time(output_time);
            }

            let late = start.to(PreciseTime::now()).num_microseconds().unwrap() - rel_deadline as i64;
            if  late > 0 {
                println!("Deadline missed with {} microseconds", late);
            }
        }
    }

}

impl<'a, T : AudioEffect + Eq + Hash + Copy> AudioEffect for AudioGraph<'a, T> {
    ///A non adaptive version of the execution of the audio graph
    fn process(&mut self, buffer: &mut [f32], samplerate : u32, channels : usize) {
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

            // for (_,buf) in self.outputs(*index) {
            //     // for (s1,s2) in buf.iter_mut().zip(buffer) {
            //     //     *s1 = *s2
            //     // }
            //     buf.as_mut_slice().copy_from_slice(buffer);
            // }

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
    Mixer,
}

impl Eq for DspNode {}

impl AudioEffect for DspNode {
    fn process(&mut self, buffer : &mut [f32], samplerate : u32, channels : usize) {
        match *self {
            DspNode::Mixer => (),
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

    fn nb_effects() -> usize {2}

    fn id(&self) -> usize {
        match *self {
            DspNode::Mixer => 0,
            DspNode::Oscillator(_,_,_) => 1
        }
    }
}
impl Hash for DspNode {
    fn hash<H>(&self, state : &mut H) where H: Hasher {
        state.write_u32 (match *self {
            DspNode::Mixer => 1,
            DspNode::Oscillator(_, _, _) => 2
        })
    }
}

fn sine_wave(phase : f32, volume : f32) -> f32 {
    use std::f64::consts::PI;
    (phase * PI as f32 * 2.0).sin() as f32 * volume
}

#[derive(Debug, Clone, Copy)]
struct Stats {
    mean : f64,
    var : f64,//We may use it later, but we don't compute it so far
    n : u64,
}

/// Compute an online mean: mean_{n+1} = f(mean_n, x)
impl Stats {
    fn new() -> Stats {
        Stats {mean : 0., var : 0., n : 0}
    }
    //Better to make it generic on Num types?
    //TODO: rather calculate a moving average
    #[inline(always)]
    fn update(&mut self, x : f64) -> f64 {
        self.n += 1;
        let delta = x - self.mean;
        self.mean = delta / self.n as f64;
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

        for i in 1..11 {
            audio_graph.add_input(DspNode::Oscillator(i as f32, 44100 / i, 0.9 / i as f32), mixer);
        }

        audio_graph.update_schedule().expect("There is a cycle here");

        for _ in 0..10000 {
            audio_graph.process_adaptive(buffer.as_mut_slice(), 44100, 2, 500.)
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
