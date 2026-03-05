use std::{f64, ops::Add};

use sinistra::graph::{
    BasicGraph, Graph, GraphMut, HashMapStorage, HashMapTopology, Weight, Weighted,
    dijkstra_distances,
};

fn main() {
    let storage = HashMapStorage::<&str, Cost>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let c = graph.add_vertex("C");
    let d = graph.add_vertex("D");

    graph.add_edge(Cost(1.0), a, b);
    graph.add_edge(Cost(4.0), a, c);
    graph.add_edge(Cost(2.0), b, c);
    graph.add_edge(Cost(7.0), b, d);
    graph.add_edge(Cost(1.0), c, d);

    println!("Shortest distances from A:");
    println!("A: 0.0");

    for (vertex, cost) in dijkstra_distances::<_, Cost>(&graph, a) {
        println!("{}: {:.1}", graph.vertex(vertex).unwrap(), cost.0);
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Cost(f64);

impl Add for Cost {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cost(self.0 + rhs.0)
    }
}

impl Weight for Cost {
    fn infinity() -> Self {
        Cost(f64::INFINITY)
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Cost {}

impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Weighted for Cost {
    type Weight = Cost;

    fn weight(&self) -> Self::Weight {
        *self
    }
}
