use std::collections::VecDeque;

use crate::graph::{
    Color, ColorMap, EdgeHandle, EdgeTopology, Graph, GraphEdgesExt, GraphEndpointsExt,
    VertexHandle,
};

struct Dfs<'graph, G: Graph, C: ColorMap> {
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
    DiscoverVertex(VertexHandle),
    ExamineEdge(VertexHandle, EdgeHandle, VertexHandle),
    TreeEdge(VertexHandle, EdgeHandle, VertexHandle),
    BackEdge(VertexHandle, EdgeHandle, VertexHandle),
    ForwardEdge(VertexHandle, EdgeHandle, VertexHandle),
    CrossEdge(VertexHandle, EdgeHandle, VertexHandle),
    FinishVertex(VertexHandle),
}

impl<'graph, G: Graph, C: ColorMap> Dfs<'graph, G, C> {
    pub fn new(graph: &'graph G, start: VertexHandle, mut colors: C) -> Self {
        let mut stack = Vec::new();
        let mut pending = VecDeque::new();

        colors.set_color(start, Color::Black);
        stack.push(start);
        pending.push_back(Event::DiscoverVertex(start));

        Self {
            graph,
            colors,
            stack,
            current: None,
            pending,
        }
    }
}

impl<'graph, G: Graph, C: ColorMap> Iterator for Dfs<'graph, G, C> {
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

                    self.pending
                        .push_back(Event::ExamineEdge(*handle, edge, target));

                    match self.colors.color(target) {
                        Color::White => {
                            self.colors.set_color(target, Color::Gray);
                            self.stack.push(target);

                            self.pending
                                .push_back(Event::TreeEdge(*handle, edge, target));
                            self.pending.push_back(Event::DiscoverVertex(target));
                        }

                        Color::Gray => {
                            self.pending
                                .push_back(Event::BackEdge(*handle, edge, target));
                        }

                        Color::Black => {
                            self.pending
                                .push_back(Event::CrossEdge(*handle, edge, target));
                        }
                    }

                    return self.pending.pop_front();
                }

                let vertex = *handle;
                self.current = None;

                self.colors.set_color(vertex, Color::Black);

                return Some(Event::FinishVertex(vertex));
            }

            let handle = self.stack.pop()?;
            let edges = self.graph.out_edges(handle);
            self.current = Some((handle, edges));
        }
    }
}

pub fn dfs<G: Graph, C: ColorMap + Default>(
    graph: &G,
    start: VertexHandle,
) -> impl Iterator<Item = Event> {
    Dfs::new(graph, start, C::default())
}
