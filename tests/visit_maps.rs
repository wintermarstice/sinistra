use std::collections::{HashMap, HashSet};

use sinistra::graph::{
    BasicGraph, Color, ColorMap, GenerationalColorMap, GenerationalVisitMap, GraphVertexSetMutExt,
    HashMapStorage, HashMapTopology, VertexHandle, VisitMap,
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

    assert!(visited.visit(v));
    assert!(!visited.visit(v));
    assert!(visited.is_visited(v));
}

#[test]
fn hashmap_color_map_defaults_to_white() {
    let mut colors = HashMap::<VertexHandle, Color>::new();
    let v = any_vertex();

    assert_eq!(colors.color(v), Color::White);
    colors.set_color(v, Color::Gray);
    assert_eq!(colors.color(v), Color::Gray);
}

#[test]
fn generational_visit_map_reset_clears_logical_state() {
    let mut visited = GenerationalVisitMap::new(1);
    let v = any_vertex();

    assert!(visited.visit(v));
    assert!(!visited.visit(v));

    visited.reset();

    assert!(visited.visit(v));
}

#[test]
fn generational_visit_map_expands_capacity() {
    let mut visited = GenerationalVisitMap::new(0);
    let v = any_vertex();

    assert!(visited.visit(v));
    assert!(visited.is_visited(v));
}

#[test]
fn generational_color_map_reset_returns_to_white() {
    let mut colors = GenerationalColorMap::new(2);
    let v = any_vertex();

    colors.set_color(v, Color::Black);
    assert_eq!(colors.color(v), Color::Black);

    colors.reset();

    assert_eq!(colors.color(v), Color::White);
}

#[test]
fn generational_color_map_expands_capacity() {
    let mut colors = GenerationalColorMap::new(0);
    let v = any_vertex();

    colors.set_color(v, Color::Gray);
    assert_eq!(colors.color(v), Color::Gray);
}
