use std::collections::VecDeque;

use crate::graph::{
    EdgeHandle, EndpointTopology, Graph, GraphEdgesExt, GraphEndpointsExt, Policy, PropertySet,
    Traversal, TraversalEvent, VertexHandle,
};

pub struct Bfs<'graph, G, V> {
    graph: &'graph G,
    frontier: VecDeque<VertexHandle>,
    visited: V,
}

impl<'graph, G, V> Bfs<'graph, G, V>
where
    G: Graph,
    V: PropertySet<Key = VertexHandle> + Default,
    G::Topology: EndpointTopology,
{
    pub fn new(graph: &'graph G, start: VertexHandle, visited: V) -> Traversal<Self> {
        let bfs = Self {
            graph,
            frontier: VecDeque::new(),
            visited,
        };

        Traversal::new(bfs, start)
    }
}

impl<'graph, G, V> Policy for Bfs<'graph, G, V>
where
    G: Graph,
    V: PropertySet<Key = VertexHandle>,
    G::Topology: EndpointTopology,
{
    type Event = Event;
    type Item = VertexHandle;

    fn start(&mut self, start: VertexHandle) {
        self.frontier.push_back(start);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.frontier.pop_front()
    }

    fn process(&mut self, source: VertexHandle, pending: &mut VecDeque<Self::Event>) {
        for edge in self.graph.out_edges(source) {
            let (_, target) = self.graph.edge_endpoints(edge).unwrap();

            pending.push_back(Event::Core(TraversalEvent::Examine {
                source,
                target,
                edge,
            }));

            if self.visited.mark(target, true) {
                self.frontier.push_back(target);

                pending.push_back(Event::TreeEdge {
                    source,
                    edge,
                    target,
                });

                pending.push_back(Event::Core(TraversalEvent::Discover { vertex: target }));
            }
        }

        pending.push_back(Event::Core(TraversalEvent::Finish { vertex: source }));
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Core(TraversalEvent),

    TreeEdge {
        source: VertexHandle,
        edge: EdgeHandle,
        target: VertexHandle,
    },
}

pub fn bfs<G, V>(graph: &G, start: VertexHandle) -> Traversal<Bfs<'_, G, V>>
where
    G: Graph,
    V: PropertySet<Key = VertexHandle> + Default,
    G::Topology: EndpointTopology,
{
    Bfs::new(graph, start, V::default())
}

pub fn bfs_vertices<G, V>(graph: &G, start: VertexHandle) -> impl Iterator<Item = VertexHandle>
where
    G: Graph,
    V: PropertySet<Key = VertexHandle> + Default,
    G::Topology: EndpointTopology,
{
    bfs::<G, V>(graph, start).filter_map(|event| {
        if let Event::Core(TraversalEvent::Discover { vertex }) = event {
            Some(vertex)
        } else {
            None
        }
    })
}

pub fn bfs_tree_edges<G, V>(
    graph: &G,
    start: VertexHandle,
) -> impl Iterator<Item = (VertexHandle, EdgeHandle, VertexHandle)>
where
    G: Graph,
    V: PropertySet<Key = VertexHandle> + Default,
    G::Topology: EndpointTopology,
{
    bfs::<G, V>(graph, start).filter_map(|event| {
        if let Event::TreeEdge {
            source,
            edge,
            target,
        } = event
        {
            Some((source, edge, target))
        } else {
            None
        }
    })
}
