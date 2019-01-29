//! Audiographs
//! Compared to the ones in effect.rs, these ones are more
//! complex. Multiple inputs and outputs.
//! They are also meant to be used statically: no specific resamplers
//! on connections but a resampler is a node akin to the other ones
use petgraph::{Graph, EdgeDirection, Directed};
use petgraph::graph::{NodeIndex, EdgeIndex, Edges, WalkNeighbors};
use petgraph::algo::toposort;
use petgraph::dot::{Dot};
use petgraph::visit::{Reversed, Dfs, VisitMap, Visitable};
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

#[derive(Debug, Clone)]
pub struct DspEdge {
    src_port : u32,
    dst_port : u32,
    buffer : Vec<f32>,
}

impl DspEdge {
    pub fn new(src_port:u32, dst_port:u32, buffer_size:usize) -> DspEdge {
        assert!(src_port >=1 && dst_port >= 1);
        DspEdge {src_port, dst_port, buffer: vec![0.;buffer_size]}
    }

    pub fn buffer(&self) -> &[f32] {
        self.buffer.as_slice()
    }

    pub fn buffer_mut(&mut self) -> &mut [f32] {
        self.buffer.as_mut_slice()
    }

    pub fn resize(&mut self, new_size: usize) {
        self.buffer.resize(new_size, 0.);
    }
}

impl fmt::Display for  DspEdge {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.src_port, self.dst_port)
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

impl fmt::Display for  DspNode {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        self.node_processor.fmt(f)
    }
}


pub trait AudioEffect : fmt::Display {
    fn process(& mut self, inputs: &[DspEdge], outputs: &mut [DspEdge], samplerate : u32);

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

/// Represents an audiograph of nodes with ports.
/// Beware that NodeIndex are invalidated after removal of nodes (so just don't remove anything).
/// If removing is necessary, we could consider using stable_graph instead of graph.
pub struct AudioGraph {
    graph : Graph<DspNode,DspEdge>,
    schedule : Vec< NodeIndex<u32> >,
    size : usize,//Default size of a buffer
    channels : u32,//Number of channels,
    frames_per_buffer : u32,
    input_node_index: NodeIndex,
    input_edges: Vec<DspEdge>,
    has_source: bool,//Whether there is a source from the audio soundcard or only synth
    output_node_index: NodeIndex,
    output_edges: Vec<DspEdge>,
}

impl AudioGraph {
    pub fn new(frames_per_buffer : u32, channels : u32) -> AudioGraph {
        let input_node_infos = audiograph_parser::Node::new();
        let output_node_infos = audiograph_parser::Node::new();
        let mut graph = Graph::new();
        let input_node = DspNode::new(input_node_infos, Box::new( Source {nb_channels:channels as usize, frames_per_buffer}));
        let output_node = DspNode::new(output_node_infos, Box::new( Sink {nb_channels:channels as usize, frames_per_buffer }));
        let input_node_index = graph.add_node(input_node);
        let output_node_index = graph.add_node(output_node);

        let size_io = (channels * frames_per_buffer) as usize;

        AudioGraph {graph, schedule : Vec::new(),
            size : frames_per_buffer as usize,
            frames_per_buffer,
            channels,
            input_node_index,
            input_edges: Vec::new(),
            has_source: false,
            output_node_index,
            output_edges : Vec::new(),
        }
    }

    pub fn source_node(&self) -> NodeIndex {
        self.input_node_index
    }

