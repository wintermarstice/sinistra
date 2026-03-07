use std::collections::HashSet;

use sinistra::graph::{
    BasicGraph, EdgeTopologyMut, Graph, GraphEdgesMutExt, GraphEndpointsExt, GraphMut,
    GraphVertexSetMutExt, HashMapStorage, HashMapTopology, NeighborTopology, Storage, StorageMut,
    Topology, VertexSetMut,
};

#[test]
fn add_and_read_vertices_and_edges() {
    let storage = HashMapStorage::<&str, i32>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let e = graph.add_edge(7, a, b);

    assert_eq!(graph.vertex(a), Some(&"A"));
    assert_eq!(graph.vertex(b), Some(&"B"));
    assert_eq!(graph.edge(e), Some(&7));
    assert_eq!(graph.edge_endpoints(e), Some((a, b)));
}

#[test]
fn mutate_vertex_and_edge_values() {
    let storage = HashMapStorage::<String, i32>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("old".to_string());
    let b = graph.add_vertex("B".to_string());
    let e = graph.add_edge(10, a, b);

    *graph.vertex_mut(a).unwrap() = "new".to_string();
    *graph.edge_mut(e).unwrap() = 11;

    assert_eq!(graph.vertex(a).map(String::as_str), Some("new"));
    assert_eq!(graph.edge(e), Some(&11));
}

#[test]
fn remove_edge_then_vertex() {
    let storage = HashMapStorage::<&str, ()>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let e = graph.add_edge((), a, b);

    assert_eq!(graph.remove_edge(e), Some(()));
    assert_eq!(graph.edge(e), None);

    assert_eq!(graph.remove_vertex(a), Some("A"));
    assert_eq!(graph.vertex(a), None);
}

#[test]
fn removing_vertex_removes_incident_edges() {
    let storage = HashMapStorage::<&str, i32>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let c = graph.add_vertex("C");

    let e1 = graph.add_edge(1, a, b);
    let e2 = graph.add_edge(2, c, a);

    assert_eq!(graph.remove_vertex(a), Some("A"));
    assert_eq!(graph.edge_endpoints(e1), None);
    assert_eq!(graph.edge_endpoints(e2), None);
}

#[test]
fn storage_setters_replace_values() {
    let mut storage = HashMapStorage::<&str, i32>::new();

    let v = storage.add_vertex("A");
    let e = storage.add_edge(1);

    assert_eq!(storage.set_vertex(v, "B"), Some("A"));
    assert_eq!(storage.vertex(v), Some(&"B"));

    assert_eq!(storage.set_edge(e, 2), Some(1));
    assert_eq!(storage.edge(e), Some(&2));
}

#[test]
fn topology_tracks_neighbors_and_adjacency() {
    let mut topology = HashMapTopology::new();
    let mut storage = HashMapStorage::<(), ()>::new();

    let a = storage.add_vertex(());
    let b = storage.add_vertex(());
    let c = storage.add_vertex(());

    let e1 = storage.add_edge(());
    let e2 = storage.add_edge(());

    assert!(topology.add_vertex(a));
    assert!(topology.add_vertex(b));
    assert!(topology.add_vertex(c));

    assert!(topology.add_edge(e1, a, b));
    assert!(topology.add_edge(e2, c, a));

    let out: HashSet<_> = topology.out_neighbors(a).collect();
    let inn: HashSet<_> = topology.in_neighbors(a).collect();

    assert_eq!(out, HashSet::from([b]));
    assert_eq!(inn, HashSet::from([c]));

    let adj: HashSet<_> = topology.adjacent(a).collect();
    assert_eq!(adj, HashSet::from([(b, e1), (c, e2)]));
}
