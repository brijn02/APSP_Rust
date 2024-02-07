use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// use crate::graph::{GraphAL, GraphAM, MutByNodeId};
use crate::graph::{GraphAM, MutByNodeId};
use num::Num;
use petgraph::graph::Graph;
use petgraph::matrix_graph::MatrixGraph;

pub trait FromGraphFile<W>
where
    W: Num,
{
    fn instantiate_graph(nodes: usize, edges: usize) -> Self;
    fn add_edge(graph: &mut Self, from: usize, to: usize, weight: W);
    fn from_sparse_file(file_path: &str) -> Self;
    fn from_dense_file(file_path: &str) -> Self;
}

fn parse_number<T>(num: &str) -> T
where
    T: Num,
{
    // num.parse::<T>()
    match <T as Num>::from_str_radix(num, 10) {
        Ok(number) => number,
        Err(_) => panic!("Can't parse input"),
    }
}

fn open_file(file_path: &str) -> File {
    let filepath = Path::new(file_path);
    File::open(filepath).expect("Failed to open file")
}

// Read the file from the command line
pub fn load_sparse_graph<W, G>(
    file_path: &str,
    apply_index_shift: bool, // if true, substracts 1 from the node id in the file
    directed: bool,
    instantiate_graph: fn(nodes: usize, edges: usize) -> G,
    add_edge: fn(graph: &mut G, from: usize, to: usize, weight: W),
) -> G
where
    W: Num + Copy,
{
    let file = open_file(file_path);
    let mut lines = BufReader::new(file).lines();

    // Check the first line to get the graph size
    let header = lines.next().expect("Can't find header of file").unwrap();
    let size_line = header
        .split_whitespace()
        .map(parse_number)
        .collect::<Vec<usize>>();

    // let [node_count, edge_count] = &size_line[..];
    let (node_count, edge_count) = match &size_line[..] {
        [node_count, edge_count] => (*node_count, *edge_count),
        _ => panic!("Failed to parse graph header."),
    };

    let mut graph: G = instantiate_graph(node_count, edge_count);

    // Read all the other lines
    for line in lines {
        let line = line.unwrap();
        let read_line: Vec<&str> = line.split_whitespace().collect();

        // Write the values in the line to a store
        let offset = if apply_index_shift { 1 } else { 0 };
        let node_a = parse_number::<usize>(read_line[0]) - offset;
        let node_b = parse_number::<usize>(read_line[1]) - offset;

        let weight = parse_number::<W>(read_line[2]);

        add_edge(&mut graph, node_a, node_b, weight);

        if !directed {
            add_edge(&mut graph, node_b, node_a, weight);
        }
    }

    graph
}

fn load_dense_graph<W, G>(
    file_path: &str,
    apply_index_shift: bool,
    instantiate_graph: fn(nodes: usize, edges: usize) -> G,
    add_edge: fn(graph: &mut G, from: usize, to: usize, weight: W),
) -> G
where
    W: Num + Copy,
{
    let file = open_file(file_path);
    let mut lines = BufReader::new(file).lines();

    let header = lines.next().expect("Can't find header of file").unwrap();
    let size_line = header
        .split_whitespace()
        .map(parse_number)
        .collect::<Vec<usize>>();

    let size = match &size_line[..] {
        [size] => size,
        _ => panic!("Failed to parse graph header."),
    };

    let mut graph = instantiate_graph(*size, size * size);

    // Read the remaining lines to populate the adjacency matrix
    for (from, line) in lines.enumerate() {
        let row: Vec<W> = line
            .expect("Failed to read line")
            .split_whitespace()
            .map(parse_number)
            .collect();

        // Check if the number of columns matches the expected number of stores
        if row.len() != *size {
            panic!("Unexpected number of columns in line {}", from + 1);
        }

        for (to, edge) in row.iter().enumerate() {
            if apply_index_shift {
                add_edge(&mut graph, from - 1, to - 1, *edge);
            } else {
                add_edge(&mut graph, from, to, *edge);
            }
        }
    }

    graph
}

impl<W: Num + Copy> FromGraphFile<W> for GraphAM<W> {
    fn instantiate_graph(nodes: usize, _: usize) -> Self {
        GraphAM::with_capacity(nodes)
    }

    fn add_edge(graph: &mut Self, from: usize, to: usize, weight: W) {
        graph.add_edge(from, to, weight);
    }

    fn from_sparse_file(file_path: &str) -> Self {
        load_sparse_graph(
            file_path,
            true,
            false,
            Self::instantiate_graph,
            FromGraphFile::add_edge,
        )
    }

    fn from_dense_file(file_path: &str) -> Self {
        load_dense_graph(file_path, true, Self::instantiate_graph, Self::add_edge)
    }
}

// impl<W: Num + Copy> FromGraphFile<W> for GraphAL<W> {
//     fn instantiate_graph(nodes: usize, _: usize) -> Self {
//         GraphAL::with_capacity(nodes)
//     }

//     fn add_edge(graph: &mut Self, from: usize, to: usize, weight: W) {
//         graph.add_edge(from, to, weight);
//     }

//     fn from_sparse_file(file_path: &str) -> Self {
//         load_sparse_graph(
//             file_path,
//             true,
//             false,
//             Self::instantiate_graph,
//             FromGraphFile::add_edge,
//         )
//     }

//     fn from_dense_file(file_path: &str) -> Self {
//         load_dense_graph(file_path, true, Self::instantiate_graph, Self::add_edge)
//     }
// }

impl<W: Num + Copy> FromGraphFile<W> for MatrixGraph<usize, W> {
    fn instantiate_graph(nodes: usize, _: usize) -> Self {
        MatrixGraph::with_capacity(nodes)
    }

    fn add_edge(graph: &mut Self, from: usize, to: usize, weight: W) {
        graph.add_edge_by_ids(from, to, weight);
    }

    fn from_sparse_file(file_path: &str) -> Self {
        load_sparse_graph(
            file_path,
            true,
            false,
            FromGraphFile::instantiate_graph,
            FromGraphFile::add_edge,
        )
    }

    fn from_dense_file(file_path: &str) -> Self {
        load_dense_graph(
            file_path,
            true,
            FromGraphFile::instantiate_graph,
            FromGraphFile::add_edge,
        )
    }
}

impl<W: Num + Copy> FromGraphFile<W> for Graph<usize, W> {
    fn instantiate_graph(nodes: usize, edges: usize) -> Self {
        Graph::with_capacity(nodes, edges)
    }

    fn add_edge(graph: &mut Self, from: usize, to: usize, weight: W) {
        graph.add_edge_by_ids(from, to, weight);
    }

    fn from_sparse_file(file_path: &str) -> Self {
        load_sparse_graph(
            file_path,
            true,
            false,
            FromGraphFile::instantiate_graph,
            FromGraphFile::add_edge,
        )
    }

    fn from_dense_file(file_path: &str) -> Self {
        load_dense_graph(
            file_path,
            true,
            FromGraphFile::instantiate_graph,
            FromGraphFile::add_edge,
        )
    }
}
