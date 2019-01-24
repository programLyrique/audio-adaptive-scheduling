//! Audiographs
//! Compared to the ones in effect.rs, these ones are more
//! complex. Multiple inputs and outputs.
//! They are also meant to be used statically: no specific resamplers
//! on connections but a resampler is a node akin to the other ones
use petgraph::{Graph, EdgeDirection, Directed};
use petgraph::graph::{NodeIndex, EdgeIndex, Edges, WalkNeighbors};
use petgraph::algo::toposort;
use petgraph::dot::{Dot};
use petgraph::visit::EdgeRef;
use petgraph;

use std::fmt;

use audiograph_parser;

#[derive(Debug)]
pub enum AudioGraphError {
    Cycle,
}

impl From<petgraph::algo::Cycle<NodeIndex>> for AudioGraphError {
    fn from(_e : petgraph::algo::Cycle<NodeIndex>) -> AudioGraphError {
        AudioGraphError::Cycle
    }
}

pub struct DspEdge {
    src_port : u32,
    dst_port : u32,
    buffer : Vec<f32>,
}

impl DspEdge {
    pub fn new(src_port:u32, dst_port:u32, buffer_size:usize) -> DspEdge {
        DspEdge {src_port, dst_port, buffer_size: vec![0.;buffer_size]}
    }
}


pub struct DspNode {
    node_infos : audiograph_parser::Node,
    node_processor : Box<dyn AudioEffect>
}

impl DspNode {
    pub fn new(node_infos : audiograph_parser::Node, node_processor : Box<dyn AudioEffect>) -> DspNode {
        DspNode {node_infos, node_processor}
    }
}


pub trait AudioEffect : fmt::Display {
    fn process(& mut self, inputs: &[DspEdge], outputs: &mut [DspEdge], samplerate : u32);
    // See also <Buffers: AsRef<[Buffer]>, Buffer: AsRef<[f32]>>(buffers: Buffers)

    fn nb_inputs(&self) -> usize;
    fn nb_outputs(&self) -> usize;
}


impl AudioEffect for Box<AudioEffect>
{
    #[inline]
    fn process(& mut self, inputs: &[DspEdge], outputs: &mut[DspEdge], samplerate : u32){
        (**self).process(inputs, outputs, samplerate);
    }

    #[inline]
    fn nb_inputs(&self) -> usize { (**self).nb_inputs()}

    #[inline]
    fn nb_outputs(&self) -> usize { (**self).nb_outputs()}
}

pub struct AudioGraph {
    graph : Graph<DspNode,DspEdge>,
    schedule : Vec< NodeIndex<u32> >,
    size : usize,//Default size of a buffer
    channels : u32,//Number of channels,
    frames_per_buffer : u32,
}

impl AudioGraph {
    pub fn new(frames_per_buffer : u32, channels : u32) -> AudioGraph {
        AudioGraph {graph : Graph::new(), schedule : Vec::new(),
            size : frames_per_buffer as usize,
            frames_per_buffer,
            channels,
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

        /*pub fn add_node(&mut self, node : T) -> NodeIndex {
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

        */

        pub fn nb_outputs(& self, src : NodeIndex) -> u32 {
            self.outputs(src).count() as u32
        }

        pub fn outputs_mut(&self, src : NodeIndex) -> WalkNeighbors<u32> {
            self.graph.neighbors_directed(src, EdgeDirection::Outgoing).detach()
        }

        pub fn inputs(& self, dest : NodeIndex) -> Edges<DspEdge, Directed> {
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
                println!(" Sink");//TODO. Add explicit sink and source nodes !! (Or at least say explicitly that we add them automatically)
            }


            Ok(())
        }

        pub fn nb_active_nodes(&self) -> usize {
            self.schedule.len()
        }

}

fn sine_wave(phase : f32, volume : f32) -> f32 {
    use std::f64::consts::PI;
    (phase * PI as f32 * 2.0).sin() as f32 * volume
}

#[derive(Debug)]
struct Oscillator {
    phase: f32,
    frequency: u32,
    volume: f32
}

impl Oscillator {
    pub fn new(initial_phase: f32, frequency: u32, volume: f32) -> Oscillator {
        Oscillator {phase: initial_phase, frequency, volume}
    }
}

impl fmt::Display for  Oscillator {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "osc({}, {}, {})", self.phase, self.frequency, self.volume)
    }
}

impl AudioEffect for Oscillator {
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge], samplerate : u32) {
        debug_assert!(inputs.len() == self.nb_inputs());
        debug_assert!(outputs.len() == self.nb_outputs());

        for sample in outputs[0].buffer.iter_mut() {
            *sample = sine_wave(self.phase, self.volume);
            self.phase += self.frequency as f32 / samplerate as f32;
        }
    }

    fn nb_inputs(&self) -> usize {0}
    fn nb_outputs(&self) -> usize {1}
}

/// Similar to :> or <: in Faust. Can be used as a mixer if :>
#[derive(Debug)]
pub struct InputsOutputsAdaptor {
    nb_inputs: usize,
    nb_outputs: usize,
    stride: usize,
}

impl InputsOutputsAdaptor {
    pub fn new(nb_inputs: usize, nb_outputs: usize) -> InputsOutputsAdaptor {
        assert!(nb_inputs % nb_outputs == 0 || nb_outputs % nb_inputs == 0 );
        let stride = if nb_outputs > nb_inputs {nb_outputs % nb_inputs} else {nb_inputs % nb_outputs};
        InputsOutputsAdaptor {nb_inputs, nb_outputs, stride}
    }
}

impl fmt::Display for  InputsOutputsAdaptor {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "adaptor({}, {})", self.nb_inputs, self.nb_outputs)
    }
}

fn mixer(buffer: &mut [f32], input_buffer: & [f32]) {
    for (s1,s2) in buffer.iter_mut().zip(input_buffer) {
        *s1 += *s2
    }
}

impl AudioEffect for InputsOutputsAdaptor {
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge], samplerate : u32) {
        debug_assert!(inputs.len() == self.nb_inputs());
        debug_assert!(outputs.len() == self.nb_outputs());
        debug_assert!(self.nb_inputs % self.nb_outputs == 0 || self.nb_outputs % self.nb_inputs == 0 );

        if self.nb_outputs > self.nb_inputs {
            for (i,group) in outputs.chunks_mut(self.stride).enumerate() {
                for output in group.iter_mut() {
                    output.buffer.copy_from_slice(&inputs[i].buffer);
                }
            }
        }
        else {
            for (i,group) in inputs.chunks(self.stride).enumerate() {
                for input in group {
                    mixer(&mut outputs[i].buffer, &input.buffer);
                }
            }
        }
    }

    fn nb_inputs(&self) -> usize {self.nb_inputs}
    fn nb_outputs(&self) -> usize {self.nb_outputs}
}

#[derive(Debug)]
pub struct Sink<'a> {
    nb_channels: usize,
    frames_per_buffer: u32,
    sink_buffer: &'a mut [f32],
}

#[derive(Debug)]
pub struct Source<'a> {
    nb_channels: usize,
    frames_per_buffer: u32,
    source_buffer: &'a [f32],
}
