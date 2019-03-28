//! Audiographs
//! Compared to the ones in effect.rs, these ones are more
//! complex. Multiple inputs and outputs.
//! They are also meant to be used statically: no specific resamplers
//! on connections but a resampler is a node akin to the other ones
use petgraph::{Graph, EdgeDirection, Directed, Direction};
use petgraph::graph::{NodeIndex, EdgeIndex, Edges, WalkNeighbors};
use petgraph::algo::toposort;
use petgraph::dot::{Dot};
use petgraph::visit::{Reversed, Dfs, VisitMap};
use petgraph;

use std::collections::HashSet;

use std::fmt;

use audiograph_parser;
use samplerate;


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
    pub samplerate: u32,
}

impl DspEdge {
    pub fn new(src_port:u32, dst_port:u32, buffer_size:usize, samplerate:u32) -> DspEdge {
        assert!(src_port >=1 && dst_port >= 1);
        DspEdge {src_port, dst_port, buffer: vec![0.;buffer_size], samplerate}
    }

    pub fn src_port(&self) -> u32 {self.src_port}

    pub fn dst_port(&self) -> u32 {self.dst_port}

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
    node_processor : Box<dyn AudioEffect>,
}

impl DspNode {
    pub fn from_parts(node_infos : audiograph_parser::Node, node_processor : Box<dyn AudioEffect>) -> DspNode {
        DspNode {node_infos, node_processor}
    }

    pub fn new(node_infos: audiograph_parser::Node, nb_channels: usize) -> DspNode {
        let node_processor : Box<dyn AudioEffect> = match node_infos.class_name.as_str() {
            "osc" => Box::new(Oscillator::from_node_infos(&node_infos)),
            "mod" => Box::new(Modulator::from_node_infos(&node_infos)),
            "mix" => Box::new(InputsOutputsAdaptor::from_node_infos(&node_infos)),
            "resampler" => Box::new(Resampler::from_node_infos(&node_infos)),
            "source" => Box::new(InputsOutputsAdaptor::new(nb_channels, node_infos.nb_outlets as usize)),
            "sink" => Box::new(InputsOutputsAdaptor::new(node_infos.nb_inlets as usize, nb_channels)),
            _ => {//We replace it by a default effect
                println!("Unkwown node {:?}. Replacing it by a known one.", node_infos);
                if node_infos.nb_inlets == 0 && node_infos.nb_outlets == 1 {
                    Box::new(Oscillator::new(0., 440, 1.))
                }
                else if node_infos.nb_inlets == 1 && node_infos.nb_outlets == 1 {
                    Box::new(Modulator::new(0.,445,1.))
                }
                else if node_infos.nb_outlets == 0 {
                    panic!("Not handled yet!")
                }
                else {
                    Box::new(InputsOutputsAdaptor::from_node_infos(&node_infos))
                }
            }
        };
        DspNode {node_infos, node_processor}
    }

    pub fn node_infos(&self) -> &audiograph_parser::Node {
        &self.node_infos
    }
}

impl fmt::Display for  DspNode {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        self.node_processor.fmt(f)
    }
}


pub trait AudioEffect : fmt::Display {
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut [DspEdge]);

    fn nb_inputs(&self) -> usize;
    fn nb_outputs(&self) -> usize;

    fn check_io_node_infos(&self, node_infos: &audiograph_parser::Node) {
        assert_eq!(node_infos.nb_inlets as usize, self.nb_inputs());
        assert_eq!(node_infos.nb_outlets as usize, self.nb_outputs());
    }
}


