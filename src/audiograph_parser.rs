//! Parse a fileformat describing audiographs

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_audiograph_test() {
        assert!(parse_audiograph(" sdf4 "))
    }


    #[test]
    fn audiograph_ident() {
        assert!(AudiographParser::parse(Rule::ident, "rte45").is_ok());
        assert!(AudiographParser::parse(Rule::ident, "56rhe").is_err());
    }
}
