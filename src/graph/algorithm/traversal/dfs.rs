use std::collections::VecDeque;

use crate::graph::{
    Color, EdgeHandle, EdgeTopology, EndpointTopology, Graph, GraphEdgesExt, GraphEndpointsExt,
    PropertyMap, TraversalEvent, VertexHandle,
};

struct Dfs<'graph, G, C>
where
    G: Graph,
    C: PropertyMap<Key = VertexHandle, Value = Color>,
    G::Topology: EdgeTopology,
{
    graph: &'graph G,
    colors: C,
    stack: Vec<VertexHandle>,
    current: Option<(
        VertexHandle,
        <G::Topology as EdgeTopology>::OutEdges<'graph>,
    )>,
    pending: VecDeque<Event>,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Core(TraversalEvent),

    TreeEdge {
        source: VertexHandle,
        edge: EdgeHandle,
        target: VertexHandle,
    },

    BackEdge {
        source: VertexHandle,
        edge: EdgeHandle,
        target: VertexHandle,
    },

    ForwardEdge {
        source: VertexHandle,
        edge: EdgeHandle,
        target: VertexHandle,
    },

    CrossEdge {
        source: VertexHandle,
        edge: EdgeHandle,
        target: VertexHandle,
    },
}

impl<'graph, G, C: PropertyMap<Key = VertexHandle, Value = Color>> Dfs<'graph, G, C>
where
    G: Graph,
    C: PropertyMap<Key = VertexHandle, Value = Color>,
    G::Topology: EdgeTopology,
{
    pub fn new(graph: &'graph G, start: VertexHandle, mut colors: C) -> Self {
        let mut stack = Vec::new();
        let mut pending = VecDeque::new();

        colors.set_property(start, Color::Gray);
        stack.push(start);
        pending.push_back(Event::Core(TraversalEvent::Discover { vertex: start }));

        Self {
            graph,
            colors,
            stack,
            current: None,
            pending,
        }
    }
}

impl<'graph, G, C> Iterator for Dfs<'graph, G, C>
where
    G: Graph,
    C: PropertyMap<Key = VertexHandle, Value = Color>,
    G::Topology: EndpointTopology,
{
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.pending.pop_front() {
            return Some(event);
        }

        loop {
            if let Some((handle, edges)) = &mut self.current {
                if let Some(edge) = edges.next() {
                    let Some((_, target)) = self.graph.edge_endpoints(edge) else {
                        continue;
                    };

                    self.pending.push_back(Event::Core(TraversalEvent::Examine {
                        source: *handle,
                        edge,
                        target,
                    }));

                    match self.colors.get_property(&target).unwrap_or(&Color::White) {
                        Color::White => {
                            self.colors.set_property(target, Color::Gray);
                            self.stack.push(target);

                            self.pending.push_back(Event::TreeEdge {
                                source: *handle,
                                edge,
                                target,
                            });
                            self.pending
                                .push_back(Event::Core(TraversalEvent::Discover {
                                    vertex: target,
                                }));
                        }

                        Color::Gray => {
                            self.pending.push_back(Event::BackEdge {
                                source: *handle,
                                edge,
                                target,
                            });
                        }

                        Color::Black => {
                            self.pending.push_back(Event::CrossEdge {
                                source: *handle,
                                edge,
                                target,
                            });
                        }
                    }

                    return self.pending.pop_front();
                }

                let vertex = *handle;
                self.current = None;

                self.colors.set_property(vertex, Color::Black);

                return Some(Event::Core(TraversalEvent::Finish { vertex }));
            }

            let handle = self.stack.pop()?;
            let edges = self.graph.out_edges(handle);
            self.current = Some((handle, edges));
        }
    }
}

pub fn dfs<G, C>(graph: &G, start: VertexHandle) -> impl Iterator<Item = Event>
where
    G: Graph,
    C: PropertyMap<Key = VertexHandle, Value = Color> + Default,
    G::Topology: EndpointTopology,
{
    Dfs::new(graph, start, C::default())
}
