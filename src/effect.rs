//! Define an audio graph, an effect and so on
use petgraph::Graph;
use petgraph::graph::{NodeIndex, EdgeIndex};
use petgraph::algo::toposort;

#[derive(Debug)]
enum AudioGraphError {
    Cycle,
}

trait AudioEffect {
    //But several channels and several outputs? Later. Now, we rather mix the inputs in the buffer
    fn process(&mut self, buffer: &mut [f32], samplerate : u32);
}

struct AudioGraph<T : AudioEffect> {
    graph : Graph<T,u32>,
    schedule : Vec<NodeIndex<u32> >,
}


impl<T : AudioEffect> AudioGraph<T> {
    fn new() -> AudioGraph<T> {
        AudioGraph {graph : Graph::new(), schedule : Vec::new()}
    }

    fn add_node(&mut self, node : T) -> NodeIndex {
        self.graph.add_node(node)
    }

    fn add_input(&mut self, src : T, dest : NodeIndex) -> NodeIndex {
        let parent = self.graph.add_node(src);
        self.graph.add_edge(parent, dest, 0);
        parent
    }

    fn add_output(&mut self, src : NodeIndex, dest : T) -> NodeIndex {
        let child = self.graph.add_node(dest);
        self.graph.add_edge(src, child, 0);
        child
    }

    fn remove_connection(&mut self, src: NodeIndex, dest : NodeIndex) {
        if let Some(edge_index) = self.graph.find_edge(src, dest) {
            self.graph.remove_edge(edge_index).expect("Edge should exist");
        }
    }

    fn remove_edge(&mut self, edge : EdgeIndex) {
        self.graph.remove_edge(edge);
    }

    fn add_connection(&mut self, src: NodeIndex, dest : NodeIndex) -> EdgeIndex {
        self.graph.add_edge(src, dest, 0)
    }

    fn update_schedule(&mut self) -> Result<(), AudioGraphError> {
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

impl<T : AudioEffect> AudioEffect for AudioGraph<T> {
    ///A non adaptive version of the execution of teh audio graph
    fn process(&mut self, buffer: &mut [f32], samplerate : u32) {
        for index in self.schedule.iter() {
            self.graph.node_weight_mut(*index).unwrap().process(buffer, samplerate);
        }
    }
}

#[derive(Debug)]
enum DspNode {
    Oscillator(f32, u32, f32),
    Mixer,
}

impl AudioEffect for DspNode {
    fn process(&mut self, buffer : &mut [f32], samplerate : u32) {
        match *self {
            DspNode::Mixer => unimplemented!(),
            DspNode::Oscillator(ref mut phase, frequency, volume) => unimplemented!()
        }
    }
}

fn sine_wave(phase : f32) -> f32 {
    use std::f64::consts::PI;
    (phase * PI as f32 * 2.0).sin() as f32
}