impl AudioEffect for Box<AudioEffect>
{
    #[inline]
    fn process(& mut self, inputs: &[DspEdge], outputs: &mut[DspEdge]){
        (**self).process(inputs, outputs);
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
    pub graph : Graph<DspNode,DspEdge>,
    schedule : Vec< NodeIndex<u32> >,
    size : usize,//Default size of a buffer
    channels : u32,//Number of channels,
    frames_per_buffer : u32,
    input_node_index: NodeIndex,
    input_edges: Vec<DspEdge>,
    has_source: bool,//Whether there is a source from the audio soundcard or only synth
    output_node_index: NodeIndex,
    output_edges: Vec<DspEdge>,
    nominal_samplerate: u32,
}

impl AudioGraph {
    pub fn new(frames_per_buffer : u32, channels : u32, samplerate: u32) -> AudioGraph {
        let input_node_infos = audiograph_parser::Node {class_name: "real_source".to_string(), nb_outlets:1, .. Default::default()};
        let output_node_infos = audiograph_parser::Node {class_name: "real_sink".to_string(), nb_inlets:1, .. Default::default()};
        let mut graph = Graph::new();
        let input_node = DspNode::from_parts(input_node_infos, Box::new( Source {nb_channels:channels as usize, frames_per_buffer}));
        let output_node = DspNode::from_parts(output_node_infos, Box::new( Sink {nb_channels:channels as usize, frames_per_buffer }));
        let input_node_index = graph.add_node(input_node);
        let output_node_index = graph.add_node(output_node);
        let size = frames_per_buffer as usize;

        AudioGraph {graph, schedule : Vec::new(),
            size,
            frames_per_buffer,
            channels,
            input_node_index,
            input_edges: vec![DspEdge::new(1, 1, size, samplerate);channels as usize],
            has_source: false,
            output_node_index,
            output_edges : vec![DspEdge::new(1, 1, size, samplerate);channels as usize],
            nominal_samplerate: samplerate,
        }
    }

