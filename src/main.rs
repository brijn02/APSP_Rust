// The main function to analyse the different algorithms from the folder algorithms
// 30-01-2024

// Modules
mod algorithms;
mod analysis;
mod base;
mod fwb_function;
mod graph;
mod loader;

// Import crates
use num::Num;
use std::env;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

// Import crates from different files
use crate::algorithms::dijkstra_am::Dijkstra;
use crate::algorithms::dijkstra_par::DijkstraPar;
use crate::algorithms::example::BaseLineFloydWarshall;
use crate::algorithms::floyd_warshall_am::FloydWarshall;
use crate::algorithms::floyd_warshall_block::FloydWarshallBlock;
use crate::algorithms::floyd_warshall_block_par::FloydWarshallBlockPar;
use crate::analysis::{compute_average, compute_std};
use crate::base::{APSPAlgorithm, APSPResult};

// Function to measure the algorithm and which returns the result together with the execution time
fn measure_algo<W: Num + Copy + Debug>(
    file_path: &str,
    algorithm: &mut dyn APSPAlgorithm<W>,
) -> (u128, APSPResult<W>) {
    algorithm.load_graph(file_path, true);

    let exec_start = Instant::now();
    algorithm.execute();
    let exec_time = exec_start.elapsed();

    // check for correctness
    let result = algorithm.get_result();

    (exec_time.as_millis(), result)
}

// Function to measure the stats of an algorithm
// There is also a possibility to write the output to a file
fn measure_algo_stats<W: Num + Copy + Debug>(
    file_path: &str,
    algorithm: &mut dyn APSPAlgorithm<W>,
    num_iter: usize,
    write: bool,
    write_to: &str,
) -> (f64, f64) {
    // Load the graph
    algorithm.load_graph(file_path, true);

    // Define an empty vector for the time
    let mut times = vec![];

    // Loop num_iter amount the same time
    for _ in 0..num_iter {
        // Start measuring time execute file and return the execution time which is pushed to times vector
        let exec_start = Instant::now();
        algorithm.execute();
        let exec_time = exec_start.elapsed();
        times.push(exec_time.as_millis() as f64);
    }

    // Compute the average and the std
    let average = compute_average(&times);
    let std = compute_std(&times, average);

    // If you want to write, write the results to a file
    if write {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(write_to)
        {
            if let Err(e) = writeln!(file, "{:.05} {:.05}", average, std) {
                eprintln!("Error writing to file: {}", e);
            }
        } else {
            eprintln!("Error opening file");
        }
    }
    // return the average and standard deviation
    (average, std)
}

fn print_type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

macro_rules! measure_all {    // Base case: when there are no more structs to instantiate
    () => {};
    ($instance_path:expr, $num_iter:expr, $write: expr, $write_to:expr, []) => {};
    ($instance_path:expr, $num_iter:expr, $write: expr, $write_to:expr, [$instance: expr$(, $($rest:tt)*)?]) => {
        let mut instance = $instance;
        let algo_id = print_type_of(&instance);
        let algo_id = algo_id.split("::").last().unwrap();

        let (avg_fw, std_fw) = measure_algo_stats(
            $instance_path,
            &mut instance,
            $num_iter,
            $write,
            $write_to,
        );
        println!("{: <30} {:.03} +/- {:.03} ms", algo_id, avg_fw, std_fw);

        measure_all!($instance_path, $num_iter, $write, $write_to, [$($($rest)*)?]);
    };
}

macro_rules! eval_all {    // Base case: when there are no more structs to instantiate
    () => {};
    ($instance_path:expr, $result:expr, []) => {};
    ($instance_path:expr, $result:expr, [$instance: expr$(, $($rest:tt)*)?]) => {
        let mut instance = $instance;
        let algo_id = print_type_of(&instance);
        let algo_id = algo_id.split("::").last().unwrap();
        let (duration_fw, fw_result) = measure_algo($instance_path, &mut instance);
        let evaluation: bool = fw_result.result_compare($result);
        println!("{: <30} Runtime: {}ms", algo_id, duration_fw);
        println!("{: <30} Evaluation: {}", algo_id, evaluation);
        println!("{:-<30}", "");

        eval_all!($instance_path, $result, [$($($rest)*)?]);
    };
}

fn main() {
    // Collect arguments from the command line
    let args: Vec<String> = env::args().collect();

    // Define the root path
    let root_path = env!("CARGO_MANIFEST_DIR");
    let mut instance_path = format!("{root_path}/instances/b18.gph");
    let mut write = false;
    let mut write_to = root_path.to_string();

    // Read arguments from the command line
    if args.len() == 2 {
        let filename: String = args[1].parse().expect("This filename does not work");
        instance_path = format!("{root_path}/{filename}");
    } else if args.len() == 3 {
        let filename: String = args[1].parse().expect("This filename does not work");
        instance_path = format!("{root_path}/{filename}");
        write = true;
        let filename: String = args[2].parse().expect("This filename does not work");
        write_to = format!("{root_path}/results/{filename}");
    }

    // Print the instance path
    println!("{}", instance_path);

    // compute the results for all our different algorithms and compare the results with the PetGraph package
    // If correct print true, otherwise false
    let mut base_floyd_warshall: BaseLineFloydWarshall<u16> = BaseLineFloydWarshall::new();
    let (duration_bfw, bfw_result) = measure_algo(&instance_path, &mut base_floyd_warshall);
    println!("FW petgraph:                   Runtime: {}ms", duration_bfw);

    const PARALLEL_FW_THREADS: usize = 10;
    const PARALLEL_FW_BLOCK_SIZE: usize = 10;
    eval_all!(
        &instance_path,
        &bfw_result.shortest_paths,
        [
            FloydWarshall::<u16>::new(),
            Dijkstra::<u16>::new(),
            DijkstraPar::<u16>::new(PARALLEL_FW_THREADS),
            FloydWarshallBlock::<u16>::new(PARALLEL_FW_BLOCK_SIZE),
            FloydWarshallBlockPar::<u16>::new(PARALLEL_FW_THREADS)
        ]
    );

    let num_iter = 10;
    let threads = 10;
    let num_blocks = 10;
    measure_all!(
        &instance_path,
        num_iter,
        write,
        &write_to,
        [
            BaseLineFloydWarshall::<u16>::new(),
            FloydWarshall::<u16>::new(),
            Dijkstra::<u16>::new(),
            DijkstraPar::<u16>::new(threads),
            FloydWarshallBlock::<u16>::new(num_blocks),
            FloydWarshallBlockPar::<u16>::new(threads)
        ]
    );
}
