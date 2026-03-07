use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

use crate::graph::{
    EdgeHandle, EndpointTopology, Graph, GraphEdgesExt, GraphEndpointsExt, Policy, Storage,
    Traversal, TraversalEvent, VertexHandle, Weight, Weighted,
};

type Edge<G> = <<G as Graph>::Storage as Storage>::Edge;
// type Vertex<G> = <<G as Graph>::Storage as Storage>::Vertex;

pub struct Dijkstra<'graph, G: Graph, W: Weight> {
    graph: &'graph G,
    frontier: BinaryHeap<(Reverse<W>, VertexHandle)>,
    distances: HashMap<VertexHandle, W>,
}

#[derive(Debug, Clone, Copy)]
pub enum Event<W> {
    Core(TraversalEvent),

    RelaxEdge {
        source: VertexHandle,
        edge: EdgeHandle,
        target: VertexHandle,
        weight: W,
    },
}

impl<'graph, G, W> Dijkstra<'graph, G, W>
where
    W: Weight,
    G: Graph,
{
    pub fn new(graph: &'graph G) -> Self {
        Self {
            graph,
            frontier: BinaryHeap::new(),
            distances: HashMap::new(),
        }
    }
}

impl<'graph, G, W> Policy for Dijkstra<'graph, G, W>
where
    G: Graph,
    Edge<G>: Weighted<Weight = W>,
    W: Weight,
    G::Topology: EndpointTopology,
{
    type Event = Event<W>;
    type Item = (W, VertexHandle);

    fn start(&mut self, start: VertexHandle) {
        self.distances.insert(start, W::default());
        self.frontier.push((Reverse(W::default()), start));
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.frontier.pop().map(|(Reverse(w), v)| (w, v))
    }

    fn process(&mut self, item: Self::Item, pending: &mut VecDeque<Self::Event>) {
        let (distance, source) = item;

        if distance > self.distances[&source] {
            return;
        }

        for edge in self.graph.out_edges(source) {
            let (_, target) = self.graph.edge_endpoints(edge).unwrap();

            pending.push_back(Event::Core(TraversalEvent::Examine {
                source,
                target,
                edge,
            }));

            let weight = self.graph.edge(edge).unwrap().weight();
            let alt = distance + weight;

            if alt < *self.distances.get(&target).unwrap_or(&W::infinity()) {
                self.distances.insert(target, alt);
                self.frontier.push((Reverse(alt), target));

                pending.push_back(Event::RelaxEdge {
                    source,
                    edge,
                    target,
                    weight: alt,
                });

                pending.push_back(Event::Core(TraversalEvent::Discover { vertex: target }));
            }
        }

        pending.push_back(Event::Core(TraversalEvent::Finish { vertex: source }));
    }
}

pub fn dijkstra<G, W>(graph: &G, start: VertexHandle) -> Traversal<Dijkstra<'_, G, W>>
where
    G: Graph,
    Edge<G>: Weighted<Weight = W>,
    W: Weight,
    G::Topology: EndpointTopology,
{
    let policy = Dijkstra::new(graph);
    Traversal::new(policy, start)
}

pub fn dijkstra_distances<G, W>(
    graph: &G,
    start: VertexHandle,
) -> impl Iterator<Item = (VertexHandle, W)>
where
    G: Graph,
    Edge<G>: Weighted<Weight = W>,
    W: Weight,
    G::Topology: EndpointTopology,
{
    dijkstra::<G, W>(graph, start).filter_map(|event| {
        if let Event::RelaxEdge { target, weight, .. } = event {
            Some((target, weight))
        } else {
            None
        }
    })
}
