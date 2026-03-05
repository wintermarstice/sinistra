use std::{collections::HashMap, f64, ops::Add};

use sinistra::graph::{
    BasicGraph, DijkstraEvent, Graph, GraphMut, HashMapStorage, HashMapTopology, VertexHandle,
    Weight, Weighted, dijkstra,
};

fn main() {
    let storage = HashMapStorage::<City, Distance>::new();
    let topology = HashMapTopology::new();
    let mut graph = BasicGraph::new(storage, topology);

    let istanbul = graph.add_vertex(City::İstanbul);
    let ankara = graph.add_vertex(City::Ankara);
    let izmir = graph.add_vertex(City::İzmir);
    let ordu = graph.add_vertex(City::Ordu);
    let canakkale = graph.add_vertex(City::Çanakkale);

    graph.add_edge(Distance(450.0), istanbul, ankara);
    graph.add_edge(Distance(330.0), istanbul, izmir);
    graph.add_edge(Distance(380.0), ankara, izmir);
    graph.add_edge(Distance(190.0), ankara, ordu);
    graph.add_edge(Distance(300.0), ankara, canakkale);
    graph.add_edge(Distance(250.0), izmir, canakkale);
    graph.add_edge(Distance(350.0), izmir, ordu);
    graph.add_edge(Distance(200.0), ordu, canakkale);
    graph.add_edge(Distance(400.0), canakkale, ordu);

    println!("Cities:\n");
    for v in graph.vertices() {
        let city = graph.vertex(v).unwrap();
        println!("{:?} (v{})", city, v.index());
    }
    println!("\nRunning Dijkstra from İstanbul...\n");
    let mut distances: HashMap<VertexHandle, Distance> = HashMap::new();
    for event in dijkstra::<_, Distance>(&graph, istanbul) {
        if let DijkstraEvent::RelaxEdge { target, weight, .. } = event {
            distances.insert(target, weight);
        }
    }
    println!("Shortest distances:\n");
    println!("İstanbul: 0 km");
    for (v, d) in distances {
        let city = graph.vertex(v).unwrap();
        println!("{:?}: {:.1} km", city, d.0);
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

impl Add for Distance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Distance(self.0 + rhs.0)
    }
}

impl Default for Distance {
    fn default() -> Self {
        Distance(0.0)
    }
}

impl Weight for Distance {
    fn infinity() -> Self {
        Distance(f64::INFINITY)
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Distance {}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Weighted for Distance {
    type Weight = Distance;

    fn weight(&self) -> Self::Weight {
        *self
    }
}
