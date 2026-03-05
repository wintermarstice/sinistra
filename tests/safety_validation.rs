use sinistra::graph::{
    BasicGraph, Graph, GraphMut, HashMapStorage, HashMapTopology, Storage, Topology, TopologyMut,
    VecStorage, VecTopology,
};

#[test]
fn storage_operations_reject_foreign_handles_without_panicking() {
    let mut s1 = HashMapStorage::<&str, i32>::new();
    let mut s2 = HashMapStorage::<&str, i32>::new();

    let v1 = s1.add_vertex("A");
    let e1 = s1.add_edge(1);

    let v2 = s2.add_vertex("B");
    let e2 = s2.add_edge(2);

    assert_eq!(s1.set_vertex(v2, "X"), None);
    assert_eq!(s1.set_edge(e2, 9), None);
    assert_eq!(s1.remove_vertex(v2), None);
    assert_eq!(s1.remove_edge(e2), None);

    assert_eq!(s1.vertex(v1), Some(&"A"));
    assert_eq!(s1.edge(e1), Some(&1));
}

#[test]
fn vec_topology_rejects_cross_graph_handles() {
    let mut g1 = BasicGraph::new(VecStorage::<(), ()>::new(), VecTopology::new());
    let mut g2 = BasicGraph::new(VecStorage::<(), ()>::new(), VecTopology::new());

    let a1 = g1.add_vertex(());
    let b1 = g1.add_vertex(());
    let e1 = g1.add_edge((), a1, b1);

    let a2 = g2.add_vertex(());
    let b2 = g2.add_vertex(());
    let e2 = g2.add_edge((), a2, b2);

    assert_eq!(g1.edge_endpoints(e2), None);
    assert_eq!(g1.out_neighbors(a2).next(), None);
    assert_eq!(g1.in_neighbors(a2).next(), None);
    assert_eq!(g1.out_edges(a2).next(), None);
    assert_eq!(g1.topology().adjacent(a2).next(), None);

    assert_eq!(g1.remove_edge(e2), None);
    assert_eq!(g1.remove_vertex(a2), None);

    assert_eq!(g1.edge_endpoints(e1), Some((a1, b1)));
    assert_eq!(g2.edge_endpoints(e2), Some((a2, b2)));
}

#[test]
fn hashmap_topology_validates_vertex_membership_on_add_edge() {
    let mut topology = HashMapTopology::new();
    let mut s1 = HashMapStorage::<(), ()>::new();
    let mut s2 = HashMapStorage::<(), ()>::new();

    let a = s1.add_vertex(());
    let b = s1.add_vertex(());
    let foreign = s2.add_vertex(());
    let e = s1.add_edge(());

    assert!(topology.add_vertex(a));
    assert!(topology.add_vertex(b));

    assert!(!topology.add_edge(e, a, foreign));
    assert_eq!(topology.edge_endpoints(e), None);

    assert!(topology.add_edge(e, a, b));
    assert_eq!(topology.edge_endpoints(e), Some((a, b)));
}
