use sinistra::graph::{BasicGraph, Graph, GraphMut, HashMapStorage, HashMapTopology, Storage};

fn main() {
    type Vertex = String;
    type Edge = ();

    let storage = HashMapStorage::<Vertex, Edge>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    // Add some vertices
    let a = graph.add_vertex("A".into());
    let b = graph.add_vertex("B".into());
    let c = graph.add_vertex("C".into());

    // Add some edges
    graph.add_edge((), a, b); // A -> B
    graph.add_edge((), b, c); // B -> C
    graph.add_edge((), a, c); // A -> C

    println!("Vertices:");

    for vertex in graph.vertices() {
        let value = graph.storage().vertex(vertex).unwrap();
        println!("{} (v{})", value, vertex.index());
    }

    println!("\nEdges:");

    for edge in graph.edges() {
        let (u, v) = graph.edge_endpoints(edge).unwrap();
        let source = graph.storage().vertex(u).unwrap();
        let target = graph.storage().vertex(v).unwrap();
        println!("{} (v{}) -> {} (v{})", source, u.index(), target, v.index());
    }

    println!("\nOutbound Neighbors of A:");

    for neighbor in graph.out_neighbors(a) {
        let value = graph.storage().vertex(neighbor).unwrap();
        println!("{} (v{})", value, neighbor.index());
    }
}
