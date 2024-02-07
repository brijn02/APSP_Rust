// This is our implemenatation of the Floyd-Warshall Blocked algorithm
// The implementation uses the advantages of the min max matrix multiplication to be able
// to parallelize the FW algorithm in another program.
// 30-01-2023

// Import crates
use num::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::sync::Arc;

// Import crates from our other files
use crate::base::{APSPAlgorithm, APSPResult};
use crate::fwb_function::*;
use crate::graph::GraphAM;
use crate::loader::FromGraphFile;

// Struct for the Floyd Warshall Blocked algorithm
pub struct FloydWarshallBlock<W>
where
    W: Num + Copy + Debug,
{
    // Define the graph, shortest path and the number of blocks
    pub graph: GraphAM<W>,
    pub shortest_paths: Vec<Vec<Option<W>>>,
    pub num_blocks: usize,
}

// Implement a function to compute an empty struct with the number of blocks assigned
impl<W: Num + Copy + PartialOrd + Debug + Clone> FloydWarshallBlock<W> {
    pub fn new(blocks: usize) -> Self {
        Self {
            graph: GraphAM::new(),
            shortest_paths: Vec::new(),
            num_blocks: blocks,
        }
    }
}

// Implement the struct for the APSPAlgorithm struct
impl<W: Num + Copy + PartialOrd + Debug + Clone> APSPAlgorithm<W> for FloydWarshallBlock<W> {
    // Load the graph from a file
    fn load_graph(&mut self, file_path: &str, is_sparse_format: bool) {
        let graph: GraphAM<W> = if is_sparse_format {
            GraphAM::from_sparse_file(file_path)
        } else {
            GraphAM::from_dense_file(file_path)
        };

        self.graph = graph;
    }

    // Excecute the Floyd warshall blocked function
    fn execute(&mut self) {
        let graph = &self.graph;
        self.shortest_paths = floyd_warshall_blocked(graph, self.num_blocks);
    }

    // Write the result to a Hashmap to be able to compare results
    fn get_result(&mut self) -> APSPResult<W> {
        let graph = &self.graph;
        let shortest_paths = &self.shortest_paths;

        let mut result = APSPResult::new();
        for (i, row) in shortest_paths.iter().enumerate().take(graph.node_count) {
            for (j, &value) in row.iter().enumerate().take(graph.node_count) {
                if let Some(value) = value {
                    result.add(i, j, value);
                }
            }
        }
        // Return the results
        result
    }
}

// A function for the floyd warshall blocked algorithm
fn floyd_warshall_blocked<W: Num + Copy + PartialOrd + Debug>(
    graph: &GraphAM<W>,
    num_blocks: usize,
) -> Vec<Vec<Option<W>>> {
    // Clone the adjacency matrix to the distance matrix
    let mut distance = graph.adjacency_matrix.to_owned();

    // Read the number of nodes and number of nodes per block
    let n = graph.node_count;
    let block_size = n / num_blocks;

    // Put all the values in the main diagional to be zero
    for (i, _) in (0..n).enumerate() {
        distance[i][i] = Some(W::zero());
    }

    // Define the number of blocks
    let blocks = (n as f64 / block_size as f64).ceil() as usize;

    // Loop over all the blocks
    for k in 0..blocks {
        // Slice the matrix and compute FW on the smaller matrix
        let akk = slice_matrix_block(&distance, k, k, block_size);
        let akk = floyd_warshall_in_place1(&akk, block_size);
        let akk_move = Arc::new(&akk);

        // Loop over all the blocks
        for j in 0..blocks {
            // If it is the main diagional then ignore and continue
            if j == k {
                continue;
            }
            // Slice matrix akj
            let akj = slice_matrix_block(&distance, k, j, block_size);

            // Compute the min max matrix multiplication for the akj = akk * akj matrix
            let akj = floyd_warshall_in_place2(&akj, &akk, block_size);

            // Write the results back to the distance matrix
            // write_back_to_distance(&mut distance, &akj, k * b, (k + 1) * b, j * b, (j + 1) * b);
            write_back_to_distance(&mut distance, &akj, k * block_size, j * block_size);
        }

        // Loop over all the blocks
        for i in 0..blocks {
            // If i is on the main diagional then continue to the next i
            if i == k {
                continue;
            }

            // Slice matrix aik
            let aik = slice_matrix_block(&distance, i, k, block_size);

            // Compute aik using the min max matrix multiplication
            let aik = floyd_warshall_in_place3(&aik, &akk_move, block_size);

            // Loop over all the blocks
            for j in 0..blocks {
                // If j == k continue
                if j == k {
                    continue;
                }
                // Slice matrix aij and akj from the distance matrix
                let aij = slice_matrix_block(&distance, i, j, block_size);
                let akj = slice_matrix_immut(&distance, k, j, block_size);
                let aij = floyd_warshall_in_place4(&aij, &aik, &akj, block_size);

                // Compute aij with the min max matrix multiplication and write to the small distance matrix
                write_back_to_distance(&mut distance, &aij, i * block_size, j * block_size);
            }
            // Write aik back to the distance matrix
            write_back_to_distance(&mut distance, &aik, i * block_size, k * block_size);
        }
        // Write akk back to the distance matrix
        write_back_to_distance(&mut distance, &akk, k * block_size, k * block_size);
    }

    // return the distance matrix
    distance
}
