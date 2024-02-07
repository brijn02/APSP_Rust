// This is our implemenatation of the Floyd-Warshall Blocked parallel algorithm
// The implementation uses the advantages of the min max matrix multiplication to be able
// to parallelize the FW algorithm.
// 30-01-2024

// Import crates
use num::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::marker::{Send, Sync};
use std::sync::Arc;
use std::thread;

// Import crates from our other files
use crate::base::{APSPAlgorithm, APSPResult};
use crate::fwb_function::*;
use crate::graph::GraphAM;
use crate::loader::FromGraphFile;

// Struct for the Floyd Warshall Blocked parallel algorithm
pub struct FloydWarshallBlockPar<W>
where
    W: Num + Copy + Debug + Send + Sync + 'static,
{
    // Define the graph, shortest path and the number of cores
    pub graph: GraphAM<W>,
    pub shortest_paths: Vec<Vec<Option<W>>>,
    pub num_cores: usize,
}

// Implement a function to compute an empty struct with the number of cores assigned
impl<W: Num + Copy + PartialOrd + Debug + Clone + Send + Sync + 'static> FloydWarshallBlockPar<W> {
    pub fn new(cores: usize) -> Self {
        Self {
            graph: GraphAM::new(),
            shortest_paths: Vec::new(),
            num_cores: cores,
        }
    }
}

// Implement the struct for the APSPAlgorithm struct
impl<W: Num + Copy + PartialOrd + Debug + Clone + Send + Sync + 'static> APSPAlgorithm<W>
    for FloydWarshallBlockPar<W>
{
    // Load the graph from a file
    fn load_graph(&mut self, file_path: &str, is_sparse_format: bool) {
        let graph: GraphAM<W> = if is_sparse_format {
            GraphAM::from_sparse_file(file_path)
        } else {
            GraphAM::from_dense_file(file_path)
        };

        self.graph = graph;
    }

    // Excecute the Floyd warshall blocked parallel function
    fn execute(&mut self) {
        let graph = &self.graph;
        self.shortest_paths = floyd_warshall_blocked_par(graph, self.num_cores);
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

// A function for the parallel floyd warshall blocked algorithm
fn floyd_warshall_blocked_par<W: Num + Copy + PartialOrd + Debug + Send + Sync + 'static>(
    graph: &GraphAM<W>,
    num_cores: usize,
) -> Vec<Vec<Option<W>>> {
    // Clone the adjacency matrix to the distance matrix
    let mut distance = graph.adjacency_matrix.to_owned();

    // Read the number of nodes and number of nodes per block
    let n = graph.node_count;
    let block_size = (n as f64 / num_cores as f64).ceil() as usize;
    let og_block_size = (n as f64 / num_cores as f64).ceil() as usize;

    // Put all the values in the main diagional to be zero
    for (i, _) in (0..n).enumerate() {
        distance[i][i] = Some(W::zero());
    }

    let num_cores = (n as f64 / block_size as f64).ceil() as usize;

    // Loop over all the blocks
    for k in 0..num_cores {
        // Slice the matrix and compute FW on the smaller matrix
        let akk = slice_matrix_block(&distance, k, k, block_size);

        let akk = floyd_warshall_in_place1(&akk, block_size);
        let akk_move = Arc::new(akk.clone());

        // Define empty vector for the handles
        let mut handles = vec![];

        // Loop over all the blocks
        for j in 0..num_cores {
            // If it is the main diagional then ignore and continue
            if j == k {
                continue;
            }
            // Slice matrix akj and clone akk
            let akj = slice_matrix_block(&distance, k, j, block_size);

            let akk_move = akk_move.clone();

            // Compute the min max matrix multiplication for the akj = akk * akj matrix
            // on each thread
            let handle =
                thread::spawn(move || (floyd_warshall_in_place2(&akj, &akk_move, block_size), j));
            // Pusht the handle to the handles vector
            handles.push(handle);
        }

        // Return the values from the handles and write back to the original distance matrix
        for handle in handles {
            let (akj, j) = handle.join().expect("Thread Panicked!");
            write_back_to_distance(&mut distance, &akj, k * block_size, j * block_size);
        }

        // Define again an empty handles vector
        let mut handles = vec![];

        let akk_move = Arc::new(akk.clone());

        // Loop over all the blocks
        for i in 0..num_cores {
            // If i is on the main diagional then continue to the next i
            if i == k {
                continue;
            }
            // Slice matrix ai and ak and clone akk
            let ai = slice_matrix(&distance, i * block_size, (i + 1) * block_size, 0, n);
            let ak = slice_matrix(&distance, k * block_size, (k + 1) * block_size, 0, n);

            let akk_move: Arc<Vec<Vec<Option<W>>>> = akk_move.clone();

            // spawn a new thread which returns a small distance matrix
            let handle = thread::spawn(move || {
                let b_rows = if i == num_cores - 1 && n % og_block_size != 0 {
                    n % og_block_size
                } else {
                    block_size
                };

                // Define empty distance matrix of size (n x b)
                let mut distance_matrix: Vec<Vec<Option<W>>> = vec![vec![None; n]; b_rows];
                // slice matrix aik from slice ai
                let aik = slice_matrix_block(&ai, 0, k, block_size);

                // Compute aik using the min max matrix multiplication
                let aik = floyd_warshall_in_place3(&aik, &akk_move, block_size);

                // Loop over all the blocks
                for j in 0..num_cores {
                    // If j == k write the result to our small distance matrix and continue to the next value
                    if j == k {
                        write_back_to_distance(&mut distance_matrix, &aik, 0, j * block_size);
                        continue;
                    }
                    // Slice matrix aij and akj from slices ai and ak
                    let aij = slice_matrix_block(&ai, 0, j, block_size);
                    let akj = slice_matrix_immut(&ak, 0, j, block_size);

                    // Compute aij with the min max matrix multiplication and write to the small distance matrix
                    let aij = floyd_warshall_in_place4(&aij, &aik, &akj, block_size);
                    write_back_to_distance(&mut distance_matrix, &aij, 0, j * block_size)
                }
                // Return the distance matrix and the current index i
                (distance_matrix, i)
            });

            // Push the handle to the handles vector
            handles.push(handle)
        }

        // Loop over all handles to return the values and write to our distance matrix
        for handle in handles {
            let (slice, i) = handle.join().expect("Thread panicked!");

            write_back_to_distance(&mut distance, &slice, i * block_size, 0)
        }

        // write akk to the distance matrix
        write_back_to_distance(&mut distance, &akk, k * block_size, k * block_size);
    }

    // Return the distance matrix
    distance
}
