use std::collections::{HashMap, HashSet};

use sinistra::graph::{
    BasicGraph, Color, GraphVertexSetMutExt, HashMapStorage, HashMapTopology, PropertyMap,
    PropertySet, VertexHandle,
};

fn any_vertex() -> sinistra::graph::VertexHandle {
    let storage = HashMapStorage::<(), ()>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);
    graph.add_vertex(())
}

#[test]
fn hashset_visit_map_marks_vertices() {
    let mut visited = HashSet::<VertexHandle>::new();
    let v = any_vertex();

    assert!(visited.mark(v, true));
    assert!(!visited.mark(v, true));
    assert!(visited.check(&v));
}

#[test]
fn hashmap_color_map_defaults_to_white() {
    let mut colors = HashMap::<VertexHandle, Color>::new();
    let v = any_vertex();

    assert_eq!(
        colors.get_property(&v).unwrap_or(&Color::White),
        &Color::White
    );
    colors.set_property(v, Color::Gray);
    assert_eq!(colors.get_property(&v), Some(&Color::Gray));
}
