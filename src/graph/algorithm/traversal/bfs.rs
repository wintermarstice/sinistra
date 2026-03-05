use std::collections::VecDeque;

use crate::graph::{EdgeHandle, Graph, Policy, Traversal, TraversalEvent, VertexHandle, VisitMap};

pub struct Bfs<'graph, G: Graph, V: VisitMap> {
    graph: &'graph G,
    frontier: VecDeque<VertexHandle>,
    visited: V,
}

impl<'graph, G: Graph, V: VisitMap> Bfs<'graph, G, V> {
    pub fn new(graph: &'graph G, start: VertexHandle, visited: V) -> Traversal<Self> {
        let bfs = Self {
            graph,
            frontier: VecDeque::new(),
            visited,
        };

        Traversal::new(bfs, start)
    }
}

impl<'graph, G: Graph, V: VisitMap> Policy for Bfs<'graph, G, V> {
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

            if self.visited.visit(target) {
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

pub fn bfs<G: Graph, V: VisitMap + Default>(
    graph: &G,
    start: VertexHandle,
) -> Traversal<Bfs<'_, G, V>> {
    Bfs::new(graph, start, V::default())
}

pub fn bfs_vertices<G: Graph, V: VisitMap + Default>(
    graph: &G,
    start: VertexHandle,
) -> impl Iterator<Item = VertexHandle> {
    bfs::<G, V>(graph, start).filter_map(|event| {
        if let Event::Core(TraversalEvent::Discover { vertex }) = event {
            Some(vertex)
        } else {
            None
        }
    })
}

pub fn bfs_tree_edges<G: Graph, V: VisitMap + Default>(
    graph: &G,
    start: VertexHandle,
) -> impl Iterator<Item = (VertexHandle, EdgeHandle, VertexHandle)> {
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
