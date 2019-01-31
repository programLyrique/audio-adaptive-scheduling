//! Parse a fileformat describing audiographs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use pest::Parser;
use std::collections::HashMap;
use std::collections::HashSet;

use petgraph::graph::NodeIndex;

use itertools::Itertools;

use pest::error::Error as ParseError;

use audiograph::*;

#[derive(Debug)]
pub struct Node  {
    pub id: String,
    pub nb_inlets: u32,
    pub nb_outlets: u32,
    pub class_name: String,
    pub text: Option<String>,
    pub wcet: Option<f64>,
    pub more: HashMap<String, String>,
    pub volume: f32
}

impl Node {
    pub fn new() -> Node  {
        Node { id : String::new(),
            nb_inlets : 0,
            nb_outlets : 0,
            class_name : String::new(),
            text : None,
            wcet : None,
            volume: 1.,
            more : HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    src_id: String,
    src_port: u32,
    dst_id: String,
    dst_port: u32,
}


#[derive(Parser)]
#[grammar = "audiograph.pest"]
pub struct AudiographParser;

fn parse_audiograph(audiograph : &str) -> Result<AudioGraph, ParseError<Rule>> {
    let audiograph = AudiographParser::parse(Rule::file, audiograph)?.next().unwrap();

    use pest::iterators::*;

    fn parse_node(pair : Pair<Rule>) -> Node {
        let mut inner_rules = pair.into_inner();
        let mut node = Node::new();
        node.id = inner_rules.next().unwrap().as_str().to_string();
        //Attributes
        for attribute in inner_rules {
            let mut attr = attribute.into_inner();
            let id = attr.next().unwrap().as_str();
            let v = attr.next().unwrap().as_str();
            match id {
                "in" => node.nb_inlets = v.parse().unwrap(),
                "out" => node.nb_outlets = v.parse().unwrap(),
                "text" => node.text = Some(v.to_string()),
                "kind" => node.class_name = v.to_string(),
                "wcet" => node.wcet = Some(v.parse().unwrap()),
                "volume" => node.volume = v.parse().unwrap(),
                _ => {node.more.insert(id.to_string(), v.to_string()).unwrap();},
            }
        }
        node
    }

    use std::vec::IntoIter;

    fn parse_edge(pair : Pair<Rule>) -> IntoIter<Edge> {
        let mut inner_rules = pair.into_inner().tuples();
        let (src_id_r, src_port_r) = inner_rules.next().unwrap();
        let mut src_id = src_id_r.as_str().to_string();
        let mut src_port = src_port_r.as_str().parse().unwrap();

        let mut edges = Vec::new();

        for (mut dst_id_r, mut dst_port_r) in inner_rules {
            let dst_id = dst_id_r.as_str().to_string();
            let dst_port = dst_port_r.as_str().parse().unwrap();
            edges.push(Edge {src_id, src_port, dst_id : dst_id.clone(), dst_port});
            src_id = dst_id;
            src_port = dst_port;
        }
        edges.into_iter()
    }

    let (nodes, edges) : (Vec<_>, Vec<_>)= audiograph.into_inner().filter(|ref r| r.as_rule() != Rule::deadline).partition(|ref r| r.as_rule() == Rule::node);

    let nodes = nodes.into_iter().map(parse_node).collect::<Vec<_>>();
    let edges = edges.into_iter().flat_map(parse_edge).collect::<Vec<_>>();
    let mut node_indexes : HashMap<String, NodeIndex> = HashMap::new();

    let mut audiograph = AudioGraph::new(64,1);

    for (i,node_infos) in nodes.into_iter().enumerate() {
        let id = node_infos.id.clone();
        let node = DspNode::new(node_infos);
        let node_index = audiograph.add_node(node);
        node_indexes.insert(id, node_index);
    }

    for edge in edges.iter() {
        let src_node = node_indexes[&edge.src_id];
        let dst_node = node_indexes[&edge.dst_id];
        audiograph.add_connection(src_node, edge.src_port, dst_node, edge.dst_port);
    }
    //automatically connect to adc and dac nodes which have inlets and outlets without node on the other side.
    let mut io_edges = Vec::new();
    for node_index in audiograph.graph.node_indices() {
        let node = audiograph.graph.node_weight(node_index).unwrap();
        // Theoretical I/O
        let nb_in_t = node.node_infos().nb_inlets;
        let nb_out_t = node.node_infos().nb_outlets;
        // Actually
        let nb_in_r = audiograph.nb_inputs(node_index);
        let nb_out_r = audiograph.nb_outputs(node_index);

        // Check if some ports are not connected
        if nb_in_t > nb_in_r {
            //Collect connected input ports
            let input_edges = audiograph.inputs(node_index);
            let input_ports = input_edges.map(|e| e.weight().dst_port()).collect::<HashSet<_>>();
            //Connect them to audio source
            for port in 1..nb_in_t {
                if !input_ports.contains(&port) {//It's a non-connected port
                    io_edges.push((audiograph.source_node(), 1, node_index, port));
                }
            }
        }

        if nb_out_t > nb_out_r {
            //Collect connected output ports
            let output_edges = audiograph.outputs(node_index);
            let output_ports = output_edges.map(|e| e.weight().src_port()).collect::<HashSet<_>>();

            //Connect them to audio sink
            for port in 1..nb_out_t {
                if !output_ports.contains(&port) {//It's a non-connected port
                    io_edges.push((node_index, port, audiograph.sink_node(), 1));
                }
            }
        }

    }
    //Finally add the edges
    for edge in io_edges.into_iter() {
        let (src_id, src_port, dst_id, dst_port) = edge;
        audiograph.add_connection(src_id, src_port, dst_id, dst_port);
    }

    Ok(audiograph)
}

fn parse_audiograph_from_file(filename : &str) -> Result<AudioGraph, ParseError<Rule>>  {
    let path = Path::new(filename);
    let mut file = File::open(&path).expect("Impossible to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Impossible to read file.");
    parse_audiograph(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_audiograph_test() {
        let mut file = File::open("audiograph_wcet_test.ag").expect("Impossible to open file.");
        let mut s = String::new();
        file.read_to_string(&mut s).expect("Impossible to read file.");
        assert!(AudiographParser::parse(Rule::file, &s).is_ok())
    }

    #[test]
    fn audiograph_ident() {
        assert!(AudiographParser::parse(Rule::ident, "rte45").is_ok());
        assert!(AudiographParser::parse(Rule::ident, "56rhe").is_err());
    }

    #[test]
    fn audiograph_port_test() {
        assert!(AudiographParser::parse(Rule::port, "45").is_ok());
        assert!(AudiographParser::parse(Rule::port, "per4").is_err());
    }

    #[test]
    fn audiograph_port_ident_test() {
        assert!(AudiographParser::parse(Rule::port_ident, "rte45.3").is_ok());
        assert!(AudiographParser::parse(Rule::port_ident, "rte45 . 3").is_err());
    }

    #[test]
    fn audiograph_edges_test() {
        assert!(AudiographParser::parse(Rule::edges, "ra.1 -> b.2").is_ok());
        assert!(AudiographParser::parse(Rule::edges, "ra.1 -> b.2 -> e.4").is_ok());
        assert!(AudiographParser::parse(Rule::edges, "a.1 -> ").is_err());
    }

    #[test]
    fn audiograph_fnumber_test() {
        assert!(AudiographParser::parse(Rule::fnumber, "45.3").is_ok());
        assert!(AudiographParser::parse(Rule::fnumber, "45.").is_ok());
        assert!(AudiographParser::parse(Rule::fnumber, "3").is_err());
        assert!(AudiographParser::parse(Rule::fnumber, "3 . 3").is_err());
    }

    #[test]
    fn audiograph_inumber_test() {
        assert!(AudiographParser::parse(Rule::inumber, "45").is_ok());
    }

    #[test]
    fn audiograph_string_test() {
        assert!(AudiographParser::parse(Rule::string, "\"Hello, world!\"").is_ok());
        assert!(AudiographParser::parse(Rule::string, "\"Hello, world!").is_err());
        assert!(AudiographParser::parse(Rule::string, "Hello, world!").is_err());
    }

    #[test]
    fn audiograph_attribute_test() {
        assert!(AudiographParser::parse(Rule::attribute, "kind : \"test\",").is_ok());
        assert!(AudiographParser::parse(Rule::attribute, "plop : 3,").is_ok());
        assert!(AudiographParser::parse(Rule::attribute, "plop : 3.5,").is_ok());
        assert!(AudiographParser::parse(Rule::attribute, "plop : ,").is_err());
        assert!(AudiographParser::parse(Rule::attribute, "plop : ").is_err());
        assert!(AudiographParser::parse(Rule::attribute, "plop : 3").is_err());
    }

    #[test]
    fn audiograph_node_test() {
        assert!(AudiographParser::parse(Rule::node, "p = {}").is_ok());
        assert!(AudiographParser::parse(Rule::node, "p = { test : 3, }").is_ok());
        assert!(AudiographParser::parse(Rule::node, "p = { test : 3, plop : 4.5,}").is_ok());
        assert!(AudiographParser::parse(Rule::node, "p = { test : 3 }").is_err());
        assert!(AudiographParser::parse(Rule::node, "= { test : 3, }").is_err());
    }

}
