use std::collections::{HashMap, HashSet};

use sinistra::graph::{
    BasicGraph, Graph, GraphEdgesExt, GraphEdgesMutExt, GraphEndpointsExt, GraphVertexSetMutExt,
    HashMapStorage, HashMapTopology, VertexHandle, bfs_tree_edges,
};

fn main() {
    let storage = HashMapStorage::<City, Distance>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    // Add cities to the graph
    let istanbul = graph.add_vertex(City::İstanbul);
    let ankara = graph.add_vertex(City::Ankara);
    let izmir = graph.add_vertex(City::İzmir);
    let ordu = graph.add_vertex(City::Ordu);
    let çanakkale = graph.add_vertex(City::Çanakkale);

    // Add edges to the graph (their distances)
    graph.add_edge(Distance(450.0), istanbul, ankara);
    graph.add_edge(Distance(330.0), istanbul, izmir);
    graph.add_edge(Distance(380.0), ankara, izmir);
    graph.add_edge(Distance(190.0), ankara, ordu);
    graph.add_edge(Distance(300.0), ankara, çanakkale);
    graph.add_edge(Distance(250.0), izmir, çanakkale);
    graph.add_edge(Distance(350.0), izmir, ordu);
    graph.add_edge(Distance(200.0), ordu, çanakkale);
    graph.add_edge(Distance(400.0), çanakkale, ordu);

    println!("Cities:");

    for vertex in graph.vertices() {
        let city = graph.vertex(vertex).unwrap();
        println!("{:?} (v{})", city, vertex.index());
    }

    println!("\nConnections:");

    for edge in graph.edges() {
        let (u, v) = graph.edge_endpoints(edge).unwrap();
        let source = graph.vertex(u).unwrap();
        let target = graph.vertex(v).unwrap();
        let distance = graph.edge(edge).unwrap();

        println!("{:?} -[ {:>5.1} km ]-> {:?}", source, distance.0, target);
    }

    println!("\nBFS starting from İstanbul:\n");
    let start = istanbul;

    // This hash map is for counting the level of each vertex in the BFS traversal
    let mut levels: HashMap<VertexHandle, usize> = HashMap::new();
    levels.insert(start, 0);

    println!(
        "visit {:?} (v{}) at level 0",
        graph.vertex(start).unwrap(),
        start.index()
    );

    // HashSet<VertexHandle> is used as the visit map
    for (u, _, v) in bfs_tree_edges::<_, HashSet<VertexHandle>>(&graph, start) {
        let level = levels[&u] + 1;
        levels.insert(v, level);

        let city = graph.vertex(v).unwrap();
        println!("visit {:?} (v{}) at level {}", city, v.index(), level);
    }

    println!("\nBFS tree:");

    for (u, _, v) in bfs_tree_edges::<_, HashSet<VertexHandle>>(&graph, start) {
        println!("v{} -> v{}", u.index(), v.index());
    }
}

#[derive(Debug)]
enum City {
    İstanbul,
    Ankara,
    İzmir,
    Ordu,
    Çanakkale,
}

// Unit: KM
#[derive(Debug, Clone, Copy)]
struct Distance(f64);