    pub fn sink_node(&self) -> NodeIndex {
        self.output_node_index
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

    pub fn add_node(&mut self, node : DspNode) -> NodeIndex {
        let nb_inputs = node.node_processor.nb_inputs();
        let nb_outputs = node.node_processor.nb_outputs();
        if nb_inputs > self.input_edges.len() {self.input_edges.resize(nb_inputs, DspEdge::new(1, 1, self.size));}
        if nb_outputs > self.output_edges.len() {self.output_edges.resize(nb_outputs, DspEdge::new(1, 1, self.size));}
        self.graph.add_node(node)
    }


    pub fn add_input(&mut self, src : DspNode, src_port: u32, dst : NodeIndex, dst_port: u32) -> NodeIndex {
        {
            assert!(src_port <= src.node_processor.nb_outputs() as u32 && src_port >= 1);
            let dst_node = self.graph.node_weight(dst).unwrap();
            assert!(dst_port <= dst_node.node_processor.nb_inputs() as u32 && dst_port >= 1);
        }
        let parent = self.graph.add_node(src);
        self.graph.add_edge(parent, dst, DspEdge::new(src_port, dst_port, self.size));
        parent
    }

    pub fn add_output(&mut self, src : NodeIndex, src_port: u32, dst : DspNode, dst_port: u32) -> NodeIndex {
        {
            let src_node = self.graph.node_weight(src).unwrap();
            assert!(src_port <= src_node.node_processor.nb_outputs() as u32 && src_port >= 1);
            assert!(dst_port <= dst.node_processor.nb_inputs() as u32 && dst_port >= 1);
        }
        let child = self.graph.add_node(dst);
        self.graph.add_edge(src, child, DspEdge::new(src_port, dst_port, self.size));
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

    pub fn add_connection(&mut self, src: NodeIndex, src_port: u32, dst : NodeIndex, dst_port: u32) -> EdgeIndex {
        self.graph.add_edge(src, dst, DspEdge::new(src_port, dst_port, self.size))
    }

    pub fn outputs(& self, src : NodeIndex) -> Edges<DspEdge, Directed> {
        self.graph.edges_directed(src, EdgeDirection::Outgoing)
    }

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

    /// Remove nodes not connected to the sink from the schedule
    fn active_component(&mut self) {
        let rev_graph = Reversed(&self.graph);
        let dfs = Dfs::new(&rev_graph, self.output_node_index);

        println!("{:?}", dfs.discovered.is_visited(&self.input_node_index));

        let mut filtered_schedule = Vec::with_capacity(self.schedule.len());
        for node in self.schedule.iter() {
            if dfs.discovered.is_visited(node) {
                filtered_schedule.push(*node);
            }
        }
        self.schedule = filtered_schedule;

        //self.schedule = self.schedule.iter().filter(|v| {dfs.discovered.is_visited(v)}).collect::<Vec<_>>();
    }

    pub fn update_schedule(&mut self) -> Result<(), AudioGraphError> {
        self.schedule = toposort(&self.graph, None)?;//If Cycle, returns an AudioGraphError::Cycle

        self.active_component();

        if self.schedule.len() <= 100
        {
            print!("The schedule is: ", );
            for node_index in self.schedule.iter() {
                let node = self.graph.node_weight(*node_index).unwrap();
                print!("{} -> ", node);
            }
        }

        Ok(())
    }

    pub fn nb_active_nodes(&self) -> usize {
        self.schedule.len()
    }

}

impl fmt::Display for AudioGraph {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        /*let config = vec![Config::EdgeNoLabel];
        let dot_fmt = Dot::with_config(&self.graph, &config);*/
        let dot_fmt = Dot::new(&self.graph);
        write!(f, "Default size: {}\n", self.size)?;
        write!(f, "Channels: {}\n", self.channels)?;
        dot_fmt.fmt(f)
    }
}

impl AudioEffect for AudioGraph {
    fn process(& mut self, inputs: &[DspEdge], outputs: &mut [DspEdge], samplerate : u32) {
        let interlaced_size = (self.channels * self.frames_per_buffer) as usize;
        let input_buffer = &inputs[0].buffer();
        assert!(input_buffer.len()  == interlaced_size);
        let output_buffer = &mut outputs[0].buffer_mut();
        assert!(output_buffer.len()  == interlaced_size );

        // To prevent
        if self.has_source {
            //Prepare input
            self.input_edges[0].resize(interlaced_size);
            self.input_edges[0].buffer_mut().copy_from_slice(input_buffer);
            //Process
            self.graph.node_weight_mut(self.input_node_index).unwrap().node_processor.process(&self.input_edges[0..1], &mut self.output_edges[0..self.channels as usize], samplerate);
            //Prepare outputs
            //Prepare Outputs
            //Quite inefficient, with allocating. Rather use a fixed vec with max number of inputs and outputs and a buffer pool
            let mut edges = self.outputs_mut(self.input_node_index);
            let mut i = 0;
            while let Some(edge) = edges.next_edge(&self.graph) {
                self.graph.edge_weight_mut(edge).unwrap().buffer_mut().copy_from_slice(self.output_edges[i].buffer());
                i += 1;
            }
        }
        self.input_edges[0].resize(self.size);

        //We assume that sink is the last node in the schedule and execute it separately
        for node in self.schedule[0..self.schedule.len() - 1].iter() {

            let (nb_inputs, nb_outputs)  = {
                let n = &self.graph.node_weight(*node).unwrap().node_processor;
                (n.nb_inputs(), n.nb_outputs())
            };
            //Prepare inputs
            //Quite inefficient, with allocating. Rather use a fixed vec with max number of inputs and outputs and a buffer pool
            // Or just use &[&DspEdge]??
            let mut edges = self.inputs_mut(*node);
            let mut i = 0;
            while let Some(edge) = edges.next_edge(&self.graph) {
                self.input_edges[i].buffer_mut().copy_from_slice(self.graph.edge_weight(edge).unwrap().buffer());
                i += 1;
            }

            //Process
            self.graph.node_weight_mut(*node).unwrap().node_processor.process(&self.input_edges[0..nb_inputs], &mut self.output_edges[0..nb_outputs], samplerate);

            //Prepare Outputs
            //That's also quite inefficient!!
            let mut edges = self.outputs_mut(*node);
            let mut i = 0;
            while let Some(edge) = edges.next_edge(&self.graph) {
                self.graph.edge_weight_mut(edge).unwrap().buffer_mut().copy_from_slice(self.output_edges[i].buffer());
                i += 1;
            }
        }

        // Sink
        //Prepare inputs
        //Quite inefficient, with allocating. Rather use a fixed vec with max number of inputs and outputs and a buffer pool
        // Or just use &[&DspEdge]??
        let mut edges = self.inputs_mut(self.output_node_index);
        let mut i = 0;
        while let Some(edge) = edges.next_edge(&self.graph) {
            self.input_edges[i].buffer_mut().copy_from_slice(self.graph.edge_weight(edge).unwrap().buffer());
            i += 1;
        }
        //Output edge needs to be the interlaced_size
        self.output_edges[0].resize(interlaced_size);
        //Process
        self.graph.node_weight_mut(self.output_node_index).unwrap().node_processor.process(&self.input_edges[0..self.channels as usize], &mut self.output_edges[0..1], samplerate);
        //Prepare Output to soundcard
        output_buffer.copy_from_slice(self.output_edges[0].buffer());
        self.output_edges[0].resize(self.size);
    }

