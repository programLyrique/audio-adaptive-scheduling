//! To generate random acyclic directed graphs
//! with random effects and test the adaptive scheduling algorithm on it

use rand::{Rng,SeedableRng, StdRng};
use std::hash::Hash;

use effect::*;

use std::fmt;


pub enum NodeClass {
    Input,
    Transformer,
    Output,
}

pub trait GraphGenerator<T : Copy + fmt::Display + AudioEffect + Eq> {
    //Generate several audio nodes
    //Give a function that generates an audio node as argument, maybe.
    // Or a vector of possible nodes?
    // Depending on the topology of the graph?
    fn generate(&mut self, node : &Fn(NodeClass) -> T) -> AudioGraph<T>;
}


pub struct RandomGenerator {
    rng : StdRng,
    adjacency_matrix : Vec<Vec<bool>>
}

impl fmt::Debug for RandomGenerator {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        self.adjacency_matrix.fmt(f)
    }
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
            row.resize(i,false);
            for column in row[0..i].iter_mut() {
                *column = self.rng.gen()
            }
        }
    }
}



impl<T : Copy + fmt::Display+ AudioEffect + Hash + Eq> GraphGenerator<T> for RandomGenerator {
    fn generate(&mut self, node : &Fn(NodeClass) -> T) -> AudioGraph<T> {
        //Gen low triangular matrix
        self.gen_matrix();

        let size = self.adjacency_matrix.len();

        //Fin input and outputs
        let mut children_cnt = vec![0;size];
        let mut parents_cnt = vec![0 ;size];
        //Inputs have 0 parents, outputs have 0 children.

        for (i, row) in self.adjacency_matrix.iter().enumerate() {
            for (j,node) in row.iter().enumerate() {
                if *node {
                    children_cnt[i] += 1;
                    parents_cnt[j] += 1;
                }
            }
        }

        //Generate graphfrom that
        let mut graph = AudioGraph::new(64, 1);

        //Add required number of nodes and store their indexes
        // (should be sequential and from 0 or 1 anyway)
        let mut indexes = Vec::with_capacity(size);
        for i in 0..size {
            //If it is an input
            if parents_cnt[i] == 0 {
                //Insert a generator of sound
                indexes.push(graph.add_node(node(NodeClass::Input)));
            }
            else {
                indexes.push(graph.add_node(node(NodeClass::Transformer)));
            }

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
        {
            let graph = randGen.generate(& |c|
                match c  {
                    NodeClass::Input => DspNode::Oscillator(5., 500, 1.0),
                    NodeClass::Transformer | NodeClass::Output => DspNode::Modulator(5., 500, 1.0),
                }
                );

            println!("{}", graph);
        }

        //Check if it is low triangular indeed
        assert!(randGen.adjacency_matrix.iter().enumerate().all(|(i,row)| row.len() == i))
    }
}
