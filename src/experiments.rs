//! To generate random acyclic directed graphs
//! with random effects and test the adaptive scheduling algorithm on it

use rand::{Rng,SeedableRng, StdRng};
use std::hash::Hash;

use effect::*;

pub trait GraphGenerator<T : Copy + AudioEffect + Eq> {
    //Generate several audio nodes
    //Give a function that generates an audio node as argument, maybe.
    // Or a vector of possible nodes?
    // Depending on the topology of the graph?
    fn generate(&mut self, node : &Fn() -> T) -> AudioGraph<T>;
}


pub struct RandomGenerator {
    rng : StdRng,
    adjacency_matrix : Vec<Vec<bool>>
}

impl RandomGenerator {
    pub fn new(size : usize) -> RandomGenerator {
        let seed : &[_] = &[1, 21, 37, 4];
        let rng : StdRng = SeedableRng::from_seed(seed);
        RandomGenerator {
            rng : rng,
            adjacency_matrix : vec![Vec::with_capacity(size); size]
        }
    }

    pub fn gen_matrix(&mut self) {
        //generate low triangular adjacency matrix
        for (i,row) in self.adjacency_matrix.iter_mut().enumerate() {
            row.resize(i+1,false);
            for column in row[0..i+1].iter_mut() {
                *column = self.rng.gen()
            }
        }
    }
}

impl<T : Copy + AudioEffect + Hash + Eq> GraphGenerator<T> for RandomGenerator {
    fn generate(&mut self, node : &Fn() -> T) -> AudioGraph<T> {
        //Gen low triangular matrix
        self.gen_matrix();

        //Generate graphfrom that
        let mut graph = AudioGraph::new(64, 1);

        let size = self.adjacency_matrix.len();

        //Add required number of nodes and store their indexes
        // (should be sequential and from 0 or 1 anyway)
        let mut indexes = Vec::with_capacity(size);
        for _ in 0..size {
            indexes.push(graph.add_node(node()));
        };

        for (i,row) in self.adjacency_matrix.iter().enumerate() {
            for (j,node) in row.iter().enumerate() {
                if *node {
                    graph.add_connection(indexes[i], indexes[j]);
                };
            };
        };

        graph
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use effect::*;

    #[test]
    fn test_graph_gen() {
        let size = 10;
        let mut randGen = RandomGenerator::new(size);
        randGen.generate(& || DspNode::Modulator(5., 500, 1.0));

        //Check if it is low triangular indeed
        assert!(randGen.adjacency_matrix.iter().enumerate().all(|(i,row)| row.len() == i + 1))
    }
}
