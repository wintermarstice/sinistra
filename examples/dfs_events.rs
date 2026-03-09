use sinistra::graph::{
    BasicGraph, DfsEvent, Graph, GraphEdgesMutExt, GraphVertexSetMutExt, HashMapStorage,
    HashMapTopology, TraversalEvent, dfs,
};

fn main() {
    let storage = HashMapStorage::<&str, ()>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let c = graph.add_vertex("C");
    let d = graph.add_vertex("D");

    graph.add_edge((), a, b);
    graph.add_edge((), a, c);
    graph.add_edge((), b, d);
    graph.add_edge((), c, d);

    println!("DFS events from A:");

    for event in dfs::<_, std::collections::HashMap<_, _>>(&graph, a) {
        match event {
            DfsEvent::Core(TraversalEvent::Discover { vertex }) => {
                println!(
                    "discover {} (v{})",
                    graph.vertex(vertex).unwrap(),
                    vertex.index()
                )
            }
            DfsEvent::Core(TraversalEvent::Examine {
                source,
                edge,
                target,
            }) => println!(
                "examine edge e{}: {} -> {}",
                edge.index(),
                graph.vertex(source).unwrap(),
                graph.vertex(target).unwrap()
            ),
            DfsEvent::TreeEdge {
                source,
                edge: _,
                target,
            } => println!(
                "tree-edge: {} -> {}",
                graph.vertex(source).unwrap(),
                graph.vertex(target).unwrap()
            ),
            DfsEvent::BackEdge {
                source,
                edge: _,
                target,
            } => println!(
                "back-edge: {} -> {}",
                graph.vertex(source).unwrap(),
                graph.vertex(target).unwrap()
            ),
            DfsEvent::ForwardEdge {
                source,
                edge: _,
                target,
            } => println!(
                "forward-edge: {} -> {}",
                graph.vertex(source).unwrap(),
                graph.vertex(target).unwrap()
            ),
            DfsEvent::CrossEdge {
                source,
                edge: _,
                target,
            } => println!(
                "cross-edge: {} -> {}",
                graph.vertex(source).unwrap(),
                graph.vertex(target).unwrap()
            ),
            DfsEvent::Core(TraversalEvent::Finish { vertex }) => {
                println!(
                    "finish {} (v{})",
                    graph.vertex(vertex).unwrap(),
                    vertex.index()
                )
            }
        }
    }
}
