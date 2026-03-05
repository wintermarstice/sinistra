# sinistra

A lightweight, generic graph library in Rust.

`sinistra` provides composable storage/topology abstractions and traversal algorithms such as BFS, DFS, and Dijkstra.

## Features

- Generic `Graph` and `GraphMut` traits.
- `BasicGraph` implementation with pluggable storage/topology.
- `HashMapStorage` + `HashMapTopology` out of the box.
- `VecStorage` + `VecTopology` for dense/indexed graph layouts.
- Directed and undirected graph topologies (`Undirected<T>` wrapper).
- Traversal and shortest-path algorithms:
  - `bfs`, `bfs_vertices`, `bfs_tree_edges`, `bfs_layers`, `has_path`
  - `dfs`
  - `dijkstra`, `dijkstra_distances`
  - `topological_sort`, `is_dag`

## Getting started

Add to your `Cargo.toml`:

```toml
[dependencies]
sinistra = "0.1.0"
```

Create a graph:

```rust
use sinistra::graph::{BasicGraph, GraphMut, HashMapStorage, HashMapTopology};

let storage = HashMapStorage::<&str, ()>::new();
let topology = HashMapTopology::new();
let mut graph = BasicGraph::new(storage, topology);

let a = graph.add_vertex("A");
let b = graph.add_vertex("B");
graph.add_edge((), a, b);
```

## Examples

Run all examples:

```bash
cargo run --example basic_graph
cargo run --example bfs_distances
cargo run --example dijkstra_cities
cargo run --example dfs_events
cargo run --example dijkstra_distances
```

## Development

```bash
cargo fmt
cargo test
```

## Coverage

Generate an LCOV report locally:

```bash
cargo install cargo-llvm-cov
cargo llvm-cov --workspace --all-targets --lcov --output-path lcov.info
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE).
