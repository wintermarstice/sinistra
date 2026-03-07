use std::collections::{HashMap, VecDeque};

use crate::graph::{EndpointTopology, Graph, GraphEdgesExt, GraphEndpointsExt, VertexHandle};

pub fn topological_sort<G>(graph: &G) -> Vec<VertexHandle>
where
    G: Graph,
    G::Topology: EndpointTopology,
{
    let mut indegree: HashMap<VertexHandle, usize> = HashMap::new();

    for vertex in graph.vertices() {
        indegree.entry(vertex).or_insert(0);
    }

    for edge in graph.edges() {
        if let Some((_, target)) = graph.edge_endpoints(edge) {
            *indegree.entry(target).or_insert(0) += 1;
        }
    }

    // Do we have to really collect() it?
    let mut queue: VecDeque<VertexHandle> = indegree
        .iter()
        .filter_map(|(vertex, degree)| if *degree == 0 { Some(*vertex) } else { None })
        .collect();

    let mut order = Vec::with_capacity(indegree.len());

    while let Some(vertex) = queue.pop_front() {
        order.push(vertex);

        for edge in graph.out_edges(vertex) {
            let Some((_, target)) = graph.edge_endpoints(edge) else {
                continue;
            };

            let degree = indegree
                .get_mut(&target)
                .expect("target vertex should exist in indegree map");

            if *degree > 0 {
                *degree -= 1;

                if *degree == 0 {
                    queue.push_back(target);
                }
            }
        }
    }

    order
}

pub fn is_dag<G>(graph: &G) -> bool
where
    G: Graph,
    G::Topology: EndpointTopology,
{
    topological_sort(graph).len() == graph.vertices().count()
}

pub fn bfs_layers<G>(graph: &G, start: VertexHandle) -> Vec<Vec<VertexHandle>>
where
    G: Graph,
    G::Topology: EndpointTopology,
{
    let mut visited = std::collections::HashSet::new();
    let mut queue = VecDeque::new();
    let mut layers = Vec::new();

    visited.insert(start);
    queue.push_back((start, 0usize));

    while let Some((vertex, level)) = queue.pop_front() {
        if layers.len() == level {
            layers.push(Vec::new());
        }

        layers[level].push(vertex);

        for edge in graph.out_edges(vertex) {
            let Some((_, target)) = graph.edge_endpoints(edge) else {
                continue;
            };

            if visited.insert(target) {
                queue.push_back((target, level + 1));
            }
        }
    }

    layers
}

pub fn has_path<G>(graph: &G, source: VertexHandle, target: VertexHandle) -> bool
where
    G: Graph,
    G::Topology: EndpointTopology,
{
    if source == target {
        return true;
    }

    bfs_layers(graph, source)
        .into_iter()
        .flatten()
        .any(|vertex| vertex == target)
}
