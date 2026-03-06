# sinistra

**Sinistra** is a lightweight, generic graph framework for Rust with pluggable storage and topology and streaming traversal algorithms.

It provides composable abstractions for representing graph structure and data separately, allowing algorithms such as **BFS**, **DFS**, **Dijkstra**, and **topological sort** to operate on many kinds of graph representations.

---

## Why sinistra?

Most graph libraries tightly couple graph storage with connectivity.

Sinistra separates **graph data** from **graph structure**: Graph = Storage (vertex/edge data) + Topology (connectivity)


This design allows algorithms to run on different graph backends without modification.

Possible uses include:

- in-memory graphs
- dense/indexed graphs
- implicit graphs
- database-backed graphs
- custom graph views and adapters

Algorithms operate on the `Graph` trait, making them reusable across different graph representations.

---

## Features

- Generic `Graph` and `GraphMut` traits
- `BasicGraph` implementation with pluggable storage and topology
- Built-in storage and topology implementations:
  - `HashMapStorage` + `HashMapTopology`
- Directed and undirected graph topologies (`Undirected<T>` wrapper)
- Traversal and shortest-path algorithms:
  - `bfs`, `bfs_vertices`, `bfs_tree_edges`, `bfs_layers`, `has_path`
  - `dfs`
  - `dijkstra`, `dijkstra_distances`
  - `topological_sort`, `is_dag`
- Streaming algorithms using iterators and events
- Zero required allocations inside algorithms

---

## Getting started

Add `sinistra` to your `Cargo.toml`:

```toml
[dependencies]
sinistra = "0.1.0-202603060803"
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

---

## Examples

Run all examples:

```bash
cargo run --example basic_graph
cargo run --example bfs_distances
cargo run --example dijkstra_cities
cargo run --example dfs_events
cargo run --example dijkstra_distances
```

---

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
