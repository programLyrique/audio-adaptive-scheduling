//! Define an audio graph, an effect and so on
use petgraph::{Graph, EdgeDirection};
use petgraph::graph::{NodeIndex, EdgeIndex, Edges, WalkNeighbors};
use petgraph::algo::toposort;

use std::collections::HashMap;
use std::hash::{Hash,Hasher};

#[derive(Debug)]
pub enum AudioGraphError {
    Cycle,
}

pub trait AudioEffect {
    //But several channels and several outputs? Later. Now, we rather mix the inputs in the buffer
    fn process(&mut self, buffer: &mut [f32], samplerate : u32, channels : usize);
}

pub struct AudioGraph<T : AudioEffect + Eq> {
    graph : Graph<T,Vec<f32> >,
    schedule : Vec<NodeIndex<u32> >,
    size : usize,//Default size of a connection buffer
    execution_times : HashMap<T, u64>,//Hashtable to keep mean execution time for every type of node
}


impl<T : AudioEffect + Eq + Hash> AudioGraph<T> {
    pub fn new(size : usize) -> AudioGraph<T> {
        AudioGraph {graph : Graph::new(), schedule : Vec::new(), size : size, execution_times : HashMap::new()}
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

}

impl<T : AudioEffect + Eq + Hash> AudioEffect for AudioGraph<T> {
    ///A non adaptive version of the execution of teh audio graph
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
}

#[derive(Debug, PartialEq)]
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