    pub fn set_nominal_samplerate(&mut self, samplerate: u32) {
        self.nominal_samplerate = samplerate;
        for input_edge in self.input_edges.iter_mut() {
            input_edge.samplerate = samplerate;
        }
        for output_edge in self.output_edges.iter_mut() {
            output_edge.samplerate = samplerate;
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
        let max_nb = std::cmp::max(nb_inputs, nb_outputs);


        if max_nb > self.input_edges.len() {
            //println!("Input: old={} new={}", self.input_edges.len(), max_nb);
            self.input_edges.resize(max_nb, DspEdge::new(1, 1, self.size, self.nominal_samplerate));}
        // Actually not enough because there can be one port but several edges coming from it.
        // So we need to call update_temp_buffers in update_schedule
        if max_nb > self.output_edges.len() {
            //println!("Output: old={} new={}", self.output_edges.len(), max_nb);
            self.output_edges.resize(max_nb, DspEdge::new(1, 1, self.size, self.nominal_samplerate));}
        self.graph.add_node(node)
    }


    pub fn add_input(&mut self, src : DspNode, src_port: u32, dst : NodeIndex, dst_port: u32) -> NodeIndex {
        {
            assert!(src_port <= src.node_processor.nb_outputs() as u32 && src_port >= 1);
            let dst_node = self.graph.node_weight(dst).unwrap();
            assert!(dst_port <= dst_node.node_processor.nb_inputs() as u32 && dst_port >= 1);
        }
        let parent = self.graph.add_node(src);
        self.graph.add_edge(parent, dst, DspEdge::new(src_port, dst_port, self.size, self.nominal_samplerate));
        parent
    }

    pub fn add_output(&mut self, src : NodeIndex, src_port: u32, dst : DspNode, dst_port: u32) -> NodeIndex {
        {
            let src_node = self.graph.node_weight(src).unwrap();
            assert!(src_port <= src_node.node_processor.nb_outputs() as u32 && src_port >= 1);
            assert!(dst_port <= dst.node_processor.nb_inputs() as u32 && dst_port >= 1);
        }
        let child = self.graph.add_node(dst);
        self.graph.add_edge(src, child, DspEdge::new(src_port, dst_port, self.size, self.nominal_samplerate));
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
        self.graph.add_edge(src, dst, DspEdge::new(src_port, dst_port, self.size, self.nominal_samplerate))
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
        let mut dfs = Dfs::new(&rev_graph, self.output_node_index);
        while let Some(_) = dfs.next(&rev_graph) {};//Just traverse the graph to populate dfs.discovered

        //println!("Source is connected? {:?}", dfs.discovered.is_visited(&self.input_node_index));

        let mut filtered_schedule = Vec::with_capacity(self.schedule.len());

        for node in self.schedule.iter() {
            if dfs.discovered.is_visited(node) {
                filtered_schedule.push(*node);
            }
        }
        //println!("Schedule nodes before active components filtering:");
        //self.print_schedule(&self.schedule);
        self.schedule = filtered_schedule;
        //self.schedule = self.schedule.iter().filter(|v| {dfs.discovered.is_visited(v)}).collect::<Vec<_>>();
        let mut source_index = 0;
        for (i,e) in self.schedule.iter().enumerate() {
            if *e == self.input_node_index {
                self.has_source = true;
                source_index = i;
                break;
            }
        }
        if self.has_source {
            self.schedule.remove(source_index);
        }
    }

    /// Adjust buffer sizes and samplerates for nodes between two resamplers
    fn buffer_size_resamplers(&mut self) {
        /*let sources = self.graph.externals(Direction::Incoming).collect::<Vec<_>>();
        let mut dfs = Dfs::empty(&self.graph);

        for source in sources.iter() {
        dfs.move_to(*source);//Change root for dfs but don't erase the visited map!
        while let Some(node) = dfs.next(&self.graph) {*/
        assert_eq!(self.schedule.len(), self.graph.node_count());
        for  &node in self.schedule.iter() {
            let class_name = self.graph[node].node_infos.class_name.clone();

                let ratio : f64 = self.graph[node].node_infos.more.get("ratio").map_or(1.0, |v| v.parse().unwrap());
                //Get min incoming buffer size
                let buf_size = self.inputs(node).map(|edge| edge.weight().buffer().len()).min().unwrap_or(self.default_buffer_size());
                //Get min incoming samplerate
                let samplerate = self.inputs(node).map(|edge| edge.weight().samplerate).min().unwrap_or(self.nominal_samplerate);


                let (new_buf_size, new_samplerate) = if class_name == "resampler" {
                    /*
                    Some incoming edges may have not their buffer size downsampled yet (but at least one has it thanks to the dfs ordering)
                    Some incoming edges could have not had their buffer size with the normal sample size yet if an incoming branch was downsampled (but at least one has it thanks to the dfs ordering).
                    But actually this case does not happen, as if the path is downsampled, then thanks to dfs ordering, all the subsequent edges including the incoming one of the current node, must have been explored.
                    If it had happened, we would have taken max in the case of ratio < 1.0
                    */
                    ((buf_size as f64 * ratio) as usize, (samplerate as f64 * ratio) as u32)
                }
                else {//We are not after a resampler so we propagate the min buffer size of the previous edges.
                    (buf_size, samplerate)
                };

                //println!("Buffer size={}; samplerate={}", new_buf_size, new_samplerate);
                //Modify all outcoming buffer sizes
                let mut output_edges = self.outputs_mut(node);
                while let Some(edge) = output_edges.next_edge(&self.graph) {
                    /*let (_,dst) = self.graph.edge_endpoints(edge).unwrap();
                    if self.graph[dst].node_infos.class_name == "real_sink" {
                        println!("{}:{} -> {}:real_sink: ratio={}; samplerate={}; new_samplerate={}; buf_size={}; new_buf_size={}", self.graph[node].node_infos.id, class_name,
                        self.graph[dst].node_infos.class_name,
                        ratio, samplerate, new_samplerate, buf_size, new_buf_size);
                    }*/
                    self.graph.edge_weight_mut(edge).unwrap().resize(new_buf_size);
                    self.graph.edge_weight_mut(edge).unwrap().samplerate = new_samplerate;
                }
            }

        // Check sink
        self.inputs(self.output_node_index).for_each(|ref edge| assert_eq!(edge.weight().samplerate, self.nominal_samplerate));
    }

    /// Reset all buffer sizes to the default
    fn reset_buffer_sizes(&mut self) {
        let default_size = self.default_buffer_size();
        for edge in self.graph.edge_weights_mut() {
            edge.resize(default_size);
        }
    }

    /// Check that all nodes are isochronous
    fn validate_buffer_sizes(&self) -> bool {
        self.graph.node_indices().all(|node| {
            let mut input_edges = self.graph.edges_directed(node, Direction::Incoming);
            let first = input_edges.next().map(|e| (e.weight().buffer().len(), e.weight().samplerate));
            let res = if let Some((size_edge, samplerate)) = first {
                input_edges.all(|e| e.weight().buffer().len() == size_edge && e.weight().samplerate == samplerate)
            } else {true};
            res && {
                let mut output_edges = self.graph.edges_directed(node, Direction::Outgoing);
                let first = output_edges.next().map(|e| (e.weight().buffer().len(), e.weight().samplerate));
                if let Some((size_edge, samplerate)) = first {
                output_edges.all(|e| e.weight().buffer().len() == size_edge && e.weight().samplerate == samplerate)
                } else {true}
            }
        })
    }

    /// Autoconnect ports without edges to sources and sinks. If only externals, only autoconect virtual sources and sinks(externals)
    pub fn autoconnect(&mut self, only_externals: bool) {
        //automatically connect to adc and dac nodes which have inlets and outlets without node on the other side.
        let mut io_edges = Vec::new();
        for node_index in self.graph.node_indices() {
            let node = self.graph.node_weight(node_index).unwrap();
            let permitted_node = if only_externals {  node.node_infos.class_name == "source" || node.node_infos.class_name == "sink" } else {true};
            if permitted_node && node_index != self.input_node_index && node_index != self.output_node_index {
                // Theoretical I/O
                let nb_in_t = node.node_processor.nb_inputs() as u32;
                let nb_out_t = node.node_processor.nb_outputs() as u32;
                // Actually
                let nb_in_r = self.nb_inputs(node_index);
                let nb_out_r = self.nb_outputs(node_index);
                //println!("In_t={};In_r={}", nb_in_t, nb_in_r);
                //println!("Out_t={};Out_r={}", nb_out_t, nb_out_r);
                // Check if some ports are not connected
                if nb_in_t > nb_in_r {
                    //Collect connected input ports
                    let input_edges = self.inputs(node_index);
                    let input_ports = input_edges.map(|e| e.weight().dst_port()).collect::<HashSet<_>>();
                    //Connect them to audio source
                    for port in 1..(nb_in_t+1) {
                        if !input_ports.contains(&port) {//It's a non-connected port
                            println!("Autoconnect edge from source to {}:{} on port {}", node.node_infos.id, node, port);
                            io_edges.push((self.input_node_index, 1, node_index, port));
                        }
                    }
                }
                if nb_out_t > nb_out_r {
                    //Collect connected output ports
                    let output_edges = self.outputs(node_index);
                    let output_ports = output_edges.map(|e| e.weight().src_port()).collect::<HashSet<_>>();
                    //Connect them to audio sink
                    for port in 1..(nb_out_t+1) {
                        if !output_ports.contains(&port) {//It's a non-connected port
                            println!("Autoconnect edge to sink from {}:{} on port {}", node.node_infos.id, node, port);
                            io_edges.push((node_index, port, self.output_node_index, 1));
                        }
                    }
                }
            }
        }
        //Finally add the edges
        for edge in io_edges.into_iter() {
            let (src_id, src_port, dst_id, dst_port) = edge;
            self.add_connection(src_id, src_port, dst_id, dst_port);
        }

    }

    pub fn print_schedule(&self, schedule: & [NodeIndex]) {
        print!("The schedule is: ", );
        for node_index in schedule.iter() {
            let node = self.graph.node_weight(*node_index).unwrap();
            print!("{} {} ", node, if node.node_infos.class_name == "real_sink" {""} else {"->"});
        }
        println!("");
    }

    /// Adjust interchange buffers: temporary buffers used to copy audio between edges
    fn update_temp_buffers(&mut self) {
        for node in self.graph.node_indices() {
            let nb_inputs = self.nb_inputs(node) as usize;
            let nb_outputs = self.nb_outputs(node) as usize;
            let max_inputs = std::cmp::max(nb_inputs, self.input_edges.len());
            let max_outputs = std::cmp::max(nb_outputs, self.output_edges.len());
            // Several edges entering the same port. Should be rare
            // (kind of an error because we don't do automatic mixing in that case actually). Only happens
            // when there are several virtual sinks (to the real sink node).
            if max_inputs > self.input_edges.len() {
                self.input_edges.resize(max_inputs, DspEdge::new(1, 1, self.size, self.nominal_samplerate));
            }
            // This can often happen, when a port has several edges coming from it.
            if max_outputs > self.output_edges.len() {
                self.output_edges.resize(max_outputs, DspEdge::new(1, 1, self.size, self.nominal_samplerate));
            }
        }
    }

    pub fn update_schedule(&mut self) -> Result<(), AudioGraphError> {
        self.update_temp_buffers();
        self.reset_buffer_sizes();
        self.schedule = toposort(&self.graph, None)?;//If Cycle, returns an AudioGraphError::Cycle
        self.buffer_size_resamplers();//Requires the topological sort
        assert!(self.validate_buffer_sizes());

        self.active_component();

        if self.schedule.len() <= 100
        {
            self.print_schedule(&self.schedule);
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
    fn process(& mut self, inputs: &[DspEdge], outputs: &mut [DspEdge]) {
        let interlaced_size = (self.channels * self.frames_per_buffer) as usize;
        let input_buffer = &inputs[0].buffer();
        assert_eq!(input_buffer.len(), interlaced_size);
        let output_buffer = &mut outputs[0].buffer_mut();
        assert_eq!(output_buffer.len(), interlaced_size );

        // To prevent
        if self.has_source {
            //println!("Executing {}", self.graph.node_weight(self.input_node_index).unwrap().node_processor);
            //Prepare input
            self.input_edges[0].resize(interlaced_size);
            self.input_edges[0].buffer_mut().copy_from_slice(input_buffer);
            //Process
            self.graph.node_weight_mut(self.input_node_index).unwrap().node_processor.process(&self.input_edges[0..1], &mut self.output_edges[0..self.channels as usize]);
            //Prepare Outputs
            //We could decrease memory usage by using a buffer pool
            let mut edges = self.outputs_mut(self.input_node_index);
            let mut i = 0;
            while let Some(edge) = edges.next_edge(&self.graph) {
                self.output_edges[i].resize(self.graph.edge_weight(edge).unwrap().buffer().len());
                self.output_edges[i].samplerate = self.graph.edge_weight(edge).unwrap().samplerate;
                debug_assert_eq!(self.graph.edge_weight(edge).unwrap().buffer().len(), self.output_edges[i].buffer().len());
                self.graph.edge_weight_mut(edge).unwrap().buffer_mut().copy_from_slice(self.output_edges[i].buffer());
                i += 1;
            }
        }
        self.input_edges[0].resize(self.size);

        //We assume that sink is the last node in the schedule and execute it separately
        for node in self.schedule[0..self.schedule.len() - 1].iter() {
            //println!("Executing {}", self.graph.node_weight(*node).unwrap().node_processor);

            let (nb_inputs, nb_outputs)  = {
                let n = &self.graph.node_weight(*node).unwrap().node_processor;
                (n.nb_inputs(), n.nb_outputs())
            };

            //Fix input_edges and output_edges buffer sizes and samplerates.
            let mut inputs = self.inputs_mut(*node);
            let mut i = 0;
            while let Some(edge) = inputs.next_edge(&self.graph) {
                self.input_edges[i].resize(self.graph.edge_weight(edge).unwrap().buffer().len());
                self.input_edges[i].samplerate = self.graph.edge_weight(edge).unwrap().samplerate;
                i += 1;
            }
            //If there are ports which are not connected, we still have to adapt their size and samplerate.
            // If i= 0, all non connected ports
            let buf_size = if i > 0 {self.input_edges[i-1].buffer().len()} else {self.frames_per_buffer as usize};
            let samplerate = if i > 0 {self.input_edges[i-1].samplerate} else {self.nominal_samplerate};
            for j in i..nb_inputs {
                self.input_edges[j].resize(buf_size);
                self.input_edges[j].samplerate = samplerate;
            }


            let mut outputs = self.outputs_mut(*node);
            let mut i = 0;
            while let Some(edge) = outputs.next_edge(&self.graph) {
                self.output_edges[i].resize(self.graph.edge_weight(edge).unwrap().buffer().len());
                self.output_edges[i].samplerate = self.graph.edge_weight(edge).unwrap().samplerate;
                i += 1;
            }
            //For the remaining edges, which are actually
            let buf_size = if i > 0 {self.output_edges[i-1].buffer().len()} else {self.frames_per_buffer as usize};
            let samplerate = if i > 0 {self.output_edges[i-1].samplerate} else {self.nominal_samplerate};
            for j in i..nb_outputs {
                self.output_edges[i].resize(buf_size);
                self.output_edges[i].samplerate = samplerate;
            }

            //Prepare inputs
            // Or just use &[&DspEdge]??
            //TODO: not fill sequentially but fill using port numbers!! (not equivalent if there are non connected ports)
            let mut edges = self.inputs_mut(*node);
            let mut i = 0;
            while let Some(edge) = edges.next_edge(&self.graph) {
                debug_assert_eq!(self.graph.edge_weight(edge).unwrap().buffer().len(), self.input_edges[i].buffer().len());
                self.input_edges[i].buffer_mut().copy_from_slice(self.graph.edge_weight(edge).unwrap().buffer());
                i += 1;
            }

            //Process
            self.graph.node_weight_mut(*node).unwrap().node_processor.process(&self.input_edges[0..nb_inputs], &mut self.output_edges[0..nb_outputs]);

            //Prepare Outputs
            //That's also quite inefficient!!
            let mut edges = self.outputs_mut(*node);
            let mut i = 0;
            while let Some(edge) = edges.next_edge(&self.graph) {
                debug_assert_eq!(self.graph.edge_weight(edge).unwrap().buffer().len(), self.output_edges[i].buffer().len());
                //The size of a DspEdge is the right one (computed at scheduling)
                self.graph.edge_weight_mut(edge).unwrap().buffer_mut().copy_from_slice(self.output_edges[i].buffer());
                i += 1;
            }
        }

        // Sink
        //println!("Executing {}", self.graph.node_weight(self.output_node_index).unwrap().node_processor);
        //Prepare inputs

        //Quite inefficient, with allocating. Rather use a fixed vec with max number of inputs and outputs and a buffer pool
        // Or just use &[&DspEdge]??
        let mut edges = self.inputs_mut(self.output_node_index);
        debug_assert_eq!(self.graph.node_weight(self.output_node_index).unwrap().node_infos.class_name.as_str(), "real_sink");
        let mut i = 0;
        while let Some(edge) = edges.next_edge(&self.graph) {
            self.input_edges[i].resize(self.graph.edge_weight(edge).unwrap().buffer().len());
            self.input_edges[i].samplerate = self.graph.edge_weight(edge).unwrap().samplerate;
            //println!("Input edge of sink: {}", i);
            debug_assert_eq!(self.input_edges[i].samplerate , self.nominal_samplerate);
            debug_assert_eq!(self.graph.edge_weight(edge).unwrap().buffer().len(), self.input_edges[i].buffer().len());
            self.input_edges[i].buffer_mut().copy_from_slice(self.graph.edge_weight(edge).unwrap().buffer());
            i += 1;
        }
        //Output edge needs to be the interlaced_size
        self.output_edges[0].resize(interlaced_size);
        //Process
        self.graph.node_weight_mut(self.output_node_index).unwrap().node_processor.process(&self.input_edges[0..self.channels as usize], &mut self.output_edges[0..1]);
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
pub struct Oscillator {
    phase: f32,
    frequency: u32,
    volume: f32
}

impl Oscillator {
    pub fn new(initial_phase: f32, frequency: u32, volume: f32) -> Oscillator {
        Oscillator {phase: initial_phase, frequency, volume}
    }

    pub fn from_node_infos(node_infos: &audiograph_parser::Node) -> Oscillator {
        let osc = Oscillator::new(0., node_infos.more["freq"].parse().expect("freq must be an integer"), node_infos.volume);
        osc.check_io_node_infos(node_infos);
        osc
    }
}

impl fmt::Display for  Oscillator {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "osc({}, {}, {})", self.phase, self.frequency, self.volume)
    }
}

impl AudioEffect for Oscillator {
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());

        let samplerate = outputs[0].samplerate;

        for sample in outputs[0].buffer_mut().iter_mut() {
            *sample = sine_wave(self.phase, self.volume);
            self.phase += self.frequency as f32 / samplerate as f32;
        }
    }

    fn nb_inputs(&self) -> usize {0}
    fn nb_outputs(&self) -> usize {1}
}

#[derive(Debug)]
pub struct Modulator {
    phase: f32,
    frequency: u32,
    volume: f32
}

impl Modulator {
    pub fn new(initial_phase: f32, frequency: u32, volume: f32) -> Modulator {
        Modulator {phase: initial_phase, frequency, volume}
    }

