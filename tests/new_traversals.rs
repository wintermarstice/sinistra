use std::collections::HashMap;

use sinistra::graph::{
    BasicGraph, Graph, GraphMut, HashMapStorage, HashMapTopology, bfs_layers, has_path, is_dag,
    topological_sort,
};

fn dag_graph() -> (
    BasicGraph<HashMapStorage<&'static str, ()>, HashMapTopology>,
    sinistra::graph::VertexHandle,
    sinistra::graph::VertexHandle,
    HashMap<&'static str, sinistra::graph::VertexHandle>,
) {
    let storage = HashMapStorage::<&str, ()>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let c = graph.add_vertex("C");
    let d = graph.add_vertex("D");
    let e = graph.add_vertex("E");

    graph.add_edge((), a, b);
    graph.add_edge((), a, c);
    graph.add_edge((), b, d);
    graph.add_edge((), c, d);
    graph.add_edge((), d, e);

    let labels = HashMap::from([("A", a), ("B", b), ("C", c), ("D", d), ("E", e)]);

    (graph, a, e, labels)
}

#[test]
fn topological_sort_contains_every_vertex_for_dag() {
    let (graph, _, _, _) = dag_graph();
    let order = topological_sort(&graph);

    assert_eq!(order.len(), graph.vertices().count());
}

#[test]
fn topological_sort_respects_edge_directions() {
    let (graph, _, _, _) = dag_graph();
    let order = topological_sort(&graph);

    let mut position = HashMap::new();
    for (i, vertex) in order.iter().enumerate() {
        position.insert(*vertex, i);
    }

    for edge in graph.edges() {
        let (u, v) = graph.edge_endpoints(edge).unwrap();
        assert!(position[&u] < position[&v]);
    }
}

#[test]
fn is_dag_true_for_acyclic_graph() {
    let (graph, _, _, _) = dag_graph();
    assert!(is_dag(&graph));
}

#[test]
fn is_dag_false_for_graph_with_cycle() {
    let storage = HashMapStorage::<&str, ()>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let c = graph.add_vertex("C");

    graph.add_edge((), a, b);
    graph.add_edge((), b, c);
    graph.add_edge((), c, a);

    assert!(!is_dag(&graph));
}

#[test]
fn bfs_layers_groups_vertices_by_distance() {
    let (graph, start, _, labels) = dag_graph();
    let layers = bfs_layers(&graph, start);

    assert_eq!(layers.len(), 4);
    assert_eq!(layers[0], vec![labels["A"]]);

    assert_eq!(layers[1].len(), 2);
    assert!(layers[1].contains(&labels["B"]));
    assert!(layers[1].contains(&labels["C"]));

    assert_eq!(layers[2], vec![labels["D"]]);
    assert_eq!(layers[3], vec![labels["E"]]);
}

#[test]
fn has_path_detects_reachability() {
    let (graph, start, end, labels) = dag_graph();

    assert!(has_path(&graph, start, end));
    assert!(has_path(&graph, start, start));
    assert!(!has_path(&graph, labels["E"], labels["A"]));
}
