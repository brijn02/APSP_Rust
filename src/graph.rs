use num::Num;
use petgraph::{
    graph,
    matrix_graph::{self, MatrixGraph},
    visit::IntoNodeReferences,
    Graph,
};

// Implementation of Graph with Adjacency Matrix (Dense)
pub struct GraphAM<W>
where
    W: Num + Copy,
{
    pub node_count: usize,
    pub adjacency_matrix: Vec<Vec<Option<W>>>,
}

impl<W: Num + Copy> GraphAM<W> {
    pub fn new() -> Self {
        GraphAM {
            node_count: 0,
            adjacency_matrix: Vec::new(),
        }
    }

    // pub fn from(matrix: Vec<Vec<Option<W>>>) -> Self {
    //     let node_count = matrix[0].len();

    //     GraphAM {
    //         node_count,
    //         adjacency_matrix: matrix,
    //     }
    // }

    pub fn with_capacity(node_count: usize) -> Self {
        let empty_vec: Vec<Option<W>> = vec![Option::None; node_count];

        GraphAM {
            node_count,
            adjacency_matrix: vec![empty_vec; node_count],
        }
    }

    // pub fn get(&mut self, from: usize, to: usize) -> &Option<W> {
    //     &self.adjacency_matrix[from][to]
    // }

    // pub fn get_mut(&mut self, from: usize, to: usize) -> &mut Option<W> {
    //     &mut self.adjacency_matrix[from][to]
    // }

    // pub fn set_edge(&mut self, from: usize, to: usize, weight: Option<W>) {
    //     self.adjacency_matrix[from][to] = weight
    // }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        self.adjacency_matrix[from][to] = Some(weight);
    }
}

#[derive(Clone, Copy)]
pub struct Edge<W>
where
    W: Num + Copy,
{
    pub from: usize,
    pub to: usize,
    pub weight: W,
}

// Implementation of Graph with Adjacency List (Sparse)
// pub struct GraphAL<W>
// where
//     W: Num + Copy,
// {
//     pub node_count: usize,
//     pub edges: Vec<Vec<Edge<W>>>,
// }

// impl<W: Num + Copy> GraphAL<W> {
//     pub fn new() -> Self {
//         GraphAL {
//             node_count: 0,
//             edges: Vec::new(),
//         }
//     }

//     pub fn with_capacity(nodes: usize) -> Self {
//         let edges: Vec<Vec<Edge<W>>> = vec![Vec::new(); nodes];
//         GraphAL {
//             node_count: 0,
//             edges,
//         }
//     }

//     pub fn add_edge(&mut self, from: usize, to: usize, weight: W) {
//         assert!(
//             from < self.node_count && to < self.node_count,
//             "node indices exceed graph capacity!"
//         );
//         self.edges[from].push(Edge { from, to, weight });
//     }

//     pub fn get_neighbors(&mut self, node: usize) -> &[Edge<W>] {
//         &self.edges[node]
//     }

//     pub fn get_neighbors_mut(&mut self, node: usize) -> &mut [Edge<W>] {
//         &mut self.edges[node]
//     }
// }

pub trait MutByNodeId<W, I> {
    fn add_edge_by_ids(&mut self, from: usize, to: usize, weight: W);
    fn get_node_by_id(&self, node: usize) -> Option<I>;
    fn get_node_id(&self, node: I) -> Option<usize>;
}

impl<W: Num> MutByNodeId<W, matrix_graph::NodeIndex> for MatrixGraph<usize, W> {
    fn add_edge_by_ids(&mut self, from: usize, to: usize, weight: W) {
        let mut from_node: Option<matrix_graph::NodeIndex> = None;
        let mut to_node: Option<matrix_graph::NodeIndex> = None;

        for (node, node_id) in self.node_references() {
            if *node_id == from {
                from_node = Some(node);
            }
            if *node_id == to {
                to_node = Some(node);
            }
        }

        let from_node = from_node.unwrap_or_else(|| self.add_node(from));
        let to_node = to_node.unwrap_or_else(|| self.add_node(to));

        self.add_edge(from_node, to_node, weight);
    }

    fn get_node_by_id(&self, target: usize) -> Option<matrix_graph::NodeIndex> {
        for (node, node_id) in self.node_references() {
            if *node_id == target {
                return Some(node);
            }
        }
        None
    }

    fn get_node_id(&self, target: matrix_graph::NodeIndex) -> Option<usize> {
        for (node, node_id) in self.node_references() {
            if node == target {
                return Some(*node_id);
            }
        }
        None
    }
}

impl<W: Num> MutByNodeId<W, graph::NodeIndex> for Graph<usize, W> {
    fn add_edge_by_ids(&mut self, from: usize, to: usize, weight: W) {
        let mut from_node: Option<graph::NodeIndex> = None;
        let mut to_node: Option<graph::NodeIndex> = None;

        for (node, node_id) in self.node_references() {
            if *node_id == from {
                from_node = Some(node);
            }
            if *node_id == to {
                to_node = Some(node);
            }
        }

        let from_node = from_node.unwrap_or_else(|| self.add_node(from));
        let to_node = to_node.unwrap_or_else(|| self.add_node(to));

        self.add_edge(from_node, to_node, weight);
    }

    fn get_node_by_id(&self, target: usize) -> Option<graph::NodeIndex> {
        for (node, node_id) in self.node_references() {
            if *node_id == target {
                return Some(node);
            }
        }
        None
    }

    fn get_node_id(&self, target: graph::NodeIndex) -> Option<usize> {
        for (node, node_id) in self.node_references() {
            if node == target {
                return Some(*node_id);
            }
        }
        None
    }
}
