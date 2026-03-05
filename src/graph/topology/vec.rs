use crate::graph::{EdgeHandle, Topology, TopologyMut, VertexHandle};

#[derive(Debug, Default, Clone)]
pub struct VecTopology {
    graph: Option<u32>,
    vertices: Vec<bool>,
    edges: Vec<Option<(VertexHandle, VertexHandle)>>,
    out_edges: Vec<Vec<EdgeHandle>>,
    in_edges: Vec<Vec<EdgeHandle>>,
}

impl VecTopology {
    pub fn new() -> Self {
        Self::default()
    }

    fn ensure_graph(&mut self, graph: u32) -> bool {
        match self.graph {
            Some(current) => current == graph,
            None => {
                self.graph = Some(graph);
                true
            }
        }
    }

    fn ensure_vertex_capacity(&mut self, index: usize) {
        if self.vertices.len() <= index {
            self.vertices.resize(index + 1, false);
            self.out_edges.resize(index + 1, Vec::new());
            self.in_edges.resize(index + 1, Vec::new());
        }
    }

    fn edge_ref(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)> {
        self.edges.get(edge.index()).and_then(|entry| *entry)
    }
}

impl Topology for VecTopology {
    type Vertices<'a>
        = std::vec::IntoIter<VertexHandle>
    where
        Self: 'a;

    type Edges<'a>
        = std::vec::IntoIter<EdgeHandle>
    where
        Self: 'a;

    type OutNeighbors<'a>
        = std::vec::IntoIter<VertexHandle>
    where
        Self: 'a;

    type InNeighbors<'a>
        = std::vec::IntoIter<VertexHandle>
    where
        Self: 'a;

    type OutEdges<'a>
        = std::vec::IntoIter<EdgeHandle>
    where
        Self: 'a;

    type InEdges<'a>
        = std::vec::IntoIter<EdgeHandle>
    where
        Self: 'a;

    type Adjacent<'a>
        = std::vec::IntoIter<(VertexHandle, EdgeHandle)>
    where
        Self: 'a;

    fn vertices(&self) -> Self::Vertices<'_> {
        self.vertices
            .iter()
            .enumerate()
            .filter_map(|(index, exists)| {
                if !exists {
                    return None;
                }

                Some(VertexHandle::new(index as u64, 1, self.graph.unwrap_or(0)))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn edges(&self) -> Self::Edges<'_> {
        self.edges
            .iter()
            .enumerate()
            .filter_map(|(index, endpoints)| {
                endpoints.map(|_| EdgeHandle::new(index as u64, 1, self.graph.unwrap_or(0)))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn edge_endpoints(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)> {
        self.edge_ref(edge)
    }

    fn out_neighbors(&self, v: VertexHandle) -> Self::OutNeighbors<'_> {
        self.out_edges
            .get(v.index())
            .map(|edges| {
                edges
                    .iter()
                    .filter_map(|edge| self.edge_ref(*edge).map(|(_, target)| target))
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .unwrap_or_default()
    }

    fn in_neighbors(&self, v: VertexHandle) -> Self::InNeighbors<'_> {
        self.in_edges
            .get(v.index())
            .map(|edges| {
                edges
                    .iter()
                    .filter_map(|edge| self.edge_ref(*edge).map(|(source, _)| source))
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .unwrap_or_default()
    }

    fn out_edges(&self, v: VertexHandle) -> Self::OutEdges<'_> {
        self.out_edges
            .get(v.index())
            .cloned()
            .unwrap_or_default()
            .into_iter()
    }

    fn in_edges(&self, v: VertexHandle) -> Self::InEdges<'_> {
        self.in_edges
            .get(v.index())
            .cloned()
            .unwrap_or_default()
            .into_iter()
    }

    fn adjacent(&self, v: VertexHandle) -> Self::Adjacent<'_> {
        let mut adjacent = Vec::new();

        if let Some(out) = self.out_edges.get(v.index()) {
            for edge in out {
                if let Some((_, target)) = self.edge_ref(*edge) {
                    adjacent.push((target, *edge));
                }
            }
        }

        if let Some(inn) = self.in_edges.get(v.index()) {
            for edge in inn {
                if let Some((source, _)) = self.edge_ref(*edge) {
                    adjacent.push((source, *edge));
                }
            }
        }

        adjacent.into_iter()
    }
}

impl TopologyMut for VecTopology {
    fn remove_vertex(&mut self, handle: VertexHandle) -> bool {
        let index = handle.index();
        if index >= self.vertices.len() || !self.vertices[index] {
            return false;
        }

        self.vertices[index] = false;

        let mut incident = Vec::new();
        incident.extend(self.out_edges[index].iter().copied());
        incident.extend(self.in_edges[index].iter().copied());

        self.out_edges[index].clear();
        self.in_edges[index].clear();

        for edge in incident {
            self.remove_edge(edge);
        }

        true
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> bool {
        let index = handle.index();
        let Some((source, target)) = self.edges.get_mut(index).and_then(|entry| entry.take())
        else {
            return false;
        };

        if let Some(edges) = self.out_edges.get_mut(source.index()) {
            edges.retain(|current| *current != handle);
        }

        if let Some(edges) = self.in_edges.get_mut(target.index()) {
            edges.retain(|current| *current != handle);
        }

        true
    }

    fn add_vertex(&mut self, handle: VertexHandle) -> bool {
        if !self.ensure_graph(handle.graph()) {
            return false;
        }

        let index = handle.index();
        self.ensure_vertex_capacity(index);

        if self.vertices[index] {
            return false;
        }

        self.vertices[index] = true;
        true
    }

    fn add_edge(&mut self, handle: EdgeHandle, source: VertexHandle, target: VertexHandle) -> bool {
        if !self.ensure_graph(handle.graph())
            || !self.ensure_graph(source.graph())
            || !self.ensure_graph(target.graph())
            || source.index() >= self.vertices.len()
            || target.index() >= self.vertices.len()
            || !self.vertices[source.index()]
            || !self.vertices[target.index()]
        {
            return false;
        }

        let index = handle.index();
        if self.edges.len() <= index {
            self.edges.resize(index + 1, None);
        }

        if self.edges[index].is_some() {
            return false;
        }

        self.edges[index] = Some((source, target));
        self.out_edges[source.index()].push(handle);
        self.in_edges[target.index()].push(handle);
        true
    }
}
