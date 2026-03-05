use std::collections::HashSet;

use sinistra::graph::{
    BasicGraph, Graph, GraphMut, Topology, VecStorage, VecTopology, bfs_layers, bfs_vertices,
};

#[test]
fn vec_storage_and_vec_topology_support_basic_graph_operations() {
    let storage = VecStorage::<&str, i32>::new();
    let topology = VecTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let e = graph.add_edge(5, a, b);

    assert_eq!(graph.vertex(a), Some(&"A"));
    assert_eq!(graph.vertex(b), Some(&"B"));
    assert_eq!(graph.edge(e), Some(&5));

    assert_eq!(graph.remove_edge(e), Some(5));
    assert_eq!(graph.edge(e), None);

    assert_eq!(graph.remove_vertex(a), Some("A"));
    assert_eq!(graph.vertex(a), None);
}

#[test]
fn vec_storage_setters_and_mutation_work() {
    let storage = VecStorage::<String, i32>::new();
    let topology = VecTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A".to_string());
    let b = graph.add_vertex("B".to_string());
    let e = graph.add_edge(1, a, b);

    *graph.vertex_mut(a).unwrap() = "A1".to_string();
    *graph.edge_mut(e).unwrap() = 2;

    assert_eq!(graph.vertex(a).map(String::as_str), Some("A1"));
    assert_eq!(graph.edge(e), Some(&2));
}

#[test]
fn vec_topology_neighbor_queries_are_consistent() {
    let storage = VecStorage::<&str, ()>::new();
    let topology = VecTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let c = graph.add_vertex("C");

    let e1 = graph.add_edge((), a, b);
    let e2 = graph.add_edge((), c, a);

    let out: HashSet<_> = graph.out_neighbors(a).collect();
    let inn: HashSet<_> = graph.in_neighbors(a).collect();
    let adj: HashSet<_> = graph.topology().adjacent(a).collect();

    assert_eq!(out, HashSet::from([b]));
    assert_eq!(inn, HashSet::from([c]));
    assert_eq!(adj, HashSet::from([(b, e1), (c, e2)]));
}

#[test]
fn vec_topology_works_with_traversal_algorithms() {
    let storage = VecStorage::<&str, ()>::new();
    let topology = VecTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let c = graph.add_vertex("C");
    let d = graph.add_vertex("D");

    graph.add_edge((), a, b);
    graph.add_edge((), a, c);
    graph.add_edge((), c, d);

    let discovered: HashSet<_> = bfs_vertices::<_, HashSet<_>>(&graph, a).collect();
    assert_eq!(discovered.len(), 3);

    let layers = bfs_layers(&graph, a);
    assert_eq!(layers.len(), 3);
    assert_eq!(layers[0], vec![a]);
    assert_eq!(layers[2], vec![d]);
}
