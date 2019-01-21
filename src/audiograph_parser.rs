//! Parse a fileformat describing audiographs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use pest::Parser;

#[derive(Parser)]
#[grammar = "audiograph.pest"]
pub struct AudiographParser;

fn parse_audiograph(audiograph : &str) -> bool {
    let parse_result = AudiographParser::parse(Rule::file, audiograph);
    let res = parse_result.is_ok();
    if res {
        let parse_result = parse_result.unwrap();
        let tokens = parse_result.tokens();
        for token in tokens {
            println!("{:?}", token);
        }
        true
    }
    else {
        false
    }
}

fn parse_audiograph_from_file(filename : &str) -> bool  {
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
        assert!(parse_audiograph_from_file("audiograph_wcet_test.ag"))
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
        assert!(AudiographParser::parse(Rule::fnumber, "3").is_err());
        assert!(AudiographParser::parse(Rule::fnumber, "3.").is_err());
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
