// This is our implemenatation of the Floyd-Warshall algorithm
// The implementation is straight forward and uses three for-loops to compute
// the all pair shortest path of a graph parsed in as a adjacency matrix
// 29-01-2024

// Import crates
use num::Num;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use crate::graph::GraphAM;
use crate::base::{APSPAlgorithm, APSPResult};
use crate::loader::FromGraphFile;

// FloydWarshall Algorithm
pub fn floyd_warshall<W: Num + Copy + PartialOrd + Debug>(
    graph: &GraphAM<W>,
) -> Vec<Vec<Option<W>>> {
    // Clone the adjacency matrix to a new distance matrix 
    let mut distance = graph.adjacency_matrix.clone();

    // Update the main diagional to all zeros such that the distance to the original point is
    // equal 0
    for (i, _) in (0..graph.node_count).enumerate() {
        distance[i][i] = Some(W::zero());
    }

    // Loop over all the nodes three times
    for k in 0..graph.node_count {
        for i in 0..graph.node_count {
            for j in 0..graph.node_count {
                // Check if there is a value and if so assign to ik or kj
                if let (Some(ik), Some(kj)) = (distance[i][k], distance[k][j]) {
                    // Check if there is an distance available, if yes assign to ij.
                    // otherwise add the sum ik and kj to distance
                    if let Some(ij) = distance[i][j] {
                        // If the sum is smaller then update the distance matrix
                        if ik + kj < ij {
                            distance[i][j] = Some(ik + kj);
                        }
                    } else {
                        distance[i][j] = Some(ik + kj);
                    }
                }
            }
        }
    }
    // Return distance
    distance
}

// Define FloydWarshall struct
pub struct FloydWarshall<W>
where
    W: Num + Copy + Debug,
{
    // Define a graph and shortest_paths
    pub graph: GraphAM<W>,
    pub shortest_paths: Vec<Vec<Option<W>>>,
}

// Implementation for Floywd Warshall struct
impl<W: Num + Copy + PartialOrd + Debug> FloydWarshall<W> {
    // Function to implement an empty struct
    pub fn new() -> Self {
        Self {
            graph: GraphAM::new(),
            shortest_paths: Vec::new(),
        }
    }
}

// Define the APSPAlgorithm for FW
impl<W: Num + Copy + PartialOrd + Debug> APSPAlgorithm<W> for FloydWarshall<W> {
    // Load the graph from a file using the Graph struct
    fn load_graph(&mut self, file_path: &str, is_sparse_format: bool) {
        let graph: GraphAM<W> = if is_sparse_format {
            GraphAM::from_sparse_file(file_path)
        } else {
            GraphAM::from_dense_file(file_path)
        };
        // Return the graph
        self.graph = graph;
    }

    // Function to execute the FW algorithm
    fn execute(&mut self) {
        // input the graph to the FW 
        let graph = &self.graph;
        self.shortest_paths = floyd_warshall(graph);
    }

    // Put the result in a Hashmap to be able to compare with other programs
    fn get_result(&mut self) -> APSPResult<W> {

        // Get the graph and the SP
        let graph = &self.graph;
        let shortest_paths = &self.shortest_paths;

        // Write into APSPResults
        let mut result = APSPResult::new();
        for (i, row) in shortest_paths.iter().enumerate().take(graph.node_count) {
            for (j, &value) in row.iter().enumerate().take(graph.node_count) {
                if let Some(value) = value {
                    result.add(i, j, value);
                }
            }
        }
        

        // Return result
        result
    }
}
