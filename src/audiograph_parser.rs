//! Parse a fileformat describing audiographs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use pest::Parser;
use std::collections::HashMap;

use itertools::Itertools;

use pest::error::Error as ParseError;

use audiograph::*;

#[derive(Debug)]
pub struct Node  {
    id: String,
    nb_inlets: u32,
    nb_outlets: u32,
    class_name: String,
    text: Option<String>,
    wcet: Option<f64>,
    more: HashMap<String, String>
}

impl Node {
    fn new() -> Node  {
        Node { id : String::new(),
            nb_inlets : 0,
            nb_outlets : 0,
            class_name : String::new(),
            text : None,
            wcet : None,
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

fn parse_audiograph(audiograph : &str) -> Result<AudioGraph<DspNode>, ParseError<Rule>> {
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

    Ok(AudioGraph::new(64,1))
}

fn parse_audiograph_from_file(filename : &str) -> Result<AudioGraph<DspNode>, ParseError<Rule>>  {
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
