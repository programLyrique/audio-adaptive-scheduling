//! Define an audio graph, an effect and so on
use petgraph::{Graph, EdgeDirection};
use petgraph::graph::{NodeIndex, EdgeIndex, Edges, WalkNeighbors};
use petgraph::algo::toposort;

use std::collections::HashMap;
use std::hash::{Hash,Hasher};


use time::{PreciseTime, Duration};

#[derive(Debug)]
pub enum AudioGraphError {
    Cycle,
}

pub trait AudioEffect {
    //But several channels and several outputs? Later. Now, we rather mix the inputs in the buffer
    fn process(&mut self, buffer: &mut [f32], samplerate : u32, channels : usize);

    /// How many differents effects there are
    fn nb_effects(&self) -> usize;

    ///Id effect: should be in [|0, nb_effects - 1|]
    fn id(&self) -> usize;
}

pub struct AudioGraph<T : Copy + AudioEffect + Eq> {
    graph : Graph<T,Vec<f32> >,
    schedule : Vec< NodeIndex<u32> >,
    size : usize,//Default size of a connection buffer
    time_nodes : Vec<Stats>,//Hashtable to keep mean execution time for every type of node
    time_input : Stats, //Mean time to populate one input connection
    time_output : Stats,//Mean time to populate one output connection
}


impl<T : AudioEffect + Eq + Hash + Copy> AudioGraph<T> {
    pub fn new(size : usize) -> AudioGraph<T> {
        AudioGraph {graph : Graph::new(), schedule : Vec::new(), size : size,
            time_nodes : vec![Stats::new();2], time_input : Stats::new(), time_output : Stats::new()}
    }

    pub fn add_node(&mut self, node : T) -> NodeIndex {
        self.graph.add_node(node)
    }

    pub fn add_input(&mut self, src : T, dest : NodeIndex) -> NodeIndex {
        let parent = self.graph.add_node(src);
        self.graph.add_edge(parent, dest, vec![0.;self.size]);
        parent
    }

    pub fn add_output(&mut self, src : NodeIndex, dest : T) -> NodeIndex {
        let child = self.graph.add_node(dest);
        self.graph.add_edge(src, child, vec![0.;self.size]);
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
        self.graph.add_edge(src, dest, vec![0.;self.size])
    }

    pub fn outputs(&self, src : NodeIndex) -> Edges<Vec<f32>, u32> {
        self.graph.edges_directed(src, EdgeDirection::Outgoing)
    }

    pub fn outputs_mut(&self, src : NodeIndex) -> WalkNeighbors<u32> {
        self.graph.neighbors_directed(src, EdgeDirection::Outgoing).detach()
    }

    pub fn inputs(&self, dest : NodeIndex) -> Edges<Vec<f32>, u32> {
        self.graph.edges_directed(dest, EdgeDirection::Incoming)
    }

    pub fn update_schedule(&mut self) -> Result<(), AudioGraphError> {
        self.schedule = toposort(&self.graph);
        //we take this occasion to check if the graph is cyclic
        //For that, we just need to check if the schedule has less elements than the size of the graph
        if self.schedule.len() < self.graph.node_count() {
            Err(AudioGraphError::Cycle)
        }
        else {
            Ok(())
        }
    }

    /// Adaptive version of the process method for the audio graph
    /// rel_dealine must be in milliseconds (actually, we could even have a nanoseconds granularity)
    pub fn process_adaptive(& mut self, buffer: &mut [f32], samplerate : u32, channels : usize, rel_deadline : f64) {
        let start = PreciseTime::now();

        for index in self.schedule.iter() {
            //Get input edges here, and the buffers on this connection, and mix them
            let mut stats = self.time_input;
            for (_, buf) in self.inputs(*index) {
                //TODO: for later, case with connections that change of size
                let input_time = PreciseTime::now();
                for (s1,s2) in buffer.iter_mut().zip(buf) {
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
                self.graph.edge_weight_mut(edge).unwrap().copy_from_slice(buffer);
                self.time_output.update_time(output_time);
            }

            let late = start.to(PreciseTime::now()).num_microseconds().unwrap() - rel_deadline as i64;
            if  late > 0 {
                println!("Deadline missed with {} microseconds", late);
            }
        }
    }

}

impl<T : AudioEffect + Eq + Hash + Copy> AudioEffect for AudioGraph<T> {
    ///A non adaptive version of the execution of the audio graph
    fn process(&mut self, buffer: &mut [f32], samplerate : u32, channels : usize) {
        for index in self.schedule.iter() {
            //Get input edges here, and the buffers on this connection, and mix them
            for (_, buf) in self.inputs(*index) {
                //TODO: for later, case with connections that change of size
                for (s1,s2) in buffer.iter_mut().zip(buf) {
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
                self.graph.edge_weight_mut(edge).unwrap().copy_from_slice(buffer)
            }
        }
    }

    fn nb_effects(&self) -> usize { 1}
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

    fn nb_effects(&self) -> usize {2}

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
        let mut audio_graph = AudioGraph::new(64);
        let mut buffer = vec![0.;64];

        let mixer = audio_graph.add_node(DspNode::Mixer);

        for i in 1..11 {
            audio_graph.add_input(DspNode::Oscillator(i as f32, 44100 / i, 0.9 / i as f32), mixer);
        }

        audio_graph.update_schedule().expect("There is a cycle here");

        for _ in 0..10000 {
            audio_graph.process(buffer.as_mut_slice(), 44100, 2)
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
