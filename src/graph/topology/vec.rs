use crate::graph::{Checked, EdgeHandle, Topology, TopologyMut, VertexHandle};

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

    fn checked_vertex(&self, handle: VertexHandle) -> Option<Checked<VertexHandle>> {
        let graph = self.graph?;
        let checked = Checked::graph_and_generation(handle, graph)?;
        if checked.index() >= self.vertices.len() || !self.vertices[checked.index()] {
            return None;
        }

        Some(checked)
    }

    fn checked_edge(&self, handle: EdgeHandle) -> Option<Checked<EdgeHandle>> {
        let graph = self.graph?;
        let checked = Checked::graph_and_generation(handle, graph)?;
        if checked.index() >= self.edges.len() {
            return None;
        }

        Some(checked)
    }

    fn edge_ref(&self, edge: EdgeHandle) -> Option<(VertexHandle, VertexHandle)> {
        let checked = self.checked_edge(edge)?;
        self.edges.get(checked.index()).and_then(|entry| *entry)
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
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => return Vec::new().into_iter(),
        };

        self.out_edges
            .get(checked.index())
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
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => return Vec::new().into_iter(),
        };

        self.in_edges
            .get(checked.index())
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
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => return Vec::new().into_iter(),
        };

        self.out_edges
            .get(checked.index())
            .cloned()
            .unwrap_or_default()
            .into_iter()
    }

    fn in_edges(&self, v: VertexHandle) -> Self::InEdges<'_> {
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => return Vec::new().into_iter(),
        };

        self.in_edges
            .get(checked.index())
            .cloned()
            .unwrap_or_default()
            .into_iter()
    }

    fn adjacent(&self, v: VertexHandle) -> Self::Adjacent<'_> {
        let checked = match self.checked_vertex(v) {
            Some(checked) => checked,
            None => return Vec::new().into_iter(),
        };

        let mut adjacent = Vec::new();

        if let Some(out) = self.out_edges.get(checked.index()) {
            for edge in out {
                if let Some((_, target)) = self.edge_ref(*edge) {
                    adjacent.push((target, *edge));
                }
            }
        }

        if let Some(inn) = self.in_edges.get(checked.index()) {
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
        let checked = match self.checked_vertex(handle) {
            Some(checked) => checked,
            None => return false,
        };

        self.vertices[checked.index()] = false;

        let mut incident = Vec::new();
        incident.extend(self.out_edges[checked.index()].iter().copied());
        incident.extend(self.in_edges[checked.index()].iter().copied());

        self.out_edges[checked.index()].clear();
        self.in_edges[checked.index()].clear();

        for edge in incident {
            self.remove_edge(edge);
        }

        true
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> bool {
        let checked = match self.checked_edge(handle) {
            Some(checked) => checked,
            None => return false,
        };

        let Some((source, target)) = self
            .edges
            .get_mut(checked.index())
            .and_then(|entry| entry.take())
        else {
            return false;
        };

        if let Some(edges) = self.out_edges.get_mut(source.index()) {
            edges.retain(|current| *current != checked.into_inner());
        }

        if let Some(edges) = self.in_edges.get_mut(target.index()) {
            edges.retain(|current| *current != checked.into_inner());
        }

        true
    }

    fn add_vertex(&mut self, handle: VertexHandle) -> bool {
        let Some(checked) = Checked::generation(handle) else {
            return false;
        };

        if !self.ensure_graph(handle.graph()) {
            return false;
        }

        self.ensure_vertex_capacity(checked.index());

        if self.vertices[checked.index()] {
            return false;
        }

        self.vertices[checked.index()] = true;
        true
    }

    fn add_edge(&mut self, handle: EdgeHandle, source: VertexHandle, target: VertexHandle) -> bool {
        let Some(checked_handle) = Checked::generation(handle) else {
            return false;
        };
        let Some(checked_source) = Checked::generation(source) else {
            return false;
        };
        let Some(checked_target) = Checked::generation(target) else {
            return false;
        };

        if !self.ensure_graph(handle.graph())
            || !self.ensure_graph(source.graph())
            || !self.ensure_graph(target.graph())
            || checked_source.index() >= self.vertices.len()
            || checked_target.index() >= self.vertices.len()
            || !self.vertices[checked_source.index()]
            || !self.vertices[checked_target.index()]
        {
            return false;
        }

        if self.edges.len() <= checked_handle.index() {
            self.edges.resize(checked_handle.index() + 1, None);
        }

        if self.edges[checked_handle.index()].is_some() {
            return false;
        }

        self.edges[checked_handle.index()] = Some((source, target));
        self.out_edges[checked_source.index()].push(handle);
        self.in_edges[checked_target.index()].push(handle);
        true
    }
}