    pub fn from_node_infos(node_infos: &audiograph_parser::Node) -> Modulator {
        let modu = Modulator::new(0., node_infos.more["freq"].parse().expect("freq must be an integer"), node_infos.volume);
        modu.check_io_node_infos(node_infos);
        modu
    }
}

impl fmt::Display for  Modulator {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mod({}, {}, {})", self.phase, self.frequency, self.volume)
    }
}

impl AudioEffect for Modulator {
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());
        debug_assert_eq!(outputs[0].buffer().len(), inputs[0].buffer().len());

        debug_assert!(inputs[0].samplerate == outputs[0].samplerate);
        let samplerate = inputs[0].samplerate;

        for (sample_out, sample_in) in outputs[0].buffer_mut().iter_mut().zip(inputs[0].buffer().iter()) {
            *sample_out = *sample_in * sine_wave(self.phase, self.volume);
            self.phase += self.frequency as f32 / samplerate as f32;
        }
    }

    fn nb_inputs(&self) -> usize {1}
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
        //assert!(nb_inputs % nb_outputs == 0 || nb_outputs % nb_inputs == 0 );//We don't need that anymore
        let stride = if nb_outputs > nb_inputs {nb_outputs / nb_inputs} else {nb_inputs / nb_outputs};
        InputsOutputsAdaptor {nb_inputs, nb_outputs, stride}
    }

    pub fn from_node_infos(node_infos: &audiograph_parser::Node) -> InputsOutputsAdaptor {
        let io_adapt = InputsOutputsAdaptor::new(node_infos.nb_inlets as usize, node_infos.nb_outlets as usize);
        io_adapt.check_io_node_infos(node_infos);
        io_adapt
    }
}

