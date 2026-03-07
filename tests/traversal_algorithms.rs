use std::collections::{HashMap, HashSet};
use std::{f64, ops::Add};

use sinistra::graph::{
    BasicGraph, BfsEvent, DfsEvent, DijkstraEvent, Graph, GraphEdgesMutExt, GraphVertexSetMutExt,
    HashMapStorage, HashMapTopology, TraversalEvent, VertexHandle, Weight, Weighted,
    bfs_tree_edges, bfs_vertices, dfs, dijkstra,
};

fn make_graph() -> (
    BasicGraph<HashMapStorage<&'static str, Cost>, HashMapTopology>,
    VertexHandle,
) {
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

    (graph, a)
}

#[test]
fn bfs_vertices_discovers_expected_set() {
    let (graph, start) = make_graph();

    let discovered: HashSet<_> = bfs_vertices::<_, HashSet<VertexHandle>>(&graph, start).collect();
    assert_eq!(discovered.len(), 3);
}

#[test]
fn bfs_tree_edges_count_matches_reachable_minus_one() {
    let (graph, start) = make_graph();

    let edges: Vec<_> = bfs_tree_edges::<_, HashSet<VertexHandle>>(&graph, start).collect();
    assert_eq!(edges.len(), 3);
}

#[test]
fn bfs_emits_discover_and_finish_events() {
    let (graph, start) = make_graph();

    let events: Vec<_> = sinistra::graph::bfs::<_, HashSet<VertexHandle>>(&graph, start).collect();

    let discover_count = events
        .iter()
        .filter(|event| matches!(event, BfsEvent::Core(TraversalEvent::Discover { .. })))
        .count();
    let finish_count = events
        .iter()
        .filter(|event| matches!(event, BfsEvent::Core(TraversalEvent::Finish { .. })))
        .count();

    assert_eq!(discover_count, 3);
    assert_eq!(finish_count, 4);
}

#[test]
fn dfs_discovers_and_finishes_all_reachable_vertices() {
    let (graph, start) = make_graph();

    let events: Vec<_> = dfs::<_, HashMap<VertexHandle, _>>(&graph, start).collect();

    let discover = events
        .iter()
        .filter(|event| matches!(event, DfsEvent::DiscoverVertex(_)))
        .count();
    let finish = events
        .iter()
        .filter(|event| matches!(event, DfsEvent::FinishVertex(_)))
        .count();

    assert_eq!(discover, 4);
    assert_eq!(finish, 4);
}

#[test]
fn dijkstra_relaxes_shortest_paths() {
    let (graph, start) = make_graph();
    let mut best: HashMap<VertexHandle, Cost> = HashMap::new();

    for event in dijkstra::<_, Cost>(&graph, start) {
        if let DijkstraEvent::RelaxEdge { target, weight, .. } = event {
            best.insert(target, weight);
        }
    }

    let labels: HashMap<_, _> = best
        .into_iter()
        .map(|(v, w)| (*graph.vertex(v).unwrap(), w.0))
        .collect();

    assert_eq!(labels.get("B"), Some(&1.0));
    assert_eq!(labels.get("C"), Some(&3.0));
    assert_eq!(labels.get("D"), Some(&4.0));
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
