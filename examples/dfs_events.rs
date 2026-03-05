use sinistra::graph::{
    BasicGraph, DfsEvent, Graph, GraphMut, HashMapStorage, HashMapTopology, dfs,
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
            DfsEvent::DiscoverVertex(v) => {
                println!("discover {} (v{})", graph.vertex(v).unwrap(), v.index())
            }
            DfsEvent::ExamineEdge(u, e, v) => println!(
                "examine edge e{}: {} -> {}",
                e.index(),
                graph.vertex(u).unwrap(),
                graph.vertex(v).unwrap()
            ),
            DfsEvent::TreeEdge(u, _, v) => println!(
                "tree-edge: {} -> {}",
                graph.vertex(u).unwrap(),
                graph.vertex(v).unwrap()
            ),
            DfsEvent::BackEdge(u, _, v) => println!(
                "back-edge: {} -> {}",
                graph.vertex(u).unwrap(),
                graph.vertex(v).unwrap()
            ),
            DfsEvent::ForwardEdge(u, _, v) => println!(
                "forward-edge: {} -> {}",
                graph.vertex(u).unwrap(),
                graph.vertex(v).unwrap()
            ),
            DfsEvent::CrossEdge(u, _, v) => println!(
                "cross-edge: {} -> {}",
                graph.vertex(u).unwrap(),
                graph.vertex(v).unwrap()
            ),
            DfsEvent::FinishVertex(v) => {
                println!("finish {} (v{})", graph.vertex(v).unwrap(), v.index())
            }
        }
    }
}