    fn nb_inputs(&self) -> usize {
        self.channels as usize
    }
    fn nb_outputs(&self) -> usize {
        self.channels as usize
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

        for sample in outputs[0].buffer_mut().iter_mut() {
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
                    output.buffer_mut().copy_from_slice(inputs[i].buffer());
                }
            }
        }
        else {
            for (i,group) in inputs.chunks(self.stride).enumerate() {
                for input in group {
                    mixer(outputs[i].buffer_mut(), input.buffer());
                }
            }
        }
    }

    fn nb_inputs(&self) -> usize {self.nb_inputs}
    fn nb_outputs(&self) -> usize {self.nb_outputs}
}

#[derive(Debug)]
pub struct Sink {
    nb_channels: usize,
    frames_per_buffer: u32
}

impl fmt::Display for  Sink {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sink({})", self.nb_channels)
    }
}


impl AudioEffect for Sink {
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge], samplerate : u32) {
        debug_assert!(inputs.len() == self.nb_inputs());
        debug_assert!(outputs.len() == self.nb_outputs());

        let sink_buffer = outputs[0].buffer_mut();
        assert!(sink_buffer.len() == self.nb_channels * self.frames_per_buffer as usize);

        //We have to interlace the ouput buffers (one per channel) into the sink buffer (output buffer of the audio API)
        for (i,chunk) in sink_buffer.chunks_mut(self.nb_channels).enumerate() {
            for (j,channel) in chunk.iter_mut().enumerate() {
                *channel = inputs[j].buffer()[i];
            }
        }
    }
    fn nb_inputs(&self) -> usize {self.nb_channels}
    fn nb_outputs(&self) -> usize {1}
}

#[derive(Debug)]
pub struct Source {
    nb_channels: usize,
    frames_per_buffer: u32,
}

impl fmt::Display for  Source {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "source({})", self.nb_channels)
    }

}


impl AudioEffect for Source {
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge], samplerate : u32) {
        debug_assert!(inputs.len() == self.nb_inputs());
        debug_assert!(outputs.len() == self.nb_outputs());

        let source_buffer = inputs[0].buffer();
        assert!(source_buffer.len() == self.nb_channels * self.frames_per_buffer as usize);

        //We have to desinterlace the source buffer (input buffer of the audio API) into the input buffers (one per channel)
        for (i,chunk) in source_buffer.chunks(self.nb_channels).enumerate() {
            for (j,channel) in chunk.iter().enumerate() {
                outputs[j].buffer_mut()[i] = *channel;
            }
        }
    }
    fn nb_inputs(&self) -> usize {1}
    fn nb_outputs(&self) -> usize {self.nb_channels}
}
