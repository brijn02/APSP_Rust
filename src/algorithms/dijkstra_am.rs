// This is our implemenatation of the Dijkstra algorithm
// for the all pair shortest path of a graph. The data is parsed as a adjecency matrix.
// The Dijkstra algorithm is implemented for all nodes
// 30-01-2024

// Import crates
use num::Num;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Eq, Ordering};

// Import crates from our files
use crate::graph::GraphAM;
use crate::base::{APSPAlgorithm, APSPResult};
use crate::loader::FromGraphFile;

// Struct which is used for the BinaryHeap. Since we use traits for our implementation
// it is necessary to parse in a struct like this.
// The struct is adapted from the petgraph crate.
#[derive(Copy, Clone, Debug)]
struct HeapElements<Weight, Node>(Weight, Node);

impl<Weight: PartialOrd, Node> PartialEq for HeapElements<Weight, Node> {
    #[inline]
    fn eq(&self, other: &HeapElements<Weight, Node>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<Weight: PartialOrd, Node> Eq for HeapElements<Weight, Node> {}

impl<Weight: PartialOrd, Node> PartialOrd for HeapElements<Weight, Node> {
    #[inline]
    fn partial_cmp(&self, other: &HeapElements<Weight, Node>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Weight: PartialOrd, Node> Ord for HeapElements<Weight, Node> {
    #[inline]
    fn cmp(&self, other: &HeapElements<Weight, Node>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}




/// Dijkstra Algorithm
pub fn dijkstra<W: Num + Copy + PartialOrd + Debug + Default>(
    graph: &GraphAM<W>,
) -> Vec<Vec<Option<W>>> {
    // Define the distance matrix which is empty and of size (node count x node count)
    let mut distance_matrix: Vec<Vec<Option<W>>> = vec![vec![None; graph.node_count]; graph.node_count];

    // Loop over all nodes
    for start_node in 0..graph.node_count {
        // Initialize distance vector with None values
        let mut distance: Vec<Option<W>> = vec![None; graph.node_count];

        // Set the distance of the start node to 0
        distance[start_node] = Some(W::zero());

        // Priority queue to keep track of nodes and their distances and push first value
        let mut priority_queue = BinaryHeap::new();
        let zero_score = W::zero();
        priority_queue.push(HeapElements(zero_score, start_node));
        
        // Continue running until the priority queue is empty
        while !priority_queue.is_empty() {
            // Grap the next element in the BinaryHeap
            let HeapElements(current_weight , current_node) = priority_queue.pop().unwrap();

            // Check if the weight until this point is more then the distance at the current node.
            // If so continue to the next element in the loop
            if let Some(check) = distance[current_node] {
                if current_weight > check {
                    continue;
                }
            }

            // Loop over all values with weight (not equal to None) and update distance + update priority queue 
            for (neighbour, weight) in graph.adjacency_matrix[current_node].iter().enumerate() {
                if let Some(w) = weight {
                    // Compute the new distance
                    let new_distance = current_weight + *w;

                    // Check if there already exists a distance for this neighbour
                    if let Some(dist) = distance[neighbour] {
                        // If the new distance is smaller update the distance matrix and update the BinaryHeap
                        if dist > new_distance {
                            distance[neighbour] = Some(new_distance);
                            priority_queue.push(HeapElements(new_distance, neighbour))
                        }
                    // If there is no connection yet plug it in and update BinaryHeap
                    } else {
                        distance[neighbour] = Some(new_distance);
                        priority_queue.push(HeapElements(new_distance, neighbour))
                    }
                }
            }
           
            // Copy the distance vector to the distance matric
            distance_matrix[start_node][..graph.node_count].copy_from_slice(&distance[..graph.node_count]);

            
        }

    }
    // Return the distance matrix
    distance_matrix
}

// Struct for Dijkstra algorithm
pub struct Dijkstra<W>
where
    W: Num + Copy + Debug,
{
    // Struct containing the Graph and the Shortest Path
    pub graph: GraphAM<W>,
    pub shortest_paths: Vec<Vec<Option<W>>>,
}

// Implementation to compute a new struct 
impl<W: Num + Copy + PartialOrd + Debug + Default> Dijkstra<W> {
    pub fn new() -> Self {
        Self {
            graph: GraphAM::new(),
            shortest_paths: Vec::new(),
        }
    }
}

// Implementation for the APSPAlgorithm struct
impl<W: Num + Copy + PartialOrd + Debug + Default> APSPAlgorithm<W> for Dijkstra<W> {
    // Load the graph
    fn load_graph(&mut self, file_path: &str, is_sparse_format: bool) {
        let graph: GraphAM<W> = if is_sparse_format {
            GraphAM::from_sparse_file(file_path)
        } else {
            GraphAM::from_dense_file(file_path)
        };

        self.graph = graph;
    }

    // Excecute the algorithm
    fn execute(&mut self) {
        let graph = &self.graph;
        self.shortest_paths = dijkstra(graph);
    }

    // Write the results to a Hashmap to make comparison easy
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

        // Return result
        result
    }
}