impl fmt::Display for  InputsOutputsAdaptor {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "adaptor({}, {}, {})", self.nb_inputs, self.nb_outputs, self.stride)
    }
}

fn mixer(buffer: &mut [f32], input_buffer: & [f32]) {
    for (s1,s2) in buffer.iter_mut().zip(input_buffer) {
        *s1 += *s2
    }
}

impl AudioEffect for InputsOutputsAdaptor {
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());
        //println!("{}", self);
        //Actually, not a problem if it's not the case. The last inputs/outputs will just be the same number
        //debug_assert!(self.nb_inputs % self.nb_outputs == 0 || self.nb_outputs % self.nb_inputs == 0 );
        debug_assert_eq!(inputs[0].samplerate, outputs[0].samplerate);
        debug_assert!(inputs.iter().all(|ref x| x.samplerate == inputs[0].samplerate));
        debug_assert!(outputs.iter().all(|ref x| x.samplerate == outputs[0].samplerate));

        if self.nb_outputs > self.nb_inputs {
            let iter = outputs.chunks_exact_mut(self.stride);
            for (i,group) in iter.enumerate() {
                // Copy the last input in the remaining outputs
                let index = std::cmp::min(i, inputs.len() - 1);
                for output in group.iter_mut() {
                    debug_assert_eq!(output.buffer().len(), inputs[index].buffer().len());
                    output.buffer_mut().copy_from_slice(inputs[index].buffer());
                }
            }
        }
        else {

            for (i,group) in inputs.chunks(self.stride).enumerate() {
                //To handle the last chunk which will be mixed in the last output with the previous chunk
                let index = std::cmp::min(i, outputs.len() - 1);
                for input in group {
                    debug_assert_eq!(outputs[index].buffer().len(), input.buffer().len());
                    mixer(outputs[index].buffer_mut(), input.buffer());
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
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());

        let sink_buffer = outputs[0].buffer_mut();
        assert_eq!(sink_buffer.len(), self.nb_channels * self.frames_per_buffer as usize);
        for input in inputs.iter() {
            debug_assert_eq!(input.buffer().len(), self.frames_per_buffer as usize);
        }

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
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge]) {
        debug_assert!(inputs.len() == self.nb_inputs());
        debug_assert!(outputs.len() == self.nb_outputs());

        let source_buffer = inputs[0].buffer();
        assert_eq!(source_buffer.len(), self.nb_channels * self.frames_per_buffer as usize);

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


#[derive(Debug)]
pub struct Resampler {
    resampler: samplerate::Resampler,
}

impl Resampler {
    pub fn new(converter_type : samplerate::ConverterType, src_ratio: f64) -> Resampler {
        Resampler { resampler: samplerate::Resampler::new(converter_type, 1, src_ratio)}
    }

    pub fn from_node_infos(node_infos : &audiograph_parser::Node) -> Resampler {
        let converter_type = node_infos.more.get("conv")
            .map_or(samplerate::ConverterType::Linear,
                |s|  match s.as_str() {
                "sinc_best" => samplerate::ConverterType::SincBestQuality,
                "sinc_medium" => samplerate::ConverterType::SincMediumQuality,
                "sinc_fastest" => samplerate::ConverterType::SincFastest,
                "zero_hold" => samplerate::ConverterType::ZeroOrderHold,
                "linear" => samplerate::ConverterType::Linear,
                _ => samplerate::ConverterType::Linear,
            });
        let ratio = node_infos.more.get("ratio").expect(&format!("Resampler {} needs explicit ratio.", node_infos.id)).parse().unwrap();
        let resampler = Resampler::new(converter_type, ratio);
        resampler.check_io_node_infos(node_infos);
        resampler
    }

    pub fn get_ratio(&self) -> f64 {self.resampler.src_ratio}
}

impl fmt::Display for  Resampler {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "resampler({})", self.resampler.src_ratio)
    }
}


impl AudioEffect for Resampler {
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut[DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());
        debug_assert_eq!((inputs[0].samplerate as f64 * self.resampler.src_ratio) as u32, outputs[0].samplerate);

        // Already done in the process method so outputs should already be the right size
        let new_buf_size = (self.resampler.src_ratio * inputs[0].buffer().len() as f64) as usize;
        assert_eq!(outputs[0].buffer().len(), new_buf_size);
        outputs[0].resize(new_buf_size);//No-op as it is already the right size


        self.resampler.resample(inputs[0].buffer(), outputs[0].buffer_mut()).unwrap();

    }
    fn nb_inputs(&self) -> usize {1}
    fn nb_outputs(&self) -> usize {1}
}
