//! To generate random acyclic directed graphs
//! with random effects and test the adaptive scheduling algorithm on it

use petgraph::{Graph, EdgeDirection};
use petgraph::graph::{NodeIndex, EdgeIndex, Edges, WalkNeighbors};
use rand::{Rng,SeedableRng, StdRng};

pub trait GraphGenerator {
    //Rather generate audio effects...
    fn generate(&mut self) -> Graph<bool, u32>;
}


pub struct RandomGenerator {
    rng : StdRng,
    adjacency_matrix : Vec<Vec<bool>>
}

impl RandomGenerator {
    pub fn new(size : usize) -> RandomGenerator {
        let seed : &[_] = &[1, 21, 37, 4];
        let mut rng : StdRng = SeedableRng::from_seed(seed);
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

impl GraphGenerator for RandomGenerator {
    fn generate(&mut self) -> Graph<bool, u32> {
        //Gen low triangular matrix
        self.gen_matrix();

        //Generate graphfrom that
        let mut graph = Graph::new();

        let size = self.adjacency_matrix.len();

        //Add required number of nodes and store their indexes
        // (should be sequential and from 0 or 1 anyway)
        let mut indexes = Vec::with_capacity(size);
        for _ in 0..size {
            indexes.push(graph.add_node(true));
        };

        for (i,row) in self.adjacency_matrix.iter().enumerate() {
            for (j,node) in row.iter().enumerate() {
                graph.add_edge(indexes[i], indexes[j], 0);
            };
        };

        graph
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_graph_gen() {
        let size = 10;
        let mut randGen = RandomGenerator::new(size);
        randGen.generate();

        //Check if it is low triangular indeed
        assert!(randGen.adjacency_matrix.iter().enumerate().all(|(i,row)| row.len() == i + 1))
    }
}
