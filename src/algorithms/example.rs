use std::collections::HashMap;

use num::Num;
use petgraph::algo::{floyd_warshall, BoundedMeasure};
use petgraph::{graph, Graph};

use crate::base::{APSPAlgorithm, APSPResult};
use crate::graph::MutByNodeId;
use crate::loader::FromGraphFile;

pub struct BaseLineFloydWarshall<W>
where
    W: Num + Copy + BoundedMeasure, // BoundedMeasure is only required to call `floyd_warshall`
{
    pub graph: Graph<usize, W>,
    pub shortest_paths: HashMap<(graph::NodeIndex, graph::NodeIndex), W>,
}

impl<W: Num + Copy + BoundedMeasure> BaseLineFloydWarshall<W> {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            shortest_paths: HashMap::new(),
        }
    }
}

impl<W: Num + Copy + BoundedMeasure> APSPAlgorithm<W> for BaseLineFloydWarshall<W> {
    fn load_graph(&mut self, file_path: &str, is_sparse_format: bool) {
        let graph: Graph<usize, W> = if is_sparse_format {
            Graph::from_sparse_file(file_path)
        } else {
            Graph::from_dense_file(file_path)
        };

        self.graph = graph;
    }

    fn execute(&mut self) {
        let graph = &self.graph;

        self.shortest_paths = floyd_warshall(&graph, |edge| *edge.weight()).unwrap();
    }

    fn get_result(&mut self) -> APSPResult<W> {
        let graph = &self.graph;
        let mut result = APSPResult::new();
        for ((from, to), cost) in &self.shortest_paths {
            let from = graph.get_node_id(*from).unwrap();
            let to = graph.get_node_id(*to).unwrap();

            result.add(from, to, *cost)
        }

        result
    }
}
