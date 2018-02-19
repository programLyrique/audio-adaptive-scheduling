//! To generate random acyclic directed graphs
//! with random effects and test the adaptive scheduling algorithm on it

use rand::{Rng,SeedableRng, StdRng};
use std::hash::Hash;

use effect::*;

use std::fmt;

#[derive(Copy, Clone)]
pub enum NodeClass {
    Input,
    Transformer,
    Output,
}

pub trait GraphGenerator<T : Copy + fmt::Display + AudioEffect + Eq > {
    //Generate several audio nodes
    //Give a function that generates an audio node as argument, maybe.
    // Or a vector of possible nodes?
    // Depending on the topology of the graph?
    fn generate(&mut self, node : &Fn(NodeClass, &mut StdRng) -> T) -> AudioGraph<T>;
    //fn generate(&mut self) -> AudioGraph<DspNode>;
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

impl<T : fmt::Display+ AudioEffect + Copy + Hash + Eq> GraphGenerator<T> for RandomGenerator {
    fn generate(&mut self, node : &Fn(NodeClass, &mut StdRng) -> T) -> AudioGraph<T> {
        //Gen low triangular matrix
        self.gen_matrix();

        let size = self.adjacency_matrix.len();

        //Fin input and outputs
        let mut children_cnt = vec![0;size];
        let mut parents_cnt = vec![0;size];
        //Inputs have 0 parents, outputs have 0 children.

        for (i, row) in self.adjacency_matrix.iter().enumerate() {
            for (j,node) in row.iter().enumerate() {
                if *node {
                    children_cnt[i] += 1;
                    parents_cnt[j] += 1;
                }
            }
        }

        //Generate graph from that
        let mut graph = AudioGraph::new(64, 1);

        //Add required number of nodes and store their indexes
        // (should be sequential and from 0 or 1 anyway)
        let mut indexes = Vec::with_capacity(size);
        for i in 0..size {
            //If it is an input
            if parents_cnt[i] == 0 {
                //Insert a generator of sound
                let new_node = node(NodeClass::Input, &mut self.rng);
                indexes.push(graph.add_node(new_node));
            }
            else {
                let new_node = node(NodeClass::Transformer, &mut self.rng);
                indexes.push(graph.add_node(new_node));
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
    use rand::{thread_rng, Rng};

    #[test]
    fn test_graph_gen() {
        let size = 10;

        let generators = vec![DspNode::Modulator(5., 500, 1.0), DspNode::LowPass([5.,6.,7.,8.],200.,0.8)];
        let mut rand_gen = RandomGenerator::new(size);
        {
            let graph = rand_gen.generate(& |c, rng|
                {
                    match c  {
                        NodeClass::Input => DspNode::Oscillator(6., 500, 1.0),
                        NodeClass::Transformer | NodeClass::Output => *rng.choose(&generators).unwrap()
                    }
                }
                );

            println!("{}", graph);
        }

        //Check if it is low triangular indeed
        assert!(rand_gen.adjacency_matrix.iter().enumerate().all(|(i,row)| row.len() == i))
    }
}
