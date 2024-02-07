# APSP_Rust
A Rust program to solve the all pairs shortest path problem
It contains six algorithm:
- Floyd-Warshall (pethraph)
- Floyd-Warshall (own implementation)
- Dijsktra
- Dijkstra parallel
- Floyd-Warshall Blocked
- Floyd-Warshall Parallel
![Averega Runtime different algorithms](https://github.com/brijn02/APSP_Rust/blob/main/images_dense/dense_all_loglog-1.png)

## Structure
The Git contains multiple files
- images_dense contains all the figures generated for the dense graphs
- images_sparse contains all the figures generated for the sparse graphs
- instances contains some example instances
- results contains all the results generated using dense and sparse graphs, the number of vertices is given in the file name
- src contains all the rust files. For the structure please see the report
- The other files are files to generate or process data

## Run with cargo run
cargo run --release (filename to read) (filename to save results)
The last two are optional. If empty it will run file b18.gph

